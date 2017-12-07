.PHONY: run docker

docker:
	docker build -t dathan/day1 .

run: docker
	docker run --rm --name day1 -it --mount type=bind,source="$(shell pwd)",target=/day1 dathan/day1 /bin/bash

