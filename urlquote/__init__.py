# -*- coding: utf-8 -*-
import six

def quote(value):
    """
    Performs string encoding and urlencodes the given string. If the given value is not
    of string type, it will be cast using the `str` constructor
    """
    pass
    from six.moves.urllib.parse import quote as six_quote
    if not isinstance(value, (six.text_type, six.binary_type)):
        value = str(value)
    if isinstance(value, six.binary_type):
        value = value.decode('utf-8')
    r_val = six_quote(value.encode('utf-8'))
    return r_val


def unquote(value):
    """
    Decodes a urlencoded string and performs necessary decoding depending on the used python version.
    """
    from six.moves.urllib.parse import unquote
    if six.PY2:
        value = six.binary_type(value)
        return unquote(value).decode('utf-8')
    else:
        return unquote(value)