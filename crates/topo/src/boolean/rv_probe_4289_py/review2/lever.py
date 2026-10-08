import sys, json, math
sys.path.insert(0,'.')
from gen import *
from cases import write_case
def pt(a,z): return [math.cos(a), math.sin(a), z]
meta=[]
eta=5e-8
with open('cases_lev.txt','w') as f:
    cid=0
    for hollow in (False,True):
      for Lb in (0.001, 1.0):
        for mv in (0.0, 0.5, 1.5, 3.0):   # far-point move, in units of eta*Lb (1.0 = onto the probe)
            b_ang = -mv*eta
            ring=[(pt(b_ang,0.0),True,Lb),(pt(1.0,0.0),True,1.0),(pt(2.5,-0.5),True,1.0),(pt(4.0,-0.5),True,1.0),(pt(-1.0,-0.5),True,1.0)]
            secs=sectors_from_ring(ring,hollow)
            P=[]
            for e in (eta, 2*eta):
                u=norm(pt(-e,0.0)); P.append((u,mul(u,1.0),f'eta{e}'))
            write_case(f,cid,secs,P)
            meta.append(dict(name=f'lev_{"h" if hollow else "s"}_Lb{Lb}_mv{mv}',secs=secs,P=P))
            print(cid, meta[-1]['name'], 'far move m', mv*eta*Lb); cid+=1
json.dump(meta,open('meta_lev.json','w'))
