# -*- coding: utf-8 -*-
from urlquote import quote, unquote
import timeit

if __name__ == "__main__":
    print("Quote")
    expression = "quote('/El Niño/')"
    setup = "from urlquote import quote, unquote"
    repetitions = 100
    duration = timeit.timeit(expression, setup=setup, number = repetitions)
    print("Non trivial case:")
    print("Total duration: ", duration, "Average Duration: ", duration / repetitions)
    expression = "quote('/El-Nino/')"
    duration = timeit.timeit(expression, setup=setup, number = repetitions)
    print("Trivial case:")
    print("Total duration: ", duration, "Average Duration: ", duration / repetitions)

    print("Unquote")
    expression = "unquote('/El%20Ni%C3%B1o/')"
    duration = timeit.timeit(expression, setup=setup, number = repetitions)
    print("Non trivial case:")
    print("Total duration: ", duration, "Average Duration: ", duration / repetitions)
    expression = "unquote('/El-Nino/')"
    duration = timeit.timeit(expression, setup=setup, number = repetitions)
    print("Trivial case:")
    print("Total duration: ", duration, "Average Duration: ", duration / repetitions)