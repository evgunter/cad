# Independent model of PR 4050's pairing argument (review r1).
import itertools, sys

def nc_matchings(pts):
    """Non-crossing perfect matchings of the cyclically ordered list pts."""
    if not pts: yield []; return
    a = pts[0]
    for k in range(1, len(pts), 2):
        b = pts[k]
        for left in nc_matchings(pts[1:k]):
            for right in nc_matchings(pts[k+1:]):
                yield [(a, b)] + left + right

def run_order(n, p0, p1):
    f, b = (p0 + 1) % n == p1, (p1 + 1) % n == p0
    if f and b: return ('either',)
    if f: return (False,)
    if b: return (True,)
    return None

def b_runs(n, pairs, b_pos):  # port of insert::b_runs
    spans = [(min(b_pos[i0], b_pos[i1]), max(b_pos[i0], b_pos[i1])) for i0, i1 in pairs]
    for k, (lo, hi) in enumerate(spans):
        for lo2, hi2 in spans[k+1:]:
            if (lo < lo2 < hi) != (lo < hi2 < hi): return None
    runs = []
    for i0, i1 in pairs:
        p0, p1 = b_pos[i0], b_pos[i1]
        r = run_order(n, p0, p1)
        runs.append(r[0] if r is not None else (p1 < p0))
    def interval(k):
        i0, i1 = pairs[k]
        return (b_pos[i1], b_pos[i0]) if runs[k] is True else (b_pos[i0], b_pos[i1])
    def holds(o, p):
        f, t = interval(o); return f < t and f < p < t
    out = []
    for k in range(len(pairs)):
        i0 = pairs[k][0]
        cands = [o for o in range(len(pairs)) if o != k and holds(o, b_pos[i0])]
        h = min(cands, key=lambda o: interval(o)[1] - interval(o)[0]) if cands else None
        out.append((runs[k], h, interval(k)))
    return out

def meanders(n):
    pts = list(range(n))
    for inside in nc_matchings(pts):
        for outside in nc_matchings(pts):
            # one cycle?
            mi = {}; mo = {}
            for a, b in inside: mi[a] = b; mi[b] = a
            for a, b in outside: mo[a] = b; mo[b] = a
            seen = [0]; c = mi[0]; order = [0]
            while True:
                nxt = mo[c] if len(order) % 2 == 1 else mi[c]
                order.append(c)
                if nxt == 0: break
                c = nxt
            if len(order) == n:
                yield inside, outside, order

