install:
	cargo build --release;
	cp target/release/gptx ${HOME}/.local/bin
