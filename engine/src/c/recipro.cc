#include "include/v8.h"
#include "recipro.h"

ReciproVM* init(recipro::Snapshot *snapshot) {
  auto vm = new ReciproVM {};

  vm->isolate_ = std::make_shared<recipro::Isolate>();
  vm->isolate_->Initialize(snapshot);

  auto isolate = vm->isolate_->isolate_;

  v8::Isolate::Scope isolate_scope(isolate);
  {
    v8::HandleScope handle_scope(isolate);

    auto context = v8::Context::New(isolate);

    vm->isolate_->context_.Reset(isolate, context);
  }

  return vm;
}

ReciproVM* init_snapshot() {
  auto vm = new ReciproVM {};

  auto* creator = new v8::SnapshotCreator();
  v8::Isolate* isolate = creator->GetIsolate();

  vm->isolate_ = std::make_shared<recipro::Isolate>(isolate);
  vm->isolate_->creator_ = creator;

  v8::Isolate::Scope isolate_scope(isolate);
  {
    v8::HandleScope handle_scope(isolate);

    auto context = v8::Context::New(isolate);

    vm->isolate_->context_.Reset(isolate, context);
    vm->isolate_->creator_->SetDefaultContext(context);
  }

  return vm;
}

void dispose(ReciproVM* vm) {
  delete vm;
}

void execute(ReciproVM* vm, const char* script) {
  vm->isolate_->JsEval(script);
}

recipro::Snapshot* take_snapshot(ReciproVM *vm) {
  vm->isolate_->context_.Reset();

  auto result = vm->isolate_->creator_->CreateBlob(
        v8::SnapshotCreator::FunctionCodeHandling::kKeep
  );

  auto *snapshot = new recipro::Snapshot;
  snapshot->data = result.data;
  snapshot->snapshot_size = result.raw_size;

  return snapshot;
}

void delete_snapshot(recipro::Snapshot* snapshot) {
  delete[] snapshot->data;
  delete snapshot;
}