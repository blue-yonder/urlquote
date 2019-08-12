# -*- coding: utf-8 -*-
from urlquote import quote, unquote, quoting
import pytest
import sys

def test_quote_bytes():
    expected = u'/El%20Ni%C3%B1o/'.encode('utf-8')
    actual = quote(u'/El Niño/'.encode('utf-8'))
    assert expected == actual

def test_unquote_bytes():
    expected = u'/El Niño/'.encode('utf-8')
    actual = unquote(u'/El%20Ni%C3%B1o/'.encode('utf-8'))
    assert expected == actual

def test_quote_string():
    expected = u'/El%20Ni%C3%B1o/'.encode('utf-8')
    actual = quote(u'/El Niño/')
    assert expected == actual

def test_unquote_string():
    expected = u'/El Niño/'.encode('utf-8')
    actual = unquote(u'/El%20Ni%C3%B1o/')
    assert expected == actual

@pytest.mark.skipif(
    sys.version_info < (3,7),
    reason="requires python3")
def test_python_3_7_quoting():
    '''
    This test verifies that the userinfo encoding is identical with the defaul urllib encoding
    '''

    from urllib.parse import quote as urllib_quote

    quot = quoting.PYTHON_3_7_QUOTING

    # Control characters
    ascii_bytes = bytes(range(0, 32))
    ascii_str = ascii_bytes.decode('latin1')
    utf8_bytes = ascii_str.encode('utf-8')


    expected = "%00%01%02%03%04%05%06%07%08%09%0A%0B%0C%0D%0E%0F%10%11%12%13%14%15%16%17%18%19%1A%1B%1C%1D%1E%1F"
    actual = quote(utf8_bytes, quot).decode('utf-8')
    assert expected == actual

    # Printable
    ascii_bytes = bytes(range(32, 127))
    ascii_str = ascii_bytes.decode('latin1')
    utf8_bytes = ascii_str.encode('utf-8')


    # expected = "%20!%22%23$%&'()*+,-.%2F0123456789%3A%3B%3C%3D%3E%3F%40ABCDEFGHIJKLMNOPQRSTUVWXYZ%5B%5C%5D%5E_%60abcdefghijklmnopqrstuvwxyz%7B%7C%7D~"
    expected = urllib_quote(utf8_bytes)
    actual = quote(utf8_bytes, quot).decode('utf-8')
    assert expected == actual
