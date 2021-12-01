CC=clang++
CFLAGS=--std=c++1z

part1.o: part1.cpp
	$(CC) -c -o part1.o $(CFLAGS) part1.cpp

part1: part1.o
	$(CC) -o part1 $(CFLAGS) part1.o

part2.o: part2.cpp
	$(CC) -c -o part2.o $(CFLAGS) part2.cpp

part2: part2.o
	$(CC) -o part2 $(CFLAGS) part2.o

clean:
	rm -f part1 part2 *.o

all: part1 part2

run1: part1
	./part1 input.txt

run2: part2
	./part2 input.txt
