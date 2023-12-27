import timeit
import uuid

import bson
import fastid
import ulid


def test_ulid():
    print()
    print("ulid-py    ", timeit.timeit(lambda: ulid.microsecond.new().str, number=10000))
    print("fastid.ulid", timeit.timeit(lambda: fastid.ulid(), number=10000))


def test_snowflake():
    print()
    print("fastid.snowflake_int", timeit.timeit(lambda: fastid.snowflake_int(), number=10000))
    print("fastid.snowflake_str", timeit.timeit(lambda: fastid.snowflake_str(), number=10000))


def test_uuid():
    print()
    print("uuid.uuid4    ", timeit.timeit(lambda: uuid.uuid4(), number=10000))
    print("fastid.uuid_v7", timeit.timeit(lambda: fastid.uuid_v7(), number=10000))


def test_objectid():
    print()
    print("bson.ObjectId", timeit.timeit(lambda: bson.ObjectId(), number=10000))
    print("fastid.object_id", timeit.timeit(lambda: fastid.object_id(), number=10000))
