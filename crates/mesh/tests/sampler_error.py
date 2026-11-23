#!/usr/bin/env python3
"""THE SAMPLER'S OWN ERROR, refereed exactly.

A certified bound encloses the REAL value of an expression. A dense `f64`
sample of the same expression is not that value: it carries the rounding of
its own tens of operations. So a comparison of a sample against a bound has
to widen the SAMPLE's side, and the size of that widening is a measurement
nobody had taken — `crates/mesh/src/nurbs_cert.rs`'s `SAMPLER_ULPS = 64` is
a house figure, and two work rows ask for the real one
(`work/props/the-samplers-own-error-has-three-spellings-and-no-home`,
`work/chord/soundness-sweep-allowance-is-fifty-times-the-measured-sampler-error`).

This script takes it. It reads the dump that
`nurbs_cert::tests::sampler_error_dump` prints — per trial, a bilinear
rational patch's bits, the certified triple, and the 61x61 sampler's
per-component ARGMAX — and evaluates the exact rational truth AT THAT POINT,
reusing `nurbs_exact_referee.py`'s engine. The answer per component is the
distribution of `|sampled − truth|` in ulps of the certified figure, which is
the unit `SAMPLER_ULPS` is written in.

    cargo test --release -p mesh --lib sampler_error_dump -- --ignored --nocapture \
      | python3 crates/mesh/tests/sampler_error.py

or with a saved dump as the one argument. 400 patches at
`CAD_FUZZ_EFFORT=1`, and that dial scales the breadth. The argmax is why the
dump exists:
the sampler's error has to be measured where the sampler actually looked, not
at a point this script would pick.
"""
import importlib.util
import struct
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F

getcontext().prec = 45

_REFEREE = "crates/mesh/tests/nurbs_exact_referee.py"
_spec = importlib.util.spec_from_file_location("nurbs_exact_referee", _REFEREE)
if _spec is None or _spec.loader is None:
    raise SystemExit(f"run from the repository root: {_REFEREE} not found")
ref = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(ref)


def f64(h):
    return struct.unpack(">d", bytes.fromhex(h))[0]


def fr(h):
    return F(f64(h))


def parse(stream):
    """The dump's rows. Anything else on the stream (cargo's own output, the
    fuzz seed line) is ignored, so the test can be piped straight in."""
    for line in stream:
        parts = line.split()
        if not parts or parts[0] != "DUMP":
            continue
        fields = dict(zip(parts[2::2], parts[3::2], strict=False))
        args = []
        for arg in fields["ARG"].split(","):
            value, u, v = arg.split(":")
            args.append((fr(value), fr(u), fr(v)))
        yield (
            fields["W"],
            fields["C"],
            [fr(x) for x in fields["B"].split(",")],
            args,
        )


def main(argv):
    stream = open(argv[0]) if argv else sys.stdin
    names = ("uu", "uv", "vv")
    stats = {n: [] for n in names}
    trials = 0
    for weights, control, bound, args in parse(stream):
        patch = ref.Patch(
            {
                "degree_u": 1,
                "degree_v": 1,
                "nu": 2,
                "nv": 2,
                "knots_u": ref.KNOTS,
                "knots_v": ref.KNOTS,
                "weights": weights,
                "control": control,
            }
        )
        trials += 1
        for k, name in enumerate(names):
            sampled, u, v = args[k]
            certified = bound[k]
            if certified <= 0:
                continue
            # A second partial is two-valued only where a knot sits, which a
            # single-span bilinear vector has nowhere but its clamped ends.
            # Where both readings exist, the sampler took one of them, so the
            # NEARER is the one its error is measured against.
            best = None
            for span_u in ref.nonempty_spans_at(patch.knots_u, patch.degree_u, u):
                for span_v in ref.nonempty_spans_at(patch.knots_v, patch.degree_v, v):
                    jet = patch.second_partials(span_u, span_v, u, v)[name]
                    squared = sum(x * x for x in jet)
                    truth = ref.decimal_of(squared).sqrt()
                    distance = abs(truth - ref.decimal_of(sampled))
                    if best is None or distance < best[0]:
                        best = (distance, truth)
            distance, truth = best
            ulp = ref.decimal_of(certified) * Decimal(2) ** Decimal(-52)
            stats[name].append(
                (float(distance / ulp), float(distance), float(ref.decimal_of(certified)))
            )
    if not trials:
        raise SystemExit("no DUMP rows on the input")
    print(f"{trials} trials; |sampled - truth| in ulps of the certified figure")
    for name in names:
        rows = sorted(stats[name])
        n = len(rows)
        worst = rows[-1]
        print(
            f"  {name}: n={n} median={rows[n // 2][0]:.4f} "
            f"p99={rows[int(n * 0.99)][0]:.4f} max={worst[0]:.4f} "
            f"(max absolute {worst[1]:.3e} on a certified {worst[2]:.6e})"
        )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
