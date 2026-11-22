"""The deep cell pairs' closest carrier points (min-dist-rows.tsv, deep_pa /
deep_pb) placed against the operands' implicit forms, from the tests' own
poses: a union keeps each operand's boundary outside the other, so a carrier
point inside the other operand lies on no face. Also the two carriers'
normal angle at the pair. Run: python3 -I measure/gate/min-implicit.py"""
import math

R, r = 0.6, 0.2


def rot(axis, a, p):
    n = math.sqrt(sum(c * c for c in axis))
    k = [c / n for c in axis]
    c, s = math.cos(a), math.sin(a)
    d = sum(k[i] * p[i] for i in range(3))
    cr = [k[1] * p[2] - k[2] * p[1], k[2] * p[0] - k[0] * p[2], k[0] * p[1] - k[1] * p[0]]
    return [p[i] * c + cr[i] * s + k[i] * d * (1 - c) for i in range(3)]


def donut(p):  # germ_torus_components::donut, quadric_pair_sections::donut(0.6, 0.2)
    x, y, z = p
    return math.hypot(math.hypot(x, z) - R, y) - r


def placed(a, ax, t):
    return lambda p: donut(rot(ax, -a, [p[j] - t[j] for j in range(3)]))


def ball(p):
    return math.sqrt(sum(c * c for c in p)) - 0.5


def unit_grad(f, p, h=1e-7):
    g = []
    for j in range(3):
        e = [h if k == j else 0.0 for k in range(3)]
        g.append((f([p[k] + e[k] for k in range(3)]) - f([p[k] - e[k] for k in range(3)])) / (2 * h))
    n = math.sqrt(sum(c * c for c in g))
    return [c / n for c in g]


def angle(f, p, g, q):
    a, b = unit_grad(f, p), unit_grad(g, q)
    return math.degrees(math.acos(abs(sum(x * y for x, y in zip(a, b)))))


# Two donuts, placement 1 (the body the hook caught): A = donut, B = placement 1.
B1 = placed(1.5707963, (1, 0, 0), (0.3, 0.2, 0.1))
pa = [0.7491833062448313, 0.1319609016151533, -0.037016731731439614]  # on 5v3's carrier
pb = [0.7489240791086947, 0.13248315551803436, -0.03679228072310946]  # on 6v3's carrier
print("torus x torus: pa on B1 %.2e, in A %.3e; pb on A %.2e, against B1 %.3e; angle %.1f deg"
      % (B1(pa), donut(pa), donut(pb), B1(pb), angle(B1, pa, donut, pb)))
# A ball and a donut: the donut posed by 0.7 about x and (0.1, 0.35, 0.2).
D = placed(0.7, (1, 0, 0), (0.1, 0.35, 0.2))
qa = [-0.40096798464762956, 0.004079578233121689, 0.30122428092890696]  # on 1v3's carrier
qb = [-0.39976863455395223, 0.0, 0.30030823969193465]  # on 2v1's carrier
print("torus x sphere: qa on D %.2e, against ball %.3e; qb on ball %.2e, against D %.3e; angle %.1f deg"
      % (D(qa), ball(qa), ball(qb), D(qb), angle(D, qa, ball, qb)))
