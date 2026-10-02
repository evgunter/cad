"""Row-by-row diff of two k_probe_sweep.sh output trees: every row must match
except the margin column of the three re-levered names (PR #3853)."""
import sys, itertools, collections
LEVERED = {"bool_germ_plane_normal", "bool_box_cylinder_axis", "props_torus_axis"}
a_root, b_root = sys.argv[1], sys.argv[2]
for rel in ["k-eps-1e-6.csv", "k-eps-1e-9.csv", "k-eps-1e-12.csv",
            "m2/k-eps-1e-6.csv", "m2/k-eps-1e-9.csv", "m2/k-eps-1e-12.csv",
            "driver/k-eps-1e-6.csv", "driver/k-eps-1e-9.csv", "driver/k-eps-1e-12.csv"]:
    other, moved, n = [], collections.Counter(), 0
    outcomes = collections.Counter()
    with open(f"{a_root}/{rel}") as fa, open(f"{b_root}/{rel}") as fb:
        for la, lb in itertools.zip_longest(fa, fb):
            n += 1
            if la == lb:
                continue
            if la is None or lb is None:
                other.append((n, la, lb)); continue
            ca, cb = la.rstrip("\n").split(","), lb.rstrip("\n").split(",")
            if len(ca) == len(cb) and ca[1] == cb[1] and ca[1] in LEVERED and \
               ca[:2] + ca[3:] == cb[:2] + cb[3:]:
                moved[ca[1]] += 1
                outcomes[(ca[1], cb[-1])] += 1
                continue
            other.append((n, la, lb))
    print(f"{rel}: {n} rows; margin-only moves {dict(moved)}; outcomes {dict(outcomes)}; OTHER diffs {len(other)}")
    for o in other[:5]:
        print("   ", o)
