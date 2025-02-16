.PHONY: test

build:
	cargo build

test:
	cat test/test.log | TZ='America/New York' cargo run \
		> test/actual-output.log
	diff test/expected-output.log test/actual-output.log \
		&& echo -e '\n\x1b[38;2;68;221;68mPASS\x1b[0m' \
		|| echo -e '\n\x1b[38;2;255;0;0mFAIL\x1b[0m'

release:
	cargo build --release
	@echo -e '\n\x1b[38;2;236;182;74mRemember to push a new tag to GitHub and create a release\x1b[0m'

clean:
	cargo clean
	rm test/actual-output.log
