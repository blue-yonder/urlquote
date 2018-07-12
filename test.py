# -*- coding: utf-8 -*-
from urlquote import quote, unquote

def test_quote_string():
    expected = u'/El%20Ni%C3%B1o/'
    actual = quote(u'/El Niño/')
    assert(expected == actual)

def test_unquote_string():
    expected = u'/El Niño/'
    actual = unquote(u'/El%20Ni%C3%B1o/')
    assert(expected == actual)