install:
	cargo build --release;
	cp target/release/tgpt ${HOME}/.local/bin
