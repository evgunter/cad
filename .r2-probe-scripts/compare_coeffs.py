import struct


def load(p):
    d = {}
    for line in open(p):
        line = line.strip()
        if not line or line.startswith("rows "):
            continue
        name, bits = line.rsplit(" ", 1)
        d[name] = struct.unpack("<d", struct.pack("<Q", int(bits, 16)))[0]
    return d


a = load("/home/user/scalar-ring0-r2-scratch/coeffs-head.txt")   # ring (base)
b = load("/home/user/scalar-ring0-r2-scratch/coeffs-dry.txt")    # newtype (dry run)
assert set(a) == set(b), (len(a), len(b), list(set(a) ^ set(b))[:5])
unchanged = tighter = looser = other = 0
examples = []
for k in a:
    x, y = a[k], b[k]
    if x == y or (x != x and y != y):
        unchanged += 1
        continue
    if k.endswith(".lo"):
        if y > x:
            tighter += 1
        else:
            looser += 1
            examples.append((k, x, y))
    elif k.endswith(".hi"):
        if y < x:
            tighter += 1
        else:
            looser += 1
            examples.append((k, x, y))
    else:
        other += 1
        examples.append((k, x, y))
print("rows", len(a), "unchanged", unchanged, "tighter", tighter, "looser", looser, "other", other)
for e in examples[:10]:
    print("  ", e)
