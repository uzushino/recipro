#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "isolate.h"

using namespace recipro;

void recipro::LogCallback(const v8::FunctionCallbackInfo<v8::Value>& args) {
  if (args.Length() < 1) return;

  v8::Isolate* isolate = args.GetIsolate();
  v8::HandleScope scope(isolate);

  v8::Local<v8::Value> arg = args[0];
  v8::String::Utf8Value value(isolate, arg);

  printf("Logged: %s\n", *value);
}

void Isolate::New() {
  allocator_ = v8::ArrayBuffer::Allocator::NewDefaultAllocator();
  
  v8::Isolate::CreateParams params;

  params.array_buffer_allocator = allocator_;
  params.external_references = recipro::external_references;
  if (HasSnapshot()) { // Load from snapshot.
    params.snapshot_blob = &startup_data_;
  }

  isolate_ = v8::Isolate::New(params);
}

void Isolate::NewForSnapshot() {
  creator_ = new v8::SnapshotCreator(recipro::external_references);
  isolate_ = creator_->GetIsolate();
}

bool Isolate::Eval(const char *javascript) {
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
    printf("Error: %s\n", *exception_str);
    return false;
  }

  auto ret = result.ToLocalChecked();
  v8::String::Utf8Value utf8(isolate_, ret);
  printf("Result: %s\n", *utf8);

  return true;
}

v8::StartupData Isolate::CreateSnapshotDataBlob(
  v8::SnapshotCreator::FunctionCodeHandling handling
) {
  return creator_->CreateBlob(handling); 
}