#include "recipro.h"

ReciproVM* init() {
  auto vm = new ReciproVM {};

  vm->isolate_ = std::make_shared<recipro::Isolate>();
  vm->isolate_->Initialize();

  return vm;
}

void dispose(ReciproVM* vm) {
  delete vm;
}

void execute(ReciproVM* vm, const char* script) {
  vm->isolate_->JsEval(script);
}