# ===================================================== #
#  This file is a part of the via Programming Language  #
# ----------------------------------------------------- #
#           Copyright (C) XnLogicaL 2024-2025           #
#              Licensed under GNU GPLv3.0               #
# ----------------------------------------------------- #
#         https://github.com/XnLogicaL/via-lang         #
# ===================================================== #

# Collect source files
file(GLOB_RECURSE VIA_CORE_SOURCES CONFIGURE_DEPENDS src/*.cpp)

# Create library
add_library(via SHARED ${VIA_CORE_SOURCES})

# Add include directories
target_include_directories(via
  PUBLIC     ${CMAKE_CURRENT_SOURCE_DIR}/src
  INTERFACE  ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# Add dependencies
target_link_libraries(via PUBLIC mimalloc)

# Alias library
add_library(via::core ALIAS via)

# Install DL
install(TARGETS via
  LIBRARY DESTINATION lib        # .so
  RUNTIME DESTINATION bin        # .dll
)
