#pragma once

#ifdef __cplusplus
extern "C" {
#endif

  const char* get_v8_version();

  void init_platform();

  void init() ;

  void dispose() ;

  void shutdown_platform() ;

#ifdef __cplusplus
}
#endif