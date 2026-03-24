import sys

input = sys.stdin.readline


def main():
    n, q = map(int, input().split())

    tree = [0] * (n + 1)
    bits = [0] * (n + 1)

    def update(i, delta):
        while i <= n:
            tree[i] += delta
            i += i & (-i)

    def query(i):
        s = 0
        while i > 0:
            s += tree[i]
            i -= i & (-i)
        return s

    out = []
    for _ in range(q):
        line = input().split()
        if line[0] == "F":
            i = int(line[1])
            if bits[i] == 0:
                bits[i] = 1
                update(i, 1)
            else:
                bits[i] = 0
                update(i, -1)
        else:
            l = int(line[1])
            r = int(line[2])
            out.append(str(query(r) - query(l - 1)))

    sys.stdout.write("\n".join(out) + "\n")


main()
