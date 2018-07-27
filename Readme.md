Urlquote
========

Fast percent encoding / decoding for python.

**This library is not intended to be a replacement for urllib**. `urllib` is part of the python
standard library and should be your goto choice to quote and unquote URLs. However, should quoting
/ unquoting URLs be known to be a performance bottleneck and you are fine with the encoding
described below, then by all means have fun using this library.

Usage
-----

```python
from urlquote import quote

quoted = quote('/El Niño/')
assert(quoted == '/El%20Ni%C3%B1o/'.encode('utf-8'))
```

Compatibility
-------------

Since this library uses a `cffi` interface it should work fine with any python version. For linux
the wheel has to be build against a libc with a version older or equal to the version of libc on the
platform the wheel will be executed on.

Installation
------------

```bash
pip install urlquote
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

Support
-------

This tool is provided as is under an MIT license without any warranty or SLA. You are free to use
it as part for any purpose, but the responsibility for operating it resides with you. We appreciate
your feedback though. Contributions on GitHub are welcome.