use anyhow::{Context, Result};
use gix::bstr::{BStr, ByteSlice};
use gix::object::tree::EntryKind;
use gix::Repository;
use gix_index::entry::{Flags, Mode, Stage};
use gix_index::File as IndexFile;

pub fn run(args: &[String]) -> Result<()> {
    let repo = gix::discover(".")?;
    let message = parse_commit_message(args)?;

    let mut index = repo
        .open_index()
        .context("Failed to open index (nothing staged?)")?;

    let tree_id = write_tree_from_index(&repo, &mut index)?;

    let mut head = repo.head()?;
    if head.is_detached() {
        anyhow::bail!("Detached HEAD is not supported for GitCommit");
    }

    let ref_name = head
        .referent_name()
        .ok_or_else(|| anyhow::anyhow!("No branch reference for GitCommit"))?
        .to_owned();
    let ref_name_str = ref_name
        .as_bstr()
        .to_str()
        .context("Branch name is not valid UTF-8")?;

    let parents = if head.is_unborn() {
        Vec::new()
    } else {
        vec![head.peel_to_commit_in_place()?.id]
    };

    let (name, email) = signature_from_config(&repo);
    let signature = gix::actor::SignatureRef {
        name: BStr::new(name.as_bytes()),
        email: BStr::new(email.as_bytes()),
        time: gix::date::Time::now_local_or_utc(),
    };

    let commit_id = repo.commit_as(
        signature,
        signature,
        ref_name_str,
        &message,
        tree_id,
        parents,
    )?;
    println!("Created commit {commit_id}");
    Ok(())
}

fn parse_commit_message(args: &[String]) -> Result<String> {
    let mut messages = Vec::new();
    let mut iter = args.iter().skip(1).peekable();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-m" | "--message" => {
                let msg = iter
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("Expected message after {arg}"))?;
                messages.push(msg.to_string());
            }
            _ if arg.starts_with('-') => anyhow::bail!("Unsupported GitCommit flag: {arg}"),
            _ => {}
        }
    }

    if messages.is_empty() {
        anyhow::bail!("GitCommit requires -m/--message");
    }

    Ok(messages.join("\n\n"))
}

fn signature_from_config(repo: &Repository) -> (String, String) {
    let config = repo.config_snapshot();
    let name = config
        .string("user.name")
        .map(|s| s.to_string())
        .unwrap_or_else(|| "agt".to_string());
    let email = config
        .string("user.email")
        .map(|s| s.to_string())
        .unwrap_or_else(|| "agt@local".to_string());
    (name, email)
}

fn write_tree_from_index(repo: &Repository, index: &mut IndexFile) -> Result<gix::ObjectId> {
    let empty_tree_id = repo.write_object(gix_object::Tree::empty())?.detach();
    let mut editor = repo.edit_tree(empty_tree_id)?;

    for entry in index.entries().iter() {
        if entry.stage() != Stage::Unconflicted {
            anyhow::bail!("Cannot commit with unmerged paths present");
        }
        if entry.flags.contains(Flags::REMOVE) {
            continue;
        }

        let kind = match entry.mode {
            Mode::FILE => EntryKind::Blob,
            Mode::FILE_EXECUTABLE => EntryKind::BlobExecutable,
            Mode::SYMLINK => EntryKind::Link,
            Mode::COMMIT => EntryKind::Commit,
            _ => continue,
        };

        let path = entry.path(index);
        editor.upsert(path, kind, entry.id)?;
    }

    Ok(editor.write()?.detach())
}
