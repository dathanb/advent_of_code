.PHONY: run docker

run: docker
	docker run dathan/day1

interactive: docker
	./run
	# docker run --rm --name day1 -it --mount type=bind:source="$(pwd)":target=/day1 dathan/day1 /bin/bash

docker:
	docker build -t dathan/day1 .
