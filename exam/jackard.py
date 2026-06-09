A = {1, 2, 3, 4}
B = {3, 4, 5, 6}


def jaccard_similarity(a: set, b: set) -> float:
    intersection = len(a & b)
    union = len(a | b)
    return intersection / union if union != 0 else 1.0
