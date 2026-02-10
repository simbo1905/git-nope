.PHONY: all build clean test help

OUTPUT_DIR := dist
BINARY_NAME := git-nope

all: build

help:
	@echo "git-nope Makefile"
	@echo ""
	@echo "Targets:"
	@echo "  all     - Build the release binary (default)"
	@echo "  build   - Build the release binary and place it in dist/"
	@echo "  clean   - Remove build artifacts and non-hidden files in dist/"
	@echo "  test    - Run unit tests"
	@echo "  help    - Show this help message"
	@echo ""
	@echo "Build Output:"
	@echo "  Binaries are placed in '$(OUTPUT_DIR)/'. Names include commit SHA."
	@echo "  Dirty builds also include a file hash suffix."

clean:
	cargo clean
	@# Clean non-hidden files in output dir, but keep dir and hidden files
	@if [ -d "$(OUTPUT_DIR)" ]; then \
		find "$(OUTPUT_DIR)" -mindepth 1 -maxdepth 1 ! -name ".*" -exec rm -rf {} +; \
	fi

build:
	@mkdir -p $(OUTPUT_DIR)
	RUSTFLAGS="-D warnings" cargo build --release
	@# Determine SHA and Status
	@SHA=$$(git rev-parse --short HEAD); \
	STATUS=$$(git status --porcelain); \
	if [ -z "$$STATUS" ]; then \
		FINAL_NAME="$(BINARY_NAME)_$${SHA}"; \
		echo "Tree is clean. Creating releasable binary: dist/$${FINAL_NAME}"; \
	else \
		if command -v sha256sum >/dev/null 2>&1; then \
			RAW_HASH=$$(sha256sum target/release/$(BINARY_NAME) | awk '{print $$1}'); \
		else \
			RAW_HASH=$$(shasum -a 256 target/release/$(BINARY_NAME) | awk '{print $$1}'); \
		fi; \
		FILE_HASH=$$(echo $$RAW_HASH | cut -c 1-7); \
		FINAL_NAME="$(BINARY_NAME)_$${SHA}_$${FILE_HASH}"; \
		echo "Tree is dirty. Creating binary with file-hash: dist/$${FINAL_NAME}"; \
	fi; \
	mv target/release/$(BINARY_NAME) "$(OUTPUT_DIR)/$${FINAL_NAME}"; \
	echo "Running help for built binary:"; \
	"$(OUTPUT_DIR)/$${FINAL_NAME}" -h || true; \
	for applet in GitAdd GitAddAll GitAddDot GitRm GitCommit GitLog GitAudit GitChanges; do \
		ln -sf "$${FINAL_NAME}" "$(OUTPUT_DIR)/$$applet"; \
	done

test:
	cargo test
