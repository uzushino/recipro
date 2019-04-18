#pragma once

#include "isolate.h"

#ifdef __cplusplus
extern "C" {
#endif
  typedef struct {
    std::shared_ptr<recipro::Isolate> isolate_;
  } ReciproVM;

  ReciproVM* init(); // construct

  void dispose(ReciproVM* vm) ; // desctruct

  void execute(ReciproVM* vm, const char* script) ;

#ifdef __cplusplus
}
#endif 