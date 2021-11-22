CXX       := g++
CXX_FLAGS := -std=c++17 -ggdb 

LIB     := lib
SRC     := src
INCLUDE := src

# LIBRARIES   :=
EXECUTABLE  := main


all: 
	$(CXX) -shared -fpic $(CXX_FLAGS) $^ -o ../$(LIB)/libcasual_cpp.so casual_cpp.cpp
	$(CXX) -shared -fpic $(CXX_FLAGS) $^ -o ../$(LIB)/libcreate_new_file.so create_new_file.cpp
	$(CXX) -shared -fpic $(CXX_FLAGS) $^ -o ../$(LIB)/libloop.so loop.cpp
	$(CXX) -shared -fpic $(CXX_FLAGS) $^ -o ../$(LIB)/libremove_file.so remove_file.cpp
	$(CXX) -shared -fpic $(CXX_FLAGS) $^ -o ../$(LIB)/libsimple_print.so simple_print.cpp
	$(CXX) -shared -fpic $(CXX_FLAGS) $^ -o ../$(LIB)/libinput_output.so input_output.cpp

run: clean all
	clear

clean:
	rm -rf $(BIN)/*