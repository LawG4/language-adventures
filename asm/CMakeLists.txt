cmake_minimum_required(VERSION 3.16.0 FATAL_ERROR)
project(asm-adventures C ASM)

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

	# Find the name of the example without the number 
	string(REPLACE "-" ";" split_name "${example}")
	list(REMOVE_AT split_name 0)
	string(REPLACE ";" "-" name "${split_name}")
	message(STATUS "    Adding example : ${name}")

	# Find all of the sources and add the executable
	# We don't need to do anything special
	file(GLOB sources_list "${CMAKE_CURRENT_LIST_DIR}/${example}/*.asm")
	add_executable(asm_${name} ${sources_list})
	
endforeach()