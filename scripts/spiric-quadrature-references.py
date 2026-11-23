"""The 50-digit references behind the spiric quadrature lane's rows.

`crates/geom-brep/src/props/spiric.rs`'s containment test hard-codes, per
case, the f64 round-down/round-up pair of four integrals over a spiric arc
(`J = ∫ f cos t`, the wall image's `Δu`, `−∫ F du` and `−∫ G du`); the sweep
rows' cut-revolve oracles (`crates/sweep/tests/common/torus_walls.rs`) are
f64 adaptive Simpson. This script re-takes the first and cross-checks the
second, from the DEFINITIONS: `u` is differentiated numerically from
`σ·atan2(f, d)`, and the vessel's cavity is integrated over `ρ` directly
rather than through the closed-form inner integral the oracle uses.

Run with mpmath installed (`pip install mpmath`), from anywhere:

    python3 scripts/spiric-quadrature-references.py

It prints the case table in the test's order and, for each sweep oracle,
the 50-digit value beside the f64 value the oracle returned when the rows
were written.
"""

import math

import mpmath as mp

mp.mp.dps = 50
PI = math.pi


def f64_down(x):
    """The largest f64 at or below `x`."""
    y = float(x)
    return y if mp.mpf(y) <= x else math.nextafter(y, -math.inf)


def f64_up(x):
    """The smallest f64 at or above `x`."""
    y = float(x)
    return y if mp.mpf(y) >= x else math.nextafter(y, math.inf)


# (name, R, r, d, t0, t1, σ, v0) — the test's `cases()`, in order.
CASES = [
    (
        "the vessel's arc, d < 0",
        0.09375,
        0.0703125,
        -0.0078125,
        -0.8911225078866529,
        0.8911225078866528,
        -1.0,
        0.0,
    ),
    (
        "the vessel's arc, d > 0",
        0.09375,
        0.0703125,
        0.0078125,
        -0.8911225078866529,
        0.8911225078866528,
        -1.0,
        0.0,
    ),
    (
        "a span across π on the lift's second turn",
        1.2,
        0.225,
        0.05,
        PI / 2,
        3 * PI / 2,
        1.0,
        2 * PI,
    ),
    (
        "a whole period, d < 0, σ = −1, the lift one turn down",
        1.0,
        0.3,
        -0.25,
        0.0,
        2 * PI,
        -1.0,
        -2 * PI,
    ),
    ("an asymmetric span, where Δu is not zero", 1.2, 0.225, 0.05, 0.3, 2.1, -1.0, 0.0),
    ("a near-tangent cut, f_min ≈ 0.01", 1.0, 0.5, 0.4999, -PI, PI, 1.0, 0.0),
]


def case_integrals(big, r, d, t0, t1, sigma, v0, pieces):
    """One case's four integrals at 50 digits."""

    def f(t):
        return mp.sqrt((big + r * mp.cos(t)) ** 2 - d**2)

    def u(t):
        return sigma * mp.atan2(f(t), d)

    def ff(v):
        return r * (
            (big**2 + r**2) * mp.sin(v)
            + mp.mpf(3) / 2 * big * r * v
            + big * r / 2 * mp.sin(v) * mp.cos(v)
        )

    def gg(v):
        return r * (big * v + r * mp.sin(v))

    pts = [t0 + (t1 - t0) * k / pieces for k in range(pieces + 1)]
    return [
        mp.quad(lambda t: f(t) * mp.cos(t), pts),
        u(t1) - u(t0),
        mp.quad(lambda t: -ff(v0 + sigma * t) * mp.diff(u, t), pts),
        mp.quad(lambda t: -gg(v0 + sigma * t) * mp.diff(u, t), pts),
    ]


def case_references():
    """Each case's four integrals as (round-down, round-up) pairs."""
    for name, *data in CASES:
        big, r, d, t0, t1, sigma, v0 = (mp.mpf(x) for x in data)
        pieces = 64 if name.startswith("a near-tangent") else (8 if t1 - t0 > 4 else 2)
        print(f"{name}:")
        for x in case_integrals(big, r, d, t0, t1, sigma, v0, pieces):
            print(f"    ({f64_down(x)!r}, {f64_up(x)!r}),")


def kept(theta, t, caps, rho):
    """The azimuth a cut revolve keeps at radius `rho`."""
    return (
        max(theta - caps * mp.asin(t / rho), 0)
        if rho > t
        else (theta if caps == 0 else 0)
    )


def disc(theta, t, caps, rho_c, a):
    """`∬ ρ·kept dA` over the disc, by `ρ = ρ_c + a cos φ`."""

    def g(ph):
        rho = rho_c + a * mp.cos(ph)
        return 2 * a * a * mp.sin(ph) ** 2 * rho * kept(theta, t, caps, rho)

    return mp.quad(g, [0, mp.pi / 2, mp.pi])


