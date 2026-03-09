import sys
from collections import defaultdict, deque


lines = sys.stdin.read().strip().split("\n")
li = 0
n = int(lines[li])
li += 1

dependents = defaultdict(list)
dependencies = defaultdict(list)

for _ in range(n):
    parts = lines[li].split()
    li += 1
    target = parts[0].rstrip(":")
    deps = parts[1:]
    dependencies[target] = deps
    for d in deps:
        dependents[d].append(target)

changed = lines[li].strip()
li += 1

needs_recompile = set()
needs_recompile.add(changed)
queue = deque([changed])
while queue:
    f = queue.popleft()
    for dep in dependents[f]:
        if dep not in needs_recompile:
            needs_recompile.add(dep)
            queue.append(dep)

in_degree = {f: 0 for f in needs_recompile}
adj = defaultdict(list)
for f in needs_recompile:
    for d in dependencies[f]:
        if d in needs_recompile:
            adj[d].append(f)
            in_degree[f] += 1

queue = deque(f for f in needs_recompile if in_degree[f] == 0)
result = []
while queue:
    f = queue.popleft()
    result.append(f)
    for nxt in adj[f]:
        in_degree[nxt] -= 1
        if in_degree[nxt] == 0:
            queue.append(nxt)

print("\n".join(result))
