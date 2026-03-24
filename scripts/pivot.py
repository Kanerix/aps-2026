import sys

input = sys.stdin.readline


def main():
    n = int(input())
    A = list(map(int, input().split()))

    prefix_max = [0] * n
    prefix_max[0] = A[0]
    for i in range(1, n):
        prefix_max[i] = max(prefix_max[i - 1], A[i])

    suffix_min = [0] * n
    suffix_min[n - 1] = A[n - 1]
    for i in range(n - 2, -1, -1):
        suffix_min[i] = min(suffix_min[i + 1], A[i])

    count = 0
    for i in range(n):
        if prefix_max[i] == A[i] and suffix_min[i] == A[i]:
            count += 1

    print(count)


main()
