
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
  bool JsEval(const char *, std::string&);

private:
  v8::Isolate* isolate_;
  v8::ArrayBuffer::Allocator* allocator_;
  v8::Persistent<v8::Context> context_;
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

  std::string str;
  isolate.JsEval(script, str);

  printf("%s\n", str.c_str());

  isolate.Dispose();

  return;
}

void Isolate::New() {
  allocator_ = 
    v8::ArrayBuffer::Allocator::NewDefaultAllocator();
  
  v8::Isolate::CreateParams create_params;
  create_params.array_buffer_allocator = allocator_;

  isolate_ = v8::Isolate::New(create_params);

  v8::Isolate::Scope isolate_scope(isolate_);
  {
    v8::HandleScope handle_scope(isolate_);

    auto context = v8::Context::New(
      isolate_, 
      nullptr, 
      v8::MaybeLocal<v8::ObjectTemplate>());

    context_.Reset(isolate_, context);
  }
}

void Isolate::Dispose() {
  if (isolate_) {
    isolate_->Dispose();
    isolate_ = NULL;
  }

  if (allocator_) {
    delete allocator_;
    allocator_ = NULL;
  }
}

bool Isolate::JsEval(const char *javascript, std::string& value) {
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
  value = *utf8;

  return true;
}