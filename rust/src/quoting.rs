use percent_encoding::{percent_encode, AsciiSet, CONTROLS};

/// All ASCII charcters less than hexidecimal 20 and greater than 7E are encoded.  This includes
/// special charcters such as line feed, carriage return, NULL, etc.
#[no_mangle]
pub static SIMPLE_QUOTING: &Quoting = CONTROLS;

/// This quoting is used for url path components.
///
/// Aside from special chacters defined in the `SIMPLE_QUOTING`, space, double quote ("), hash (#),
/// inequality qualifiers (<), (>), backtick (`), question mark (?), and curly brackets ({), (}) are
/// encoded.
#[no_mangle]
pub static DEFAULT_QUOTING: &Quoting = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'?')
    .add(b'{')
    .add(b'}');

#[no_mangle]
/// This quoting is used for url query strings.
///
/// Aside from special chacters defined in the `SIMPLE_QUOTING`, space, double quote ("), hash (#),
/// and inequality qualifiers (<), (>) are encoded.
pub static QUERY_QUOTING: &Quoting = &CONTROLS.add(b' ').add(b'"').add(b'#').add(b'<').add(b'>');

/// This quoting is used for on '/'-separated path segment
///
/// Aside from special chacters defined in the `SIMPLE_QUOTING`, space, double quote ("), hash (#),
/// inequality qualifiers (<), (>), backtick (`), question mark (?), and curly brackets ({), (}),
/// percent sign (%), forward slash (/) are encoded.
#[no_mangle]
pub static PATH_SEGMENT_QUOTING: &Quoting = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'?')
    .add(b'{')
    .add(b'}')
    .add(b'%')
    .add(b'/');

/// This quoting is used for username and password.
///
/// Aside from special chacters defined in the `SIMPLE_QUOTING`, space, double quote ("), hash (#),
/// inequality qualifiers (<), (>), backtick (`), question mark (?), and curly brackets ({), (}),
/// forward slash (/), colon (:), semi-colon (;), equality (=), at (@), backslash (\\), square
/// brackets ([), (]), caret (\^), and pipe (|) are encoded.
#[no_mangle]
pub static USERINFO_QUOTING: &Quoting = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'?')
    .add(b'{')
    .add(b'}')
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'=')
    .add(b'@')
    .add(b'\\')
    .add(b'[')
    .add(b']')
    .add(b'^')
    .add(b'|');

/// This emulates the urllib default encoding used by Python 3.7.
#[no_mangle]
pub static PYTHON_3_7_QUOTING: &Quoting = &DEFAULT_QUOTING
    .add(b'$')
    .add(b'%')
    .add(b'&')
    .add(b'\'')
    .add(b'(')
    .add(b')')
    .add(b',')
    .add(b'=')
    .add(b';')
    .add(b':')
    .add(b'!')
    .add(b'\\')
    .add(b'@')
    .add(b'[')
    .add(b']')
    .add(b'^')
    .add(b'|')
    .add(b'+')
    .add(b'*');

// This is an opaque public strict type alias in order to avoid talking about
// `&'static AsciiSet` in the C-Interface
pub type Quoting = AsciiSet;

pub trait Quote {
    fn quote(&self, input: &[u8], output: &mut [u8]) -> usize;
}

impl Quote for &'static AsciiSet {
    fn quote(&self, input: &[u8], output: &mut [u8]) -> usize {
        let mut index = 0;
        let mut quoted_bytes = percent_encode(input, self).flat_map(str::bytes);

        for byte in (&mut quoted_bytes).take(output.len()) {
            output[index] = byte;
            index += 1;
        }

        // The number of bytes required to hold the quoted string
        index + quoted_bytes.count()
    }
}