def wall_area(theta, t, caps, rho_c, a):
    """`∫ a·ρ·kept dv` over the tube."""
    return mp.quad(
        lambda v: (
            a * (rho_c + a * mp.cos(v)) * kept(theta, t, caps, rho_c + a * mp.cos(v))
        ),
        [0, mp.pi, 2 * mp.pi],
    )


def cap_area(t, rho_c, a):
    """One moved cap's oval, by `z = a sin φ`."""

    def g(ph):
        w = a * mp.cos(ph)
        return (
            mp.sqrt((rho_c + w) ** 2 - t * t) - mp.sqrt((rho_c - w) ** 2 - t * t)
        ) * w

    return mp.quad(g, [-mp.pi / 2, 0, mp.pi / 2])


def vessel(t):
    """The sectioned vessel's hollow, its cavity integrated over `ρ`."""
    theta = mp.pi / 2
    u = mp.mpf(1) / 64
    rf, rn, yf, ys, ym = 5 * u, 7 * u, 4 * u, 12 * u, 24 * u
    a, b, c = 6 * u, 5 * u, 8 * u

    def band(rad, y):
        return a + mp.sqrt(rad**2 - (y - c) ** 2)

    outer = theta * (
        rf**2 / 2 * yf
        + mp.quad(lambda y: band(b, y) ** 2 / 2, [yf, ys])
        + rn**2 / 2 * (ym - ys)
    )

    def inner(rc):
        return mp.quad(lambda rho: rho * kept(theta, t, 2, rho), [t * mp.sqrt(2), rc])

    cav = (
        inner(rf - t) * yf
        + mp.quad(lambda y: inner(band(b - t, y)), [yf + t, ys - t])
        + inner(rn - t) * (ym - ys)
    )
    return outer - cav


def bowl(t):
    """The 120° bowl sector's hollow."""
    th = mp.radians(120)
    half = mp.mpf("1.35")
    sag = half * mp.mpf("0.8")
    rad = (half**2 + sag**2) / (2 * sag)
    yc = rad - sag

    def strip(k, lo, hi, top, rr):
        return mp.quad(
            lambda rho: (
                rho
                * kept(th, t, k, rho)
                * (top - (yc - mp.sqrt(rr**2 - (rho - mp.mpf("1.65")) ** 2)))
            ),
            [lo, hi],
        )

    return strip(0, mp.mpf("0.3"), 3, 1, rad) - strip(
        2, mp.mpf("0.3") + t, 3 - t, 1 - t, rad - t
    )


def sweep_oracles():
    """Each sweep row's oracle at 50 digits, beside its f64 Simpson value."""
    q = mp.pi / 2
    rows = [
        (
            "klein elbow, t = 0.05",
            disc(q, 0.05, 0, 1.2, 0.275) - disc(q, 0.05, 2, 1.2, 0.225),
            0.16395310971186178,
        ),
        (
            "klein elbow, t = 0.02",
            disc(q, 0.02, 0, 1.2, 0.275) - disc(q, 0.02, 2, 1.2, 0.255),
            0.07094235815005351,
        ),
        (
            "klein elbow, t = 0.1",
            disc(q, 0.1, 0, 1.2, 0.275) - disc(q, 0.1, 2, 1.2, 0.175),
            0.2857442803759651,
        ),
        (
            "one-cap klein arm",
            disc(q, 0.05, 0, 1.2, 0.275) - disc(q, 0.05, 1, 1.2, 0.225),
            0.15599858786410103,
        ),
        (
            "partial two-arc torus",
            disc(q, 0.05, 0, 2, 0.5) - disc(q, 0.05, 2, 2, 0.45),
            0.5324303519258431,
        ),
        (
            "hand-built wall pair",
            (disc(q, 0.01, 0, 1.2, 0.275) - disc(q, 0.01, 0, 1.2, 0.225))
            - (disc(q, 0.01, 2, 1.2, 0.265) - disc(q, 0.01, 2, 1.2, 0.235)),
            0.06016011586517567,
        ),
        ("bowl sector", bowl(mp.mpf("0.05")), 1.8728169147827032),
        ("sectioned vessel", vessel(mp.mpf(1) / 128), 0.0014105499847621812),
        (
            "klein elbow surface area",
            wall_area(q, 0, 0, 1.2, 0.275)
            + 2 * cap_area(0, 1.2, 0.275)
            + wall_area(q, 0.05, 2, 1.2, 0.225)
            + 2 * cap_area(0.05, 1.2, 0.225),
            6.573883926766782,
        ),
    ]
    for name, exact, simpson in rows:
        rel = abs(exact - simpson) / abs(exact)
        print(
            f"{name}: {mp.nstr(exact, 20)}  f64 oracle {simpson!r}  rel {mp.nstr(rel, 3)}"
        )


if __name__ == "__main__":
    case_references()
    sweep_oracles()
