import sys, os, collections, re
def load(d, aux=False):
    per = collections.defaultdict(list)
    for f in os.listdir(d):
        if not f.endswith('.txt'): continue
        if f.endswith('.aux.txt') != aux: continue
        for line in open(os.path.join(d,f), errors='replace'):
            line=line.rstrip('\n')
            if not line: continue
            t, rest = line.split('\t',1)
            per[t].append(rest)
    return per
a, b = sys.argv[1], sys.argv[2]
aux = len(sys.argv) > 3 and sys.argv[3] == 'aux'
A, B = load(a, aux), load(b, aux)
tests = sorted(set(A)|set(B))
n_a = sum(len(v) for v in A.values()); n_b = sum(len(v) for v in B.values())
print(f"tests base={len(A)} head={len(B)} lines base={n_a} head={n_b}")
only_a = [t for t in tests if t not in B]; only_b=[t for t in tests if t not in A]
print("only base:", len(only_a), only_a[:10]); print("only head:", len(only_b), only_b[:10])
diff=0; masked_only=0; order_diff=0
def strip_raw(l):
    return re.sub(r' rows=[0-9a-f]+', '', l)
def strip_raw_aux(l):
    return re.sub(r'raw=[0-9a-f]+ ', '', l)
for t in tests:
    if t not in A or t not in B: continue
    la, lb = A[t], B[t]
    if sorted(la) == sorted(lb):
        if la != lb: order_diff += 1
        continue
    f = strip_raw_aux if aux else strip_raw
    if sorted(map(f,la)) == sorted(map(f,lb)):
        masked_only += 1
        print("MASKED-ONLY", t, len(la))
        continue
    diff += 1
    ca, cb = collections.Counter(la), collections.Counter(lb)
    print("DIFF", t, len(la), len(lb))
    for x in (ca-cb): print("   base:", x[:400])
    for x in (cb-ca): print("   head:", x[:400])
print(f"identical-multiset tests: {sum(1 for t in tests if t in A and t in B and sorted(A[t])==sorted(B[t]))}; order-only diffs {order_diff}; masked-only {masked_only}; real diffs {diff}")
c = collections.Counter(l.split('\t')[2].split(' ')[0] for v in B.values() for l in v) if not aux else None
if c: print("head outcome kinds:", c)
