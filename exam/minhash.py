import random

# MinHash: finding a correlated pair among a collection of binary strings.
#
# A binary string s of length n is treated as a set of positions where s[i] = '1'.
# The Jaccard similarity of two strings s and t is:
#   J(s, t) = |positions(s) ∩ positions(t)| / |positions(s) ∪ positions(t)|
#
# A MinHash function h is defined by a random permutation π of {0, ..., n-1}.
# The hash value h(s) = min { π(i) : s[i] = '1' }.
#
# Key property: Pr[h(s) = h(t)] = J(s, t)


def minhash(s, pi):
    """Return the MinHash value of binary string s under permutation pi."""
    ones = [i for i, c in enumerate(s) if c == "1"]
    return min(pi[i] for i in ones)


def build_minhash_table(strings, num_hashes=20):
    """
    Build a hash table to find a correlated pair.

    For each of `num_hashes` random hash functions h_k:
      - Compute h_k(s) for every string s.
      - Insert s into bucket (k, h_k(s)).
    Any two strings that land in the same bucket are a candidate correlated pair.

    Running time: O(num_hashes * n * len(strings))
    """
    n = len(strings[0])

    # Generate num_hashes random permutations of {0, ..., n-1}
    permutations = []
    for _ in range(num_hashes):
        pi = list(range(n))
        random.shuffle(pi)
        permutations.append(pi)

    # Buckets: (hash_index, hash_value) -> list of string indices
    buckets = {}
    for k, pi in enumerate(permutations):
        for idx, s in enumerate(strings):
            h = minhash(s, pi)
            key = (k, h)
            if key not in buckets:
                buckets[key] = []
            buckets[key].append(idx)

    # Return the first bucket that contains at least two strings
    for key, indices in buckets.items():
        if len(indices) >= 2:
            return indices[0], indices[1]

    return None  # No correlated pair found


def jaccard(s, t):
    a = set(i for i, c in enumerate(s) if c == "1")
    b = set(i for i, c in enumerate(t) if c == "1")
    if not a and not b:
        return 1.0
    return len(a & b) / len(a | b)


# --- Demo ---

# A collection of binary strings of length 10
strings = [
    "1010101010",  # 0
    "1010101011",  # 1  -- very similar to 0
    "0101010101",  # 2
    "1100110011",  # 3
    "0000000001",  # 4
]

result = build_minhash_table(strings, num_hashes=50)
if result:
    i, j = result
    print(f"Candidate correlated pair: strings[{i}] and strings[{j}]")
    print(f"  s = {strings[i]}")
    print(f"  t = {strings[j]}")
    print(f"  Jaccard similarity: {jaccard(strings[i], strings[j]):.3f}")
else:
    print("No correlated pair found (try more hash functions).")
