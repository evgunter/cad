import sys, json, random, collections
sys.path.insert(0,'.')
from gen import *
seed, old, new, eps = sys.argv[1], sys.argv[2], sys.argv[3], float(sys.argv[4])
meta=json.load(open(f'meta_{seed}.json'))
def load(f):
    res=[];cur=None
    for l in open(f):
        w=l.split()
        if not w: continue
        if w[0]=='case': cur=[];res.append(cur)
        else: cur.append((w[2],w[3]))
    return res
O,N=load(old),load(new)
dec=('In','Out','On')
grew=[]
for ci,(m,ro,rn) in enumerate(zip(meta,O,N)):
    for pi,((u,far,tag),(wo,co),(wn,cn)) in enumerate(zip(m['P'],ro,rn)):
        if co in dec and cn not in dec: grew.append((ci,pi,'cs',co,cn))
        if wo in dec and wn not in dec: grew.append((ci,pi,'wc',wo,wn))
print('grew', collections.Counter((g[2],g[4].split(':')[0]+(':'+g[4].split(':')[1] if ':' in g[4] else '')) for g in grew))
random.seed(5); samp=random.sample([g for g in grew if g[2]=='cs'], min(30,len([g for g in grew if g[2]=='cs'])))
cls=collections.Counter()
allstat=collections.Counter()
for g in grew:
    ci,pi,w,o,n=g; m=meta[ci]; u,far,tag=m['P'][pi]
    t=Oracle(m['secs']).cls(far); d=angdist(m['secs'],u)
    kind=m['name'].split('/')[0]; kind='random' if kind.startswith('r') and '_n' in kind else ''.join(c for c in kind if not c.isdigit() and c not in '.-e_')
    allstat[(w, kind, 'oldright' if o==t or ((o=='On' or t=='On') and d<4*eps) else 'oldWRONG', 'far' if d>1e-6 else ('mid' if d>1e-8 else 'near'))]+=1
for k,v in sorted(allstat.items()): print(k,v)
print('--- sample of 30 cs')
for g in samp:
    ci,pi,w,o,n=g; m=meta[ci]; u,far,tag=m['P'][pi]
    t=Oracle(m['secs']).cls(far); d=angdist(m['secs'],u)
    print(ci,pi,m['name'],tag,'old',o,'new',n,'truth',t,'dist %.2g'%d)
