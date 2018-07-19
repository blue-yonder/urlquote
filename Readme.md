Urlquote
========

Fast percent encoding / decoding for python.

Installation
------------

```bash
pip install urlquote
```

Usage
-----

```python
from urlquote import quote

quoted = quote('/El Niño/')
assert(quoted == '/El%20Ni%C3%B1o/'.encode('utf-8'))
```

`quote` operates utf-8 encoded bytes. If passed a string, it will enocde it into utf-8 first. It
will always return `utf-8` encoded bytes. `unquote` behaves the same way.

Encoding
--------

All ASCII characters less than hexidecimal 20 and greater than 7E are encoded. This includes special
charcters such as line feed, carriage return, NULL, etc. . Aside from these, space, double quote (")
, hash (#), inequality qualifiers (<), (>), backtick (`), question mark (?), and curly brackets ({),
(}) are also encoded.

Development
-----------

This library is a thin wrapper around the
[`percent-encoding`](https://crates.io/crates/percent-encoding) rust crate. It exposes part of its
functionality to python via a C-Interface using
[`milksnake`](https://github.com/getsentry/milksnake).