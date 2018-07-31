# -*- coding: utf-8 -*-
from urlquote._native import ffi, lib
import six

# This buffer is passed to the C-Interface in order to obtain the quoted string. It will be
# reallocated automatically by `_native_quote`, if its size should not be large enough. It is ok to
# reset this buffer to a smaller value, but it always needs to be a valid buffer.
buffer = ffi.new('uint8_t[]', 1)

def _native_quote(value):
    """
    Urlencodes the given bytes
    """
    global buffer
    buffer_len = len(buffer)
    quoted_len = lib.quote(value, len(value), buffer, buffer_len)
    if quoted_len > buffer_len:
        # Our buffer has not been big enough to hold the quoted url. Let's allocate a buffer large
        # enough and try again.
        buffer = ffi.new('uint8_t[]', quoted_len)
        lib.quote(value, len(value), buffer, quoted_len)

    return ffi.string(buffer, quoted_len)

def _native_unquote(value):
    """
    Urldecodes the given bytes
    """
    decoded_len = lib.unquoted_len(value, len(value))
    if decoded_len == len(value):
        # Nothing to decode, let's save the allocation and return the original value
        return value
    else:
        # Unquoting the string is not the identity operation, so we need to allocate a new string.
        # We do this in python, so we don't need to explain to the gc how to deallocate it, once it
        # goes out of scope.
        buffer = ffi.new('uint8_t[]', decoded_len)
        # Let's fill the buffer with the unquoted string
        lib.unquote(value, len(value), buffer, decoded_len)
        return ffi.string(buffer)


def quote(value):
    """
    Performs string encoding and urlencodes the given string. Works faster with bytes. Always
    returns utf-8 encoded bytes
    """
    if not isinstance(value, six.binary_type):
        if not isinstance(value, six.text_type):
            value = str(value)
        value = value.encode('utf-8')
    
    return _native_quote(value)


def unquote(value):
    """
    Decodes a urlencoded string and performs necessary decoding depending on the used python version.
    """
    if not isinstance(value, six.binary_type):
        if not isinstance(value, six.text_type):
            value = str(value)
        value = value.encode('utf-8')
    
    return _native_unquote(value)