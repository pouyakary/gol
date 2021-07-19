
play: build run

run:
	./out/gol

build:
	rm -rf ./out
	mkdir out
	rustc -o ./out/gol ./source/main.rs