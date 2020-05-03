project("libtrader" LANGUAGES C)

set(CMAKE_C_STANDARD 11)

set(libtrader_hdrs)
set(libtrader_srcs 
	test.c
)

add_library(${PROJECT_NAME} ${libtrader_hdrs} ${libtrader_srcs})

# Set compiler arguments.
if(MSVC)
	target_compile_options(${PROJECT_NAME} PUBLIC "/W4" "$<$<CONFIG:RELEASE>:/O3>")
else()
	target_compile_options(${PROJECT_NAME} PUBLIC "-Wall" "-Wextra" "-Werror" "$<$<CONFIG:RELEASE>:-O3>")
endif()
