# ===================================================== #
#  This file is a part of the via Programming Language  #
# ----------------------------------------------------- #
#           Copyright (C) XnLogicaL 2024-2025           #
#              Licensed under GNU GPLv3.0               #
# ----------------------------------------------------- #
#         https://github.com/XnLogicaL/via-lang         #
# ===================================================== #

# Check for test option
if(VIA_BUILD_TEST)
  # Collect source files
  file(GLOB_RECURSE VIA_TEST_SOURCES CONFIGURE_DEPENDS src/*.cpp)

  # Create executable
  add_executable(via-test ${VIA_TEST_SOURCES})

  # Add include directories
  target_include_directories(via-test
    PUBLIC     ${CMAKE_CURRENT_SOURCE_DIR}/src
    INTERFACE  ${CMAKE_CURRENT_SOURCE_DIR}/include
  )

  # Add dependencies
  target_link_libraries(via-test
    PUBLIC
      via::core
      Catch2::Catch2WithMain
  )

  # Alias exectuable
  add_executable(via::test ALIAS via-test)
endif()
