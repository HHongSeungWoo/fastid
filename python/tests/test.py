import fastid
from ulid import microsecond

if __name__ == '__main__':
    import time

    t = time.time()

    for i in range(1000):
        a = microsecond.new().str
        print(a)

    print(time.time() - t)

    t = time.time()

    for i in range(1000):
        a = fastid.ulid()
        print(a)

    print(time.time() - t)
