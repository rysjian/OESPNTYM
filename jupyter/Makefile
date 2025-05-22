ARGS=start-notebook.py --NotebookApp.token='my-token'
LOGS=jupyter.log
IMAGE=quay.io/jupyter/base-notebook:2025-05-19
DOCKER=sudo docker run --rm
PORT=-p 8889:8888
VOLUME=-v "~/projects:/home/jovyan/work"

jupyter:
	$(DOCKER) $(PORT) $(VOLUME) \
		$(IMAGE) $(ARGS) > $(LOGS) &
