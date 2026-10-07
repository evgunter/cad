import sys,collections
def load(f):
    d={}
    for l in open(f):
        p=l.split(); kv=dict(x.split('=') for x in p[2:]); d[int(p[0])]=(p[1],kv)
    return d
h=load(sys.argv[1]); m=load(sys.argv[2])
tr=collections.Counter(); longer=short=0; wrong=[]; wrongm=[]
for i,(oh,kv) in h.items():
    om=m[i][0]; tr[(om,oh)]+=1
    lh=float(kv['lev_head']); lm=float(kv['lev_main']); tl=float(kv['truel'])
    if lh>lm*(1+1e-12): longer+=1
    if lh<tl*(1-1e-9): short+=1
    esc=float(kv['esc_over_zero']); dev=float(kv['dev_over_zero']); tilt=float(kv['tiltL_over_zero'])
    if oh=='S' and (dev>=2 or tilt>=1.0+1e-9): wrong.append((i,kv))
    if om=='S' and (dev>=2 or tilt>=1.0+1e-9): wrongm.append((i,kv))
print("head lever > main:",longer," head lever < true:",short)
print("transitions main->head (changed only):")
for (a,b),c in sorted(tr.items(),key=lambda x:-x[1]):
    if a!=b: print(f"  {a:22s} -> {b:22s} {c}")
print("head served with dev>=2zero or tilt*L>=zero:",len(wrong)); 
for w in wrong[:10]: print("  ",w)
print("main served with same:",len(wrongm))
for w in wrongm[:5]: print("  ",w)
import statistics
mv=[(i,kv) for i,(oh,kv) in h.items() if oh=='S' and m[i][0]!='S']
print("moved to served:",len(mv),"max dev/zero",max(float(k['dev_over_zero']) for _,k in mv),"max tiltL",max(float(k['tiltL_over_zero']) for _,k in mv))
print(" by s:",collections.Counter(k['s'] for _,k in mv).most_common())
print(" by kind:",collections.Counter(k['kind'] for _,k in mv))
se=[(i,kv) for i,(oh,kv) in h.items() if oh!='S' and m[i][0]=='S']
for i,k in se[:6]: print(" S->",h[i][0],k['kd'],k['kt'],k['s'],k['dev_over_zero'],k['tiltL_over_zero'])
cons=[(i,kv,oh) for i,(oh,kv) in h.items() if oh!='S' and float(kv['dev_over_zero'])<0.5 and float(kv['tiltL_over_zero'])<0.5]
print("head non-served though clearly coaxial at region:",len(cons),collections.Counter(o for _,_,o in cons))
ms=[float(kv['dev_over_zero']) for i,(om,kv) in m.items() if om=='S']
hs=[float(kv['dev_over_zero']) for i,(oh,kv) in h.items() if oh=='S']
print("main served max dev/zero",max(ms)," >1:",sum(x>1 for x in ms)," head served max",max(hs)," >1:",sum(x>1 for x in hs))
print(" moved-to-served by sign:",collections.Counter(k['sign'] for _,k in mv)," by scale:",collections.Counter(k['scale'] for _,k in mv))
for sc in ['500','100']:
    xs=[float(k['dev_over_zero']) for i,(oh,k) in h.items() if oh=='S' and k['scale']==sc]
    print(" scale",sc,"served",len(xs),"max dev",max(xs) if xs else None)
