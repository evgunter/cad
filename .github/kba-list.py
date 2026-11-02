import sys, xml.etree.ElementTree as ET
def load(p):
    return {(t.get("classname"), t.get("name")): float(t.get("time") or 0) for t in ET.parse(p).getroot().iter("testcase")}
a, b = load(sys.argv[1]), load(sys.argv[2])
rows = sorted(((v, a.get(k, -1), k) for k, v in b.items() if v >= 0.5), reverse=True)
print(f"MEASURE B2 >=0.5: {len(rows)}  >=0.8: {sum(r[0] >= 0.8 for r in rows)}  A >=0.5: {sum(v >= 0.5 for v in a.values())}")
for v, av, k in rows: print(f"LIST {v:.2f} A {av:.2f} {k[0]} {k[1]}")
