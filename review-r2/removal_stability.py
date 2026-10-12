"""R2 probe for PR 4769's proof step "removing candidates other than a
germ's nearest leaves it its nearest", under the banded trilean.

A germ's nearest is find_match's scan: the incumbent is replaced only by a
definitely nearer candidate (|m| >= escalate, negative); |m| <= zero is a
tie (incumbent keeps); anything between escalates. Distances on one line.

Result: holds whenever escalate > 2*zero (K > 2); fails silently at K <= 2.
Minimal witness at K = 2 (zero = 1, escalate = 2), distances in eps:
  scan [3, 2, 1] -> 1   (2 ties 3; 1 beats 3 by 2 = escalate)
  remove 3       -> [2, 1] -> 2   (1 ties 2)
"""
import random, sys

def run(Z, E, trials):
    class Esc(Exception): pass
    def sign(m):
        if abs(m) <= Z: return 0
        if abs(m) < E: raise Esc
        return -1 if m < 0 else 1
    def scan(ds):
        best = None
        for i, d in enumerate(ds):
            if best is None or sign(d - ds[best]) == -1: best = i
        return best
    bad = n = 0
    for _ in range(trials):
        k = random.randint(2, 6)
        ds = [random.choice([random.uniform(0, 30), round(random.uniform(0, 30))]) for _ in range(k)]
        try: r = scan(ds)
        except Esc: continue
        for x in range(k):
            if x == r: continue
            idx = [j for j in range(k) if j != x]
            try: r2 = idx[scan([ds[j] for j in idx])]
            except Esc: continue
            n += 1; bad += r2 != r
    return bad, n

random.seed(1)
for K in (1.5, 2.0, 2.01, 2.5, 10.0):
    print(f"K={K}: changed-nearest {run(1.0, K, 300000)}")
