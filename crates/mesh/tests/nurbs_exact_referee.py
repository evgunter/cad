#!/usr/bin/env python3
"""EXACT-RATIONAL REFEREE for the rational patch bound's soundness claim.

`geom_brep::patch_bound` promises that every cell it reports ENCLOSES the
described patch's partial on that cell, and `mesh`'s `nurbs_face_bound`
collapses those cells into `muu`/`muv`/`mvv`. A dense `f64` sample of the
true partial cannot referee that claim to the last bit: the sampler carries
its own rounding (tens of operations), so a sample sitting under a bound
says nothing about whether the REAL value does.

This script is the referee that can. Every `f64` in a NURBS description is
an exact rational, so the whole quotient-rule jet — knot differencing of the
homogeneous nets `w` and `w·P`, Cox-de Boor on a NAMED span, and the
quotient rule's corrections — is computed in `fractions.Fraction` with no
rounding anywhere. The printed truth is then rounded DOWN to the nearest
`f64`, which is the largest `f64` that is provably ≤ the real value: exactly
the literal a Rust row may assert `<= muu` against without an allowance.

The two fixtures below are the surfaces whose escape this referee measured
(bilinear rational patches, `pu = pv = 1`, knots `[0,0,1,1]²`; the diagnostic
that drew them recorded their bits in hex, which is how a description crosses
a language boundary without rounding). `crates/mesh/src/nurbs_cert.rs`'s
`the_exact_truth_of_a_bilinear_rational_escapes_no_certified_sup` asserts the
numbers this prints.

    python3 crates/mesh/tests/nurbs_exact_referee.py

Independent of the kernel by construction: it shares no code with the Rust
side, only the description's bits and the mathematics.
"""

import struct
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F

getcontext().prec = 45

# Each fixture: the described bilinear rational patch's bits, plus the
# (u, v) corner and component at which the certificate was measured to be
# exceeded. `control` is x,y,z per control point, row-major in u.
FIXTURES = {
    # `mesh`'s r1 sweep, hosted run 33904520538, seed 0xdae51dbd4e1b79fd
    # trial 8 under the generator of the day; weights ≈ 0.0103–0.0135.
    "A": {
        "degree_u": 1,
        "degree_v": 1,
        "nu": 2,
        "nv": 2,
        "knots_u": "0000000000000000,0000000000000000,3ff0000000000000,3ff0000000000000",
        "knots_v": "0000000000000000,0000000000000000,3ff0000000000000,3ff0000000000000",
        "weights": "3f851d67062533ec,3f85e951aad106d4,3f8bae9fb9213c25,3f89570fcf39bbfe",
        "control": (
            "bfd45c5ea128e388,bfe353dc5381e28c,3feee4c74f58da14,"
            "3ffadea53eea524a,bffe1bfee0da6176,bfb57ac6093c8d60,"
            "3ff2421785e09da0,bff7ec6adc243a5c,bffd007c723fc4a6,"
            "bfd9ac7d34a077d8,3ff220925d90b010,bfc50c20054b4c20"
        ),
        "at": (F(0), F(0)),
    },
    # Hosted run 35050942262, seed 0x5ca58da03160d407 trial 29;
    # weights ≈ 3.3–5.2, so the escape is not an artefact of tiny weights.
    "B": {
        "degree_u": 1,
        "degree_v": 1,
        "nu": 2,
        "nv": 2,
        "knots_u": "0000000000000000,0000000000000000,3ff0000000000000,3ff0000000000000",
        "knots_v": "0000000000000000,0000000000000000,3ff0000000000000,3ff0000000000000",
        "weights": "4014b6e6992891ad,400a864c999793db,4010ca99b2590ee1,40106199a0a89a72",
        "control": (
            "bff54a0ceafe35ba,3f95e6ac362bdf80,bfd0260ded559cd0,"
            "3feabff6729274d0,bfe5f005b8ebadd4,3fe464f413811098,"
            "bff5221fc005d982,bff86db284fd9286,bfe9576c71483e30,"
            "3fe4da546f120828,3ff47c37162b9746,bfcb0fc4c9a47fd0"
        ),
        "at": (F(0), F(1)),
    },
}


def rational_of_hex(word):
    """The exact rational value of one big-endian IEEE-754 double."""
    return F(struct.unpack(">d", bytes.fromhex(word))[0])


def rationals(words):
    return [rational_of_hex(w) for w in words.split(",")]


def diff_along_first(net, knots, degree):
    """NURBS Book eq. 3.24 along the FIRST index: the derivative net and its
    once-differenced knot vector (the outer knot pair dropped)."""
    out = []
    for i in range(len(net) - 1):
        span = knots[i + degree + 1] - knots[i + 1]
        out.append([degree * (b - a) / span for a, b in zip(net[i], net[i + 1], strict=True)])
    return out, knots[1:-1]


def transpose(net):
    return [list(row) for row in zip(*net, strict=True)]


def basis_on_span(knots, degree, span, t):
    """Cox-de Boor restricted to a NAMED span, so a parameter sitting on a
    knot is read one-sided — which is the only well-posed reading for a
    second derivative, discontinuous there on a merely-C¹ patch."""
    active = {span: F(1)}
    for d in range(1, degree + 1):
        nxt = {}
        for i in range(span - d, span + 1):
            acc = F(0)
            if i in active and knots[i + d] != knots[i]:
                acc += (t - knots[i]) / (knots[i + d] - knots[i]) * active[i]
            if (i + 1) in active and knots[i + d + 1] != knots[i + 1]:
                acc += (
                    (knots[i + d + 1] - t)
                    / (knots[i + d + 1] - knots[i + 1])
                    * active[i + 1]
                )
            nxt[i] = acc
        active = nxt
    return active


