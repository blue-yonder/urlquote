Urlquote
========

Fast percent encoding / decoding for python.

**This library is not intended to be a replacement for urllib**. `urllib` is part of the Python standard library and should be your go-to choice to quote and unquote URLs. However, should quoting or unquoting of URLs be known to be a performance bottleneck and you are fine with the encoding described below, then by all means have fun using this library.

Usage
-----

```python
from urlquote import quote

quoted = quote('/El Niño/')
assert(quoted == '/El%20Ni%C3%B1o/'.encode('utf-8'))
```

Compatibility
-------------

Since this library uses a `cffi` interface it should work fine with any version of Python. For Linux the wheel has to be build against a version of libc older or equal to the version of libc on the platform the wheel will be used on.

Installation
------------

```bash
pip install urlquote
```

`quote` operates on UTF-8-encoded bytes. If passed a string, it will encode it into UTF-8 first. It will always return UTF-8-encoded bytes. `unquote` behaves the same way.

Encoding
--------

The following encodings are offered. `DEFAULT_QUOTING` is used in case the quoting parameter is not specified.

| Name                 | Additional encoded characters                                                                                          |
|----------------------|------------------------------------------------------------------------------------------------------------------------|
| SIMPLE_QUOTING       |                                                                                                                        |
| DEFAULT_QUOTING      | space, `<`,`>`,`` ` ``,`?`,`{`,`}`                                                                                     |
| QUERY_QUOTING        | space, `"`,`#`,`<`,`>`                                                                                                 |
| PATH_SEGMENT_QUOTING | space, `"`,`#`,`<`,`>`,`` ` ``,`?`,`%`,`/`                                                                             |
| USERINFO_QUOTING     | space, `"`,`#`,`<`,`>`,`` ` ``,`?`,`{`,`}`,`/`,`:`,`;`,`=`,`@`,`\`,`[`,`]`,`^`,`|`                                     |
| PYTHON_3_7_QUOTING   | space, `"`,`#`,`<`,`>`,`` ` ``,`?`,`{`,`}`,`$`,`%`,`&`,`\`,`(`,`)`,`,`,`=`,`;`,`:`,`!`,`\`,`@`,`[`,`]`,`^`,`|`,`+`,`*` |

Non printable and non standard ASCII characters are always quoted. The `PYTHON_3_7_QUOTING` is going to work the same way in every Python version the name is only refering to the `urllib` default encoding used in Python 3.7.

Development
-----------
a
This library is a thin wrapper around the Rust crate [`percent-encoding`](https://crates.io/crates/percent-encoding). It exposes part of its functionality to python via a C interface using [`milksnake`](https://github.com/getsentry/milksnake).

To build it you need to [install Rust and Cargo](https://www.rust-lang.org/en-US/install.html). Than you can proceed to build the wheel with:

```bash
python setup.py build sdist bdist_wheel
```

To execute the Python tests use:

```bash
pip install -e .
pytest test.py
```

There are also some Rust-only unit tests. To execute them change into the `rust` subdirectory and call.

```bash
cargo test
```

With the nightly toolchain installed you may also call the Rust-only benchmarks using:

```bash
cargo +nightly bench
```

Support
-------

This tool is provided as is under an MIT license without any warranty or SLA. You are free to use it as part for any purpose, but the responsibility for operating it resides with you. We appreciate your feedback though. Contributions on GitHub are welcome.
