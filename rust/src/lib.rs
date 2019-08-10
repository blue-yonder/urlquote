use percent_encoding::percent_decode;
use quoting::Quoting;
use std::slice;

mod quoting;

pub use quoting::{DEFAULT_QUOTING, PATH_SEGMENT_QUOTING, QUERY_QUOTING, USERINFO_QUOTING};

/// Fill the provided output buffer with the quoted string.
///
/// # Parameters
///
/// * input_buf: Non-null pointer to UTF-8-encoded character sequence to be quoted. A terminating
///              zero is not required.
/// * input_len: Number of bytes in input_buf (without terminating zero).
/// * output_buf: Non-null pointer to buffer which will hold the UTF-8-encoded output string. The
///               buffer should be big enough to hold the quoted string. This function is not going
///               to write beyond the bounds specified by `output_len`.
/// * output_len: Length of the output buffer.
/// * quoting: Determines which characters are going to be percent encoded and which ones are not
///
/// # Return value
///
/// The number of bytes required to hold the quoted string. By comparing `output_len` with the
/// returned value one can determine if the provided output buffer has been sufficient.
#[no_mangle]
pub unsafe extern "C" fn quote(
    input_buf: *const u8,
    input_len: usize,
    output_buf: *mut u8,
    output_len: usize,
    quoting: *const Quoting,
) -> usize {
    let input = slice::from_raw_parts(input_buf, input_len);
    let output = slice::from_raw_parts_mut(output_buf, output_len);

    (*quoting).quote(input, output)
}

/// Fill the provided output buffer with the unquoted string.
///
/// # Parameters
///
/// * input_buf: Non-null pointer to UTF-8-encoded character sequence to be unquoted. A terminating
///              zero is not required.
/// * input_len: Number of bytes in input_buf (without terminating zero).
/// * output_buf: Non-null pointer to buffer which will hold the UTF-8-encoded output string. The
///               buffer should be big enough to hold the unquoted string. This function is not
///               going to write beyond the bounds specified by `output_len`.
/// * output_len: Length of the output buffer.
///
/// # Return value
///
/// The number of bytes required to hold the unquoted string. By comparing `output_len` with the
/// returned value one can determine if the provided output buffer has been sufficient.
#[no_mangle]
pub unsafe extern "C" fn unquote(
    input_buf: *const u8,
    input_len: usize,
    output_buf: *mut u8,
    output_len: usize,
) -> usize {
    let input = slice::from_raw_parts(input_buf, input_len);
    let output = slice::from_raw_parts_mut(output_buf, output_len);

    let mut index = 0;
    let mut unquoted_bytes = percent_decode(input);

    for byte in (&mut unquoted_bytes).take(output_len) {
        output[index] = byte;
        index += 1;
    }

    // The number of bytes required to hold the unquoted string
    index + unquoted_bytes.count()
}

#[cfg(test)]
mod tests {

    use super::*;
    use percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
    use quoting::DEFAULT_QUOTING;

    #[test]
    fn quoting_works() {
        assert_eq!(
            utf8_percent_encode("/El Niño/", DEFAULT_ENCODE_SET).to_string(),
            "/El%20Ni%C3%B1o/"
        );

        let input = "/El Niño/";
        unsafe {
            let mut buf = vec![0; 10];
            let buf_len = quote(
                input.as_ptr(),
                input.len(),
                buf.as_mut_ptr(),
                buf.len(),
                DEFAULT_QUOTING,
            );
            assert_eq!(buf_len, "/El%20Ni%C3%B1o/".len());
            let mut buf = vec![0; buf_len];
            quote(
                input.as_ptr(),
                input.len(),
                buf.as_mut_ptr(),
                buf.len(),
                DEFAULT_QUOTING,
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
        let mut buf = vec![0; 1];
        unsafe {
            let buf_len = unquote(input.as_ptr(), input.len(), buf.as_mut_ptr(), buf.len());
            assert_eq!(buf_len, "/El Niño/".len());
            let mut buf = vec![0; buf_len];
            unquote(input.as_ptr(), input.len(), buf.as_mut_ptr(), buf.len());
            let unquoted = String::from_utf8(buf).unwrap();
            assert_eq!(unquoted, "/El Niño/");
        }
    }
}
