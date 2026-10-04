#!/usr/bin/env python3
"""Per-round refused margins from CAD_SCRATCH_REFINE traces: is the sequence monotone?"""
import glob, re, sys, os
OUT = sys.argv[1]
rr = re.compile(r"REFINE round=(\d+) samples=(\d+) located=(\d+) err=(\w+) \{(.*)\}")
val = re.compile(r"(?:value: |MarginDiag\(Value\()([0-9.e+-]+|inf|NaN)")
limb = re.compile(r"limb: (\w+)")
for f in sorted(glob.glob(os.path.join(OUT, "*.log"))):
    rounds = []
    ok = None
    rows = []
    hull = []
    for line in open(f, errors="replace"):
        if line.startswith("REFINE time"):
            hull.append([]); continue
        if line.startswith("HULLSUP "):
            if not hull: continue
            try: hull[-1].append(float(line.split()[1]))
            except ValueError: hull[-1].append(float("nan"))
            continue
        m = rr.search(line)
        if m:
            r, n, loc, kind, body = m.groups()
            v = val.search(body)
            l = limb.search(body)
            rounds.append((int(r), int(n), int(loc), kind, l.group(1) if l else "", float(v.group(1)) if v else float("nan")))
            continue
        if "REFINE ok" in line:
            ok = line.strip()
            rows.append(("ok", ok, list(rounds)))
            rounds = []
        if line.startswith("ROW "):
            rows.append(("row", line.strip()[:140], None))
        if "RefinementExhausted" in line and "REFINE" not in line:
            rows.append(("exh", line.strip()[:200], list(rounds)))
            rounds = []
    if rounds:
        rows.append(("unfinished", "", list(rounds)))
    print("==", os.path.basename(f))
    for kind, text, rs in rows:
        if rs is None:
            print("  ", text)
            continue
        seq = [x[5] for x in rs]
        nonmono = sum(1 for a, b in zip(seq, seq[1:]) if not (b < a))
        print(f"   {kind} rounds={len(rs)} nonmono_steps={nonmono} n: {[x[1] for x in rs]}")
        print("      limbs:", [x[4] for x in rs])
        print("      located:", [x[2] for x in rs])
        print("      margins:", ["%.3g" % v for v in seq])
        if hull:
            k = len(rs) + (1 if kind == "ok" else 0)
            hs = [max(g) if g else float("nan") for g in hull[:k]]; hull = hull[k:]
            hn = sum(1 for a, b in zip(hs, hs[1:]) if not (b < a))
            print("      hullsup:", ["%.3g" % v for v in hs], "nonmono=", hn)
        if kind == "ok":
            print("     ", text)
