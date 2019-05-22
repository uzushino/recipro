#pragma once

#include <functional>
#include <map>
#include <string>
#include <utility>
#include <vector>

#include "include/v8.h"
#include "src/base/logging.h"

namespace recipro {
  typedef int (* ReciproResolevCallback)(void *data, const char *, int);

  typedef struct {
    const char *data;
    int data_size;
  } SnapshotData ;

  struct ModuleInfo {
    std::string filename_;
    std::vector<std::string> specifier_;
    v8::Persistent<v8::Module> module_;

    ModuleInfo(v8::Isolate* isolate, v8::Local<v8::Module> module, 
      const char *filename, std::vector<std::string> specifier) 
    : filename_(filename), specifier_(specifier) {
      module_.Reset(isolate, module);
    }
  };

  typedef std::map<int, ModuleInfo> specifier_map;
  typedef std::map<std::string, int> module_map;

  void LogCallback(const v8::FunctionCallbackInfo<v8::Value>& args); 
  void ReadfileCallback(const v8::FunctionCallbackInfo<v8::Value>& args); 

  static intptr_t external_references[] = {
    reinterpret_cast<intptr_t>(LogCallback),
    reinterpret_cast<intptr_t>(ReadfileCallback),
    0
  };

  class Isolate {
    public:
      Isolate() 
        : isolate_(nullptr), allocator_(nullptr), creator_(nullptr), resolve_data_(nullptr) { };

      Isolate(recipro::SnapshotData snapshot) 
        : isolate_(nullptr), allocator_(nullptr), creator_(nullptr), resolve_data_(nullptr) { 
          if (snapshot.data && snapshot.data_size > 0) {
            startup_data_.data = snapshot.data;
            startup_data_.raw_size = snapshot.data_size;
          }
      };

      ~Isolate() {
        if (creator_) {
          delete creator_;
          creator_ = NULL;
        } else {
          if (isolate_) {
            isolate_->Dispose();
            isolate_ = NULL;
          }
        }

        if (allocator_) {
          delete allocator_;
          allocator_ = NULL;
        }
      }

      void New();
      void NewForSnapshot();

      void RunIsolateScope(std::function<void(v8::Isolate *)> f) {
          v8::Isolate::Scope isolate_scope(isolate_);
          v8::HandleScope handle_scope(isolate_);
          
          f(isolate_);
      }

      bool Eval(const char *);

      void Reset() {
        context_.Reset();
      }
      void Reset(v8::Local<v8::Context> context) {
        context_.Reset(isolate_, context);
      }

      void DefaultContext(v8::Local<v8::Context> context) {
        if (creator_) {
          creator_->SetDefaultContext(context);
        }
      }

      v8::StartupData CreateSnapshotDataBlob(v8::SnapshotCreator::FunctionCodeHandling);
      
      int ModuleTree(const char *, const char *);

      ModuleInfo* FindModuleInfo(int id) {
        if (id == 0) {
          return nullptr;
        }

        auto it = specifier_map_.find(id);
        if (it != specifier_map_.end()) {
          return &it->second;
        } 
        
        return nullptr;
      }

      v8::Isolate* Raw() { return isolate_; }
      
      v8::Local<v8::Context> GetContext() {
        return context_.Get(isolate_);
      }

      v8::Local<v8::Context> GetContext(v8::Isolate *isolate) {
        return context_.Get(isolate);
      }

    private:
      bool HasSnapshot() {
        return startup_data_.raw_size > 0 && startup_data_.data != nullptr;
      }

      v8::ScriptOrigin GetScriptOrigin(const char *);

    private:
      v8::StartupData startup_data_;

      v8::Isolate* isolate_;
      v8::ArrayBuffer::Allocator* allocator_;
      v8::Persistent<v8::Context> context_;

      v8::SnapshotCreator* creator_;

      specifier_map specifier_map_;
      module_map module_map_;

    public:
      void *resolve_data_;
      ReciproResolevCallback resolve_callback_;
  };
};
