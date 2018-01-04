interactive: docker_build
	docker run -it --rm --mount type=bind,source=$(shell pwd),target=/app day13 /bin/bash

docker_build:
	docker build -t day13 .

testpart1: INPUT=testinput
testpart1: runpart1

part1: INPUT=input
part1: runpart1

runpart1:
	clear
	gst part1.st < $(INPUT)

testpart2: INPUT=testinput
testpart2: runpart2

part2: INPUT=input
part2: runpart2

runpart2:
	clear
	gst part2.st < $(INPUT)
