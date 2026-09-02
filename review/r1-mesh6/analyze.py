#!/usr/bin/env python3
import glob,re,collections,sys,os
D="/root/lanes/mesh-6r1/review/r1-mesh6/ab"
tag=sys.argv[1] if len(sys.argv)>1 else "def"
rows=collections.defaultdict(lambda: collections.defaultdict(list))  # (body,d) -> mode -> [tess_us]
chk=collections.defaultdict(list)
for f in sorted(glob.glob(f"{D}/{tag}_r*_*.txt")):
    mode=f.rsplit("_",1)[1][:-4]
    for l in open(f):
        m=re.match(r"S65-COST (\S+) (\S+) (\d+) ([\d.]+) ([\d.]+) ([\d.]+)",l)
        if m:
            b,d,tris,tess,check,pct=m.groups(); rows[(b,d,tris)][mode].append(float(tess))
            if mode=="none": chk[(b,d,tris)].append(float(pct))
def mn(v): return min(v) if v else float('nan')
print(f"tag={tag}  rounds per mode: "+", ".join(f"{m}={len(rows[next(iter(rows))][m])}" for m in ['none','seam','chord','both','check'] if rows))
print(f"{'body':13}{'d':7}{'tris':>7} {'today_us':>9} {'+seam%':>7} {'+chord%':>8} {'+both%':>7} {'+check%':>8} {'chk/tess%':>9} {'noise(none)%':>12}")
for (b,d,tris),m in rows.items():
    n=mn(m['none']); s=mn(m['seam']); c=mn(m['chord']); bo=mn(m['both']); ck=mn(m['check'])
    noise=(max(m['none'])-min(m['none']))/min(m['none'])*100 if m['none'] else float('nan')
    print(f"{b:13}{d:7}{tris:>7} {n:9.1f} {(n-s)/s*100:7.1f} {(n-c)/c*100:8.1f} {(n-bo)/bo*100:7.1f} {(ck-n)/n*100:8.1f} {mn(chk[(b,d,tris)]):9.1f} {noise:12.1f}")
