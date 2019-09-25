# -*- coding: utf-8 -*-
from urlquote._native import ffi, lib
from .quoting import DEFAULT_QUOTING
import six
import threading

threadlocal = threading.local()

# This buffer is passed to the C interface in order to obtain the quoted string. It will be
# reallocated automatically by `_native_quote` and `_native_unquote` if its size should not be
# large enough. It is ok to reset this buffer to a smaller value but it always needs to be a valid
# buffer.
threadlocal.buffer = ffi.new('uint8_t[]', 1)

def _native_quote(value, quoting):
    """
    Urlencodes the given bytes
    """
    buffer_len = len(threadlocal.buffer)
    quoted_len = lib.quote(value, len(value), threadlocal.buffer, buffer_len, quoting)
    if quoted_len > buffer_len:
        # Our buffer has not been big enough to hold the quoted url. Let's allocate a buffer large
        # enough and try again.
        threadlocal.buffer = ffi.new('uint8_t[]', quoted_len)
        lib.quote(value, len(value), threadlocal.buffer, quoted_len, quoting)

    return ffi.string(threadlocal.buffer, quoted_len)

def _native_unquote(value):
    """
    Urldecodes the given bytes
    """
    buffer_len = len(threadlocal.buffer)
    unquoted_len = lib.unquote(value, len(value), threadlocal.buffer, buffer_len)
    if unquoted_len > buffer_len:
        # Our buffer has not been big enough to hold the unquoted url. Let's allocate a buffer large
        # enough and try again.
        threadlocal.buffer = ffi.new('uint8_t[]', unquoted_len)
        lib.unquote(value, len(value), threadlocal.buffer, unquoted_len)

    return ffi.string(threadlocal.buffer, unquoted_len)

def quote(value, quoting = DEFAULT_QUOTING):
    """
    Performs string encoding and urlencodes the given string. Always returns utf-8 encoded bytes.
    """
    if not isinstance(value, six.binary_type):
        if not isinstance(value, six.text_type):
            value = str(value)
        value = value.encode('utf-8')

    return _native_quote(value, quoting)


def unquote(value):
    """
    Decodes a urlencoded string and performs necessary decoding depending on the used Python version.
    """
    if not isinstance(value, six.binary_type):
        if not isinstance(value, six.text_type):
            value = str(value)
        value = value.encode('utf-8')

    return _native_unquote(value)
