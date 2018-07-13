# -*- coding: utf-8 -*-
from urlquote import quote, unquote

def test_quote_bytes():
    expected = u'/El%20Ni%C3%B1o/'.encode('utf-8')
    actual = quote(u'/El Niño/'.encode('utf-8'))
    assert(expected == actual)

def test_unquote_bytes():
    expected = u'/El Niño/'.encode('utf-8')
    actual = unquote(u'/El%20Ni%C3%B1o/'.encode('utf-8'))
    assert(expected == actual)

def test_quote_string():
    expected = u'/El%20Ni%C3%B1o/'.encode('utf-8')
    actual = quote(u'/El Niño/')
    assert(expected == actual)

def test_unquote_string():
    expected = u'/El Niño/'.encode('utf-8')
    actual = unquote(u'/El%20Ni%C3%B1o/')
    assert(expected == actual)