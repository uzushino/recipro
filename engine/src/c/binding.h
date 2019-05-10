#pragma once

#include <string.h>
#include "include/v8.h"

extern "C" {
  const char* v8_get_version();

  void v8_init() ;
  void v8_dispose() ;
  void v8_shutdown_platform();
}