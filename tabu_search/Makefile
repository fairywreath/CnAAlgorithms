# Compiler and flags
CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra

# Source files and object files
SRCS := $(wildcard *.cpp)
OBJS := $(SRCS:.cpp=.o)

# Target executable
TARGET = tabu_simple 

# Default target
all: $(TARGET)

# Compile source files into object files
%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Link object files into target executable
$(TARGET): $(OBJS)
	$(CXX) $(CXXFLAGS) $^ -o $@

# Clean the generated files
clean:
	rm -f $(OBJS) $(TARGET)
