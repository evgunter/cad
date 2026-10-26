import re,sys,collections
pat=re.compile(r'^(\S*) PROBE5734_ROUND round=(\d+) width=(\S+) removable=(\S+) areamid=(\S+) flo=\S+ mbd=Ok\((\S+)\)')
for fn in sys.argv[1:]:
    faces=collections.defaultdict(dict)
    for line in open(fn):
        m=pat.match(line.strip())
        if not m: continue
        t,r,w,rem,a,mbd=m.groups()
        faces[(t,a)].setdefault(int(r),(float(w),float(rem),float(mbd)))
    worst=[]; n=0
    for k,rs in faces.items():
        rounds=sorted(rs)
        r0=next((r for r in rounds if rs[r][1]*1024<=rs[r][0]),None)
        if r0 is None: continue
        later=[r for r in rounds if r>r0]
        if not later: continue
        n+=1
        wn=min(rs[r][0] for r in rounds if r<=r0)
        mn=min(rs[r][0] for r in later)
        if wn==0: continue
        worst.append((mn/wn, k[0], k[1], r0, len(later)))
    worst.sort()
    print(fn, 'faces with a floor and later rounds:', n, 'below 1-1/1024:', sum(1 for x in worst if x[0]<1-1/1024))
    for x in worst[:6]: print('  ',x)
