build:
	sudo docker build -t vsc2005 .

tunnel:
	sudo docker run --rm \
		-v ~/projects:/workspace \
		-v ~/.vscode:/root/.vscode \
		-p 8080:8080 \
		vsc2005 tunnel \
		--name vsc20052025 \
		--log warn > ./tunnel.log&
