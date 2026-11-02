import sys, xml.etree.ElementTree as ET
def load(p):
    r = {}
    for tc in ET.parse(p).getroot().iter("testcase"):
        r[(tc.get("classname"), tc.get("name"))] = float(tc.get("time") or 0)
    return r
a, b = load(sys.argv[1]), load(sys.argv[2])
for bar in (0.5, 1.0):
    print(f">= {bar}s: A {sum(v >= bar for v in a.values())}  B {sum(v >= bar for v in b.values())}")
print("sum A %.0f B %.0f" % (sum(a.values()), sum(b.values())))
over = sorted(((b[k], a.get(k, 0), k) for k in b if b[k] >= 1.0 and a.get(k, 0) < 1.0), reverse=True)
print("newly over 1.0s in B:", len(over))
for x in over[:25]: print(f"  B {x[0]:.2f} A {x[1]:.2f} {x[2][0]} {x[2][1]}")
