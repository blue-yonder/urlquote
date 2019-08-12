#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * A `Quoting` decides which characters are going to be quoted.
 */
typedef struct Quoting Quoting;

extern const Quoting* DEFAULT_QUOTING;

extern const Quoting* PATH_SEGMENT_QUOTING;

extern const Quoting* PYTHON_3_7_QUOTING;

extern const Quoting* QUERY_QUOTING;

extern const Quoting* SIMPLE_QUOTING;

extern const Quoting* USERINFO_QUOTING;

/**
 * Fill the provided output buffer with the quoted string.
 * # Parameters
 *  input_buf: Non-null pointer to UTF-8-encoded character sequence to be quoted. A terminating
 * zero is not required.
 *  input_len: Number of bytes in input_buf (without terminating zero).
 *  output_buf: Non-null pointer to buffer which will hold the UTF-8-encoded output string. The
 * buffer should be big enough to hold the quoted string. This function is not going
 * to write beyond the bounds specified by `output_len`.
 *  output_len: Length of the output buffer.
 *  quoting: Determines which characters are going to be percent encoded and which ones are not
 * # Return value
 * The number of bytes required to hold the quoted string. By comparing `output_len` with the
 * returned value one can determine if the provided output buffer has been sufficient.
 */
uintptr_t quote(const uint8_t *input_buf,
                uintptr_t input_len,
                uint8_t *output_buf,
                uintptr_t output_len,
                const Quoting *quoting);

/**
 * Fill the provided output buffer with the unquoted string.
 * # Parameters
 *  input_buf: Non-null pointer to UTF-8-encoded character sequence to be unquoted. A terminating
 * zero is not required.
 *  input_len: Number of bytes in input_buf (without terminating zero).
 *  output_buf: Non-null pointer to buffer which will hold the UTF-8-encoded output string. The
 * buffer should be big enough to hold the unquoted string. This function is not
 * going to write beyond the bounds specified by `output_len`.
 *  output_len: Length of the output buffer.
 * # Return value
 * The number of bytes required to hold the unquoted string. By comparing `output_len` with the
 * returned value one can determine if the provided output buffer has been sufficient.
 */
uintptr_t unquote(const uint8_t *input_buf,
                  uintptr_t input_len,
                  uint8_t *output_buf,
                  uintptr_t output_len);
