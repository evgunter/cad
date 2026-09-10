"""PERF LANE (perf/explore-kernel): the Python edit-and-re-evaluate loop
and the FFI mesh surface.
"""

import sys
import time

import pncad
from pncad import (
    BooleanOp,
    Doc,
    DocEdit,
    Expr,
    Node,
    TubeWindow,
    evaluate,
    m,
    mm,
)


def med(fn, reps=5):
    ts = []
    out = None
    for _ in range(reps):
        t0 = time.perf_counter()
        out = fn()
        ts.append((time.perf_counter() - t0) * 1e3)
    ts.sort()
    return ts[len(ts) // 2], ts[0], ts[-1], out


def row(name, r, note=""):
    print(f"{name}\t{r[0]:.2f}\t{r[1]:.2f}\t{r[2]:.2f}\t{note}")


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
    doc = Doc()
    base = slab(doc, (0 * mm, 100 * mm), (0 * mm, 100 * mm), (0 * mm, depth * mm))
    cur = base
    tools = []
    for i in range(n_cuts):
        cx = 4.0 + (i % 8) * 12.0
        cy = 4.0 + (i // 8) * 12.0
        tool = slab(
            doc,
            (cx * mm, (cx + 6) * mm),
            (cy * mm, (cy + 6) * mm),
            (-2 * mm, (depth / 2) * mm),
        )
        tools.append(tool)
        cur = doc.insert(Node.boolean(BooleanOp.Subtract, cur, tool))
    return doc, base, cur, tools


def main():
    print("# python perf 2 — release wheel; ms (median, min, max of 5)")
    print("op\tmed\tmin\tmax\tnote")

    n = 21
    doc, base, result, tools = chain_doc(n)
    ev0 = evaluate(doc)
    assert ev0.succeeded(result)

    # An edit at the TAIL of the chain: the last tool's depth. Only the
    # last boolean's cone should recompute.
    doc.apply(DocEdit.set_param(tools[-1], "distance", Expr.literal(6.5 * mm)))
    r = med(lambda: evaluate(doc))
    row(f"evaluate/tail-edit/no-prior/chain{n}", r)
    r2 = med(lambda: evaluate(doc, prior=ev0))
    row(
        f"evaluate/tail-edit/with-prior/chain{n}",
        r2,
        f"reused={r2[3].reused} recomputed={r2[3].recomputed}",
    )

    # An edit at the HEAD: the first tool. Everything below it re-runs.
    ev1 = evaluate(doc)
    doc.apply(DocEdit.set_param(tools[0], "distance", Expr.literal(6.25 * mm)))
    r3 = med(lambda: evaluate(doc, prior=ev1))
    row(
        f"evaluate/head-edit/with-prior/chain{n}",
        r3,
        f"reused={r3[3].reused} recomputed={r3[3].recomputed}",
    )

    # A CURVED body: the FFI mesh surface under a real triangle count.
    doc2 = Doc()
    spine = doc2.insert(
        Node.datum_axis(
            (Expr.length_in(0, m), Expr.length_in(0, m), Expr.length_in(0, m)),
            (Expr.literal(0.0), Expr.literal(0.0), Expr.literal(1.0)),
        )
    )
    ring = doc2.insert(
        Node.tube(
            spine,
            (Expr.literal(1.0), Expr.literal(0.0), Expr.literal(0.0)),
            Expr.length_in(0.5, m),
            TubeWindow.full(),
            Expr.length_in(0.1, m),
        )
    )
    evr = evaluate(doc2)
    body = evr.value(ring).body()
    for delta_mm in (5.0, 1.0):
        r = med(lambda d=delta_mm: body.tessellate(d * mm), reps=3)
        mesh = r[3]
        row(
            f"tube.tessellate({delta_mm} mm)",
            r,
            f"{mesh.triangle_count} triangles",
        )
        rt = med(lambda mh=mesh: mh.triangles, reps=3)
        row("  mesh.triangles -> python list", rt)
        rp = med(lambda mh=mesh: mh.positions, reps=3)
        row("  mesh.positions -> python list (Length objects)", rp)
        rs = med(lambda mh=mesh: mh.to_stl_binary(), reps=3)
        row("  mesh.to_stl_binary", rs)
    rmp = med(body.mass_properties, reps=3)
    row("tube.mass_properties", rmp)
    rv = med(body.validate_geometric, reps=3)
    row("tube.validate_geometric (tier 3)", rv)

    # The product doors on a multi-solid document: the Python seam's
    # gather + at-rest census.
    doc3 = Doc()
    roots = []
    for i in range(12):
        roots.append(slab(doc3, (i * 20 * mm, (i * 20 + 10) * mm), (0 * mm, 10 * mm), (0 * mm, 10 * mm)))
    for node in roots:
        doc3.apply(DocEdit.set_root(node)) if hasattr(DocEdit, "set_root") else None
    ev3 = evaluate(doc3)
    try:
        r = med(lambda: pncad.run_checks(doc3, ev3), reps=3)
        row("run_checks/12 solids (memoized after 1st)", r)
    except Exception as exc:  # noqa: BLE001
        print(f"# run_checks refused: {exc}")
    try:
        r = med(lambda: pncad.assemble(doc3, ev3), reps=3)
        row("assemble/12 solids", r)
    except Exception as exc:  # noqa: BLE001
        print(f"# assemble refused: {exc}")


if __name__ == "__main__":
    sys.exit(main())
