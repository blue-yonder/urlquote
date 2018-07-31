extern crate percent_encoding;
use percent_encoding::{percent_decode, percent_encode, DEFAULT_ENCODE_SET};
use std::slice;

/// Returns the quoted length of the provided utf-8 encoded input string.
///
/// # Parameters
///
/// * input_buf: Non Null pointer to utf-8 encoded character sequence to be quoted. A terminating
///              zero is not required.
/// * input_len: Number of bytes in input_buf (Without terminating zero).
#[no_mangle]
pub unsafe extern "C" fn quoted_len(input_buf: *const u8, input_len: usize) -> usize {
    let input = slice::from_raw_parts(input_buf, input_len);
    percent_encode(input, DEFAULT_ENCODE_SET)
        .map(str::len)
        .sum()
}

/// Fill the provided output buffer with the quoted string.
///
/// # Parameters
///
/// * input_buf: Non Null pointer to utf-8 encoded character sequence to be quoted. A terminating
///              zero is not required.
/// * input_len: Number of bytes in input_buf (Without terminating zero).
/// * output_buf: Non Null pointer to buffer which will hold the utf-8 encoded output string. The
///               buffer should be big enough to hold the quoted string. This function is not going
///               to write beyond the bounds specified by `output_len`.
/// * output_len: Length of the output buffer.
/// 
/// # Return value
/// 
/// The number of bytes requiered to hold the quoted string. By comparing `output_len` with the
/// returned value one can determine, if the provided output buffer has been sufficient.
#[no_mangle]
pub unsafe extern "C" fn quote(
    input_buf: *const u8,
    input_len: usize,
    output_buf: *mut u8,
    output_len: usize,
) -> usize {
    let input = slice::from_raw_parts(input_buf, input_len);
    let output = slice::from_raw_parts_mut(output_buf, output_len);

    let mut index = 0;
    let mut quoted_bytes = percent_encode(input, DEFAULT_ENCODE_SET).flat_map(str::bytes);

    for byte in (&mut quoted_bytes)
        .take(output_len)
    {
        output[index] = byte;
        index += 1;
    }

    if index < output_len{
        // Buffer has been large enough to hold the quoted string, with space to spare.
        index
    } else {
        quoted_bytes.count() + output_len
    }
}

/// Returns the unquoted length of the provided utf-8 encoded and percent quoted input string.
///
/// # Parameters
///
/// * input_buf: Non Null pointer to utf-8 encoded character sequence to be unquoted. A terminating
///              zero is not required.
/// * input_len: Number of bytes in input_buf (Without terminating zero).
#[no_mangle]
pub unsafe extern "C" fn unquoted_len(input_buf: *const u8, input_len: usize) -> usize {
    let input = slice::from_raw_parts(input_buf, input_len);
    percent_decode(input).count()
}

/// Fill the provided output buffer with the unquoted string.
///
/// # Parameters
///
/// * input_buf: Non Null pointer to utf-8 encoded character sequence to be unquoted. A terminating
///              zero is not required.
/// * input_len: Number of bytes in input_buf (Without terminating zero).
/// * output_buf: Non Null pointer to buffer which will hold the utf-8 encoded output string. The
///               buffer should be big enough to hold the unquoted string. This function is not
///               going to write beyond the bounds specified by `output_len`. This makes it important
///               to call `quoted_len` before allocating the buffer.
/// * output_len: Length of the output buffer.
#[no_mangle]
pub unsafe extern "C" fn unquote(
    input_buf: *const u8,
    input_len: usize,
    output_buf: *mut u8,
    output_len: usize,
) {
    let input = slice::from_raw_parts(input_buf, input_len);
    let output = slice::from_raw_parts_mut(output_buf, output_len);

    for (index, byte) in percent_decode(input).enumerate().take(output_len) {
        output[index] = byte
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use percent_encoding::utf8_percent_encode;

    #[test]
    fn quoting_works() {
        assert_eq!(
            utf8_percent_encode("/El Niño/", DEFAULT_ENCODE_SET).to_string(),
            "/El%20Ni%C3%B1o/"
        );

        let input = "/El Niño/";
        unsafe {
            let buf_len = quoted_len(input.as_ptr(), input.len());
            assert_eq!(buf_len, "/El%20Ni%C3%B1o/".len());
            let mut buf = vec![0; buf_len];
            quote(
                input.as_bytes().as_ptr(),
                input.len(),
                buf.as_mut_ptr(),
                buf.len(),
            );
            let quoted = String::from_utf8(buf).unwrap();
            assert_eq!(quoted, "/El%20Ni%C3%B1o/");
        }
    }

    #[test]
    fn unquoting_works() {
        assert_eq!(
            utf8_percent_encode("/El Niño/", DEFAULT_ENCODE_SET).to_string(),
            "/El%20Ni%C3%B1o/"
        );

        let input = "/El%20Ni%C3%B1o/";
        unsafe {
            let buf_len = unquoted_len(input.as_ptr(), input.len());
            assert_eq!(buf_len, "/El Niño/".len());
            let mut buf = vec![0; buf_len];
            unquote(
                input.as_bytes().as_ptr(),
                input.len(),
                buf.as_mut_ptr(),
                buf.len(),
            );
            let unquoted = String::from_utf8(buf).unwrap();
            assert_eq!(unquoted, "/El Niño/");
        }
    }
}
