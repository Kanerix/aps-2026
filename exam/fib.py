def fib_rec(n):
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fib_rec(n - 1) + fib_rec(n - 2)


def fib_topdown(n, memo={}):
    if n <= 1:
        return n
    if n not in memo:
        memo[n] = fib_topdown(n - 1) + fib_topdown(n - 2)
    return memo[n]


def fib_bottomup(n):
    x, y = 0, 1

    for _ in range(n):
        tmp = x
        x = y
        y = tmp + y

    return x
