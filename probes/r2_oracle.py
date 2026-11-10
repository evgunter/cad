import sys
from decimal import Decimal as D, getcontext
getcontext().prec = 60
K = 6
def dist(R, r, p, z):
    # exact distance in the meridian half-plane to the arc {(R + r cos v, r sin v): |v| <= v*}
    dx, dz = p - R, z
    m = (dx*dx + dz*dz).sqrt()
    if R >= r:  # horn or beyond: whole circle on rho >= 0
        return abs(m - r)
    h = (r*r - R*R).sqrt()
    if m == 0:
        return min(r, (p*p + (z-h)**2).sqrt())
    foot = R + r*dx/m
    if foot >= 0:
        return abs(m - r)
    return min((p*p + (z-h)**2).sqrt(), (p*p + (z+h)**2).sqrt())
bad = {"ah": [], "ahf": [], "cert": []}
worst = {"ah": 0, "ahf": 0, "cert": 0}
cnt = 0
for line in open(sys.argv[1]):
    k, R, r, rl, rh, zl, zh, ah, ahf, cert = line.split()
    R, r, rl, rh, zl, zh = map(lambda s: D(float(s)), (R, r, rl, rh, zl, zh))
    far = D(0)
    for a in range(K+1):
        for b in range(K+1):
            p = rl + (rh-rl)*a/K; z = zl + (zh-zl)*b/K
            far = max(far, dist(R, r, p, z))
    cnt += 1
    for name, val in (("ah", ah), ("ahf", ahf), ("cert", cert)):
        if val == "none": continue
        v = D(float(val))
        if far > v and not (name == "cert" and v < D("1e-250")):
            rel = (far - v) / far
            bad[name].append((float(rel), line.strip(), float(far)))
for n, l in bad.items():
    l.sort(reverse=True)
    print(n, "violations:", len(l))
    for x in l[:5]: print("  rel", x[0], "far", x[2], "|", x[1])
print("boxes", cnt)
