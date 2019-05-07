#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "isolate.h"

using namespace recipro;

void Isolate::New() {
  allocator_ = v8::ArrayBuffer::Allocator::NewDefaultAllocator();
  
  v8::Isolate::CreateParams params;
  params.array_buffer_allocator = allocator_;

  if (HasSnapshot()) { // Load from snapshot.
    params.snapshot_blob = &startup_data_;
  }

  isolate_ = v8::Isolate::New(params);
}

void Isolate::NewForSnapshot() {
  creator_ = new v8::SnapshotCreator();
  isolate_ = creator_->GetIsolate();
}

static void LogCallback(const v8::FunctionCallbackInfo<v8::Value>& args) {
  printf("aaaa\n");
  if (args.Length() < 1) return;

  v8::Isolate* isolate = args.GetIsolate();
  v8::HandleScope scope(isolate);

  v8::Local<v8::Value> arg = args[0];
  v8::String::Utf8Value value(isolate, arg);

  printf("Logged: %s\n", *value);
}

bool Isolate::Eval(const char *javascript) {
  v8::Isolate::Scope isolate_scope(isolate_);
  v8::HandleScope handle_scope(isolate_);

  v8::Local<v8::ObjectTemplate> global = v8::ObjectTemplate::New(isolate_);
  global->Set(v8::String::NewFromUtf8(isolate_, "log", v8::NewStringType::kNormal).ToLocalChecked(),
            v8::FunctionTemplate::New(isolate_, LogCallback));
  
  v8::Local<v8::Context> context = v8::Context::New(isolate_, NULL, global);

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