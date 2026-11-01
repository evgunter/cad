import math
# Adjudicate the leaf volumes: OCC values (measured) vs a roundoff-controlled
# numeric integral (fsum'd Simpson, two n values for convergence evidence)
# and vs two algebraically different closed forms.
occ = {'lily_leaf_a': 3180159.511941958, 'lily_leaf_b': 2227624.0542962593,
       'lily_leaf_c': 1272436.9377670977}
params = {'lily_leaf_a': (1.45, 0.16, 0.035, 0.026),
          'lily_leaf_b': (1.25, 0.14, 0.030, 0.024),
          'lily_leaf_c': (0.95, 0.115, 0.025, 0.022)}

def band(c, wo, wi):
    def y(h, x):
        R = (c*c/4 + h*h)/(2*h)
        return (h - R) + math.sqrt(R*R - (x - c/2)**2)
    return lambda x: y(wo, x) - y(wi, x)

def simpson(f, a, b, n):
    h = (b - a)/n
    terms = [f(a), f(b)]
    terms += [4*f(a + i*h) for i in range(1, n, 2)]
    terms += [2*f(a + i*h) for i in range(2, n, 2)]
    return math.fsum(terms)*h/3

def seg_acos(c, h):
    R = (c*c/4 + h*h)/(2*h)
    return R*R*math.acos((R-h)/R) - (R-h)*c/2

def seg_asin(c, h):
    R = (c*c/4 + h*h)/(2*h)
    al = math.asin(min(c/(2*R), 1.0))
    return R*R*(al - math.sin(al)*math.cos(al))

for name, (c, wo, wi, t) in params.items():
    f = band(c, wo, wi)
    n1 = simpson(f, 0.0, c, 1 << 14) * t * 1e9
    n2 = simpson(f, 0.0, c, 1 << 16) * t * 1e9
    ca = (seg_acos(c, wo) - seg_acos(c, wi)) * t * 1e9
    cs = (seg_asin(c, wo) - seg_asin(c, wi)) * t * 1e9
    o = occ[name]
    print(name)
    print('  simpson n=16k vs 64k rel drift %.1e' % ((n1-n2)/n2))
    print('  occ vs simpson  %.3e' % ((o-n2)/n2))
    print('  occ vs acos-form %.3e   occ vs asin-form %.3e' % ((o-ca)/ca, (o-cs)/cs))
    print('  acos vs asin closed forms differ by %.1e (double roundoff floor)' % ((ca-cs)/cs))
