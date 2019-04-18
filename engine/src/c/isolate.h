#pragma once

#include "include/v8.h"

namespace recipro {
  class Isolate {
    public:
      Isolate() : isolate_(nullptr), allocator_(nullptr) { }
      ~Isolate() {
        Dispose();
      }

      void Initialize();
      void Dispose();
      bool JsEval(const char *);

    private:
      v8::Isolate* isolate_;
      v8::ArrayBuffer::Allocator* allocator_;
      v8::Persistent<v8::Context> context_;
  };
};
