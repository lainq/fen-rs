INSTALL_DIR := /usr/lib/
CC := gcc
CFLAGS := -Wall -Wextra -pedantic -fPIC -shared

TARGET := fenc
LIB_NAME := lib$(TARGET).so

build:
	$(CC) $(CFLAGS) ./lib/fenc.h -o ./fenc.o -s
	$(CC) $(CFLAGS) ./fenc.o -o LIB_NAME


# 	gcc -shared ./fenc.o
