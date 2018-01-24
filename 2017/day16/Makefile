interactive: docker_build
	docker run -it --rm --mount type=bind,source=$(shell pwd),target=/app day16 /bin/bash

docker_build:
	docker build -t day16 .

testpart1: INPUT=testinput
testpart1: runpart1

part1: INPUT=input
part1: runpart1

build_part1:
	clear
	kotlinc src/part1.kt -include-runtime -d build/part1.jar

runpart1: build_part1
	java -jar build/part1.jar

testpart2: INPUT=testinput
testpart2: runpart2

part2: INPUT=input
part2: runpart2

build_part2:
	clear
	kotlinc src/part2.kt -include-runtime -d build/part2.jar

runpart2: build_part2
	clear
	java -jar build/part2.jar
