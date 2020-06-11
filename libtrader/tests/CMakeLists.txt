enable_testing()

find_package(Catch2 REQUIRED)

include(CTest)
include(Catch)

function(test test_name source_file)
	add_executable(${test_name} 
		tests_main.cpp
		${source_file})
	target_link_libraries(${test_name} Catch2::Catch2 libtrader)
	target_include_directories(${test_name} PUBLIC ../)
	add_test(NAME ${test_name} COMMAND $<TARGET_FILE:${test_name}>)
endfunction()

test(test_linkedList ds/generic/linkedlist/test_list.cpp)
