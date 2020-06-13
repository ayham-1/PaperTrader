project("libtrader" LANGUAGES C)


option(COVERALLS "Generate coveralls data" ON)

set(CMAKE_MODULE_PATH "${CMAKE_MODULE_PATH};${CMAKE_SOURCE_DIR}/cmake")

if (COVERALLS)
    include(Coveralls)
	coveralls_turn_on_coverage()
endif()

set(CMAKE_C_STANDARD 11)

set(libtrader_hdrs)
set(libtrader_srcs 
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
endif(MSVC)

if (COVERALLS)
    set(COVERAGE_SRCS ${libtrader_hdrs} ${libtrader_srcs})

    # Create the coveralls target.
    coveralls_setup("${COVERAGE_SRCS}" ON)
endif()

# Setup testing
enable_testing()
add_subdirectory(tests)