def nonempty_spans_at(knots, degree, t):
    """Every nonempty span whose CLOSED extent contains `t` — at an interior
    knot there are two, and a second partial genuinely takes two values."""
    return [
        i
        for i in range(degree, len(knots) - degree - 1)
        if knots[i] < knots[i + 1] and knots[i] <= t <= knots[i + 1]
    ]


class Patch:
    """A described rational patch, as exact rationals: the weight net `w` and
    the three homogeneous nets `w·P`."""

    def __init__(self, spec):
        self.degree_u = spec["degree_u"]
        self.degree_v = spec["degree_v"]
        nu, nv = spec["nu"], spec["nv"]
        self.knots_u = rationals(spec["knots_u"])
        self.knots_v = rationals(spec["knots_v"])
        weights = rationals(spec["weights"])
        control = rationals(spec["control"])
        self.weight_net = [
            [weights[i * nv + j] for j in range(nv)] for i in range(nu)
        ]
        self.homogeneous = [
            [
                [weights[i * nv + j] * control[3 * (i * nv + j) + c] for j in range(nv)]
                for i in range(nu)
            ]
            for c in range(3)
        ]

    def derivative_net(self, net, k, ell):
        """`net` differenced `k` times in u and `ell` times in v, with the
        knot vectors and degrees that survive."""
        knots_u, degree_u = self.knots_u, self.degree_u
        knots_v, degree_v = self.knots_v, self.degree_v
        for _ in range(k):
            if degree_u == 0:
                return None
            net, knots_u = diff_along_first(net, knots_u, degree_u)
            degree_u -= 1
        for _ in range(ell):
            if degree_v == 0:
                return None
            flipped, knots_v = diff_along_first(transpose(net), knots_v, degree_v)
            net = transpose(flipped)
            degree_v -= 1
        return net, knots_u, degree_u, knots_v, degree_v

    def net_value(self, net, k, ell, span_u, span_v, u, v):
        """`∂^(k+ell) net / ∂u^k ∂v^ell` at `(u, v)`, read on the named span.
        Each differencing trims the front of a knot vector, so the span index
        shifts down by one per order taken in that direction."""
        derived = self.derivative_net(net, k, ell)
        if derived is None:
            return F(0)
        net, knots_u, degree_u, knots_v, degree_v = derived
        bu = basis_on_span(knots_u, degree_u, span_u - k, u)
        bv = basis_on_span(knots_v, degree_v, span_v - ell, v)
        return sum(bu[i] * bv[j] * net[i][j] for i in bu for j in bv)

    def second_partials(self, span_u, span_v, u, v):
        """The three second partials of `S = A/w` as exact rational vectors,
        by the quotient rule (`NurbsSurface::ders_in_span`'s corrections)."""
        orders = [(k, ell) for k in range(3) for ell in range(3) if k + ell <= 2]
        w = {
            o: self.net_value(self.weight_net, o[0], o[1], span_u, span_v, u, v)
            for o in orders
        }
        out = {"uu": [], "uv": [], "vv": []}
        for net in self.homogeneous:
            a = {
                o: self.net_value(net, o[0], o[1], span_u, span_v, u, v) for o in orders
            }
            s = a[0, 0] / w[0, 0]
            s_u = (a[1, 0] - s * w[1, 0]) / w[0, 0]
            s_v = (a[0, 1] - s * w[0, 1]) / w[0, 0]
            out["uu"].append((a[2, 0] - 2 * s_u * w[1, 0] - s * w[2, 0]) / w[0, 0])
            out["vv"].append((a[0, 2] - 2 * s_v * w[0, 1] - s * w[0, 2]) / w[0, 0])
            out["uv"].append(
                (a[1, 1] - s_u * w[0, 1] - s_v * w[1, 0] - s * w[1, 1]) / w[0, 0]
            )
        return out


def decimal_of(q):
    return Decimal(q.numerator) / Decimal(q.denominator)


def rounded_down_to_f64(q):
    """The largest `f64` that is ≤ the exact rational `q`. `float(q)` rounds
    to NEAREST, so it can land above; one step down fixes that, and the loop
    is written as a loop rather than a single `nextafter` because nothing
    here may assume which side the nearest rounding fell on."""
    x = float(q)
    while F(x) > q:
        x = struct.unpack(">d", struct.pack(">q", struct.unpack(">q", struct.pack(">d", x))[0] - 1))[0]
    return x


def main():
    for name, spec in sorted(FIXTURES.items()):
        patch = Patch(spec)
        u, v = spec["at"]
        print(f"=== fixture {name}: described bilinear rational at (u, v) = ({float(u)}, {float(v)})")
        for span_u in nonempty_spans_at(patch.knots_u, patch.degree_u, u):
            for span_v in nonempty_spans_at(patch.knots_v, patch.degree_v, v):
                jet = patch.second_partials(span_u, span_v, u, v)
                for component in ("uu", "uv", "vv"):
                    squared = sum(x * x for x in jet[component])
                    exact_norm = decimal_of(squared).sqrt()
                    # The norm is a square root, so it is irrational: round the
                    # SQUARE down in the rationals, then take the `f64` below
                    # the decimal square root. Both steps only ever go down.
                    literal = rounded_down_to_f64(F(str(exact_norm)))
                    print(
                        f"    span({span_u},{span_v}) ‖S_{component}‖ "
                        f"= {exact_norm:.25E}"
                    )
                    print(
                        f"        f64 rounded DOWN = {literal!r}  "
                        f"(hex {struct.pack('>d', literal).hex()})"
                    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
