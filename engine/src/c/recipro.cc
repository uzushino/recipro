#include "include/v8.h"
#include "recipro.h"

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
  });

  return vm;
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