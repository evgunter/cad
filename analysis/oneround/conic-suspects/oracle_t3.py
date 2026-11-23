#!/usr/bin/env python3
"""Task 3 oracle (60-digit mpmath, no kernel code). For every body the boolean BUILT:
 - each section ellipse, as the curve its stored numbers state (S, axis, u, a, b, phase; v = axis x u),
   at its span's ends and middle: distance from the operands' cone and cutting plane (their stored numbers);
 - the body's volume against the closed form of frustum ∩ half-space, integrated in mpmath.
Prints the worst of each per (file, semi-axis decade)."""
import json,sys,glob,collections
from fractions import Fraction
from mpmath import mp, mpf, sqrt, cos, sin, tan, acos, pi, quad
mp.dps=60
F=lambda x: mpf(Fraction(float(x)).numerator)/mpf(Fraction(float(x)).denominator)
V=lambda v:[F(x) for x in v]
dot=lambda a,b:sum(x*y for x,y in zip(a,b)); sub=lambda a,b:[x-y for x,y in zip(a,b)]
cross=lambda a,b:[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
norm=lambda a:sqrt(dot(a,a))
def pt(e,t):
    S,ax,u=V(e['S']),V(e['axis']),V(e['u']); v=cross(ax,u); a,b=F(e['a']),F(e['b'])
    if e['held']=='arc': kc,ks=F(e['phase'][0]),F(e['phase'][1])
    else: kc=ks=mpf(0)
    return [s+a*uu*(cos(t)-kc)+b*vv*(sin(t)-ks) for s,uu,vv in zip(S,u,v)]
def cone_dist(p,c):
    w=sub(p,V(c['o'])); n=V(c['n']); h=dot(w,n)/norm(n); rho=sqrt(max(dot(w,w)-h*h,0)); al=F(c['alpha'])
    return abs(rho*cos(al)-abs(h)*sin(al))
def plane_dist(p,o,n): return abs(dot(sub(p,o),n))/norm(n)
def vol_cut(alpha,phi,h=mpf(1),r1=mpf('0.5')):
    r0=r1+h*tan(alpha)
    def area(y):
        r=r0-y*tan(alpha); x0=-(y-h/2)*cos(phi)/sin(phi)
        if x0<=-r: return pi*r*r
        if x0>=r: return mpf(0)
        return r*r*acos(x0/r)-x0*sqrt(r*r-x0*x0)
    # breakpoints where x0 = +-r
    t=tan(alpha); c=cos(phi)/sin(phi)
    ys=sorted(set([mpf(0),h]+[y for y in ((h/2*c - r0)/(c - t) if c!=t else None, (h/2*c + r0)/(c + t)) if y is not None and 0<y<h]))
    return quad(area, ys), pi*h*(r0*r0+r0*r1+r1*r1)/3, r0
for path in sorted(glob.glob(sys.argv[1]+'/t3-a*.jsonl')):
    rows=[json.loads(l) for l in open(path)]
    fr=rows[0]; cone=[s for s in fr['body']['surfs'] if s['k']=='cone'][0]
    alpha=F(fr['alpha']); worst=collections.OrderedDict(); bad=[]
    for r in rows[1:]:
        if r['row']!='bool' or r['kind']!='built': continue
        q,n=V(r['q']),V(r['n'])
        # the cutting plane as the result states it
        pl=[s for s in r['body']['surfs'] if s['k']=='plane' and abs(abs(dot(V(s['n']),n))-1)<mpf(10)**-12]
        es=[e for e in r['body']['ellipses'] if e['a']>30]
        if not es: continue
        amax=max(e['a'] for e in es); dec='%.0e'%amax
        dc=dp=mpf(0)
        for e in es:
            t0,t1=F(e['span'][0]),F(e['span'][1])
            for t in (t0,(t0+t1)/2,t0+(t1-t0)/3,t1):
                p=pt(e,t); dc=max(dc,cone_dist(p,cone))
                dp=max(dp,min([plane_dist(p,V(s['o']),V(s['n'])) for s in pl] or [plane_dist(p,q,n)]))
        vi,vf,r0=vol_cut(alpha,F(r['phi'])); big=4*(r0+1)
        vb=F(float(4*(float(r0)+1.0)))  # scale 1 only  # the example's f64 `big`
        vbox=(2*vb)*(2*vb)*vb
        want={'intersect':vi,'subtract':vf-vi,'union':vbox+vf-vi}.get(r['op'],mpf(1))
        dv=abs(F(r['vol']['v'])-want) if 'v' in r['vol'] else None
        w=worst.setdefault(dec,[mpf(0),mpf(0),mpf(0),0,0])
        w[0]=max(w[0],dc); w[1]=max(w[1],dp); w[3]+=1
        if dv is None: w[4]+=1
        else: w[2]=max(w[2],dv/max(abs(want),1))
        eps=float(r['eps'])
        if dc>10*eps or dp>10*eps or (dv is not None and dv/max(abs(want),1)>1e-9): bad.append((r['gap'],r['op'],float(dc),float(dp),None if dv is None else float(dv)))
    print('==',path.split('/')[-1])
    for dec,w in worst.items(): print(f'   a~{dec}: n={w[3]} carrier off cone {float(w[0]):.1e} off plane {float(w[1]):.1e} | rel volume miss {float(w[2]):.1e} | no-volume {w[4]}')
    for b in bad: print('   PAST BAND:',b)