stats = {}
fail = []
for n in (4, 6, 8, 10):
    cnt = 0; shapes = {}
    for inside, outside, border in meanders(n):
        cnt += 1
        # B arcs: border[j] -> border[j+1]; which disk? first arc 0->mi[0] is inside.
        mi = {}
        for a, b in inside: mi[a] = b; mi[b] = a
        for b_in_parity in (0, 1):      # which side of B's cycle is B's interior
            for a_keep_inB in (True, False):
                for b_keep_inA in (True, False):
                    # A arc i (between A germs i, i+1) side wrt B: alternate.
                    a_in_b = [(i + b_in_parity) % 2 == 0 for i in range(n)]
                    # pairing start (A order 0..n-1)
                    start = 0 if a_in_b[0] == a_keep_inB else 1
                    a_order = list(range(start, n)) + list(range(0, start))
                    pairs = [(a_order[2*k], a_order[2*k+1]) for k in range(n // 2)]
                    # B arcs: arc j from border[j] to border[j+1]; inside A iff it is an inside chord.
                    b_arc_inA = [mi[border[j]] == border[(j + 1) % n] for j in range(n)]
                    # result link: kept arcs; at each crossing connect the kept A arc and kept B arc.
                    a_kept = [a_in_b[i] == a_keep_inB for i in range(n)]
                    b_kept = [b_arc_inA[j] == b_keep_inA for j in range(n)]
                    parent = {}
                    def find(x):
                        while parent.setdefault(x, x) != x: x = parent[x]
                        return x
                    def union(x, y): parent[find(x)] = find(y)
                    bpos_of = {g: p for p, g in enumerate(border)}
                    for c in range(n):
                        ka = [('A', i) for i in ((c - 1) % n, c) if a_kept[i]]
                        p = bpos_of[c]
                        kb = [('B', j) for j in ((p - 1) % n, p) if b_kept[j]]
                        if len(ka) != 1 or len(kb) != 1:
                            fail.append(('crossing-kept', n, c)); continue
                        union(ka[0], kb[0])
                    # every kept A arc is a paired arc (own copy)
                    paired_arcs = {a for a, b in pairs}  # pair (i, i+1) is arc i
                    if {i for i in range(n) if a_kept[i]} != paired_arcs:
                        fail.append(('A-kept-not-paired', n, inside, outside))
                    # B: every walk start and direction
                    for direction in (1, -1):
                        for s in range(n):
                            bord = [border[(s + direction * k) % n] for k in range(n)]
                            b_pos = [0] * n
                            for p, g in enumerate(bord): b_pos[g] = p
                            res = b_runs(n, pairs, b_pos)
                            if res is None:
                                fail.append(('crossing', n, inside, outside)); continue
                            # B arcs in this reading: arc p between positions p, p+1
                            def arc_index(p):  # original border arc index
                                if direction == 1: return (s + p) % n
                                return (s - p - 1) % n
                            # laminar check and region of each arc
                            iv = [r[2] for r in res]
                            def contains(k, p):  # arc p (p..p+1) inside run k's walk
                                f, t = iv[k]
                                if f < t: return f <= p < t
                                return p >= f or p < t  # wrapping run
                            for k in range(len(pairs)):
                                for o in range(len(pairs)):
                                    if k == o: continue
                                    f, t = iv[k]; f2, t2 = iv[o]
                                    inside_k = [contains(k, p) for p in range(n)]
                                    inside_o = [contains(o, p) for p in range(n)]
                                    a_ = {p for p in range(n) if inside_k[p]}; b_ = {p for p in range(n) if inside_o[p]}
                                    if a_ & b_ and not (a_ <= b_ or b_ <= a_):
                                        fail.append(('not-laminar', n, inside, outside, s, direction))
                            # innermost holder agrees with arc containment
                            for k, (_, h, (f, t)) in enumerate(res):
                                arcs_k = {p for p in range(n) if contains(k, p)}
                                holders = [o for o in range(len(pairs)) if o != k and arcs_k < {p for p in range(n) if contains(o, p)}]
                                inner = min(holders, key=lambda o: len({p for p in range(n) if contains(o, p)})) if holders else None
                                if inner != h:
                                    fail.append(('holder', n, pairs, b_pos, k, h, inner))
                            # regions: innermost run containing arc p, or root
                            def region(p):
                                cs = [k for k in range(len(pairs)) if contains(k, p)]
                                return min(cs, key=lambda k: sum(contains(k, q) for q in range(n))) if cs else None
                            reg = {}
                            for p in range(n):
                                j = arc_index(p)
                                if b_kept[j]:
                                    reg.setdefault(region(p), set()).add(find(('B', j)))
                            for r, cyc in reg.items():
                                if len(cyc) != 1:
                                    fail.append(('B-region-two-cycles', n, inside, outside, s, direction, r))
                            depth = max(((lambda k: (lambda d: d)(sum(1 for o in range(len(pairs)) if o != k and {p for p in range(n) if contains(k,p)} < {p for p in range(n) if contains(o,p)})))(k) for k in range(len(pairs))), default=0)
                            sib = max((sum(1 for r in res if r[1] == h) for h in range(len(pairs))), default=0)
                            shapes[(depth, sib)] = shapes.get((depth, sib), 0) + 1
    print(n, 'meanders', cnt, 'shapes(depth, max siblings)', sorted(shapes.items()))
print('failures', len(fail))
for f in fail[:10]: print(f)
