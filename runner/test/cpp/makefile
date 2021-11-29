CXX       := g++
CXX_FLAGS := -std=c++17 -ggdb 

LIB     := lib
SRC     := src
INCLUDE := src

# LIBRARIES   :=
EXECUTABLE  := main


all: 
	@ for f in $(shell ls *.cpp); do $(CXX) -shared -fpic $(CXX_FLAGS) $^ -o ../$(LIB)/lib$${f%.*}.so $${f}; done;

run: clean all
	clear

clean:
	rm -rf $(BIN)/*