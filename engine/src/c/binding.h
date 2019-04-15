#pragma once

#ifdef __cplusplus
extern "C" {
#endif

  const char* get_v8_version();

  void v8_init() ;
  void v8_dispose() ;
  void v8_shutdown_platform() ;

  void js_eval();

#ifdef __cplusplus
}
#endif