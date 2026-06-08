#set page(paper: "a4", margin: (x: 2.5cm, y: 2.5cm), numbering: "1")
#set text(font: "Charter", size: 11pt)
#set heading(numbering: "1.1")
#show heading: set text(font: "Optima")
#show raw: set text(font: "Menlo", size: 8.5pt)
#show raw.where(block: true): it => block(breakable: false, it)

#align(center)[
  #v(4cm)
  #text(size: 28pt, weight: "bold")[Algorithmic Problem Solving]
  #v(0.5cm)
  #text(size: 18pt)[Exam Notes]
  #v(0.5cm)
  #text(size: 14pt)[Kasper Jønsson]
  #v(0.3cm)
  #text(size: 13pt, fill: luma(80))[#datetime.today().display("[month repr:long] [year]")]
  #v(2cm)
  #line(length: 60%, stroke: 0.5pt)
]

#set page(numbering: none)
#pagebreak()
#set page(numbering: "1")

#outline(indent: 1em, depth: 2)

#pagebreak()

= Techniques

== Greedy

A *greedy algorithm* builds a solution piece by piece, always choosing the locally
optimal option at each step, with the hope (or proof) that local optima lead to a
global optimum.

*Key properties required:*
- *Greedy-choice property* — a globally optimal solution can be reached by making
  locally optimal (greedy) choices.
- *Optimal substructure* — an optimal solution contains optimal solutions to its
  sub-problems.

*Common examples:*
- Coin change (canonical denominations)
- Activity selection / interval scheduling
- Huffman coding
- Dijkstra's shortest-path algorithm
- Kruskal's / Prim's minimum spanning tree

*Time complexity:* depends on the problem, but usually O(n log n) due to an initial
sort.

*When greedy fails:* when the greedy choice invalidates future choices (e.g. 0/1
knapsack). Use dynamic programming instead.

*Example — generating all subsets (greedy choice: include or exclude each element):*

#raw(read("subsets.py"), lang: "python")

== Backtracking

*Backtracking* is a systematic way to iterate through all possible configurations of
a search space. It incrementally builds candidates and abandons ("backtracks") a
candidate as soon as it determines the candidate cannot lead to a valid solution.

*Template:*
```
search(state):
    if state is a complete solution:
        record / count it
        return
    for each choice c extending state:
        if c is valid (pruning):
            apply c
            search(state + c)
            undo c           ← backtrack
```

*Classic example — N-Queens:*

#raw(read("backtracking.py"), lang: "python")

*Another example — generating all subsets:*

#raw(read("subsets.py"), lang: "python")

*Time complexity:* O(b^d) in the worst case, where b = branching factor and
d = maximum depth. Pruning can reduce this dramatically in practice.

#pagebreak()

= Graphs

== Terminology

A *graph* G = (V, E) consists of a set of *vertices* (nodes) V and a set of *edges* E.

#table(
  columns: (auto, 1fr),
  stroke: 0.4pt,
  inset: 7pt,
  [*Term*], [*Definition*],
  [Directed / Digraph], [Edges have a direction (u → v ≠ v → u)],
  [Undirected], [Edges have no direction (u — v)],
  [Simple graph], [Graph with no self-loops and no multiple edges between the same pair of vertices],
  [Weighted], [Each edge carries a numeric weight],
  [Degree], [Number of edges incident to a vertex],
  [In-degree / Out-degree], [Directed: edges arriving / leaving a vertex],
  [Path], [Sequence of vertices connected by edges],
  [Cycle], [A path whose first and last vertex are the same],
  [DAG], [Directed Acyclic Graph — a directed graph with no cycles],
  [Connected], [Undirected: every vertex is reachable from every other vertex],
  [Strongly connected], [Directed: every vertex is reachable from every other vertex],
  [Bipartite], [TODO],
  [Tree], [Connected undirected graph with no cycles (|E| = |V| − 1)],
  [Forest], [Undirected graph with no cycles (collection of trees)],
)

*Representations:*

- *Adjacency list* — dict/array mapping each vertex to its neighbours. \
  Space: O(V + E). Efficient for sparse graphs.
