jupyter:
	sudo docker run --rm \
		-p 8889:8888 \
		-v "~/projects:/home/jovyan/work" \
		quay.io/jupyter/base-notebook \
		start-notebook.py \
		--NotebookApp.token='my-token' \
		> jupyter.log&
