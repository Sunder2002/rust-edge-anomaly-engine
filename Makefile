.PHONY: setup init fmt clippy

# The 'setup' command. 
# We added '|| true' to the update so that if a non-essential repo (like Yarn) 
# fails, the script continues to install our critical C++ and AI libraries.
setup:
	sudo apt-get update || true
	sudo apt-get install -y \
		build-essential \
		cmake \
		git \
		libclang-dev \
		libopencv-dev \
		clang \
		pkg-config

# Initializes the workspace structure if it doesn't exist.
init:
	mkdir -p crates/common/src crates/vision/src crates/reasoning/src crates/engine/src crates/api/src
	# 'cargo init' sets up the folder as a Rust project.
	cd crates/common && cargo init --lib || true
	cd crates/vision && cargo init --lib || true
	cd crates/reasoning && cargo init --lib || true
	cd crates/engine && cargo init --bin || true
	cd crates/api && cargo init --bin || true

# Fortune 500 Standard: Always keep code clean and readable.
fmt:
	cargo fmt --all

# Fortune 500 Standard: Linting. This catches bugs before they happen.
clippy:
	cargo clippy --workspace -- -D warnings