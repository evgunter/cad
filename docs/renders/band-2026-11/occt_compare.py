"""Chamfer and fillet the mixed-convexity corner fixtures of PR 7038 in Open CASCADE.

The fixtures are `band_mixed_corner.rs`'s flush L, inset boss, 120° flush corner and
bore-less bracket, built here as box booleans (the 120° corner as a polygon prism) and
merged with `ShapeUpgrade_UnifySameDomain`. Each is chamfered with
`BRepFilletAPI_MakeChamfer` (equal distance s) and filleted with
`BRepFilletAPI_MakeFillet` (r = s) under the requests (a) odd + one pair edge,
(b) all three, (c) odd alone, (d) one pair edge, (e) both pair edges. Writes
`occt.tsv`, a soup per build (the `draw_soup.py` format) and, where our kernel
dumped membership samples (`ours-*.pts`), the OCCT classification of the same points.

    python -I occt_compare.py <ours-dir> <out-dir>
"""

import math
import sys
from pathlib import Path

from OCP.BRep import BRep_Tool
from OCP.BRepAdaptor import BRepAdaptor_Surface
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepCheck import BRepCheck_Analyzer
from OCP.BRepClass3d import BRepClass3d_SolidClassifier
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer, BRepFilletAPI_MakeFillet
from OCP.BRepGProp import BRepGProp
from OCP.BRepMesh import BRepMesh_IncrementalMesh
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.GeomAbs import GeomAbs_Cylinder, GeomAbs_Plane, GeomAbs_Sphere, GeomAbs_Torus
from OCP.GProp import GProp_GProps
from OCP.gp import gp_Pnt, gp_Vec
from OCP.ShapeUpgrade import ShapeUpgrade_UnifySameDomain
from OCP.TopAbs import TopAbs_EDGE, TopAbs_FACE, TopAbs_IN, TopAbs_ON, TopAbs_REVERSED, TopAbs_SHELL, TopAbs_SOLID
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopLoc import TopLoc_Location
from OCP.TopoDS import TopoDS
from OCP.OCP.collections import IndexedMap_TopoDS_Shape_TopTools_ShapeMapHasher as TopTools_IndexedMapOfShape

S = 0.1


def box(lo, hi):
    return BRepPrimAPI_MakeBox(gp_Pnt(*lo), gp_Pnt(*hi)).Shape()


def prism(pts, z0, z1):
    poly = BRepBuilderAPI_MakePolygon()
    for x, y in pts:
        poly.Add(gp_Pnt(x, y, z0))
    poly.Close()
    face = BRepBuilderAPI_MakeFace(poly.Wire()).Face()
    return BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, z1 - z0)).Shape()


def unify(shape):
    u = ShapeUpgrade_UnifySameDomain(shape, True, True, True)
    u.Build()
    return u.Shape()


def fuse(a, b):
    return unify(BRepAlgoAPI_Fuse(a, b).Shape())


def count(shape, kind):
    m = TopTools_IndexedMapOfShape()
    TopExp.MapShapes_s(shape, kind, m)
    return m.Extent()


def volume(shape):
    p = GProp_GProps()
    BRepGProp.VolumeProperties_s(shape, p)
    return p.Mass()


def pnt(v):
    p = BRep_Tool.Pnt_s(v)
    return (p.X(), p.Y(), p.Z())


def edge(shape, a, b):
    near = lambda p, q: math.dist(p, q) < 1e-9  # noqa: E731
    found = []
    ex = TopExp_Explorer(shape, TopAbs_EDGE)
    seen = TopTools_IndexedMapOfShape()
    while ex.More():
        e = TopoDS.Edge(ex.Current())
        ex.Next()
        if seen.Contains(e):
            continue
        seen.Add(e)
        s, t = pnt(TopExp.FirstVertex_s(e)), pnt(TopExp.LastVertex_s(e))
        if (near(s, a) and near(t, b)) or (near(s, b) and near(t, a)):
            found.append(e)
    assert len(found) == 1, (a, b, len(found))
    return found[0]


