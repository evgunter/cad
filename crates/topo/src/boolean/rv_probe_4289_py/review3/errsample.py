# Review 3: sample probes that head refuses (Err) and base decided, and
# classify each by the exact oracle under band-sized moves of every far point.
import sys, json, random
sys.path.insert(0, '.')
from gen import *
seed, es, n = sys.argv[1], sys.argv[2], int(sys.argv[3]); eps=float(es)
meta = json.load(open(f'meta_{seed}.json'))
def rows(f):
    res=[];cur=None
    for l in open(f).read().split('\n'):
        w=l.split()
        if not w: continue
        if w[0]=='case': cur=[];res.append(cur)
        else: cur.append((w[2],w[3]))
    return res
H=rows(f'out_head_{seed}_{es}.txt'); B=rows(f'out_base_{seed}_{es}.txt')
cands=[]
for ci,m in enumerate(meta):
    for pi,_ in enumerate(m['P']):
        if H[ci][pi][1].startswith('Err') and B[ci][pi][1] in ('In','Out','On'):
            cands.append((ci,pi))
print('new cs Err (base decided):',len(cands))
random.seed(7); smp=random.sample(cands,min(n,len(cands)))
def jitter(v,r):
    d=[random.gauss(0,1) for _ in range(3)]; l=math.sqrt(sum(x*x for x in d)); return [a+r*x/l*random.random() for a,x in zip(v,d)]
def perturbed(secs,r):
    # move every far point by <= r; rebuild unit bounds and normals per face
    moved={}
    out=[]
    for s in secs:
        fs=tuple(s['fs']); fe=tuple(s['fe'])
        for f in (fs,fe):
            if f not in moved: moved[f]=jitter(list(f),r)
        st,en=norm(moved[fs]),norm(moved[fe])
        out.append(dict(s=st,e=en,n=norm(cross(st,en)),es=s['es'],ee=s['ee'],fs=moved[fs],fe=moved[fe],arm=s['arm'],face=s['face']))
    return out
tally=collections.Counter() if False else {}
import collections
tally=collections.Counter()
for ci,pi in smp:
    m=meta[ci]; u,far,tag=m['P'][pi]
    truth=Oracle(m['secs']).cls(far)
    flips=set([truth])
    for k in range(12):
        flips.add(Oracle(perturbed(m['secs'],eps)).cls(jitter(far,eps)))
    dist=angdist(m['secs'],u); minarm=min(s['arm'] for s in m['secs'])
    kind='IN-BAND' if len(flips)>1 else 'stable'
    tally[kind]+=1
    print(f"{m['name'][:28]:28s} p{pi:3d} {H[ci][pi][1]:28s} base={B[ci][pi][1]:4s} exact={truth:4s} dist={dist:.1e} minarm={minarm:.0e} band-moves give {sorted(flips)} -> {kind}")
print(dict(tally))
