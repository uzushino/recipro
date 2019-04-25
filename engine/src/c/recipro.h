#pragma once

#include <memory>
#include "isolate.h"

#ifdef __cplusplus
extern "C" {
#endif
  typedef struct {
    std::shared_ptr<recipro::Isolate> isolate_;
  } ReciproVM;

  ReciproVM* init(recipro::Snapshot* snapshot); // construct
  ReciproVM* init_snapshot(); // construct

  void dispose(ReciproVM* vm) ; // desctruct

  void execute(ReciproVM* vm, const char* script) ;

  recipro::Snapshot* take_snapshot(ReciproVM *vm); 

  void delete_snapshot(recipro::Snapshot* snapshot);

#ifdef __cplusplus
}
#endif