#include "recipro.h"

ReciproVM* init(recipro::Snapshot *snapshot) {
  auto vm = new ReciproVM {};

  vm->isolate_ = std::make_shared<recipro::Isolate>();
  vm->isolate_->Initialize(snapshot);

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
        v8::SnapshotCreator::FunctionCodeHandling::kClear
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