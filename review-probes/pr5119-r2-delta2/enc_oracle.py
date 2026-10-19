import sys, re
from fractions import Fraction as F

def vec(s): return [F(float(x)) for x in s.split(',')]
def sub(a,b): return [a[i]-b[i] for i in range(3)]
def dot(a,b): return sum(a[i]*b[i] for i in range(3))
def cross(a,b): return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]

def finish(cur, faces, out):
    if cur is None: return
    lo, hi = F(float(cur['lo'])), F(float(cur['hi']))
    centre = faces[0][1][0]
    model = F(0); poly = F(0)
    for n, pts in faces:
        anchor = pts[0]
        va = [F(0)]*3
        # rings flattened: we assume one ring (boxes)
        for i in range(len(pts)):
            p, q = pts[i], pts[(i+1) % len(pts)]
            c = cross(sub(p, anchor), sub(q, anchor))
            va = [va[k] + c[k]/2 for k in range(3)]
        model += dot(sub(anchor, centre), n) * dot(va, n) / dot(n, n)
        for i in range(1, len(pts)-1):
            poly += dot(pts[0], cross(pts[i], pts[i+1])) / 6
    model /= 3
    mid = (lo+hi)/2; width = hi-lo
    out.append((cur['tag'], float(model), lo <= model <= hi, float(poly), lo <= poly <= hi,
                float(width/abs(mid)) if mid != 0 else None, float((poly-mid)/width) if width else None, lo > 0, hi < 0))

out=[]; cur=None; faces=[]
for line in open(sys.argv[1]):
    m = re.match(r'R2E (.*) lo=(\S+) hi=(\S+) recentred=(\S+)', line)
    if m:
        finish(cur, faces, out); cur={'tag':m.group(1)+' rc='+m.group(4),'lo':m.group(2),'hi':m.group(3)}; faces=[]; continue
    m = re.match(r'R2F n=(\S+) rings=(\d+) pts=(\S+)', line)
    if m and cur is not None:
        faces.append((vec(m.group(1)), [vec(p) for p in m.group(3).split(';')]))
finish(cur, faces, out)
bad=0
for t in out:
    tag, model, min_, poly, pin, rel, off, pos, neg = t
    flag = '' if min_ else ' MODEL-OUTSIDE'
    if not pin: flag += ' POLY-OUTSIDE'
    if not (pos or neg): flag += ' HOLDS-ZERO'
    if flag: bad += 1
    print(f"{tag}: model={model:.6e} poly={poly:.6e} width/|V|={rel if rel is None else format(rel,".2e")} poly_off_mid/width={off if off is None else format(off,".3g")} sign={'+' if pos else '-' if neg else '0'}{flag}")
print("rows", len(out), "flagged", bad)
