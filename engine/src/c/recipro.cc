#include <functional>
#include "include/v8.h"
#include "src/base/logging.h"

#include "recipro.h"

using namespace recipro;

ReciproVM* init_recipro_core(recipro::SnapshotData snapshot) {
  ReciproVM* vm = new ReciproVM {};

  vm->isolate_ = std::make_shared<recipro::Isolate>(snapshot);
  vm->isolate_->New();

  vm->isolate_->RunIsolateScope([vm](auto isolate) {
    auto context = v8::Context::New(isolate);

    vm->isolate_->Reset(context);
  });

  return vm;
}

ReciproVM* init_recipro_snapshot() {
  auto vm = new ReciproVM {};
  
  vm->isolate_ = std::make_shared<recipro::Isolate>();
  vm->isolate_->NewForSnapshot();

  vm->isolate_->RunIsolateScope([vm](auto isolate) {
    auto context = v8::Context::New(isolate);

    vm->isolate_->Reset(context);
    vm->isolate_->DefaultContext(context);

    SetGlobalObject(isolate, context);
  });

  return vm;
}

void SetGlobalObject(v8::Isolate *isolate, v8::Local<v8::Context> context) {
    v8::HandleScope handle_scope(isolate);
    v8::Context::Scope context_scope(context);

    auto global = context->Global();
    auto global_val = v8::Object::New(isolate);
    auto result = global->Set(
      context,
      v8::String::NewFromUtf8(v8::Isolate::GetCurrent(), "Recipro", v8::NewStringType::kNormal).ToLocalChecked(),
      global_val
    );
    CHECK(result.FromJust());

    auto log_tmpl = v8::FunctionTemplate::New(isolate, LogCallback);
    auto log_val = log_tmpl->GetFunction(context).ToLocalChecked();
    result = global_val->Set(
      context,
      v8::String::NewFromUtf8(v8::Isolate::GetCurrent(), "log", v8::NewStringType::kNormal).ToLocalChecked(),
      log_val
    );
    CHECK(result.FromJust());
}

void dispose(ReciproVM* vm) {
  delete vm;
}

void eval(ReciproVM* vm, const char* script) {
  vm->isolate_->Eval(script);
}

recipro::SnapshotData take_snapshot(ReciproVM *vm) {
  vm->isolate_->Reset();

  v8::StartupData result = vm->isolate_->CreateSnapshotDataBlob(
    v8::SnapshotCreator::FunctionCodeHandling::kKeep
  );

  return {result.data, result.raw_size};
}

void delete_snapshot(const char *data_ptr) {
  delete[] data_ptr;
}

int module_compile(ReciproVM* vm, const char *filename, const char *script) {
  return vm->isolate_->ModuleTree(filename, script);
}

v8::MaybeLocal<v8::Module> ResolevCallback(
  v8::Local<v8::Context> context, v8::Local<v8::String> specifier, v8::Local<v8::Module> referrer
) {
  v8::Isolate *isolate = context->GetIsolate();
  v8::Isolate::Scope isolate_scope(isolate);

  auto inst = static_cast<recipro::Isolate*>(isolate->GetData(0));
  auto referrer_id = referrer->GetIdentityHash();
  auto* info = inst->FindModuleInfo(referrer_id);
  CHECK_NOT_NULL(info);

  v8::String::Utf8Value utf8(isolate, specifier);
  auto id = inst->resolve_callback_(inst->resolve_data_, *utf8, referrer_id);
  auto *module_info = inst->FindModuleInfo(id);
  if (module_info) {
    return module_info->module_.Get(isolate);
  }

  return v8::MaybeLocal<v8::Module>();
}

class ResolveDataScope {
  recipro::Isolate *isolate_;
  void *data_;

  public:
    ResolveDataScope(recipro::Isolate *isolate, void *data): data_(data), isolate_(isolate) {
      isolate_->resolve_data_ = data;
    }
    
    ~ResolveDataScope() {
      isolate_->resolve_callback_ = nullptr;
    }
};

void module_instantiate(ReciproVM* vm, int id, void *data, ReciproResolevCallback callback) {
  using namespace v8;

  v8::Isolate *isolate = vm->isolate_->Raw();
  v8::Isolate::Scope isolate_scope(isolate);
  v8::HandleScope handle_scope(isolate);
  ResolveDataScope resolve_scope(vm->isolate_.get(), data);

  auto context = vm->isolate_->GetContext();
  v8::Context::Scope context_scope(context);

  vm->isolate_->resolve_callback_ = callback;

  TryCatch try_catch(isolate);
  {
    auto info = vm->isolate_->FindModuleInfo(id);
    if (info == nullptr) {
      return ;
    }

    auto module = info->module_.Get(isolate);
    auto instantiated = module->InstantiateModule(context, ResolevCallback);

    CHECK(instantiated.IsJust() || try_catch.HasCaught());
  }
}

bool module_evaluate(ReciproVM* vm, int id) {
  using namespace v8;

  v8::Isolate *isolate = vm->isolate_->Raw();
  v8::Isolate::Scope isolate_scope(isolate);
  v8::HandleScope handle_scope(isolate);

  auto context = vm->isolate_->GetContext();
  v8::Context::Scope context_scope(context);

  TryCatch try_catch(isolate);
  {
    auto info = vm->isolate_->FindModuleInfo(id);
    if (info == nullptr) {
      return false;
    }

    auto module = info->module_.Get(isolate);
    auto maybe_result = module->Evaluate(context);

    Local<Value> result;
    if (!maybe_result.ToLocal(&result)) {
      DCHECK(try_catch.HasCaught());
      return false;
    }

    return true;
  }
}