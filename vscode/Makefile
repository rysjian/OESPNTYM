build:
	sudo docker build -t vsc2005 .

tunnel2:
	sudo docker run --rm vscode-alpine-arm64 tunnel --name vsc

tunnel:
	sudo docker run --rm \
		-v ~/projects:/workspace \
		-v ~/.vscode:/root/.vscode \
		-p 8080:8080 \
		vsc2005 tunnel \
		--name vsc20052025 \
		--log warn > ./tunnel.log&

fenics:
	sudo docker run -ti \
		-p 127.0.0.1:8000:8000 \
		-v $(pwd):/root/shared \
		-w /root/shared \
		ghcr.io/scientificcomputing/\
		fenics-gmsh:2024-05-30

jupyter:
	sudo docker run --rm \
		-p 8889:8888 \
		quay.io/jupyter/base-notebook \
		start-notebook.py \
		--NotebookApp.token='my-token'
	#docker run --rm -p 8889:8888 -v "$(pwd):/home/jovyan/work" quay.io/jupyter/base-notebook start-notebook.py --NotebookApp.token='my-token'
	#docker run --rm -p 8889:8888 -v jupyter-data:/home/jovyan/work quay.io/jupyter/base-notebook start-notebook.py --NotebookApp.token='my-token'

