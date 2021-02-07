#include <safecoin_sdk.h>

struct foo {const uint8_t *input;};
void foo(const uint8_t *input, struct foo foo) ;

extern uint64_t entrypoint(const uint8_t *input) {
  struct foo f;
  f.input = input;
  foo(input, f);
  return SUCCESS;
}

void foo(const uint8_t *input, struct foo foo) {
  safe_log_64(0, 0, 0, (uint64_t)input, (uint64_t)foo.input);
  safe_assert(input == foo.input);
}
