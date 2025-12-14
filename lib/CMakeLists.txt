# ===================================================== #
#  This file is a part of the via Programming Language  #
# ----------------------------------------------------- #
#           Copyright (C) XnLogicaL 2024-2025           #
#              Licensed under GNU GPLv3.0               #
# ----------------------------------------------------- #
#         https://github.com/XnLogicaL/via-lang         #
# ===================================================== #

function(load_library NAME)
  set(LIB_OUTPUT_DIR ${CMAKE_BINARY_DIR}/lib/${NAME})
  get_filename_component(MODULE_NAME ${NAME} NAME)

  file(MAKE_DIRECTORY ${LIB_OUTPUT_DIR})
  file(GLOB SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/${NAME}/*.cpp")

  foreach(src ${SOURCES})
    get_filename_component(src_name ${src} NAME_WE)
    add_library(vialib${src_name} SHARED ${src})

    target_link_libraries(vialib${src_name} PUBLIC via::core)
    set_target_properties(vialib${src_name}
      PROPERTIES
        OUTPUT_NAME ${src_name}         # just "foo.so" instead of "libfoo.so"
        PREFIX ""
        LIBRARY_OUTPUT_DIRECTORY ${LIB_OUTPUT_DIR}
        RUNTIME_OUTPUT_DIRECTORY ${LIB_OUTPUT_DIR}
    )

    add_custom_target(vialib${src_name}_build
      ALL
      DEPENDS vialib${src_name}
    )

    install(TARGETS vialib${src_name}
      LIBRARY DESTINATION share/via/lib/${MODULE_NAME}
      RUNTIME DESTINATION share/via/lib/${MODULE_NAME}
    )
  endforeach()
endfunction()

load_library(src/std)
