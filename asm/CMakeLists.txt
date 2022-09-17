cmake_minimum_required(VERSION 3.16.0 FATAL_ERROR)
project(asm-x86-adventures C ASM)

# Each different platform might have a different instruction set to target string
if(WIN32)
	set(target_os "windows")
endif()

if(${CMAKE_HOST_SYSTEM_PROCESSOR} STREQUAL "AMD64")
	set(target_arch "x86-64")
endif()

set(EXAMPLE_LIST 
	01-hello-world)

# Add the examples into a project
foreach(example ${EXAMPLE_LIST})
	message(STATUS "    Adding example : ${example}")
	file(GLOB sources_list "${CMAKE_CURRENT_LIST_DIR}/${example}/*.asm")
	add_executable(${example} ${sources_list})
	
endforeach()