def flush():
    body = fuse(box((-1, -1, 0), (1, 0, 2)), box((-1, 0, 0), (0, 1, 2)))
    o = (0, 0, 0)
    return body, [edge(body, o, (0, 0, 2)), edge(body, o, (1, 0, 0)), edge(body, o, (0, 1, 0))]


def inset():
    body = fuse(box((-1, -1, -1), (1, 1, 0)), box((-1, -1, -0.5), (0, 0, 1)))
    o = (0, 0, 0)
    return body, [edge(body, o, (0, 0, 1)), edge(body, o, (0, -1, 0)), edge(body, o, (-1, 0, 0))]


def flush_at(phi):
    """`band_mixed_corner::flush_at`: the material outside the wedge of angle phi."""
    a, c = (1.0, 0.0), (math.cos(phi), math.sin(phi))
    a_out, c_out = (1.0, -1.0), (c[0] - math.sin(phi), c[1] + math.cos(phi))
    back = (-1.5 * math.cos(phi / 2), -1.5 * math.sin(phi / 2))
    body = unify(prism([back, a_out, a, (0.0, 0.0), c, c_out], 0.0, 2.0))
    o = (0, 0, 0)
    return body, [edge(body, o, (0, 0, 2)), edge(body, o, (*a, 0)), edge(body, o, (*c, 0))]


def bracket():
    body = fuse(box((0, 0, 0), (6, 4, 0.5)), box((0, 0.5, 0.25), (0.5, 3.5, 2)))
    e = lambda a, b: edge(body, a, b)  # noqa: E731
    r1, r2 = e((0, 0.5, 0.5), (0.5, 0.5, 0.5)), e((0, 3.5, 0.5), (0.5, 3.5, 0.5))
    ry = e((0.5, 0.5, 0.5), (0.5, 3.5, 0.5))
    v1, v2 = e((0, 0.5, 0.5), (0, 0.5, 2)), e((0, 3.5, 0.5), (0, 3.5, 2))
    s1, s2 = e((0, 0.5, 0.5), (0, 0, 0.5)), e((0, 3.5, 0.5), (0, 4, 0.5))
    w1, w2 = e((0.5, 0.5, 0.5), (0.5, 0.5, 2)), e((0.5, 3.5, 0.5), (0.5, 3.5, 2))
    return body, {"a": [r1, r2, v1, s2, w1, w2], "b": [r1, r2, ry, v1, v2, s1, s2, w1, w2]}


def blend(verb, shape, edges, s):
    """Status, the result or None, and a note."""
    try:
        mk = BRepFilletAPI_MakeChamfer(shape) if verb == "chamfer" else BRepFilletAPI_MakeFillet(shape)
        for e in edges:
            mk.Add(s, e)
        mk.Build()
        if not mk.IsDone():
            return "fails", None, "IsDone false"
        out = mk.Shape()
    except Exception as ex:  # noqa: BLE001 - OCCT raises Standard_Failure subclasses
        return "fails", None, f"{type(ex).__name__}: {ex}".replace("\n", " ")[:120]
    if out is None or out.IsNull():
        return "fails", None, "null shape"
    valid = BRepCheck_Analyzer(out).IsValid()
    shells, solids = count(out, TopAbs_SHELL), count(out, TopAbs_SOLID)
    note = f"solids {solids} shells {shells}"
    return ("builds" if valid and solids == 1 and shells == 1 else "invalid"), out, note


