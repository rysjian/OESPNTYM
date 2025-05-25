LOGS= > jupyter.log&
ARGS=start-notebook.py --ServerApp.token=[''] --ServerApp.ip=['192.168.1.248']
IMAGE=quay.io/jupyter/base-notebook:2025-05-19
DOCKER=sudo docker run --rm
PORT=-p 8889:8888
VOLUME=-v "$$HOME/projects/:/home/jovyan/work/"

jupyter:
	$(DOCKER) $(PORT) $(VOLUME) $(IMAGE) $(ARGS) $(LOGS)
