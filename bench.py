# -*- coding: utf-8 -*-
'''I (mklein) have no idea how to write benchmarks in python. These benchmarks are highly
   doubtful!!!'''

from urlquote import quote, unquote
import timeit

if __name__ == "__main__":
    print("Quote")
    setup = "from urlquote import quote, unquote\ninput=u'/El Niño/'.encode('utf-8')"
    expression = "quote(input)"
    repetitions = 100000
    duration = timeit.timeit(expression, setup=setup, number = repetitions)
    print("Non trivial case:")
    print("Total duration: ", duration, "Average Duration: ", duration / repetitions)
    expression = "quote(input)"
    duration = timeit.timeit(expression, setup=setup, number = repetitions)
    print("Trivial case:")
    print("Total duration: ", duration, "Average Duration: ", duration / repetitions)

    # print("Unquote")
    # expression = "unquote('/El%20Ni%C3%B1o/')"
    # duration = timeit.timeit(expression, setup=setup, number = repetitions)
    # print("Non trivial case:")
    # print("Total duration: ", duration, "Average Duration: ", duration / repetitions)
    # expression = "unquote('/El-Nino/')"
    # duration = timeit.timeit(expression, setup=setup, number = repetitions)
    # print("Trivial case:")
    # print("Total duration: ", duration, "Average Duration: ", duration / repetitions)