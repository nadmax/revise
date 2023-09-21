SHELL = bash

TARGET = target/debug/editor

RELEASE ?=

ifeq ($(release), yes)
	RELEASE = --release
endif

build:
	cargo build $(RELEASE)

check:
	@cargo check $(RELEASE)

help:
	@echo "build (release=yes)     Compile editor project"
	@echo "check (release=yes)     Check warning and errors"
	@echo "help                    Show this helper"
	@echo "run (release=yes)       Compile and run editor"
	@echo "valgrind (release=yes)  Compile and run valgrind to check memory leaks"

run:
	@cargo run $(RELEASE)

valgrind:
	@cargo build $(RELEASE) && valgrind --leak-check=full --show-leak-kinds=all -s $(TARGET)

.PHONY: build check help run valgrind