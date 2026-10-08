import re, math, json, sys
sys.path.insert(0,'.')
from gen import *
t=open('short_arm.rs.txt').read()
body=t.split('fuzz_cone(&[')[1].split(']);')[0]
nums=[float(x) for x in re.findall(r'-?\d+\.?\d*(?:e-?\d+)?', body)]
rows=[nums[i:i+19] for i in range(0,len(nums),19)]
print(len(rows),'sectors')
probe=[1.9466554485576073,2.698130705263507e-10,-1.546282621768826e-07]
def case(f,cid,rows,P):
    f.write(f"case {cid} {len(rows)} {len(P)}\n")
    for r in rows: f.write("s "+" ".join(repr(float(x)) for x in r)+"\n")
    for far in P:
        u=norm(far); f.write("p "+" ".join(repr(float(x)) for x in u+far)+"\n")
with open('cases_sa.txt','w') as f:
    cid=0
    for Lb in (0.5, 0.001):
        for delta in (0.0, 1e-7, 2e-7, 3e-7):
            R=[list(r) for r in rows]
            b=norm([1.0,0.0,-delta])
            for r in R:
                for (di,ri,ed) in ((0,11,9),(3,14,10)):
                    if abs(r[di]-1.0)<1e-15 and abs(r[di+1])<1e-15 and abs(r[di+2])<1e-15:
                        r[di:di+3]=b; r[ri:ri+3]=mul(b,Lb)
                        nn=norm(cross(r[0:3],r[3:6]))
                        if dot(nn,r[6:9])<0: nn=mul(nn,-1)
                        r[6:9]=nn
                        r[17]=min(r[17],Lb) if Lb<0.5 else r[17]
            case(f,cid,R,[probe]); cid+=1
            print(cid-1,'Lb',Lb,'delta',delta, 'far-move m', delta*Lb)
