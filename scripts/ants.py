import sys

input = sys.stdin.readline


def main():
    T = int(input())
    for _ in range(T):
        data = []
        while len(data) < 2:
            data.extend(input().split())
        L = int(data[0])
        n = int(data[1])
        positions = []
        buf = data[2:]
        while len(positions) < n:
            if buf:
                positions.append(int(buf.pop(0)))
            else:
                buf = input().split()
        earliest = 0
        latest = 0
        for p in positions:
            earliest = max(earliest, min(p, L - p))
            latest = max(latest, max(p, L - p))
        print(earliest, latest)


main()
