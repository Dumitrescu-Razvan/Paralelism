cmake_minimum_required(VERSION 3.30.3)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Set the project name

project (Lab1)

# Add all source files 
set(SOURCES
    main.cpp
    Domain/Variable.cpp
    Checker.cpp
)

# Add the executable
add_executable(Lab1 ${SOURCES})