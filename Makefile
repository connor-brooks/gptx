install:
	cargo build --release;
	mkdir -p ${HOME}/.local/bin
	cp target/release/gptx ${HOME}/.local/bin
	mkdir -p ${HOME}/.config/gptx
	cp example/config.toml ${HOME}/.config/gptx
	echo "gptx installed... Please ensure that ~/.local/bin is PATH"
