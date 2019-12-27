#include <node.h>
#include <stdint.h>
#include "src/test.h"

namespace demo {

using v8::FunctionCallbackInfo;
using v8::Isolate;
using v8::Local;
using v8::NewStringType;
using v8::Object;
using v8::String;
using v8::Value;

void HelloMethod(const FunctionCallbackInfo<Value>& args) {
    args.GetReturnValue().Set(hello());
}

void GetMethod(const FunctionCallbackInfo<Value>& args) {
    args.GetReturnValue().Set(get());
}

void PostMethod(const FunctionCallbackInfo<Value>& args) {
    args.GetReturnValue().Set(post());
}

void GetWithRequestsMethod(const FunctionCallbackInfo<Value>& args) {
    args.GetReturnValue().Set(get_with_requests());
}

void VersionMethod(const FunctionCallbackInfo<Value>& args) {
    openssl_version_info();
}

void Initialize(Local<Object> exports) {
    NODE_SET_METHOD(exports, "hello", HelloMethod);
    NODE_SET_METHOD(exports, "get", GetMethod);
    NODE_SET_METHOD(exports, "post", PostMethod);
    NODE_SET_METHOD(exports, "get_with_requests", GetWithRequestsMethod);
    NODE_SET_METHOD(exports, "openssl_version_info", VersionMethod);
}

NODE_MODULE(NODE_GYP_MODULE_NAME, Initialize)

}  // namespace demo