cmake_minimum_required(VERSION 3.16.0 FATAL_ERROR)
project(jakt-adventures CXX)

set(EXAMPLE_LIST 
	01-hello-world)

# We only want to critically fail if something goes wrong in the cmake itself or if the user 
# is only looking at this language, or else this makes uses need to install all languages to 
# look at just a few!
if(PROJECT_IS_TOP_LEVEL)
	set(msg_level FATAL_ERROR)
else()
	set(msg_level STATUS "Couldn't add Jakt language due to the following error : ")
endif()

# If the user is using visual studio as a generator then this is going to through build errors
# Using the msvc tag isn't enough as clang_cl gets identified as MSVC by cmake
if(${CMAKE_GENERATOR} MATCHES "Visual Studio")
	message(${msg_level} "Can't use the visual studio generator with Jakt, make sure you use ninja and clang")
endif()

# Find the jakt compiler and the runtime directory
find_program(jakt_cmd NAMES jakt jakt.exe Jakt Jakt.exe HINTS ${CMAKE_JAKT_PATH})
if("${jakt_cmd}" STREQUAL "jakt_cmd-NOTFOUND")
	message(${msg_level} "Was not able to find the Jakt compiler! You can either add the directory "
		"containing the compiler to the PATH environment variable. Or you can "
		"provide CMake a hint to finding that directory by passing "
		"-DCMAKE_JAKT_PATH=\"<path to jakt>\" on the command line.")
	return()
endif()

get_filename_component(jakt_compiler_dir "${jakt_cmd}" DIRECTORY)
set(jakt_runtime_dir "${jakt_compiler_dir}/../include/runtime")
set(jakt_share_dir "${jakt_compiler_dir}/../share/Jakt")

if(NOT EXISTS ${jakt_runtime_dir})
	message(${msg_level} "Could not find the runtime directory at \"${jakt_runtime_dir}\"")
	return()
endif()

if(NOT EXISTS ${jakt_share_dir})
	message(${msg_level} "Could not find the share directory at \"${jakt_share_dir}\"")
	return()
endif()

# The build of the jakt compiler provides a "share" directory, it's an exported target 
# from the cmake build and can be used as a drop in to let people use jakt in a project
include("${jakt_share_dir}/JaktConfig.cmake")

# Now we can iterate through the example files
foreach(example ${EXAMPLE_LIST})
	
	# Fetch all of the jakt files in this directory and figure out the name of the 
	# main file by chopping off the example number off the name
	file(GLOB_RECURSE example_source_list RELATIVE "${CMAKE_CURRENT_LIST_DIR}" "${CMAKE_CURRENT_LIST_DIR}/${example}/*.jakt")
	string(REPLACE "-" ";" split_name "${example}")
	list(REMOVE_AT split_name 0)
	string(REPLACE ";" "-" name "${split_name}")
	message(STATUS "    Adding example : ${name}")

	# Is the main source inside the list of sources found? If not then this is a cmake problem
	set(main_source ${example}/${name}.jakt)
	if(NOT (${main_source} IN_LIST example_source_list))
		message(FATAL_ERROR "Main source \"${main_source}\" was not found "
			"in sources list for this executable \"${example_source_list}\"")
	endif()

	add_jakt_executable(jakt_${name}
	MAIN_SOURCE ${main_source}
    MODULE_SOURCES ${example_source_list}
    RUNTIME_DIRECTORY "${jakt_runtime_dir}")

	
endforeach()