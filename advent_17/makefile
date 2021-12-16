CC=g++
CFLAGS=-I.
DEPS = # .h
OBJ = main.o # .c acabados en .o

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

advent: $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS)

.PHONY: clean

clean:
	rm *.o
	rm advent