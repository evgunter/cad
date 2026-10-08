import sys, json, math
sys.path.insert(0,'.')
from gen import *
from cases import write_case
def pt(a,z): return [math.cos(a), math.sin(a), z]
meta=[]
with open('cases_crisp3.txt','w') as f:
    cid=0
    for zc in (3e-7, 1e-6):
        dent=1.5e-5*math.sin(0.06)
        ring=[(pt(-0.01,dent),True,0.4),(pt(0.05,0.0),True,1e-3),(pt(0.5,0),True,0.4),(pt(2.0,zc),True,0.4),(pt(3.5,-zc),True,0.4),(pt(5.0,zc),True,0.4)]
        B,C=pt(0.5,0),pt(2.0,zc); nBC=norm(cross(B,C))
        for hollow in (False,True):
            secs=sectors_from_ring(ring,hollow)
            P=[]
            for a in (0.6,0.8,1.0,1.2):
                for h in (2e-7,-2e-7,5e-7,-5e-7):
                    q=pt(a,0.0); z0=-(q[0]*nBC[0]+q[1]*nBC[1])/nBC[2]   # on the B-C plane
                    u=norm(pt(a,z0+h)); P.append((u,mul(u,0.5),f'a{a}h{h}'))
            write_case(f,cid,secs,P); cid+=1
            meta.append(dict(name=f'c3_zc{zc}_{"h" if hollow else "s"}',secs=secs,P=P))
json.dump(meta,open('meta_crisp3.json','w'))
