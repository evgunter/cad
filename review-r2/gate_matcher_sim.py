"""R2 delta probe for PR 4769 fix head f3e3aba23d: the whole straight-germ
matcher (keep_own scans, then first_pair with the K > 2 gate), run with the
in-band pair order rotated. A silent change = two runs that both finish
without escalating and build different matchings.

`gated=False` drops the K > 2 premise (frees every all-mutual round)."""
import math, random, sys

class Esc(Exception): pass

def run(points, edges, Z, E, rot, gated=True):
    def sign(m):
        if m != m: raise Esc
        if abs(m) <= Z: return 0
        if abs(m) < E: raise Esc
        return -1 if m < 0 else 1
    ch = lambda i, j: math.dist(points[i], points[j])
    used = [False] * len(points); matching = []
    while True:
        own = {}; at = 0
        for g in range(len(points)):
            for (i, j) in edges:
                h = j if i == g else i if j == g else None
                if h is None or used[g] or used[h]: continue
                at += 1; d = ch(g, h)
                if g not in own or sign(d - own[g][1]) == -1:
                    own[g] = (at, d, h)
        if not own:
            return tuple(sorted(matching))
        mutual = all(own.get(h, (0, 0, None))[2] == g for g, (_, _, h) in own.items())
        free = mutual and (E > 2 * Z or not gated)
        met = sorted(((a, d, (h, g)) for g, (a, d, h) in own.items()))
        if free:
            met = met[rot % len(met):] + met[:rot % len(met)]
        best = None
        for a, d, m in met:
            if best is None: best = (d, m); continue
            try: s = sign(d - best[0])
            except Esc:
                if free: continue          # in band: keep the incumbent
                raise
            if s == -1: best = (d, m)
        p, g = best[1]; used[p] = used[g] = True
        matching.append((min(p, g), max(p, g)))

def trial(K, gated, rng):
    n = rng.randint(4, 9)
    pts = [(rng.uniform(0, 3), rng.uniform(0, 3)) for _ in range(n)]
    edges = [(i, j) for i in range(n) for j in range(i + 1, n) if rng.random() < 0.5]
    Z = rng.choice([0.05, 0.1, 0.2, 0.3]); E = K * Z
    outs = set()
    for rot in range(4):
        try: outs.add(run(pts, edges, Z, E, rot, gated))
        except Esc: pass
    return len(outs) > 1

rng = random.Random(4769)
N = int(sys.argv[1]) if len(sys.argv) > 1 else 40000
for K in (1.1, 1.5, 2.0, 2.01, 2.5, 10.0):
    for gated in (True, False):
        bad = sum(trial(K, gated, rng) for _ in range(N))
        print(f"K={K:<5} gate={'on ' if gated else 'off'} silent-matching-changes={bad}/{N}")
