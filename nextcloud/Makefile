nextcloud:
	sudo docker run \
		--init \
		--sig-proxy=false \
		--name nextcloud-aio-mastercontainer \
		--restart always \
		--publish 80:80 \
		--publish 8080:8080 \
		--publish 8443:8443 \
		--volume nextcloud_aio_mastercontainer:/mnt/docker-aio-config \
		--volume /var/run/docker.sock:/var/run/docker.sock:ro \
		ghcr.io/nextcloud-releases/all-in-one:20250512_082954
