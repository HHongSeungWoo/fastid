import timeit

import fastid
from ulid import microsecond


def test_ulid():
    print()
    print(timeit.timeit(lambda: microsecond.new().str, number=10000))
    print(timeit.timeit(fastid.ulid, number=10000))
