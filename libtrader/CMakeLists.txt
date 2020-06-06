project("libtrader" LANGUAGES C)

set(CMAKE_C_STANDARD 11)

set(libtrader_hdrs)
set(libtrader_srcs 
	test.c
	ds/account/account.h
	ds/account/account.c
	ds/account/portfolio.h
	ds/account/portfolio.c
	ds/account/transaction.h
	ds/generic/linkedlist/list.h
	ds/generic/linkedlist/list.c
)

add_library(${PROJECT_NAME} ${libtrader_hdrs} ${libtrader_srcs})

# Set compiler arguments.
if(MSVC)
	target_compile_options(${PROJECT_NAME} PUBLIC "/W4" 
		"$<$<CONFIG:RELEASE>:/O3>")
else()
	target_compile_options(${PROJECT_NAME} PUBLIC "-Wall" "-Wextra" "-Werror" 
		"$<$<CONFIG:RELEASE>:-O3>")
endif()
