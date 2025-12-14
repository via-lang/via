# ===================================================== #
#  This file is a part of the via Programming Language  #
# ----------------------------------------------------- #
#           Copyright (C) XnLogicaL 2024-2025           #
#              Licensed under GNU GPLv3.0               #
# ----------------------------------------------------- #
#         https://github.com/XnLogicaL/via-lang         #
# ===================================================== #

# Collect sources
file(GLOB_RECURSE VIA_CLI_SOURCES CONFIGURE_DEPENDS src/*.cpp)

# Create executable
add_executable(via-cli ${VIA_CLI_SOURCES})

# Add include directories
target_include_directories(via-cli
  PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src/
    ${CMAKE_CURRENT_SOURCE_DIR}/../include
)

# Add dependencies
target_link_libraries(via-cli
  PRIVATE
    argvx
    via::core
)

# Alias executable
add_executable(via::cli ALIAS via-cli)

set_target_properties(via-cli PROPERTIES
  OUTPUT_NAME    via
  INSTALL_RPATH  "$ORIGIN/../lib"
)

# Install executable
install(TARGETS via-cli
  RUNTIME DESTINATION bin
)
