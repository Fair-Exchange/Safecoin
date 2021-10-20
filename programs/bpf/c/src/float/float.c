/**
 * @brief Example C based BPF program that performs operations
 * on floating point values.  The test fails if floating point
 * emulation functions were not linked to the module.
 */
#include <solana_sdk.h>

extern uint64_t entrypoint(const uint8_t *input) {
  SafeAccountInfo ka[1];
  SafeParameters params = (SafeParameters) { .ka = ka };

  if (!sol_deserialize(input, &params, SAFE_ARRAY_SIZE(ka))) {
    return ERROR_INVALID_ARGUMENT;
  }
  uint32_t *data = (uint32_t *)(params.ka[0].data);
  *data += 1.5;

  return SUCCESS;
}