- *Adjacency matrix* — V×V boolean/weight matrix. \
  Space: O(V²). Efficient for dense graphs and O(1) edge lookup.
- *Edge list* — list of (u, v) or (u, v, w) tuples. \
  Simple; used in algorithms like Kruskal's.

#pagebreak()

== Depth-First Search (DFS)

DFS explores as far as possible along each branch before backtracking.
It uses a *stack* (explicit or via recursion).

*Algorithm (iterative):*

#raw(read("dfs.py"), lang: "python")

*Properties:*
- Time: O(V + E)
- Space: O(V) (stack + visited)
- Produces a DFS tree / forest
- Detects cycles, finds connected components, used in topological sort

== Breadth-First Search (BFS)

BFS explores all neighbours at the current depth before moving deeper.
It uses a *queue* and finds *shortest paths* in unweighted graphs.

*Algorithm:*

#raw(read("bfs.py"), lang: "python")

*Properties:*
- Time: O(V + E)
- Space: O(V)
- Shortest path (unweighted): distance from start = the BFS level at which a node is
  first visited

*DFS vs BFS at a glance:*

#table(
  columns: (1fr, 1fr, 1fr),
  stroke: 0.4pt,
  inset: 7pt,
  [], [*DFS*], [*BFS*],
  [Data structure], [Stack], [Queue],
  [Shortest path], [No], [Yes (unweighted)],
  [Memory], [O(depth)], [O(width)],
  [Typical use], [Cycles, toposort, SCC], [Shortest path, level order],
)

== Topological Sorting

A *topological order* of a DAG is a linear ordering of its vertices such that for
every directed edge u → v, vertex u appears before v.

Only possible on *DAGs* (no topological order exists if there is a cycle).

*Kahn's algorithm (BFS-based):*

```python
from collections import deque

def topological_sort(graph, n):
    # graph: adjacency list  {u: [v, ...]}
    # n: number of vertices (0-indexed)
    in_degree = [0] * n
    for u in graph:
        for v in graph[u]:
            in_degree[v] += 1

    queue = deque(v for v in range(n) if in_degree[v] == 0)
    order = []

    while queue:
        u = queue.popleft()
        order.append(u)
        for v in graph[u]:
            in_degree[v] -= 1
            if in_degree[v] == 0:
                queue.append(v)

    # if len(order) != n the graph has a cycle
    return order
```

*DFS-based (post-order):*

```python
def topological_sort_dfs(graph, vertices):
    visited = set()
    order   = []

    def dfs(u):
        visited.add(u)
        for v in graph.get(u, []):
            if v not in visited:
                dfs(v)
        order.append(u)   # append after all descendants are visited

    for v in vertices:
        if v not in visited:
            dfs(v)

    return order[::-1]   # reverse post-order
```

*Time complexity:* O(V + E)

#pagebreak()

// ══════════════════════════════════════════════════════════════════════════
= Geometric Algorithms
// ══════════════════════════════════════════════════════════════════════════

== Convex Hull

The *convex hull* of a set of points is the smallest convex polygon that contains all
the points.

*Cross product* (2-D): given an origin O and two vectors a, b,
$ "cross"(O, a, b) = (a_x - O_x)(b_y - O_y) - (a_y - O_y)(b_x - O_x) $
- Positive → left turn (counter-clockwise)
- Zero → collinear
- Negative → right turn (clockwise)

*Andrew's Monotone Chain algorithm:*

#raw(read("convexhull.py"), lang: "python")

*Time:* O(n log n) — dominated by the sort.
*Space:* O(n)

The `<= 0` condition removes collinear points from the hull. Use `< 0` to keep them.

== Polygon Area

The *shoelace formula* (Gauss's area formula) computes the signed area of a simple
polygon given its vertices in order:

$ A = 1/2 lr(|sum_(i=0)^(n-1) (x_i y_{i+1} - x_{i+1} y_i)|) $

where indices are taken modulo n.

This is equivalent to summing the cross products of consecutive vertices with the
origin:

#raw(read("polygonarea.py"), lang: "python")

*Time:* O(n)
*Notes:*
- Vertices must be in order (clockwise or counter-clockwise).
- The absolute value handles both orientations.
- Works for any simple (non-self-intersecting) polygon.
