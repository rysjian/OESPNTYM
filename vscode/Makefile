DOCKER=sudo docker
NAME=vsc2005
PORT=-p 8080:8080
DIR=/root/.vscode
VOLUME=-v $$HOME/projects:/workspace \
	-v $$HOME/.vscode:$(DIR) \

build:
	$(DOCKER) build -t $(NAME) .

tunnel:
	$(DOCKER) run --rm $(VOLUME) $(PORT) $(NAME) \
		--cli-data-dir $(DIR)/cli --log error \
		tunnel --name $(NAME)2025 \
		--accept-server-license-terms \
		> ./tunnel.log&
