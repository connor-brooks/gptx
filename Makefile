build:
	@echo "* building gptx" && cargo build --release

clean:
	@echo "* cleaning gptx" && cargo clean

install:
	@echo "* checking for ~/.local/bin" && mkdir -p ${HOME}/.local/bin
	@echo "* installing gptx to ~/.local/bin" && cp target/release/gptx ${HOME}/.local/bin
	@echo "* installing config to ~/.config/gptx" mkdir -p ${HOME}/.config/gptx && cp example/config.toml ${HOME}/.config/gptx
	@echo "* gptx installed! Please ensure that ~/.local/bin is PATH"

uninstall:
	@echo "uninstalling gptx"
	@rm ~/.local/bin/gptx
