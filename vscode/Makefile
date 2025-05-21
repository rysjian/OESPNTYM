build:
	sudo docker build -t vscode-alpine-arm64 .

tunnel:
	sudo docker run vscode-alpine-arm64 tunnel --name vsc

tunnel2:
	sudo docker run -d -v ~/projects:/workspace -v ~/.vscode:/root/.vscode -p 8080:8080 vscode-alpine-arm64 tunnel --name vsc
