import timeit

import fastid
from ulid import microsecond


def test_ulid():
    print()
    print("ulid-py    ", timeit.timeit(lambda: microsecond.new().str, number=10000))
    print("fastid.ulid", timeit.timeit(lambda: fastid.ulid(), number=10000))


def test_snowflake():
    print()
    print("fastid.snowflake_int", timeit.timeit(lambda: fastid.snowflake_int(), number=10000))
    print("fastid.snowflake_str", timeit.timeit(lambda: fastid.snowflake_str(), number=10000))
