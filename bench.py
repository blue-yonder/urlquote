# -*- coding: utf-8 -*-
'''I (mklein) have no idea how to write benchmarks in python. These benchmarks are highly
   doubtful!!!'''

LOREMIPSUM = """Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod\
 tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud\
 exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in\
 reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint\
 occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.""".encode('utf-8')

import urlquote
from urllib.parse import quote as urllib_quote
import timeit

def benchmark_quote_urlquote():
    urlquote.quote(LOREMIPSUM, urlquote.quoting.PYTHON_3_7_QUOTING)

def benchmark_quote_urllib():
    urllib_quote(LOREMIPSUM)

if __name__ == "__main__":
    print("This library")
    setup = "from __main__ import benchmark_quote_urlquote"
    expression = "benchmark_quote_urlquote()"
    repetitions = 100000
    duration = timeit.timeit(expression, setup=setup, number = repetitions)
    print("Total duration: ", duration, "Average Duration: ", duration / repetitions)

    print("Urllib")
    setup = "from __main__ import benchmark_quote_urllib"
    expression = "benchmark_quote_urllib()"
    repetitions = 100000
    duration = timeit.timeit(expression, setup=setup, number = repetitions)
    print("Total duration: ", duration, "Average Duration: ", duration / repetitions)