
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "include/v8.h"
#include "include/libplatform/libplatform.h"

#include "binding.h"

const char* v8_get_version() {
  return v8::V8::GetVersion();
}

static std::unique_ptr<v8::Platform> platform;

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
  platform = nullptr;
}
