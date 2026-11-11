import mpmath as mp
mp.mp.dps = 20
BIG, SMALL = mp.mpf(2), mp.mpf('0.5')
def prim(c, y):  # ∫ sqrt(c²−y²) dy
    y = max(min(y, c), -c)
    return (y*mp.sqrt(c*c-y*y) + c*c*mp.asin(y/c))/2
def inner_len(y, z, x0, x1):
    w = mp.sqrt(SMALL**2 - z*z) if abs(z) < SMALL else mp.mpf(0)
    a, b = BIG - w, BIG + w
    # positive-x part and negative-x part of {a² ≤ x²+y² ≤ b²} ∩ [x0,x1]
    tot = mp.mpf(0)
    hb2, ha2 = b*b-y*y, a*a-y*y
    if hb2 <= 0: return tot
    hb = mp.sqrt(hb2); ha = mp.sqrt(ha2) if ha2 > 0 else mp.mpf(0)
    for lo, hi in [(ha, hb), (-hb, -ha)]:
        l, h = max(lo, x0), min(hi, x1)
        if h > l: tot += h - l
    return tot
def alongx(x0, x1, y0, z0, rc):
    x0, x1, y0, z0, rc = map(mp.mpf, (x0, x1, y0, z0, rc))
    def slice_(z):
        h2 = rc*rc - (z-z0)**2
        if h2 <= 0: return mp.mpf(0)
        h = mp.sqrt(h2)
        w = mp.sqrt(SMALL**2 - z*z)
        b, a = BIG + w, BIG - w
        cuts = {y0-h, y0+h}
        for c in [a, b]:
            for X in [x0, x1]:
                if c*c - X*X > 0:
                    r = mp.sqrt(c*c-X*X); cuts |= {r, -r}
            cuts |= {c, -c}
        cuts = sorted(cc for cc in cuts if y0-h <= cc <= y0+h)
        return mp.quad(lambda y: inner_len(y, z, x0, x1), cuts)
    zs = [z0-rc] + [z0 + rc*mp.mpf(k)/40 - rc for k in range(1,80)] + [z0+rc]
    return mp.quad(slice_, zs)
donut = 2*mp.pi**2*BIG*SMALL**2
for name, args in [("square S", (-1,1,'2.45',0,'0.1')), ("radial both",(1,3,0,0,'0.2')),
                   ("radial inner",(1,2,0,0,'0.2')),("radial outer",(2,3,0,0,'0.2')),("radial off",(1,3,'0.15','0.1','0.2'))]:
    I = alongx(*args)
    print(name, mp.nstr(donut - I, 20))
