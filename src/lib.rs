use percent_encoding::percent_decode;
use quoting::{Quote, Quoting};
use std::slice;

mod quoting;

pub use quoting::{
    DEFAULT_QUOTING, PATH_SEGMENT_QUOTING, IDEMPOTENT_QUOTING, PYTHON_3_7_QUOTING,
    QUERY_QUOTING, SIMPLE_QUOTING, USERINFO_QUOTING,
};

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
///
/// # Safety
///
/// * input_buf: Must not be zero and must point to readable memory.
/// * input_len: Must not be larger than the `input_buf` length.
/// * ouput_buf: Must not be zero and must point to writeable memory. May not overlap with
///              `input_buf`.
/// * output_len: Must not be larger than the `output_buf` length.
/// * quoting: Must point to an instance of Quoting, which is valid, for the duration of the
///            function call.
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

    (&*quoting).quote(input, output)
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
///
/// # Safety
///
/// * input_buf: Must not be zero and must point to readable memory.
/// * input_len: Must not be larger than the `input_buf` length.
/// * output_buf: Non-null pointer to buffer which will hold the UTF-8-encoded output string. The
///               buffer should be big enough to hold the unquoted string. This function is not
///               going to write beyond the bounds specified by `output_len`.
/// * output_len: Length of the output buffer.
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
    use percent_encoding::utf8_percent_encode;
    use quoting::DEFAULT_QUOTING;

    #[test]
    fn quoting_works() {
        assert_eq!(
            utf8_percent_encode("/El Niño/", &DEFAULT_QUOTING.0).to_string(),
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
            utf8_percent_encode("/El Niño/", &DEFAULT_QUOTING.0).to_string(),
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
    #[test]
    fn test_already_percent_encoded() {
        let original = "hello%20world";
        let quoted = unsafe {
            let mut buf = vec![0; 100];
            let len = quote(
                original.as_ptr(),
                original.len(),
                buf.as_mut_ptr(),
                buf.len(),
                IDEMPOTENT_QUOTING,
            );
            String::from_utf8(buf[..len].to_vec()).unwrap()
        };
        assert_eq!(
            quoted, "hello%20world",
            "Should preserve existing percent-encoding"
        );
    }

    #[test]
    fn test_double_pass_stability() {
        let quoted = "hello%20world";
        let double_quoted = unsafe {
            let mut buf = vec![0; 100];
            let len = quote(
                quoted.as_ptr(),
                quoted.len(),
                buf.as_mut_ptr(),
                buf.len(),
                IDEMPOTENT_QUOTING,
            );
            String::from_utf8(buf[..len].to_vec()).unwrap()
        };
        assert_eq!(
            double_quoted, quoted,
            "Second pass should be identical to first"
        );
    }

    #[test]
    fn test_mixed_content() {
        let mixed = "hello%20world & goodbye space";
        let quoted_mixed = unsafe {
            let mut buf = vec![0; 100];
            let len = quote(
                mixed.as_ptr(),
                mixed.len(),
                buf.as_mut_ptr(),
                buf.len(),
                IDEMPOTENT_QUOTING,
            );
            String::from_utf8(buf[..len].to_vec()).unwrap()
        };
        assert_eq!(
            quoted_mixed, "hello%20world%20%26%20goodbye%20space",
            "Should preserve existing encoding while encoding new special chars"
        );
    }

    #[test]
    fn test_special_characters() {
        let special = "hello world < > \"";
        let quoted_special = unsafe {
            let mut buf = vec![0; 100];
            let len = quote(
                special.as_ptr(),
                special.len(),
                buf.as_mut_ptr(),
                buf.len(),
                IDEMPOTENT_QUOTING,
            );
            String::from_utf8(buf[..len].to_vec()).unwrap()
        };
        assert_eq!(
            quoted_special, "hello%20world%20%3C%20%3E%20%22",
            "Should encode all special characters"
        );
    }
}
