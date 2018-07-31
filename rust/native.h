#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

/*
 * Fill the provided output buffer with the quoted string.
 *
 * # Parameters
 *
 * * input_buf: Non Null pointer to utf-8 encoded character sequence to be quoted. A terminating
 *              zero is not required.
 * * input_len: Number of bytes in input_buf (Without terminating zero).
 * * output_buf: Non Null pointer to buffer which will hold the utf-8 encoded output string. The
 *               buffer should be big enough to hold the quoted string. This function is not going
 *               to write beyond the bounds specified by `output_len`.
 * * output_len: Length of the output buffer.
 *
 * # Return value
 *
 * The number of bytes requiered to hold the quoted string. By comparing `output_len` with the
 * returned value one can determine, if the provided output buffer has been sufficient.
 */
uintptr_t quote(const uint8_t *input_buf,
                uintptr_t input_len,
                uint8_t *output_buf,
                uintptr_t output_len);

/*
 * Fill the provided output buffer with the unquoted string.
 *
 * # Parameters
 *
 * * input_buf: Non Null pointer to utf-8 encoded character sequence to be unquoted. A terminating
 *              zero is not required.
 * * input_len: Number of bytes in input_buf (Without terminating zero).
 * * output_buf: Non Null pointer to buffer which will hold the utf-8 encoded output string. The
 *               buffer should be big enough to hold the unquoted string. This function is not
 *               going to write beyond the bounds specified by `output_len`.
 * * output_len: Length of the output buffer.
 *
 * # Return value
 *
 * The number of bytes requiered to hold the unquoted string. By comparing `output_len` with the
 * returned value one can determine, if the provided output buffer has been sufficient.
 */
uintptr_t unquote(const uint8_t *input_buf,
                  uintptr_t input_len,
                  uint8_t *output_buf,
                  uintptr_t output_len);