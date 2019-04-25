#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "isolate.h"

using namespace recipro;

void Isolate::Initialize(Snapshot *snapshot) {
 allocator_ = 
    v8::ArrayBuffer::Allocator::NewDefaultAllocator();
  
  v8::Isolate::CreateParams params;
  params.array_buffer_allocator = allocator_;

  if (snapshot) { // Load from snapshot.
    params.snapshot_blob = reinterpret_cast<v8::StartupData *>(snapshot);
  }

  isolate_ = v8::Isolate::New(params);
}

void Isolate::Dispose() {
  if (creator_) {
    delete creator_;
    creator_ = NULL;
  } else {
    if (isolate_) {
      isolate_->Dispose();
      isolate_ = NULL;
    }
  }

  if (allocator_) {
    delete allocator_;
    allocator_ = NULL;
  }
}

bool Isolate::JsEval(const char *javascript) {
  v8::Isolate::Scope isolate_scope(isolate_);
  v8::HandleScope handle_scope(isolate_);
  
  auto context = context_.Get(isolate_);
  v8::Context::Scope context_scope(context);

  auto source =
      v8::String::NewFromUtf8(
        isolate_, 
        javascript,
        v8::NewStringType::kNormal
      ).ToLocalChecked(); 

  auto script = v8::Script::Compile(context, source).ToLocalChecked();

  v8::TryCatch trycatch(isolate_);
  auto result = script->Run(context);

  if (result.IsEmpty()) {
    v8::Local<v8::Value> exception = trycatch.Exception();
    v8::String::Utf8Value exception_str(isolate_, exception);

    printf("Exception: %s\n", *exception_str);

    return false;
  }

  auto ret = result.ToLocalChecked();
  v8::String::Utf8Value utf8(isolate_, ret);
  printf("Result: %s\n", *utf8);

  return true;
}