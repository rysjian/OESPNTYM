DOCKER=sudo docker
NAME=alice
PORT=-p 9050:9050
VOLUME=-v "$$PWD/data:/etc/tor"

build:
	$(DOCKER) build --build-arg TARGETARCH=$$arch\
		-t $(NAME) . 2>&1 | tee build.log

start:
	$(DOCKER) run --rm $(VOLUME) $(PORT) $(NAME) \
		| tee ./start.log &
