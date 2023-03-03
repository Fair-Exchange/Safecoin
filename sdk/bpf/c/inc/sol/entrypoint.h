#pragma once
/**
 * @brief Safecoin program entrypoint
 */

#include <sol/constants.h>
#include <sol/types.h>
#include <sol/pubkey.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Keyed Account
 */
typedef struct {
  SafePubkey *key;      /** Public key of the account */
  uint64_t *lamports;  /** Number of lamports owned by this account */
  uint64_t data_len;   /** Length of data in bytes */
  uint8_t *data;       /** On-chain data within this account */
  SafePubkey *owner;    /** Program that owns this account */
  uint64_t rent_epoch; /** The epoch at which this account will next owe rent */
  bool is_signer;      /** Transaction was signed by this account's key? */
  bool is_writable;    /** Is the account writable? */
  bool executable;     /** This account's data contains a loaded program (and is now read-only) */
} SafeAccountInfo;

/**
 * Structure that the program's entrypoint input data is deserialized into.
 */
typedef struct {
  SafeAccountInfo* ka; /** Pointer to an array of SafeAccountInfo, must already
                          point to an array of SafeAccountInfos */
  uint64_t ka_num; /** Number of SafeAccountInfo entries in `ka` */
  const uint8_t *data; /** pointer to the instruction data */
  uint64_t data_len; /** Length in bytes of the instruction data */
  const SafePubkey *program_id; /** program_id of the currently executing program */
} SafeParameters;

/**
 * Program instruction entrypoint
 *
 * @param input Buffer of serialized input parameters.  Use sol_deserialize() to decode
 * @return 0 if the instruction executed successfully
 */
uint64_t entrypoint(const uint8_t *input);

#ifdef __cplusplus
}
#endif

/**@}*/