def soup(shape, path):
    BRepMesh_IncrementalMesh(shape, 1e-3, False, 0.2, False)
    vs, fs = [], []
    ex = TopExp_Explorer(shape, TopAbs_FACE)
    while ex.More():
        face = TopoDS.Face(ex.Current())
        ex.Next()
        loc = TopLoc_Location()
        tri = BRep_Tool.Triangulation_s(face, loc)
        if tri is None:
            continue
        kind = {GeomAbs_Plane: 0, GeomAbs_Cylinder: 1, GeomAbs_Torus: 2, GeomAbs_Sphere: 3}.get(
            BRepAdaptor_Surface(face).GetType(), 4
        )
        trsf = loc.Transformation()
        base = len(vs)
        for i in range(1, tri.NbNodes() + 1):
            p = tri.Node(i).Transformed(trsf)
            vs.append((p.X(), p.Y(), p.Z()))
        rev = face.Orientation() == TopAbs_REVERSED
        for i in range(1, tri.NbTriangles() + 1):
            a, b, c = tri.Triangle(i).Get()
            if rev:
                b, c = c, b
            fs.append((base + a - 1, base + b - 1, base + c - 1, kind))
    with open(path, "w") as f:
        for v in vs:
            f.write(f"v {v[0]!r} {v[1]!r} {v[2]!r}\n")
        for t in fs:
            f.write(f"f {t[0]} {t[1]} {t[2]} {t[3]}\n")


def membership(shape, pts_path, out_path):
    """OCCT's classification of our sample points; returns (agree, disagree, skipped)."""
    agree = disagree = skipped = 0
    rows = []
    for line in open(pts_path):
        x, y, z, ours = line.split()
        ours = int(ours)
        cl = BRepClass3d_SolidClassifier(shape, gp_Pnt(float(x), float(y), float(z)), 1e-7)
        st = cl.State()
        occt = 1 if st == TopAbs_IN else (-1 if st == TopAbs_ON else 0)
        if ours < 0 or occt < 0:
            skipped += 1
        elif ours == occt:
            agree += 1
        else:
            disagree += 1
            rows.append(f"{x} {y} {z} ours {ours} occt {occt}")
    Path(out_path).write_text("\n".join(rows) + ("\n" if rows else ""))
    return agree, disagree, skipped


def main():
    ours_dir, out = Path(sys.argv[1]), Path(sys.argv[2])
    out.mkdir(parents=True, exist_ok=True)
    rows = ["fixture\treq\tverb\tstatus\tdV\tfaces\tnote\tmembership"]
    fixtures = []
    for name, make in [("flush", flush), ("inset", inset), ("flush120", lambda: flush_at(2 * math.pi / 3))]:
        body, (odd, p1, p2) = make()
        reqs = {"a": [odd, p1], "b": [odd, p1, p2], "c": [odd], "d": [p1], "e": [p1, p2]}
        fixtures.append((name, body, reqs, S))
    body, reqs = bracket()
    fixtures.append(("bracket", body, reqs, 0.05))
    for name, body, reqs, s in fixtures:
        v0 = volume(body)
        soup(body, out / f"occt-{name}-before.soup")
        rows.append(f"{name}\t-\tbody\t{'valid' if BRepCheck_Analyzer(body).IsValid() else 'INVALID'}\t"
                    f"{v0:.15e}\t{count(body, TopAbs_FACE)}\t\t")
        for req, edges in reqs.items():
            for verb in ("chamfer", "fillet"):
                status, shape, note = blend(verb, body, edges, s)
                dv, faces, mem = "", "", ""
                if shape is not None:
                    dv = f"{volume(shape) - v0:.15e}"
                    faces = str(count(shape, TopAbs_FACE))
                    tag = f"{name}-{req}-{verb}"
                    soup(shape, out / f"occt-{tag}.soup")
                    pts = ours_dir / f"ours-{tag}.pts"
                    if pts.exists():
                        a, d, k = membership(shape, pts, out / f"disagree-{tag}.txt")
                        mem = f"agree {a} disagree {d} skipped {k}"
                rows.append(f"{name}\t{req}\t{verb}\t{status}\t{dv}\t{faces}\t{note}\t{mem}")
                print(rows[-1], flush=True)
    (out / "occt.tsv").write_text("\n".join(rows) + "\n")


if __name__ == "__main__":
    main()
