
typedef struct ValeConcreteRef {
  uint64_t unused[3];
} ValeConcreteRef;

typedef struct ValeInterfaceRef {
  uint64_t unused[4];
} ValeInterfaceRef;

#define DEFINE_CLOSURE_0(NAME, THIS_TYPE, RETURN_TYPE) \
  typedef struct NAME { \
    THIS_TYPE this; \
    RETURN_TYPE (*function)(THIS_TYPE); \
  } NAME; \
  RETURN_TYPE call##NAME(NAME closure) { \
    return closure.function(closure.this); \
  } \

#define DEFINE_CLOSURE_1(NAME, THIS_TYPE, RETURN_TYPE, PARAM_TYPE_1) \
  typedef struct NAME { \
    THIS_TYPE this; \
    RETURN_TYPE (*function)(THIS_TYPE, PARAM_TYPE_1); \
  } NAME; \
  RETURN_TYPE call##NAME(NAME closure, PARAM_TYPE_1 param1) { \
    return closure.function(closure.this, param1); \
  } \

#define DEFINE_CLOSURE_2(NAME, THIS_TYPE, RETURN_TYPE, PARAM_TYPE_1, PARAM_TYPE_2) \
  typedef struct NAME { \
    THIS_TYPE this; \
    RETURN_TYPE (*function)(THIS_TYPE, PARAM_TYPE_1, PARAM_TYPE_2); \
  } NAME; \
  RETURN_TYPE call##NAME(NAME closure, PARAM_TYPE_1 param1, PARAM_TYPE_2 param2) { \
    return closure.function(closure.this, param1, param2); \
  } \
