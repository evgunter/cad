# MEASUREMENT ONLY: body-level verdict moves, PR 7329 (P0) against the narrower run with summed rates (N).
import glob,re,collections,sys
W='/home/user/wm-work'
def seqs(d):
    S={}
    for f in glob.glob(f'{W}/{d}/*.log'):
        rows=[]; test=None
        for l in open(f,errors='replace'):
            p=l.rstrip('\n').split('\t')
            if p[0]=='WMV':
                test=p[2]; kv={k:v for k,v in (x.split('=',1) for x in p[3:-1] if '=' in x)}
                rows.append((p[-1],kv))
        if test is not None: S.setdefault(test,[]).extend(rows)
    return S
def kind(v):
    m=re.match(r'(Embedded|Crosses|Refused\((\w+)(?:\s*\{[^}]*?arm: (\w+))?)',v)
    return m.group(0) if m else v[:40]
P=seqs(sys.argv[1] if len(sys.argv)>1 else 'log-P0'); N=seqs(sys.argv[2] if len(sys.argv)>2 else 'log-N')
moves=[];same=0;only=collections.Counter()
for t in set(P)|set(N):
    if t not in P or t not in N: only['P0 only' if t in P else 'N only']+=1; continue
    a,b=P[t],N[t]
    for i,((vp,kp),(vn,kn)) in enumerate(zip(a,b)):
        if kind(vp)!=kind(vn) or (vp.startswith('Refused') and vp!=vn and 'Uncertified' in vp and 'Uncertified' in vn and False):
            moves.append((t,i,vp,vn,kp,kn)); break
    else:
        same+=1
        if len(a)!=len(b): only['same prefix, different ask count']+=1
print('tests compared',same+len(moves),'unmoved',same,'moved',len(moves),dict(only))
dirn=collections.Counter()
for t,i,vp,vn,kp,kn in moves:
    dirn[('Embedded' if vp.startswith('Emb') else 'Crosses' if vp.startswith('Cr') else 'Refused')+'->'+('Embedded' if vn.startswith('Emb') else 'Crosses' if vn.startswith('Cr') else 'Refused')]+=1
print('directions',dict(dirn))
new=[m for m in moves if m[2].startswith('Embedded') and m[3].startswith('Refused')]
nk=collections.Counter()
for t,i,vp,vn,kp,kn in new:
    nm=kn.get('named','')
    kindf=re.search(r'narrow_face=\S+:(\w+)',nm); run=re.search(r'narrow=(\S+)',nm)
    nk[(kindf.group(1) if kindf else '-', 'run<1e-6' if run and float(run.group(1))<1e-6 else 'run<1e-3' if run and float(run.group(1))<1e-3 else 'run>=1e-3' if run else 'no edge named')]+=1
print('newly refusing: by narrower face kind and run',nk.most_common())
def show(m):
    t,i,vp,vn,kp,kn=m
    return f"{t[:80]} | ask#{i} faces={kn.get('faces')} | P0 {vp[:70]} | N {vn[:110]} | {kn.get('named','')[:200]}"
print('== newly refusing (Embedded -> Refused), sorted by narrower run, largest first (most surprising):')
def runof(m):
    r=re.search(r'narrow=(\S+)',m[5].get('named','')); return float(r.group(1)) if r else -1
for m in sorted(new,key=runof,reverse=True): print('  ',show(m))
print('== other moves:')
for m in moves:
    if m not in new: print('  ',show(m))
