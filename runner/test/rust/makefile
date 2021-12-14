CXX       := rustc
CXX_FLAGS :=

LIB     := lib
SRC     := src
INCLUDE := src

# LIBRARIES   :=
EXECUTABLE  := main


all: 
	@ for f in $(shell ls *.rs); do $(CXX) $(CXX_FLAGS) $^ -o ../$(LIB)/$${f%.*} $${f}; done;

run: clean all
	clear

clean:
	rm -rf $(BIN)/*