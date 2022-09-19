cmake_minimum_required(VERSION 3.16.0 FATAL_ERROR)
project(language-adventures)
if(TARGET language-adventures)
	return()
endif()

# Add all the languages which can be built with cmake you can avoid adding it by 
# Defining NOT_ADVENTURE_${language} on the command line with -D 
set(LANGUAGE_LIST 
	asm
	jakt)

foreach(language ${LANGUAGE_LIST})

	if(NOT DEFINED NOT_ADVENTURE_${language} )
		message(STATUS "Adding language : ${language}")
		add_subdirectory(${language})
	else()
		message(STATUS "Skipping language :${language}")
	endif()
	
endforeach()