import sys, json, math
sys.path.insert(0,'.')
from gen import *
from cases import write_case
def pt(a,z): return [math.cos(a), math.sin(a), z]
meta=[]
with open('cases_crisp.txt','w') as f:
    cid=0
    for zP1 in (1.6e-7, 5e-7):
      for LA in (0.01, 0.001):
        for hollow in (False, True):
            ring=[(pt(-0.3,zP1),True,1.0),(pt(0.05,0.0),True,LA),(pt(0.5,0),True,1.0),(pt(2.0,0),True,1.0),(pt(3.5,0),True,1.0),(pt(5.0,0),True,1.0)]
            secs=sectors_from_ring(ring,hollow)
            P=[]
            for th in (0.6,0.8,1.0,1.2,1.5,1.8):
                for h in (2e-7,-2e-7,5e-7,-5e-7,1e-6,-1e-6):
                    u=norm(pt(th,h)); P.append((u,mul(u,1.0),f'th{th}h{h}'))
            write_case(f,cid,secs,P); cid+=1
            meta.append(dict(name=f'crisp_z{zP1}_LA{LA}_{"h" if hollow else "s"}',secs=secs,P=P))
json.dump(meta,open('meta_crisp.json','w'))
