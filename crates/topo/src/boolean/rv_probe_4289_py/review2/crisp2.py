import sys, json, math
sys.path.insert(0,'.')
from gen import *
from cases import write_case
def pt(a,z): return [math.cos(a), math.sin(a), z]
meta=[]
with open('cases_crisp2.txt','w') as f:
    cid=0
    for th in (0.1, 0.06):
      for tiltLA in (1.5e-8, 2.5e-8):
        for LA in (1e-3, 2e-3):
          tilt = tiltLA/LA; dent = tilt*math.sin(th)
          for hollow in (False, True):
            ring=[(pt(0.05-th,dent),True,0.4),(pt(0.05,0.0),True,LA),(pt(0.5,0),True,0.4),(pt(2.0,0),True,0.4),(pt(3.5,0),True,0.4),(pt(5.0,0),True,0.4)]
            secs=sectors_from_ring(ring,hollow)
            P=[]
            for a in (0.6,0.8,1.0,1.2):
                for h in (2e-7,-2e-7,5e-7,-5e-7):
                    u=norm(pt(a,h)); P.append((u,mul(u,0.5),f'a{a}h{h}'))
            write_case(f,cid,secs,P); cid+=1
            meta.append(dict(name=f'c2_th{th}_tLA{tiltLA}_LA{LA}_dent{dent:.3g}_{"h" if hollow else "s"}',secs=secs,P=P))
json.dump(meta,open('meta_crisp2.json','w'))
