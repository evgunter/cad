import csv, math
H=None
def load(p):
    global H
    rows={}
    with open(p) as f:
        r=csv.reader(f); H=next(r)
        for i,row in enumerate(r):
            rows[(row[0],row[1],row[3])]=row
    return rows
b=load('/tmp/base.csv'); h=load('/tmp/head.csv')
onlyb=set(b)-set(h); onlyh=set(h)-set(b)
print("rows only in base:",sorted(onlyb))
print("rows only in head:",sorted(onlyh))
GATE={'triangles','cells','grid_cells','patch_cells','opt_cells','span_opt_cells','chart'}
colchg={}; scenes={}; rowsdiff=0
worst={}
for k in sorted(set(b)&set(h)):
    rb,rh=b[k],h[k]
    if rb==rh: continue
    rowsdiff+=1
    scenes.setdefault(k[0],[]).append(k)
    for i,(x,y) in enumerate(zip(rb,rh)):
        if x!=y:
            c=H[i]; colchg[c]=colchg.get(c,0)+1
            try:
                fx,fy=float(x),float(y)
                rel=abs(fy-fx)/abs(fx) if fx else float('inf')
                # ulps
                import struct
                ix=struct.unpack('<q',struct.pack('<d',fx))[0]
                iy=struct.unpack('<q',struct.pack('<d',fy))[0]
                ulp=abs(iy-ix)
                if c not in worst or rel>worst[c][0]: worst[c]=(rel,ulp,k,x,y)
            except ValueError: pass
print("\ncommon rows that differ:",rowsdiff)
print("scenes with differing rows:",{s:len(v) for s,v in scenes.items()})
print("\ncolumns that changed (count of rows):")
for c,n in sorted(colchg.items(), key=lambda kv:-kv[1]):
    mark=" <== GATE/SIZING COLUMN" if c in GATE else ""
    print(f"  {c:18s} {n:5d}{mark}")
print("\nworst relative drift per changed column:")
for c,(rel,ulp,k,x,y) in sorted(worst.items(), key=lambda kv:-kv[1][0]):
    print(f"  {c:18s} rel={rel:.3e} ulps={ulp:<8d} {k}  {x} -> {y}")
