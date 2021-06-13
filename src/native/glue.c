#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include "ValeUtils.h"
#include "rocketvale/IRequestHandler.h"
#include "rocketvale/handleRequest.h"
#include "rocketvale/runServer.h"

DEFINE_CLOSURE_1(HandleRequestCallback, rocketvale_IRequestHandlerRef, ValeStr*, ValeStr*);

extern ValeStr* rocketvale_c_handle_request(rocketvale_IRequestHandlerRef handlerRef, ValeStr *request_body) {
  // rocketvale_IRequestHandlerRef handlerRef = *handlerRefPtr;

  printf("in c, handlerRef 0: %lld\n", handlerRef.unused0);
  printf("in c, handlerRef 1: %lld\n", handlerRef.unused1);
  printf("in c, handlerRef 2: %lld\n", handlerRef.unused2);
  printf("in c, handlerRef 3: %lld\n", handlerRef.unused3);

  printf("in c, request_body ptr: %p\n", request_body);
  printf("in c, request_body len: %d\n", request_body->length);
  printf("in c, request_body chars ptr: %p\n", request_body->chars);
  printf("in c, request_body chars: %s\n", request_body->chars);

  ValeStr* result = rocketvale_handleRequest(&handlerRef, request_body);

  printf("in c, result ptr: %p\n", result);
  printf("in c, result contents ptr: %p\n", &result->chars[0]);
  printf("in c, result len: %d\n", result->length);
  printf("in c, result chars: %s\n", result->chars);
  return ValeStrFrom(result->chars);
}

extern void rocketvale_rust_run_server(HandleRequestCallback* callback);

void rocketvale_runServer(rocketvale_IRequestHandlerRef* handlerRefPtr) {
  rocketvale_IRequestHandlerRef handlerRef = *handlerRefPtr;
  HandleRequestCallback callback = { handlerRef, rocketvale_c_handle_request };
  rocketvale_rust_run_server(&callback);
}
