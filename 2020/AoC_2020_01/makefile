#Set Components
#------------------
NAME = ReportRepair
COMMONFLAGS = -std=c++17
OBJS = *.cpp
HDRS = *.h
CC = clang++

all: $(NAME)
	$(CXX) $(OBJS) $(COMMONFLAGS) -o $(NAME)

test1: $(NAME)
	./$(NAME) exampleInput.txt

test2: $(NAME)
	./$(NAME) input.txt
