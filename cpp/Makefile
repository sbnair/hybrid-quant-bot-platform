CXX = g++
CXXFLAGS = -std=c++17 -fPIC
TARGET = liborderrouter.so
SRCS = order_router.cpp

$(TARGET): $(SRCS)
	$(CXX) $(CXXFLAGS) -shared -o $@ $^

clean:
	rm -f $(TARGET)
