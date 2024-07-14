CC=gcc
CFLAGS=-g -Wall -O3 -pipe

NAME=bfic
FILES=main.o stack.o

bfic: $(FILES)
	$(CC) $(CFLAGS) $(FILES) -o $(NAME)
clean:
	rm *.o
