cmake_minimum_required(VERSION 3.10)
project(OrderRouter)

set(CMAKE_CXX_STANDARD 17)

add_library(orderrouter SHARED order_router.cpp)
