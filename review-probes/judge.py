from probe import *
import json, collections
fx=[list(map(float,l.split())) for l in open('fx.txt')]
meta=json.load(open('meta.json'))
res=[l.split() for l in open(sys.argv[1])]
stats=collections.Counter(); bad=[]
for f,m,(tag,dev) in zip(fx,meta,res):
    eps=f[0]; C=f[1:4]; n=f[4:7]; rho=f[7]; u=f[8:11]; tc=f[11:14]; ta=f[14:17]; R,r=f[17],f[18]
    tru = oracle(C,n,rho,u,tc,ta,R,r,n=512)
    stats[(m, tag.split(':')[0] if tag.startswith('Esc') else tag)]+=1
    if tag=="OnSurface" and tru>eps: bad.append((m,eps,tru/eps,float(dev)/eps,R,r,tc))
    if m=="meridian" and tag!="OnSurface": bad.append(("MERIDIAN-REFUSED",tag,eps,R,r,tc[:1]))
for k,v in sorted(stats.items()): print(k,v)
print("BAD", len(bad))
for b in bad[:30]: print(b)
