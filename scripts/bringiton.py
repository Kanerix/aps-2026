import sys


def solve():
    data = sys.stdin.read().split()
    n = int(data[0])
    words = data[1 : n + 1]

    children = [[-1] * 26]
    counts = [0]

    results = []

    for word in words:
        node = 0
        answer = 0
        for ch in word:
            idx = ord(ch) - ord("a")
            if children[node][idx] == -1:
                answer = 0
                break
            node = children[node][idx]
        else:
            answer = counts[node]
        results.append(answer)

        node = 0
        for ch in word:
            idx = ord(ch) - ord("a")
            if children[node][idx] == -1:
                children.append([-1] * 26)
                counts.append(0)
                children[node][idx] = len(counts) - 1
            node = children[node][idx]
            counts[node] += 1

    sys.stdout.write("\n".join(map(str, results)) + "\n")


solve()
