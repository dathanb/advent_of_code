.PHONY: docker_build interactive

docker_build:
	docker build -t day12 .

interactive: docker_build
	docker run --rm -it --mount type=bind,source=$(shell pwd),target=/app day12
