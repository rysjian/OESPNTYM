build:
	sudo docker build -t vscode-alpine-arm64

run:
	sudo docker run -d -p 3000:3000 -v "$PWD:/workspace" -v "$HOME/.vscode-server:/root/.vscode-server" vscode-alpine-arm64
