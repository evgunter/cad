import math

PI = math.pi


def deg(d):
    return d * PI / 180.0


def rot(v, a):
    x, z = v
    return (x * math.cos(a) - z * math.sin(a), x * math.sin(a) + z * math.cos(a))


class T:
    def __init__(s, p, t):
        s.p = p
        s.t = t

    def arc(s, ring, turn):
        n = (-s.t[1], s.t[0]) if turn >= 0 else (s.t[1], -s.t[0])
        c = (s.p[0] + ring * n[0], s.p[1] + ring * n[1])
        radial = ((s.p[0] - c[0]) / ring, (s.p[1] - c[1]) / ring)
        adv = rot(radial, turn)
        return dict(center=c, radial=radial, ring=ring, turn=turn), T(
            (c[0] + ring * adv[0], c[1] + ring * adv[1]), rot(s.t, turn)
        )


root = T((0.0, 0.0), (0.0, 1.0))
lower, at_fork = root.arc(5.0, deg(22.0))
upper, at_flower = at_fork.arc(1.1, deg(170.0))
print("at_fork.p", repr(at_fork.p), "t", repr(at_fork.t))
print("P2 = at_flower.p", repr(at_flower.p))
print("T2 = at_flower.t", repr(at_flower.t))
print("upper center", repr(upper["center"]), "ring", upper["ring"])
ARCH_R = 0.052
FLOWER_GLOBE = 0.44
FLOWER_TOP = 0.40
alpha = 70.0 * PI / 180.0
r_top = math.sqrt(FLOWER_GLOBE**2 - FLOWER_TOP**2)
print("r_top", repr(r_top))
neck_drop = (r_top - ARCH_R) / math.tan(alpha)
print("neck_drop", repr(neck_drop))
depth = neck_drop + FLOWER_TOP
print("flower_globe_depth = neck_drop+FLOWER_TOP", repr(depth))
P2 = at_flower.p
T2 = at_flower.t
C1 = (P2[0] + depth * T2[0], P2[1] + depth * T2[1])
print("SPHERE1_C derived", repr(C1))
print("PR claims        (-2.3668444700923885, 0.7942577551075498)")
# old
old_attach = (P2[0] - 0.08 * T2[0], P2[1] - 0.08 * T2[1])
oldC = (old_attach[0] + FLOWER_TOP * T2[0], old_attach[1] + FLOWER_TOP * T2[1])
print(
    "old SPHERE1_C     ",
    repr(oldC),
    " PR-before (-2.3934135869350324, 0.919255622187704)",
)
d = math.dist(C1, oldC)
print("globe moved by", repr(d))
# torus carrier for the arch
print()
print("=== torus/meridian check (exact f64 arithmetic in python = f64) ===")
Cx, Cz = upper["center"]
w0 = P2[0] - Cx
w2 = P2[1] - Cz
radlen = math.sqrt(w0 * w0 + 0.0 * 0.0 + w2 * w2)
print("|P2-C| =", repr(radlen), "  big_R =", repr(1.1), "  diff", repr(radlen - 1.1))
# tangent-cone alpha
ta = math.atan2(FLOWER_TOP, r_top)
print("tangent-cone alpha rad", repr(ta), "deg", repr(ta * 180 / PI))
print("tangent neck_drop", repr((r_top - ARCH_R) / math.tan(ta)))
