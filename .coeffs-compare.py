import struct
import sys


def load(p):
    d = {}
    for line in open(p):
        line = line.strip()
        if not line or line.startswith("rows ") or line.startswith("Summary"):
            continue
        parts = line.rsplit(" ", 1)
        if len(parts) != 2 or not parts[1].startswith("0x"):
            continue
        d[parts[0]] = int(parts[1], 16)
    return d


a = load(sys.argv[1])   # ring (base)
b = load(sys.argv[2])   # newtype (dry run)
assert set(a) == set(b), (len(a), len(b), list(set(a) ^ set(b))[:5])


def f(bits):
    return struct.unpack("<d", struct.pack("<Q", bits))[0]


unchanged = tighter = looser = nonbracket = 0
examples = []
for k in a:
    x, y = a[k], b[k]
    if k.endswith(".lo") or k.endswith(".hi"):
        fx, fy = f(x), f(y)
        if x == y or (fx != fx and fy != fy):
            unchanged += 1
            continue
        if fx != fx or fy != fy:
            looser += 1
            examples.append(("nan-flip", k, fx, fy))
            continue
        moved_in = fy > fx if k.endswith(".lo") else fy < fx
        if moved_in:
            tighter += 1
        else:
            looser += 1
            examples.append(("wider", k, fx, fy))
    else:
        # sup norms and counts: smaller is tighter for a sup bound
        if x == y:
            unchanged += 1
        else:
            nonbracket += 1
            examples.append(("non-bracket", k, x, y))
print("rows", len(a), "unchanged", unchanged, "tighter", tighter,
      "looser", looser, "non-bracket-moved", nonbracket)
for e in examples[:12]:
    print("  ", e)
