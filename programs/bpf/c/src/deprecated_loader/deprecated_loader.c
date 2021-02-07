/**
 * @brief Example C-based BPF program that prints out the parameters
 * passed to it
 */
#include <safecoin_sdk.h>
#include <deserialize_deprecated.h>

extern uint64_t entrypoint(const uint8_t *input) {
  SafeAccountInfo ka[1];
  SafeParameters params = (SafeParameters) { .ka = ka };

  safe_log(__FILE__);

  if (!safe_deserialize_deprecated(input, &params, SAFE_ARRAY_SIZE(ka))) {
    return ERROR_INVALID_ARGUMENT;
  }

  // Log the provided input parameters.  In the case of  the no-op
  // program, no account keys or input data are expected but real
  // programs will have specific requirements so they can do their work.
  safe_log_params(&params);
  return SUCCESS;
}
