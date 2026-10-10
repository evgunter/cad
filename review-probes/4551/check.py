import sys, struct
from fractions import Fraction as F
def fh(s): return struct.unpack('<d', bytes.fromhex(s)[::-1])[0]
def Q(x): return F(x)
mode = sys.argv[1]
if mode == 'between':
    bad=0; refused=0; tot=0; outside_hull=0; empty=0; unneeded=0
    for line in open('/root/between.txt'):
        v=line.split(); xl,xh,yl,yh,ll,lh=[fh(s) for s in v[:6]]; cert=v[6]=='1'; rl,rh=fh(v[7]),fh(v[8])
        tot+=1
        L0=max(ll,0.0); L1=min(lh,1.0)
        if L0>L1:
            empty+=1
            if cert: bad+=1; print("cert on empty lambda", line.strip())
            continue
        if not cert:
            refused+=1
            continue
        vals=[(1-Q(l))*Q(x)+Q(l)*Q(y) for l in (L0,L1) for x in (xl,xh) for y in (yl,yh)]
        lo,hi=min(vals),max(vals)
        if Q(rl)>lo or Q(rh)<hi:
            bad+=1
            if bad<10: print("MISS", (xl,xh),(yl,yh),(ll,lh),(rl,rh), float(lo), float(hi))
        if rl < min(xl,yl) or rh > max(xh,yh):
            outside_hull+=1
            if outside_hull<5: print("OUTSIDE HULL", (xl,xh),(yl,yh),(ll,lh),(rl,rh))
    print(f"between: total={tot} miss={bad} refused(nonempty Λ∩[0,1])={refused} emptyΛ={empty} outside_hull={outside_hull}")
else:
    bad=0; tot=0; ref=0; rbad=0; rtot=0; maxexcess=0
    def blossom_exact(knots,p,s,c,args):
        d=list(c)
        for r in range(1,p+1):
            u=args[r-1]
            for j in range(p,r-1,-1):
                i=s-p+j
                a=(u-knots[i])/(knots[i+p+1-r]-knots[i])
                d[j]=(1-a)*d[j-1]+a*d[j]
        return d[p]
    import itertools
    for line in open('/root/blossom.txt'):
        f=line.strip().split('|')
        if f[0]=='R':
            p=int(f[1]); s=int(f[2]); knots=[Q(fh(k)) for k in f[3].split(',')]
            cs=[tuple(Q(fh(t)) for t in c.split(':')) for c in f[4].split(',')]
            a,b=[Q(fh(t)) for t in f[5].split(':')]
            row=[c.split(':') for c in f[6].split(',')]
            for k in range(p+1):
                rtot+=1
                args=[a]*(p-k)+[b]*k
                basis=[blossom_exact(knots,p,s,[F(int(i==j)) for j in range(p+1)],args) for i in range(p+1)]
                lo=sum(min(N*c[0],N*c[1]) for N,c in zip(basis,cs)); hi=sum(max(N*c[0],N*c[1]) for N,c in zip(basis,cs))
                ce,rl,rh=row[k][0]=='1',Q(fh(row[k][1])),Q(fh(row[k][2]))
                if not ce or rl>lo or rh<hi:
                    rbad+=1; print("RESTRICT MISS", p, s, k, ce, float(rl), float(rh), float(lo), float(hi))
            continue
        tot+=1
        p=int(f[1]); s=int(f[2]); knots=[Q(fh(k)) for k in f[3].split(',')]
        cs=[tuple(Q(fh(t)) for t in c.split(':')) for c in f[4].split(',')]
        args=[tuple(Q(fh(t)) for t in a.split(':')) for a in f[5].split(',')]
        cert=f[6]=='1'; rl,rh=Q(fh(f[7])),Q(fh(f[8]))
        lo_s,hi_s=knots[s],knots[s+1]
        # clamp to span closure (precondition: true args in span)
        args=[(max(a,lo_s),min(b,hi_s)) for a,b in args]
        if not cert: ref+=1; continue
        lo=None; hi=None
        for vert in itertools.product(*[sorted(set(a)) for a in args]):
            basis=[blossom_exact(knots,p,s,[F(int(i==j)) for j in range(p+1)],list(vert)) for i in range(p+1)]
            l=sum(min(N*c[0],N*c[1]) for N,c in zip(basis,cs)); h=sum(max(N*c[0],N*c[1]) for N,c in zip(basis,cs))
            lo=l if lo is None else min(lo,l); hi=h if hi is None else max(hi,h)
        if rl>lo or rh<hi:
            bad+=1
            if bad<10: print("BLOSSOM MISS", f[0], p, s, float(rl), float(rh), float(lo), float(hi))
        scale=max(abs(lo),abs(hi),Q(1e-300))
        ex=float(((lo-rl)+(rh-hi))/scale)/2.220446049250313e-16
        maxexcess=max(maxexcess,ex)
    print(f"blossom: total={tot} miss={bad} refused={ref} max excess ulps-of-scale={maxexcess:.1f}; restrict coeffs={rtot} miss={rbad}")
