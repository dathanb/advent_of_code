.PHONY: part1 build_part1 test_part1

interactive: docker_build
	docker run -it --rm --mount type=bind,source=$(shell pwd),target=/app day18 /bin/bash

docker_build:
	docker build -t day18 .

test_part1: build_part1
	build/part1 < testinput

part1: build_part1
	build/part1 < input

build_part1:
	gcc part1.c -march=native -m64 -o build/part1
