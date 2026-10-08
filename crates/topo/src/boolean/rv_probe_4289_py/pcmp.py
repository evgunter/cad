import sys, json, collections
seed = sys.argv[1]; outf = sys.argv[2]
g = json.load(open(f'pgroups_{seed}.json')); K = g['K']
res = []; cur = None
for l in open(outf):
    w = l.split()
    if not w: continue
    if w[0] == 'case': cur = []; res.append(cur)
    else: cur.append((w[2], w[3]))
st = collections.Counter(); ex = []
for gi, name in enumerate(g['names']):
    copies = res[gi*K:(gi+1)*K]
    for pi in range(len(copies[0])):
        for which in (0, 1):
            dec = set(c[pi][which] for c in copies if c[pi][which] in ('In', 'Out', 'On'))
            if len(dec) > 1:
                st[('flip', which, tuple(sorted(dec)))] += 1
                st[('byname', name.split('/')[0][:14], which)] += 1
                if len(ex) < 15 and not name.startswith('sliver') and not name.startswith('long') and not name.startswith('fin'): ex.append((name, pi, which, [c[pi][which] for c in copies]))
            else: st['stable'] += 1
for k, v in st.items(): print(k, v)
for e in ex: print(e)
