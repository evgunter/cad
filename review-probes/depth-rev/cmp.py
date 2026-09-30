import sys, collections
def load(p):
    return [l.rstrip("\n") for l in open(p)]
def key_nohash(l):
    f = l.split()
    if f[0] == "GET":
        return (f[0], f[1], f[2], f[3], f[5])
    return (f[0], f[1], f[2])
def keyid(l):
    f = l.split()
    return (f[0], f[1], f[2]) if f[0] == "EVAL" else (f[0], f[1], f[2], f[3])
base, h1, h2 = load(sys.argv[1]), load(sys.argv[2]), load(sys.argv[3])
cb = collections.Counter(map(key_nohash, base)); ch = collections.Counter(map(key_nohash, h1))
print("structure (no hash) diff base vs head:", (cb - ch) + (ch - cb) or "none")
noisy = set(keyid(l) for l in set(h1) ^ set(h2))
bad = [l for l in set(base) ^ set(h1) if keyid(l) not in noisy]
print("noisy keys head-vs-head:", len(noisy))
print("base-vs-head diffs outside noisy keys:", len(bad))
for l in bad[:20]: print("  ", l)
