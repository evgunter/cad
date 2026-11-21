import os, sys, re, collections
def load(d):
    tests = collections.defaultdict(lambda: {"main": [], "other_n":0, "other_sum":0, "other_bsum":0, "preds": collections.Counter(), "pv": {}, "pb": {}})
    for f in os.listdir(d):
        lines = open(os.path.join(d,f)).read().splitlines()
        if not lines: continue
        m = re.match(r"T (.*?)\tA (.*?)\tN (\d+) SEQ (\w+) BSEQ (\w+) SUM (\w+) BSUM (\w+)", lines[0])
        if not m: continue
        tname, args, n, seq, bseq, s, bs = m.groups()
        am = re.search(r"--exact (\S+)", args)
        key = am.group(1) if am else args
        # binary disambiguation: pid unknown, use exe? keep key
        t = tests[key]
        if tname == key or tname == "main":
            t["main"].append((int(n), seq, bseq, s, bs))
        else:
            t["other_n"] += int(n); t["other_sum"] = (t["other_sum"] + int(s,16)) % 2**64; t["other_bsum"] = (t["other_bsum"] + int(bs,16)) % 2**64
        for l in lines[1:]:
            p, pn, ps, pbs = l[2:].rsplit(" ", 3)
            t["preds"][p] += int(pn)
            t["pv"][p] = (t["pv"].get(p,0) + int(ps,16)) % 2**64
            t["pb"][p] = (t["pb"].get(p,0) + int(pbs,16)) % 2**64
    return tests
a, b = load(sys.argv[1]), load(sys.argv[2])
common = sorted(set(a) & set(b))
print(f"tests with decisions: base {len(a)} head {len(b)} common {len(common)}; only base {len(set(a)-set(b))} only head {len(set(b)-set(a))}")
for k in sorted(set(a)^set(b))[:20]: print("  only", "base" if k in a else "head", k)
vdiff, bdiff, seqonly = [], [], []
total = 0
for k in common:
    x, y = a[k], b[k]
    total += sum(m[0] for m in x["main"]) + x["other_n"]
    vx = (x["preds"], x["pv"]); vy = (y["preds"], y["pv"])
    if x["preds"] != y["preds"] or x["pv"] != y["pv"]:
        preds = sorted(p for p in set(x["preds"])|set(y["preds"]) if x["preds"].get(p)!=y["preds"].get(p) or x["pv"].get(p)!=y["pv"].get(p))
        vdiff.append((k, preds))
    elif x["pb"] != y["pb"]:
        preds = sorted(p for p in set(x["pb"])|set(y["pb"]) if x["pb"].get(p)!=y["pb"].get(p))
        bdiff.append((k, preds))
    elif [m[1] for m in x["main"]] != [m[1] for m in y["main"]]:
        seqonly.append(k)
print(f"decisions compared (base): {total}")
print(f"VERDICT multiset differs: {len(vdiff)} tests")
for k,p in vdiff: print("  V", k, p[:8])
print(f"bits-only differ: {len(bdiff)} tests")
cnt = collections.Counter(p for _,ps in bdiff for p in ps)
for p,c in cnt.most_common(40): print("  B", c, p)
print(f"order-only differ: {len(seqonly)}")
for k in seqonly[:20]: print("  S", k)
