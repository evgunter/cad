import sys, collections
def load(f):
    d = {}
    for ln in open(f):
        p = ln.rstrip('\n').split(',')
        if len(p) < 11: continue
        d[(p[0], int(p[1]), p[2])] = p[3:]
    return d
for e in sys.argv[1:]:
    h = load(f'/tmp/claude-0/out-head-{e}.csv'); m = load(f'/tmp/claude-0/out-main-{e}.csv')
    cnt = collections.defaultdict(lambda: [0,0,0])
    padr = collections.defaultdict(lambda: [1e9, 0]); hullr = collections.defaultdict(lambda: [1e9,0]); tm = collections.defaultdict(lambda:[0.0,0.0])
    lost = []
    for k in sorted(set(h)|set(m)):
        fam, i, sc = k
        a, b = h.get(k), m.get(k)
        c = cnt[(fam, sc)]
        c[2] += 1
        if a and a[0]=='ok': c[0] += 1
        if b and b[0]=='ok': c[1] += 1
        if b and b[0]=='ok' and not (a and a[0]=='ok'): lost.append((k, a[0] if a else None))
        if a: tm[(fam,sc)][0] += float(a[-1])
        if b: tm[(fam,sc)][1] += float(b[-1])
        if a and b and a[0]=='ok' and b[0]=='ok':
            for idx in (3,4):
                r = float(a[idx])/float(b[idx]); pr = padr[(fam,sc)]; pr[0]=min(pr[0],r); pr[1]=max(pr[1],r)
            r = float(a[1])/float(b[1]); hr = hullr[(fam,sc)]; hr[0]=min(hr[0],r); hr[1]=max(hr[1],r)
    print(f'=== eps {e}: family scalar  head/main certified  pad ratio head/main [min,max]  hull ratio  time head/main s')
    for key in sorted(cnt):
        c = cnt[key]; pr = padr[key]; hr = hullr[key]; t = tm[key]
        print(f'  {key[0]:10s} {key[1]:4s} {c[0]:2d}/{c[1]:2d} of {c[2]}  pad [{pr[0]:.3g},{pr[1]:.3g}]  hull [{hr[0]:.3g},{hr[1]:.3g}]  t {t[0]:.2f}/{t[1]:.2f}')
    for l in lost: print('  LOST', l)
