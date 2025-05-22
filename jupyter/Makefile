jupyter:
	sudo docker run --rm \
		-p 8889:8888 \
		-v "~/projects:/home/jovyan/work" \
		quay.io/jupyter/base-notebook:\
		2025-05-19 \
		start-notebook.py \
		--NotebookApp.token='my-token' \
		> jupyter.log&
