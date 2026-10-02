"""Reviewer oracle (PR 3847 dual, lane r1): mpmath at 50 digits on the
exact f64 inputs the door was handed. Never reads the kernel's numbers
except the certified thetas under test."""
import sys
from collections import Counter, defaultdict
from mpmath import mp, mpf, cos, sin, atan2, findroot, pi, sqrt
mp.dps = 50

def cross(a, b):
    return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
def f64cross(a, b):  # the kernel's own rounding of v, as a second model
    a=[float(x) for x in a]; b=[float(x) for x in b]
    return [mpf(a[1]*b[2]-a[2]*b[1]), mpf(a[2]*b[0]-a[0]*b[2]), mpf(a[0]*b[1]-a[1]*b[0])]
def dot(a, b): return sum(x*y for x, y in zip(a, b))

def analyse(c, n, u, s, rho, r, vfun):
    v = vfun(n, u)
    E = [ci - si for ci, si in zip(c, s)]
    def p(t): return [E[i] + rho*(cos(t)*u[i] + sin(t)*v[i]) for i in range(3)]
    def f(t): q = p(t); return dot(q, q) - r*r
    def fp(t):
        q = p(t); dq = [rho*(-sin(t)*u[i] + cos(t)*v[i]) for i in range(3)]
        return 2*dot(q, dq)
    phi = atan2(dot(E, v), dot(E, u))
    tmax = findroot(fp, phi); tmin = findroot(fp, phi + pi)
    return f, fp, f(tmin), f(tmax)

def wrapdiff(a, b):
    d = (a - b) % (2*pi)
    return min(d, 2*pi - d)

def main(path):
    stats = defaultdict(Counter)
    worst = defaultdict(lambda: (0, None))
    for line in open(path):
        w = line.split()
        k, kind, eps, verdict = int(w[0]), int(w[1]), float(w[2]), w[3]
        x = [mpf(float(t)) for t in w[4:]]
        c, n, u, s = x[0:3], x[3:6], x[6:9], x[9:12]
        rho, r, t1, t2, slack = x[12], x[13], x[14], x[15], float(w[4+16])
        scale = [1e-3, 1.0, 1e3][(k // 3) % 3]
        key = (eps, kind, scale)
        stats[key][verdict[:2]] += 1
        f, fp, fmin, fmax = analyse(c, n, u, s, rho, r, cross)
        crossing = fmin < 0 < fmax
        if verdict == 'M' and crossing:
            stats[key]['MISS_ON_CROSSING'] += 1
            print('MISS-ON-CROSSING', line.strip())
        if verdict.startswith('C'):
            if not crossing:
                stats[key]['CERT_ON_NONCROSSING'] += 1
                print('CERT-NONCROSS', line.strip())
                continue
            for model, vf in (('exact_v', cross), ('f64_v', f64cross)):
                f2, fp2, _, _ = analyse(c, n, u, s, rho, r, vf)
                errs = []
                for t in (t1, t2):
                    try:
                        tt = findroot(f2, t)
                    except Exception:
                        tt = findroot(f2, (t - mpf('1e-3'), t + mpf('1e-3')), solver='anderson')
                    errs.append(float(rho * wrapdiff(tt, t)))
                e = max(errs)
                if model == 'exact_v':
                    phase_t, angle_t = float(w[4+21]), float(w[4+22])
                    if e > slack - phase_t: stats[key]['NEEDS_PHASE_TERM'] += 1
                    if e > slack - angle_t: stats[key]['NEEDS_ANGLE_TERM'] += 1
                    if e > slack - angle_t - phase_t: stats[key]['NEEDS_PHASE+ANGLE'] += 1
                    if e > eps:
                        stats[key]['ARC_ERR_GT_EPS'] += 1
                        print('ARC>EPS', e, line.strip())
                    if e > slack:
                        stats[key]['ARC_ERR_GT_SLACK'] += 1
                    ratio = e / slack
                    if ratio > worst[key][0]:
                        worst[key] = (ratio, (e, slack, k))
                else:
                    if e > eps:
                        stats[key]['ARC_ERR_GT_EPS_f64v'] += 1
    for key in sorted(stats):
        print(key, dict(stats[key]), 'worst err/slack', worst[key])

main(sys.argv[1])
