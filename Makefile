.PHONY: test

# Written for Fedora
install-dev:
	# Cross-compile support for Windows
	sudo dnf install mingw64-gcc-c++
	
	# Rust-cross is required because of the sad state of Fedora's ARM
	# toolchain
	cargo install cross --git https://github.com/cross-rs/cross

build:
	cargo build

test:
	cat test/test.log | TZ='America/New York' cargo run \
		> test/actual-output.log
	diff test/expected-output.log test/actual-output.log \
		&& echo -e '\n\x1b[38;2;68;221;68mPASS\x1b[0m' \
		|| echo -e '\n\x1b[38;2;255;0;0mFAIL\x1b[0m'

release:
	cargo clean
	
	# Cross builds must be run first. They will fail if any non-cross builds
	# have been run since the last cargo clean
	cross build --release --target armv7-unknown-linux-musleabihf
	cross build --release --target aarch64-unknown-linux-musl
	
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-pc-windows-gnu
	
	@echo -e '\n\x1b[38;2;236;182;74mRemember to push a new tag to GitHub and create a release\x1b[0m'

clean:
	cargo clean
	rm test/actual-output.log
