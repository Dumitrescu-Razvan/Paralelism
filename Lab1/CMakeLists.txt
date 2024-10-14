cmake_minimum_required(VERSION 3.30.3)

# Set the project name

project (Lab1)

# Add all source files 
set(SOURCES
    main.cpp
    Domain/Variable.cpp
)

# Add the executable
add_executable(Lab1 ${SOURCES})