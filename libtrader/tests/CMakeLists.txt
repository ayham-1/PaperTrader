enable_testing()

find_package(Catch2 REQUIRED)

include(CTest)
include(Catch)
add_executable(test_linkedList 
	tests_main.cpp
	ds/generic/linkedlist/test_list.cpp
)
target_link_libraries(test_linkedList Catch2::Catch2 libtrader)
target_include_directories(test_linkedList PUBLIC ../)
add_test(NAME linkedList COMMAND $<TARGET_FILE:test_linkedList>)
