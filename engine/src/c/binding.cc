
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "include/libplatform/libplatform.h"
#include "include/v8.h"

#include "binding.h"

const char* get_v8_version() 
{
  return v8::V8::GetVersion();
}

static std::unique_ptr<v8::Platform> platform;

class Isolate {
public:
  Isolate() : isolate_(nullptr), allocator_(nullptr) { }
  ~Isolate() {
    Dispose();
  }

  void New();
  void Dispose();
  void JsEval(const char *);

private:
  v8::Isolate* isolate_;
  v8::ArrayBuffer::Allocator* allocator_;
};

void v8_init() {
  if (platform.get() == nullptr) {
    platform = v8::platform::NewDefaultPlatform();

    v8::V8::InitializePlatform(platform.get()); 
    v8::V8::Initialize();
  }
}

void v8_dispose() {
  v8::V8::Dispose();
}

void v8_shutdown_platform() {
  v8::V8::ShutdownPlatform();
}

void js_eval(const char* script) {
  Isolate isolate = Isolate();

  isolate.New();
  isolate.JsEval(script);

  return;
}

void Isolate::New() {
  allocator_ = 
    v8::ArrayBuffer::Allocator::NewDefaultAllocator();
  
  v8::Isolate::CreateParams create_params;
  create_params.array_buffer_allocator = allocator_;

  isolate_ = v8::Isolate::New(create_params);
}

void Isolate::Dispose() {
  if (isolate_) {
    isolate_ = NULL;
  }

  if (allocator_) {
    delete allocator_;
    allocator_ = NULL;
  }
}

void Isolate::JsEval(const char *javascript) {
  v8::Isolate::Scope isolate_scope(isolate_);
  v8::HandleScope handle_scope(isolate_);
  v8::Local<v8::Context> context = v8::Context::New(isolate_);
  v8::Context::Scope context_scope(context);

  v8::Local<v8::String> source =
      v8::String::NewFromUtf8(
        isolate_, 
        javascript,
        v8::NewStringType::kNormal
      ).ToLocalChecked(); 
  
  v8::Local<v8::Script> script = v8::Script::Compile(context, source).ToLocalChecked();
  v8::Local<v8::Value> result = script->Run(context).ToLocalChecked();
  v8::String::Utf8Value utf8(isolate_, result);

  printf("%s\n", *utf8);
}

/*
void hello() {
  std::unique_ptr<v8::Platform> platform = v8::platform::NewDefaultPlatform();
  v8::V8::InitializePlatform(platform.get());
  v8::V8::Initialize();

  v8::Isolate::CreateParams create_params;

  create_params.array_buffer_allocator =
      v8::ArrayBuffer::Allocator::NewDefaultAllocator();

  v8::Isolate* isolate = v8::Isolate::New(create_params);

  {
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    v8::Local<v8::Context> context = v8::Context::New(isolate);
    v8::Context::Scope context_scope(context);

    v8::Local<v8::String> source =
        v8::String::NewFromUtf8(
          isolate, 
          "'Hello' + ', World! From Javascript.'",
          v8::NewStringType::kNormal
        ).ToLocalChecked();

    v8::Local<v8::Script> script =
        v8::Script::Compile(context, source).ToLocalChecked();

    v8::Local<v8::Value> result = script->Run(context).ToLocalChecked();

    v8::String::Utf8Value utf8(isolate, result);

    printf("%s\n", *utf8);
  }

  isolate->Dispose();

  v8::V8::Dispose();
  v8::V8::ShutdownPlatform();

  delete create_params.array_buffer_allocator;

  return ;
}
*/