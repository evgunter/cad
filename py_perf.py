"""PERF LANE (perf/explore-kernel): where a Python caller waits.

Run: PYTHONPATH=<stage> python py_perf.py
"""

import statistics
import sys
import time

from pncad import BooleanOp, Doc, Expr, Node, Open, Start, evaluate, mm


def med(fn, reps=5):
    ts = []
    out = None
    for _ in range(reps):
        t0 = time.perf_counter()
        out = fn()
        ts.append((time.perf_counter() - t0) * 1e3)
    ts.sort()
    return ts[len(ts) // 2], ts[0], ts[-1], out


def row(name, r):
    print(f"{name}\t{r[0]:.2f}\t{r[1]:.2f}\t{r[2]:.2f}")


def slab(doc, x, y, z):
    x0, x1 = x
    y0, y1 = y
    z0, z1 = z
    profile = doc.insert(
        Node.polygon(
            [
                (Expr.literal(x0), Expr.literal(y0)),
                (Expr.literal(x1), Expr.literal(y0)),
                (Expr.literal(x1), Expr.literal(y1)),
                (Expr.literal(x0), Expr.literal(y1)),
            ],
            plane=doc.sketch_frame(elevation=Expr.literal(z0)),
        )
    )
    return doc.insert(Node.extrude(profile, Expr.literal(z1 - z0)))


def chain_doc(n_cuts, depth=8.0):
    """A plate with `n_cuts` pockets subtracted in a chain — the die's
    shape (a long subtract chain), authored the natural Python way."""
    doc = Doc()
    base = slab(doc, (0 * mm, 100 * mm), (0 * mm, 100 * mm), (0 * mm, depth * mm))
    cur = base
    cuts = []
    for i in range(n_cuts):
        cx = 4.0 + (i % 8) * 12.0
        cy = 4.0 + (i // 8) * 12.0
        tool = slab(
            doc,
            (cx * mm, (cx + 6) * mm),
            (cy * mm, (cy + 6) * mm),
            (-2 * mm, (depth / 2) * mm),
        )
        cuts.append(tool)
        cur = doc.insert(Node.boolean(BooleanOp.Subtract, cur, tool))
    return doc, base, cur, cuts


def main():
    print("# python perf — release wheel; times in ms (median, min, max of 5)")
    print("op\tmed\tmin\tmax")

    for n in (4, 12, 21):
        doc, base, result, cuts = chain_doc(n)
        r = med(lambda: evaluate(doc))
        row(f"evaluate/no-prior/chain{n}", r)
        ev = r[3]
        assert ev.succeeded(result), "chain evaluated"

        # An edit: move the LAST tool (a shallow cone) and re-evaluate.
        doc2, _, result2, _ = chain_doc(n, depth=8.5)
        r2 = med(lambda: evaluate(doc2))
        row(f"evaluate/edit-no-prior/chain{n}", r2)
        r3 = med(lambda: evaluate(doc2, prior=ev))
        row(f"evaluate/edit-with-prior/chain{n}", r3)
        print(f"#   reused={r3[3].reused} recomputed={r3[3].recomputed}")

        # A prior of the SAME document (nothing changed): the memo's
        # best case, which is what a GUI re-ask costs.
        r4 = med(lambda: evaluate(doc, prior=ev))
        row(f"evaluate/same-doc-with-prior/chain{n}", r4)
        print(f"#   reused={r4[3].reused} recomputed={r4[3].recomputed}")

    doc, base, result, cuts = chain_doc(12)
    ev = evaluate(doc)
    body = ev.value(result).body()

    row("body.validate", med(body.validate))
    row("body.mass_properties", med(body.mass_properties))
    r = med(lambda: body.tessellate(0.001 * mm))
    row("body.tessellate(1e-3 mm)", r)
    mesh = r[3]
    print(f"# mesh: {mesh.triangle_count} triangles, {mesh.patch_count} patches")
    row("mesh.triangles (FFI list)", med(lambda: mesh.triangles, reps=3))
    row("mesh.positions (FFI list)", med(lambda: mesh.positions, reps=3))
    row("mesh.triangle_count", med(lambda: mesh.triangle_count))
    row("ev.step_string", med(lambda: ev.step_string(result, product_name="p"), reps=3))
    row("mesh.to_stl_binary", med(lambda: mesh.to_stl_binary(), reps=3))

    # The doors that gather the product.
    import pncad

    if hasattr(pncad, "run_checks"):
        row("run_checks (1st, gathers)", med(lambda: pncad.run_checks(doc, ev), reps=3))
        row("run_checks (memoized)", med(lambda: pncad.run_checks(doc, ev), reps=3))
    if hasattr(pncad, "assemble"):
        try:
            row("assemble (after memo)", med(lambda: pncad.assemble(doc, ev), reps=3))
        except Exception as exc:  # noqa: BLE001
            print(f"# assemble refused: {exc}")
    # A FRESH evaluation each time: what a caller who re-evaluates then
    # asks a product question pays (no memo carry-over).
    if hasattr(pncad, "run_checks"):
        def fresh_checks():
            e = evaluate(doc)
            return pncad.run_checks(doc, e)

        row("evaluate+run_checks (fresh)", med(fresh_checks, reps=3))


if __name__ == "__main__":
    sys.exit(main())
