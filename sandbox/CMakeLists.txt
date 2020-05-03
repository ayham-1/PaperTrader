project("sandbox" LANGUAGES C)

set(CMAKE_C_STANDARD 11)

set(sandbox_hdrs)
set(sandbox_srcs 
	main.c
)

add_executable(${PROJECT_NAME} ${sandbox_hdrs} ${sandbox_srcs})

# Set compiler arguments.
if(MSVC)
	target_compile_options(${PROJECT_NAME} PUBLIC "/W4" "$<$<CONFIG:RELEASE>:/O3>")
else()
	target_compile_options(${PROJECT_NAME} PUBLIC "-Wall" "-Wextra" "-Werror" "$<$<CONFIG:RELEASE>:-O3>")
endif() 
