enable_testing()

add_executable(test_linkedList 
	ds/generic/linkedlist/test_list.cpp
)
target_link_libraries(test_linkedList libtrader)
target_include_directories(test_linkedList PUBLIC ../)
add_test(NAME linkedList COMMAND $<TARGET_FILE:test_linkedList>)
