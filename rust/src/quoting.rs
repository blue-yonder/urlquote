use percent_encoding::{
    define_encode_set, percent_encode, EncodeSet, DEFAULT_ENCODE_SET, PATH_SEGMENT_ENCODE_SET,
    QUERY_ENCODE_SET, SIMPLE_ENCODE_SET, USERINFO_ENCODE_SET,
};

/// All ASCII charcters less than hexidecimal 20 and greater than 7E are encoded.  This includes
/// special charcters such as line feed, carriage return, NULL, etc.
#[no_mangle]
pub static SIMPLE_QUOTING: &Quoting = &Quoting(&SIMPLE_ENCODE_SET);

/// This quoting is used for url path components.
///
/// Aside from special chacters defined in the `SIMPLE_QUOTING`, space, double quote ("), hash (#),
/// inequality qualifiers (<), (>), backtick (`), question mark (?), and curly brackets ({), (}) are
/// encoded.
#[no_mangle]
pub static DEFAULT_QUOTING: &Quoting = &Quoting(&DEFAULT_ENCODE_SET);

/// This quoting is used for url query strings.
///
/// Aside from special chacters defined in the `SIMPLE_QUOTING`, space, double quote ("), hash (#),
/// and inequality qualifiers (<), (>) are encoded.
#[no_mangle]
pub static QUERY_QUOTING: &Quoting = &Quoting(&QUERY_ENCODE_SET);

/// This quoting is used for on '/'-separated path segment
///
/// Aside from special chacters defined in the `SIMPLE_QUOTING`, space, double quote ("), hash (#),
/// inequality qualifiers (<), (>), backtick (`), question mark (?), and curly brackets ({), (}),
/// percent sign (%), forward slash (/) are encoded.
#[no_mangle]
pub static PATH_SEGMENT_QUOTING: &Quoting = &Quoting(&PATH_SEGMENT_ENCODE_SET);

/// This quoting is used for username and password.
///
/// Aside from special chacters defined in the `SIMPLE_QUOTING`, space, double quote ("), hash (#),
/// inequality qualifiers (<), (>), backtick (`), question mark (?), and curly brackets ({), (}),
/// forward slash (/), colon (:), semi-colon (;), equality (=), at (@), backslash (\\), square
/// brackets ([), (]), caret (\^), and pipe (|) are encoded.
#[no_mangle]
pub static USERINFO_QUOTING: &Quoting = &Quoting(&USERINFO_ENCODE_SET);

define_encode_set! {
    /// This emulates the urllib default encoding used by Python 3.7.
    pub PYTHON_3_7_ENCODE_SET = [DEFAULT_ENCODE_SET] | {'$', '%', '&', '\'', '(', ')', ',', '=', ';',':','!','\\','@','[',']','^','|','+','*'}
}

#[no_mangle]
pub static PYTHON_3_7_QUOTING: &Quoting = &Quoting(&PYTHON_3_7_ENCODE_SET);

/// A `Quoting` decides which characters are going to be quoted.
pub struct Quoting(
    // This is an opaque public strict type alias in order to avoid talking about
    // `&(dyn Quote + Sync)` in the C-Interface
    &'static (dyn Quote + Sync),
);

impl Quoting {
    /// Fill the provided output buffer with the quoted string.
    ///
    /// # Parameters
    ///
    /// * input: UTF-8-encoded character sequence to be quoted.
    /// * output: Buffer which will hold the UTF-8-encoded output string.
    ///
    /// # Return value
    ///
    /// The number of bytes required to hold the quoted string. By comparing `output.len()` with the
    /// returned value one can determine if the provided output buffer has been sufficient.
    pub fn quote(&self, input: &[u8], output: &mut [u8]) -> usize {
        self.0.quote(input, output)
    }
}

trait Quote {
    fn quote(&self, input: &[u8], output: &mut [u8]) -> usize;
}

impl<E: EncodeSet> Quote for E {
    fn quote(&self, input: &[u8], output: &mut [u8]) -> usize {
        let mut index = 0;
        let mut quoted_bytes = percent_encode(input, self.clone()).flat_map(str::bytes);

        for byte in (&mut quoted_bytes).take(output.len()) {
            output[index] = byte;
            index += 1;
        }

        // The number of bytes required to hold the quoted string
        index + quoted_bytes.count()
    }
}
