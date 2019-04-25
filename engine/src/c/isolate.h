#pragma once

#include "include/v8.h"

namespace recipro {
  typedef struct {
    const char *data;
    size_t snapshot_size;
  } Snapshot ;

  class Isolate {
    public:
      Isolate() 
        : isolate_(nullptr), allocator_(nullptr), creator_(nullptr) { }
      ~Isolate() {
        Dispose();
      }

      void Initialize(Snapshot *);
      void Dispose();
      bool JsEval(const char *);

    public:
      v8::Isolate* isolate_;
      v8::ArrayBuffer::Allocator* allocator_;
      v8::Persistent<v8::Context> context_;

      v8::SnapshotCreator* creator_;
  };
};
