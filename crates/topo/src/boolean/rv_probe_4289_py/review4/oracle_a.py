# Review 4: exact oracle in the reader's own model ("model A"): a face is
# the plane through the vertex with the sector's given normal n (held as a
# surface); its sector is the region of that plane between its bounds'
# far points ((fs x q).n >= 0 and (q x fe).n >= 0). Band moves: the
# probe's far point anywhere within eps; a bound's far point anywhere
# within eps (only its in-plane part matters to each face it bounds).
import math, random
from fractions import Fraction as F

def fr(v): return [F(x) for x in v]
def dot(a, b): return a[0]*b[0] + a[1]*b[1] + a[2]*b[2]
def cross(a, b): return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
def sub(a, b): return [a[i]-b[i] for i in range(3)]
def add(a, b): return [a[i]+b[i] for i in range(3)]
def mul(a, s): return [x*s for x in a]

class OracleA:
    def __init__(self, secs):
        self.w = [(fr(s['fs']), fr(s['fe']), fr(s['n'])) for s in secs]
        self.ref = None

    @staticmethod
    def in_sector(S, E, n, q):
        a = dot(cross(S, q), n); b = dot(cross(q, E), n)
        if a < 0 or b < 0: return 'no'
        # exclude the antipodal double-wedge part (sector < 180): q.(S+E) proxy
        if a == 0 and b == 0: return 'apex' if all(x == 0 for x in q) else 'no'
        return 'edge' if (a == 0 or b == 0) else 'yes'

    def on(self, q):
        for S, E, n in self.w:
            if dot(q, n) == 0 and self.in_sector(S, E, n, q) in ('yes', 'edge'):
                return True
        return False

    def crossings(self, q, r, skip=None):
        par = 0
        for k, (S, E, n) in enumerate(self.w):
            if k == skip: continue
            a, b = dot(q, n), dot(r, n)
            if a == 0 and b == 0: return None
            if (a > 0 and b > 0) or (a < 0 and b < 0): continue
            if a == 0 or b == 0:
                x = q if a == 0 else r
                if self.in_sector(S, E, n, x) != 'no': return None
                continue
            t = a / (a - b)
            x = add(q, mul(sub(r, q), t))
            st = self.in_sector(S, E, n, x)
            if st == 'yes': par ^= 1
            elif st != 'no': return None
        return par

    def reference(self):
        if self.ref is not None: return self.ref
        for k, (S, E, n) in enumerate(self.w):
            m = add(S, E)
            m = sub(m, mul(n, dot(m, n) / dot(n, n)))
            if self.in_sector(S, E, n, m) != 'yes': continue
            eta = F(1, 64)
            for _ in range(60):
                r = sub(m, mul(n, eta))
                if self.crossings(m, r, skip=k) == 0 and not self.on(r):
                    self.ref = r; return r
                eta /= 4
        raise RuntimeError('no reference')

    def cls(self, q):
        q = fr(q)
        if self.on(q): return 'On'
        r = self.reference()
        c = self.crossings(q, r)
        tries = 0
        while c is None:
            tries += 1
            rr = add(r, [F(random.randint(-1000, 1000), 2**44) for _ in range(3)])
            if self.crossings(r, rr) == 0 and not self.on(rr):
                c = self.crossings(q, rr)
            if tries > 200: raise RuntimeError('no reading')
        return 'In' if c == 0 else 'Out'

# ---- float geometry for building moves ----
def fdot(a, b): return sum(x*y for x, y in zip(a, b))
def fnorm(a): l = math.sqrt(fdot(a, a)); return [x/l for x in a]
def flen(a): return math.sqrt(fdot(a, a))

def nearest_on_face(sec, D):
    """nearest point of the closed sector (model A) to D, floats"""
    n = sec['n']; h = fdot(D, n)
    pr = [D[i] - h*n[i] for i in range(3)]
    S, E = sec['fs'], sec['fe']
    a = fdot(cross(S, pr), n); b = fdot(cross(pr, E), n)
    if a >= 0 and b >= 0 and fdot(pr, add(S, E)) >= 0:
        return pr
    best = None
    for B in (S, E):
        Bp = [B[i] - fdot(B, n)*n[i] for i in range(3)]
        u = fnorm(Bp)
        t = max(0.0, fdot(pr, u))
        c = [t*u[i] for i in range(3)]
        if best is None or flen(sub(D, c)) < flen(sub(D, best)): best = c
    return best

def moves(secs, D, eps):
    """candidate band moves: (label, secs', D') ; each point moved <= eps"""
    out = []
    k = 0.999 * eps
    for j, s in enumerate(secs):
        c = nearest_on_face(s, D)
        v = sub(c, D); l = flen(v)
        if 0 < l:
            out.append((f'D->f{j}', secs, [D[i] + v[i]/l*k for i in range(3)]))
    # bound far points: turn in each adjacent face's plane toward D, and
    # also a joint move of D toward the moved bound
    far_ids = {}
    for j, s in enumerate(secs):
        for key in ('fs', 'fe'):
            far_ids.setdefault(tuple(s[key]), []).append((j, key))
    for far, owners in far_ids.items():
        L = flen(far)
        for (j, key) in owners:
            n = secs[j]['n']
            perp = fnorm(cross(n, far))
            h = fdot(D, n); pr = [D[i] - h*n[i] for i in range(3)]
            sgn = 1.0 if fdot(pr, perp) > 0 else -1.0
            for sg in (sgn, -sgn):
                nf = [far[i] + sg*perp[i]*k for i in range(3)]
                ns = []
                for s in secs:
                    s2 = dict(s)
                    if tuple(s['fs']) == far: s2['fs'] = nf
                    if tuple(s['fe']) == far: s2['fe'] = nf
                    ns.append(s2)
                out.append((f'B{j}{key}{"+" if sg > 0 else "-"}', ns, D))
                # joint: D toward the moved face
                c = nearest_on_face(ns[j], D); v = sub(c, D); l = flen(v)
                if l > 0:
                    out.append((f'B{j}{key}{"+" if sg > 0 else "-"}&D', ns, [D[i] + v[i]/l*k for i in range(3)]))
    return out
