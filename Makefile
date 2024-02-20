install:
	cargo build --release;
	cp target/release/tgpt /home/c/.local/bin
