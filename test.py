import time
import urllib.request
from os.path import basename

import r2py


def benchmark(func):
    def wrap(*args, **kwargs):
        start = time.perf_counter()
        r = func(*args)
        print(f"{func.__name__}: {time.perf_counter() - start} / s")
        return r
    return wrap


@benchmark
def download_rs(url):
    r2py.download(url)


@benchmark
def download_py(url):
    res = urllib.request.urlopen(url)
    filename = basename(url)
    with open(filename, 'wb') as f:
        f.write(res.read())


@benchmark
def fibonacci_rs(n):
    r2py.fibonacci(n)


@benchmark
def fibonacci_py(n):
    def fib(n):
        if n < 2:
            return n
        return fib(n-1) + fib(n-2)
    return fib(n)


if __name__ == '__main__':
    # url = 'https://www.fnordware.com/superpng/pnggrad8rgb.png'
    url = 'http://ipv4.download.thinkbroadband.com/1GB.zip'
    # download_py(url)
    download_rs(url)
    # fibonacci_py(40)
    # fibonacci_rs(40)
