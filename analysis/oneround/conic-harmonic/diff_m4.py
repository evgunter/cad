#!/usr/bin/env python3
"""M4: the door records of the two modes, call for call.

A record is (test, door, input, output). Calls are matched within a test
by (door, input) and their order of occurrence; a call present in one
mode only is counted apart (its test took another path upstream)."""
import sys, glob, collections, re
def load(d):
    out = collections.defaultdict(list)
    for f in glob.glob(d + "/*.log"):
        for line in open(f, errors="replace"):
            p = line.rstrip("\n").split("\t")
            if len(p) == 4:
                out[(p[0], p[1], p[2])].append(p[3])
    return out
NUM = re.compile(r"-?\d+\.?\d*(?:e-?\d+)?")
def shape(s):
    return NUM.sub("#", s)
def nums(s):
    return [float(x) for x in NUM.findall(s)]
for eps in ("1e-9", "1e-12"):
    a, b = load(f"{sys.argv[1]}/eps{eps}-mode0"), load(f"{sys.argv[1]}/eps{eps}-mode6")
    tot = collections.Counter(); moved = collections.defaultdict(list); only = collections.Counter()
    kinds = collections.Counter()
    for key in set(a) | set(b):
        xa, xb = a.get(key, []), b.get(key, [])
        door = key[1]; kinds[(door, "conic-is-ellipse" if "Ellipse" in key[2] else "circle")] += max(len(xa), len(xb))
        if len(xa) != len(xb):
            only[door] += abs(len(xa) - len(xb))
        for oa, ob in zip(xa, xb):
            tot[door] += 1
            if oa == ob: continue
            if shape(oa) != shape(ob):
                moved[(door, "verdict")].append((key, oa, ob))
            else:
                na, nb = nums(oa), nums(ob)
                d = max((abs(x - y) for x, y in zip(na, nb)), default=0.0)
                moved[(door, "bits")].append((key, oa, ob, d))
    print(f"== eps {eps}: calls compared {dict(tot)}; unmatched calls {dict(only)}; by carrier {dict(kinds)}")
    for (door, what), items in sorted(moved.items()):
        tests = len({k[0][0] for k in items})
        if what == "bits":
            ds = sorted(i[3] for i in items)
            print(f"  {door}: BITS moved in {len(items)} calls ({tests} tests); largest change of any number {ds[-1]:.2e}, median {ds[len(ds)//2]:.2e}")
        else:
            c = collections.Counter((shape(i[1])[:70], shape(i[2])[:70]) for i in items)
            print(f"  {door}: VERDICT moved in {len(items)} calls ({tests} tests)")
            for (x, y), n in c.most_common(12):
                print(f"      {n:4}  {x}  ->  {y}")
