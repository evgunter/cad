"""R2 e2e (M10-2 review): read a measurement and its verdict from
Python, as a first-time consumer, off a document authored elsewhere.

The document is the reviewer's ball-in-socket fit (C5's concentric
spheres), written by `r2_e2e_ball_in_socket_authored_and_saved` in
editor-core's probe suite. Authoring a measure from Python is NOT
bound (`B-MEASURES`), so this half is deliberately read-only — which is
itself the friction under test.

Run:  PYTHONPATH=target/python-stage python3 crates/pncad-py/examples/r2_read_a_verdict.py
"""

import sys
from pathlib import Path

import pncad
from pncad import evaluate, load

FIXTURE = Path(__file__).resolve().parents[3] / "target" / "r2_fit.pncad"


def main() -> int:
    if not FIXTURE.exists():
        print(f"missing {FIXTURE}; run the editor-core probe row first", file=sys.stderr)
        return 2

    doc = load(FIXTURE.read_text(encoding="utf-8")).doc
    ev = evaluate(doc)

    # A consumer who did NOT author the document has to find the
    # measure and the assertion. There is no `doc.measures()`; the only
    # route is to walk the order and dispatch on `Value.kind`.
    found = []
    for node in doc.order():
        try:
            v = ev.value(node)
        except Exception as e:  # a failed node is not this script's business
            print(f"  node {node}: no value ({type(e).__name__})")
            continue
        found.append((node, v.kind))

    print("nodes and their value kinds:")
    for node, kind in found:
        print(f"  {node}: {kind}")

    measures = [n for n, k in found if k == "measure"]
    assertions = [n for n, k in found if k == "assertion"]
    print(f"\nfound {len(measures)} measure(s), {len(assertions)} assertion(s)")
    if not measures or not assertions:
        print("FAIL: the read doors found nothing", file=sys.stderr)
        return 1

    m = ev.value(measures[0]).measure()
    print(f"\nmeasurement: {m!r}")
    print(f"  dimension = {m.dimension}")
    print(f"  value     = {m.value}")
    print(f"  length    = {m.length!r}")

    v = ev.value(assertions[0]).assertion()
    print(f"\nverdict: {v!r}")
    print(f"  status   = {v.status}")
    print(f"  holds    = {v.holds}")
    print(f"  measured = {v.measured}")
    print(f"  bound    = {v.bound}")
    print(f"  reason   = {v.reason}")

    # The authored oracle: R - r - |dc| = 1.0 - 0.9 - 0.15 = -0.05,
    # against an AtLeast bound of 0.02 -> Violated.
    ok = True
    if abs(m.value - (-0.05)) > 1e-12:
        print(f"FAIL: expected the authored gap -0.05, got {m.value}", file=sys.stderr)
        ok = False
    if v.status != "Violated" or v.holds is not False:
        print(f"FAIL: expected Violated/False, got {v.status}/{v.holds}", file=sys.stderr)
        ok = False
    if v.measured is None or abs(v.measured - m.value) > 1e-12:
        print("FAIL: the verdict does not carry the measure's own number", file=sys.stderr)
        ok = False
    if v.bound is None or abs(v.bound - 0.02) > 1e-15:
        print("FAIL: the verdict does not carry the bound", file=sys.stderr)
        ok = False

    # Friction probes, reported not asserted.
    print("\n--- friction ---")
    try:
        ev.value(measures[0]).assertion()
        print("  measure.assertion() did NOT refuse (expected a typed refusal)")
    except pncad.EvaluationError as e:
        print(f"  measure.assertion() refuses: reason={e.reason!r}")
    # Is the bound available as a typed quantity, like the measurement?
    print(f"  Measurement.length is typed: {m.length is not None}")
    print(f"  Verdict.bound is a bare float: {isinstance(v.bound, float)}")
    print(f"  Verdict exposes no direction (>= or <=): {'dir' not in dir(v)}")
    print(f"  Verdict exposes no dimension: {'dimension' not in dir(v)}")
    print(f"  no doc-level measure listing: {'measures' not in dir(doc)}")

    print("\nOK" if ok else "\nFAILURES ABOVE")
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
