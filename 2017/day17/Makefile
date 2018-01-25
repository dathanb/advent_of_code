interactive: docker_build
	docker run -it --rm --mount type=bind,source=$(shell pwd),target=/app day17 /bin/bash

docker_build:
	docker build -t day17 .

part1: docker_build
	docker run -it --mount type=bind,source=$(shell pwd),target=/app day17 lua part1.lua

part2: docker_build
	docker run -it --mount type=bind,source=$(shell pwd),target=/app day17 lua part2.lua
