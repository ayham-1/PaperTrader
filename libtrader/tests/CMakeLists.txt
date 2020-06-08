enable_testing()
add_executable(test_linkedList 
	ds/generic/linkedlist/list.c
	../libtrader.h
)
target_link_libraries(test_linkedList PRIVATE libtrader)
add_test(linkedList test_linkedList)
include_directories(../)
