#pragma once
#include <functional>

#include "include/v8.h"

namespace recipro {
  typedef struct {
    const char *data;
    int data_size;
  } SnapshotData ;

  class Isolate {
    public:
      Isolate() 
        : isolate_(nullptr), allocator_(nullptr), creator_(nullptr) { };

      Isolate(recipro::SnapshotData snapshot) 
        : isolate_(nullptr), allocator_(nullptr), creator_(nullptr) { 
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
          {
            v8::HandleScope handle_scope(isolate_);
            f(isolate_);
          }
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

      v8::Isolate* Raw() { 
        return isolate_; 
      }

    private:
      bool HasSnapshot() {
        return startup_data_.raw_size > 0 && startup_data_.data != nullptr;
      }

    private:
      v8::StartupData startup_data_;

      v8::Isolate* isolate_;
      v8::ArrayBuffer::Allocator* allocator_;
      v8::Persistent<v8::Context> context_;

      v8::SnapshotCreator* creator_;
  };
};
