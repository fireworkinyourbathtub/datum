add_executable(datumc main.cpp)

target_include_directories(datumc PRIVATE "../include")

set_target_properties(datumc PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
    CXX_STANDARD 17
    CXX_STANDARD_REQUIRED YES
    CXX_EXTENSIONS NO
)

target_compile_options(datumc PRIVATE -Wall -Wextra -Wpedantic)
