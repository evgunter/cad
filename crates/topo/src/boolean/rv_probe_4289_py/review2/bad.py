import sys, json
sys.path.insert(0,'.')
from gen import *
seed, outf, eps = sys.argv[1], sys.argv[2], float(sys.argv[3])
which = sys.argv[4] if len(sys.argv)>4 else 'cs'
meta = json.load(open(f'meta_{seed}.json'))
res=[];cur=None
for l in open(outf):
    w=l.split()
    if not w: continue
    if w[0]=='case': cur=[];res.append(cur)
    else: cur.append((w[2],w[3]))
n=0
for ci,(m,rows) in enumerate(zip(meta,res)):
    orc=Oracle(m['secs'])
    for pi,((u,far,tag),(wc,cs)) in enumerate(zip(m['P'],rows)):
        got = cs if which=='cs' else wc
        if got not in ('In','Out','On'): continue
        t=orc.cls(far); d=angdist(m['secs'],u)
        if got!=t and not ((got=='On' or t=='On') and d<4*eps) and d>=1e-14:
            print(ci,pi,m['name'],'got',got,'truth',t,'dist %.3g'%d,'u',['%.6g'%x for x in u], 'L %.3g'%math.sqrt(dot(far,far)))
            n+=1
            if n>=int(sys.argv[5] if len(sys.argv)>5 else 12): sys.exit()
