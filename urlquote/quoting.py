from urlquote._native import lib

# All ASCII charcters less than hexidecimal 20 and greater than 7E are encoded.  This includes
# special charcters such as line feed, carriage return, NULL, etc.
SIMPLE_QUOTING = lib.SIMPLE_QUOTING

# This quoting is used for url path components.
#
# Aside from special chacters defined in the `SIMPLE_QUOTING`, space, double quote ("), hash (#),
# inequality qualifiers (<), (>), backtick (`), question mark (?), and curly brackets ({), (}) are
# encoded.
DEFAULT_QUOTING = lib.DEFAULT_QUOTING

# This quoting is used for url query strings.
#
# Aside from special chacters defined in the `SIMPLE_QUOTING`, space, double quote ("), hash (#),
# and inequality qualifiers (<), (>) are encoded.
QUERY_QUOTING = lib.QUERY_QUOTING

# This quoting is used for on '/'-separated path segment
#
# Aside from special chacters defined in the `SIMPLE_QUOTING`, space, double quote ("), hash (#),
# inequality qualifiers (<), (>), backtick (`), question mark (?), and curly brackets ({), (}),
# percent sign (%), forward slash (/) are encoded.
PATH_SEGMENT_QUOTING = lib.PATH_SEGMENT_QUOTING

# This quoting is used for username and password.
#
# Aside from special chacters defined in the `SIMPLE_QUOTING`, space, double quote ("), hash (#),
# inequality qualifiers (<), (>), backtick (`), question mark (?), and curly brackets ({), (}),
# forward slash (/), colon (:), semi-colon (;), equality (=), at (@), backslash (\\), square
# brackets ([), (]), caret (\^), and pipe (|) are encoded.
USERINFO_QUOTING = lib.USERINFO_QUOTING

# This is the default quoting used by urlib.parse.quote
#
# Quotes everything but alphanumeric letters, numbers and dash (-), underscore (_), slash (/),
# point (.) and tilde (~).
PYTHON_3_7_QUOTING = lib.PYTHON_3_7_QUOTING