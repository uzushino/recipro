#pragma once

#include <memory>
#include "isolate.h"

#ifdef __cplusplus
extern "C" {
#endif
  typedef struct {
    std::shared_ptr<recipro::Isolate> isolate_;
  } ReciproVM;

  ReciproVM* init_recipro_core(recipro::SnapshotData); // construct
  ReciproVM* init_recipro_snapshot(); // construct

  void dispose(ReciproVM *) ; // desctruct
  void eval(ReciproVM *, const char *) ;

  recipro::SnapshotData take_snapshot(ReciproVM *); 
  void delete_snapshot(const char *);

  int module_compile(ReciproVM *, const char *, const char *);
  void module_instantiate(ReciproVM *, int, void *data, recipro::ReciproResolevCallback);
  bool module_evaluate(ReciproVM *, int);

#ifdef __cplusplus
}
#endif