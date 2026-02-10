# Exploratory Test Suites for git-nope

## Overview

These test suites are designed to be executed by AI coding agents (or humans) to manually verify `git-nope` functionality.

## The Oath

You job is to find reason to refuse to issue a PASS by being a perfectionist. 

You MUST NEVER script testing. Automating unit test is NOT EXPLORATORY TESTING. The purpose of these test is never release software with any documentation bugs in the docs, man pages, -h/--help, or any gaps defects in the test specs themselves. THIS MEANS IT IS NOT THE SOFTWARE THAT IS UNDER TEST IT IS THE ENTIRE VALUE PROPOSITION TO THE END USERS WHICH START WITH WORLD CLASS DOCUMENTATION. 

This means that if you struggle to use the tool THAT IS A DOCUMENTATION BUG not something to be hidden. If you find the test suite does not match the code THAT IS A TEST SUITE BUG. If you find things doe not align to the design docs in doc/* then THAT IS AN ERROR OF OMMISSION that needs to be reported. To repeat NO TEST SUITE CAN POSSIBLE PASS UNLESS EVERYTHING IS PERFECT. This test suite is run during development so failures for bugs in the code and partial passes due to missing feature IS THE NORM and a full pass SHOULD BE RATE AND QUESTIONED as you will not be told whether the user thinks they are doing final release testing or smoke testing half way through a big refactor. 

This means you must take the software quality oath:

> As exploratory tester I pledge allegence to the end user. I will defend the team against disappointing the user with poor documentation, misleading documentation, errors of commission, errors of ommission, and software defects. I will only issue a partial pass for working software. I will only begrudingly give a full pass for a suite after I can evidence the docs/* design, is aligned to the test suite, is aligned to the man pages, the -h/--help, the documentation standard and everything is perfectly aligned. The team thanks me for finding problems and will be betrayed if I let any cognative burden be passed to the end user. 

Do not be like the sicophantic LLM who declares any and all software production ready. That is betrayal of the oath. 

## Philosophy

1.  **Documentation is the spec** - The tool must behave exactly as described in `docs/git-nope.md`.
2.  **Isolation** - Tests run in temporary directories in `.tmp/` in a unique date_time folder. 
3.  **Consistent State** - We clone a known "demo" repo to ensure predictable starting conditions such as https://github.com/simbo1905/agt-demo-repo
4.  **Consistent Reporting** - The results should be a single file, in the .tmp datetime folder, as datetime _report.md see standards below.

## Suites

Whenever a new test suite is addd this table MUST be updated. 

| Suite | File | Focus |
|-------|------|-------|
| 0 | `tests/exploratory/suite0_documentation_audit.md` | Build verification, help flags, version checks, and man page consistency. |
| 1 | `tests/exploratory/suite1_basic_ops.md` | Basic operations: Applets, `git nope`, and Refusal Mode. |
| 3 | `tests/exploratory/suite3_GitRm_glob.md` | GitRm aggressive deletion with glob patterns and guardrails. |

## Running a Suite



1.  Read the suite Markdown file (e.g., `tests/exploratory/suite1_basic_ops.md`).
2.  Follow the **Setup** instructions carefully (creating `.tmp` dirs, cloning repos).
3.  Read the **Scenarios**.
4.  Create a timestamp folder in .tmp and try to use the software as self documented by `-h/--help` to achive the scenario and FAIL THE TEST for software bugs or MISSING FEATURES NOT YET BUILT WHICH IS EXPECTED SUCH THAT THE SOFTWARE CANNOT ACHIVE THE SCENARIO. 
5.  If the software as selt documented manages to make the scenario possible THIS IS NOT A PASS IT IS A PARTIAL PASS you need to attempt to not mark it as a pass by finding fault with the 

## Cleanup

No cleanup is allowed! The amount of disk used is trivial, the user usese worktrees that are GCed on merge, so there is nothing to be added by destroying test evidence. In particular tests are not perfect and seeing a history of tests on disk across worktrees is an excellent way to track down odd bugs and regresions. So YOU MUST NOT DELETE TEST FOLDERS. 
