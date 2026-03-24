import sys

input = sys.stdin.readline


def main():
    k = int(input())
    mine = input().strip()
    friend = input().strip()
    n = len(mine)
    same = sum(a == b for a, b in zip(mine, friend))
    diff = n - same
    print(min(k, same) + min(n - k, diff))


main()
