.PHONY: part1 build_part1 test_part1 build

interactive: docker_build
	docker run -it --rm --mount type=bind,source=$(shell pwd),target=/app day19 /bin/bash

docker_build:
	docker build -t day19 .

test_part1: INPUT_FILE=testinput
test_part1: run_part1

part1: INPUT_FILE=input
part1: run_part1

run_part1: build
	node build/part1.js < $(INPUT_FILE)

build:
	tsc -p .

test_part2: INPUT_FILE=testinput
test_part2: run_part2

part2: INPUT_FILE=input
part2: run_part2

test_part2: build
	node build/part2.js < testinput

part2: build
	node part2.js < input

clean:
	rm -rf build/*
