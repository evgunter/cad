import sys
sys.argv=[sys.argv[0], sys.argv[1]]
exec(open('/home/user/d2/residue_oracle.py').read().split("print(hdr)")[0])
for k in sys.argv[2:] if False else ['sunk|-1.2|AminusB','sunk|1.2|AminusB','standing|1.2|union','sunk|-1.2|intersect','sunk|-1.2|tool']:
    print("==",k)
    for (o,n,loops) in bodies[k]:
        va=(F(0),)*3
        for l in loops: va=add(va,loop_va(l,l[0]))
        allp=[p for l in loops for p in l]
        fans=[float((dot(a,va)-dot(allp[0],va))/3) for a in allp]
        if max(abs(x) for x in fans)>0:
            print(f"  face n={tuple(float(x) for x in n)} npts={len(allp)} fan(anchor_i)-fan(anchor_0) = {['%+.2e'%x for x in fans]}")
