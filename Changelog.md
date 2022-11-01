Changelog
=========

2.0.0
-----

* Dropped support for Python 2
* Lost dependency to six
* Switched build system to maturin
* Stopped vendoring `percent-encoding`
* Switched to Rust edition 2021
* Removed benches

1.1.4
-----

No changes made. This release is just to test the CI-Pipeline using API-tokens.

1.1.3
-----

* Fix: In order to prevent race conditions the global `buffer` variable has been made threadlocal.

1.1.2
-----

* Vendor dependency `percent-encoding` in order ease build for enterprises who do not have access to the internet on the build server.

1.1.1
-----

* Fix: Markdown in Readme has not been rendered correctly due to not escaping `|` inside table.
* Fix: `unquote` would trigger a `TypeError` if the buffer has been not big enough to hold the unquoted bytes.

1.1.0
-----

* Support for additional encodings, selectable at runtime:
  * `SIMPLE_QUOTING`
  * `QUERY_QUOTING`
  * `PATH_SEGMENT_QUOTING`
  * `USERINFO_QUOTING`

1.0.0
-----

* Initial Release. Provides two functions:
  * `quote`
  * `unquote`
