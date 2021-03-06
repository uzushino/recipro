#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <fstream>
#include <functional>
#include <map>
#include <string>
#include <utility>
#include <vector>

#include "src/base/logging.h"
#include "src/utils.h"

#include "recipro.h"
#include "isolate.h"
#include "binding.h"

using namespace recipro;

void recipro::LogCallback(const v8::FunctionCallbackInfo<v8::Value>& args) {
  if (args.Length() < 1) return;

  v8::Isolate* isolate = args.GetIsolate();
  v8::HandleScope scope(isolate);

  v8::Local<v8::Value> arg = args[0];
  v8::String::Utf8Value value(isolate, arg);

  printf("%s\n", *value);
}

std::string ReadFileWithCache(ReciproVM *vm, const char *filename, bool *exists) {
  std::map<std::string, std::string>::iterator it = 
    vm->ccache_.find(filename);

  std::string source;
  if (it != vm->ccache_.end()) {
    source = it->second;
    *exists = true;
  } else {
    source = v8::internal::ReadFile(filename, exists);
    vm->ccache_.insert(std::make_pair(filename, source));
  }

  return source;
}

void recipro::ReadFileCallback(const v8::FunctionCallbackInfo<v8::Value>& args) {
  if (args.Length() < 1) return;
  
  v8::Local<v8::External> ext = args.Data().As<v8::External>();
  ReciproVM* self = static_cast<ReciproVM *>(ext->Value());

  v8::Isolate* isolate = args.GetIsolate();
  v8::HandleScope scope(isolate);

  v8::Local<v8::Value> arg = args[0];
  v8::String::Utf8Value value(isolate, arg);

  bool exists = false;
  std::string buffer;
  if (self) {
    buffer = ReadFileWithCache(self, *value, &exists);
  } else {
    buffer = v8::internal::ReadFile(*value, &exists);
    exists = true;
  }

  if (exists) {
    v8::Local<v8::ArrayBuffer> ab = 
      v8::ArrayBuffer::New(isolate, (void *)buffer.c_str(), buffer.length());

    args.GetReturnValue().Set(ab);
  }
}

void recipro::RunScriptCallback(const v8::FunctionCallbackInfo<v8::Value>& args) {
  if (args.Length() < 1) return;

  v8::Local<v8::External> ext = args.Data().As<v8::External>();
  ReciproVM* self = static_cast<ReciproVM *>(ext->Value());

  v8::Isolate* isolate = args.GetIsolate();
  v8::HandleScope scope(isolate);
  auto context = isolate->GetCurrentContext();
  v8::Context::Scope context_scope(context);

  v8::Local<v8::Value> arg = args[0];
  v8::String::Utf8Value value(isolate, arg);
  
  bool exists = false;
  std::string buffer;
  if (self) {
    buffer = ReadFileWithCache(self, *value, &exists);
  } else {
    buffer = v8::internal::ReadFile(*value, &exists);
    exists = true;
  }

  if (exists) {
    auto source =
        v8::String::NewFromUtf8(isolate, buffer.c_str(), v8::NewStringType::kNormal).ToLocalChecked(); 

    v8::TryCatch trycatch(isolate);
    auto script = v8::Script::Compile(context, source).ToLocalChecked();
    auto result = script->Run(context);

    if (result.IsEmpty()) {
      v8::Local<v8::Value> exception = trycatch.Exception();
      v8::String::Utf8Value exception_str(isolate, exception);
      printf("Error: %s\n", *exception_str);
    }
  }
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
  isolate_->SetData(0, this);
}

void Isolate::NewForSnapshot() {
  creator_ = new v8::SnapshotCreator(recipro::external_references);
  isolate_ = creator_->GetIsolate();
  isolate_->SetData(0, this);
}

v8::MaybeLocal<v8::Module> InstantiateCallBack(
  v8::Local<v8::Context> context, 
  v8::Local<v8::String> specifier, 
  v8::Local<v8::Module> referrer
) {
  return v8::Local<v8::Module>();
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

/*
  auto ret = result.ToLocalChecked();
  v8::String::Utf8Value utf8(isolate_, ret);
  printf("Result: %s\n", *utf8);
*/

  return true;
}

v8::ScriptOrigin Isolate::GetScriptOrigin(const char *name) {
  using namespace v8;

  ScriptOrigin origin(
    String::NewFromUtf8(isolate_, name, NewStringType::kNormal).ToLocalChecked(),
    Local<Integer>(), Local<Integer>(), Local<Boolean>(), Local<Integer>(),
    Local<Value>(), Local<Boolean>(), Local<Boolean>(), True(isolate_));

  return origin;
}

int Isolate::ModuleTree(const char* filename, const char* script) {
  using namespace v8;
  
  v8::Isolate::Scope isolate_scope(isolate_);
  HandleScope handle_scope(isolate_);
  auto context = context_.Get(isolate_);
  v8::Context::Scope context_scope(context);

  auto source_text = String::NewFromUtf8(isolate_, script, NewStringType::kNormal).ToLocalChecked();
  auto origin = GetScriptOrigin(filename);
  ScriptCompiler::Source source(source_text, origin);

  v8::TryCatch try_catch(isolate_);
  auto compiled = ScriptCompiler::CompileModule(isolate_, &source);
  if (try_catch.HasCaught()) {
    v8::Local<v8::Value> exception = try_catch.Exception();
    v8::String::Utf8Value exception_str(isolate_, exception);
    printf("Error: %s\n", *exception_str);
    return 0;
  }

  std::vector<std::string> specifiers;
  auto module = compiled.ToLocalChecked();
  int id = module->GetIdentityHash();

  for (int i = 0, length = module->GetModuleRequestsLength(); i < length; ++ i) {
    Local<String> name = module->GetModuleRequest(i);
    v8::String::Utf8Value utf8(isolate_, name);
    specifiers.push_back(*utf8);
  }

  specifier_map_.emplace(
      std::piecewise_construct, std::make_tuple(id),
      std::make_tuple(isolate_, module, filename, specifiers));
  module_map_
    .insert(std::make_pair(filename, id)); 

  return id;
}

v8::StartupData Isolate::CreateSnapshotDataBlob(
  v8::SnapshotCreator::FunctionCodeHandling handling
) {
  return creator_->CreateBlob(handling); 
}