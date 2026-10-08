# rv4289: generator + exact oracle for polygon-cone membership
import math, random, sys
from fractions import Fraction as F

def norm(a): l = math.sqrt(sum(x*x for x in a)); return [x/l for x in a]
def cross(a,b): return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
def dot(a,b): return sum(x*y for x,y in zip(a,b))
def add(a,b): return [x+y for x,y in zip(a,b)]
def sub(a,b): return [x-y for x,y in zip(a,b)]
def mul(a,s): return [x*s for x in a]

def rot(axis, ang, p):
    k = norm(axis); c, s = math.cos(ang), math.sin(ang)
    kxp = cross(k,p); kd = dot(k,p)
    return [p[i]*c + kxp[i]*s + k[i]*kd*(1-c) for i in range(3)]

def sectors_from_ring(ring, hollow=False):
    """ring: list of (vec, is_edge). Pieces between consecutive points, minor arcs,
    material on the left (non-hollow). Face id increments after each edge point."""
    n = len(ring)
    # face ids: face k spans from edge point i to next edge point
    pts = [(norm(r[0]), r[1], (r[2] if len(r) > 2 else random.uniform(0.5, 2.0)) if r[1] else None) for r in ring]
    # far lengths for edges
    secs = []
    edge_idx = [i for i in range(n) if pts[i][1]]
    assert edge_idx, "need an edge"
    # rotate so first point is an edge
    s0 = edge_idx[0]
    pts = pts[s0:] + pts[:s0]
    face = -1
    faces = []
    for i in range(n):
        if pts[i][1]: face += 1
        faces.append(face)
    # arm per face: min of its bounding edges' lengths
    nf = face + 1
    arms = {}
    for f in range(nf):
        idx = [i for i in range(n) if faces[i] == f]
        first = idx[0]; last_next = (idx[-1] + 1) % n
        arms[f] = min(pts[first][2], pts[last_next][2])
    for i in range(n):
        a, ea, la = pts[i]; b, eb, lb = pts[(i+1) % n]
        f = faces[i]
        if not hollow:
            st, en, est, een, lst, len_ = b, a, eb, ea, lb, la
        else:
            st, en, est, een, lst, len_ = a, b, ea, eb, la, lb
        nn = norm(cross(st, en))
        farst = mul(st, lst) if est else [0, 0, 0]
        faren = mul(en, len_) if een else [0, 0, 0]
        secs.append(dict(s=st, e=en, n=nn, es=est, ee=een, fs=farst, fe=faren, arm=arms[f], face=f))
    return secs

# ---------- exact oracle ----------
def fr(v): return [F(x) for x in v]
def det3(a,b,c):
    return (a[0]*(b[1]*c[2]-b[2]*c[1]) - a[1]*(b[0]*c[2]-b[2]*c[0]) + a[2]*(b[0]*c[1]-b[1]*c[0]))

class Oracle:
    def __init__(self, secs):
        self.w = []
        for s in secs:
            S, E = fr(s['s']), fr(s['e'])
            # outward normal sign relative to S x E
            o = 1 if dot(cross(s['s'], s['e']), s['n']) > 0 else -1
            self.w.append((S, E, o))
        self.ref = None
    def on(self, q):
        for S, E, o in self.w:
            if det3(S, E, q) == 0:
                # q = a S + b E ? solve in plane: use cross products
                # a = (q x E)·(S x E) / |S x E|^2 , b = (S x q)·(S x E)/...
                SxE = [S[1]*E[2]-S[2]*E[1], S[2]*E[0]-S[0]*E[2], S[0]*E[1]-S[1]*E[0]]
                qxE = [q[1]*E[2]-q[2]*E[1], q[2]*E[0]-q[0]*E[2], q[0]*E[1]-q[1]*E[0]]
                Sxq = [S[1]*q[2]-S[2]*q[1], S[2]*q[0]-S[0]*q[2], S[0]*q[1]-S[1]*q[0]]
                a = sum(x*y for x, y in zip(qxE, SxE)); b = sum(x*y for x, y in zip(Sxq, SxE))
                if a >= 0 and b >= 0: return True
        return False
    def crossings(self, q, r, skip=None):
        """parity of segment q->r crossing wedges; None if degenerate"""
        d = [r[i]-q[i] for i in range(3)]
        par = 0
        for k, (S, E, o) in enumerate(self.w):
            if k == skip: continue
            # q + t d = a S + b E  ->  t d - a S - b E = -q
            m = det3(d, S, E)
            if m == 0:
                # parallel: degenerate if q in the plane
                if det3(q, S, E) == 0: return None
                continue
            nq = [-x for x in q]
            # Cramer: columns d, -S, -E ; rhs -q
            mS = [-x for x in S]; mE = [-x for x in E]
            D = det3(d, mS, mE)
            t = det3(nq, mS, mE) / D
            a = det3(d, nq, mE) / D
            b = det3(d, mS, nq) / D
            if t < 0 or t > 1: continue
            if a < 0 or b < 0: continue
            if t == 0 or t == 1 or a == 0 or b == 0: return None
            par ^= 1
        return par
    def reference(self):
        if self.ref: return self.ref
        for k, (S, E, o) in enumerate(self.w):
            m = [S[i]+E[i] for i in range(3)]
            n = [S[1]*E[2]-S[2]*E[1], S[2]*E[0]-S[0]*E[2], S[0]*E[1]-S[1]*E[0]]
            inner = [-o*x for x in n]  # inner = -outward
            eta = F(1, 64)
            for _ in range(40):
                r = [m[i] + eta*inner[i] for i in range(3)]
                c = self.crossings(m, r, skip=k)
                if c == 0:
                    self.ref = r; return r
                eta /= 4
        raise RuntimeError("no reference")
    def cls(self, q):
        q = fr(q)
        if self.on(q): return 'On'
        r = self.reference()
        c = self.crossings(q, r)
        if c is None:
            # use another reference: perturb
            for j in range(1, 50):
                rr = [r[i] + F(random.randint(-1000, 1000), 2**40) for i in range(3)]
                if self.crossings(r, rr) != 0: continue
                c = self.crossings(q, rr)
                if c is not None: break
        return 'In' if c == 0 else 'Out'

def m_excl(orc, m, k):
    return [F(10**9)]*3  # placeholder: m on other wedges checked by crossings skip (cheap)

def angdist(secs, q):
    q = norm(q); best = 9
    for s in secs:
        n = s['n']; h = dot(q, n)
        pr = sub(q, mul(n, h))
        inside = dot(cross(s['s'], pr), n) >= 0 and dot(cross(pr, s['e']), n) >= 0
        if inside: best = min(best, abs(h))
        for b in (s['s'], s['e']):
            best = min(best, math.sqrt(max(0, sum((x-y)**2 for x, y in zip(q, b)))))
    return best
