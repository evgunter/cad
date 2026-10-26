"""Shared helpers for the suite: frames as values, the bundle that
places a copy at one, and poses read off a body's own geometry.

A body has no location: a construction lies in its own coordinates,
and where it stands is its placement's. These helpers spell the things
a test most often needs on that footing — a rigid frame from an origin
and two axes (the pose a placement takes), the values that put a copy
there (`moves`, `placed`), and a plane at an absolute place in a body's
own coordinates, read off one of its faces (a split's tool).
"""

import math

from pncad import (
    Coord,
    Formula,
    Frame,
    MateValue,
    NodeId,
    Placed,
    PoseDef,
    RoleRead,
    SurfaceKind,
    evaluate,
    m,
    rad,
)


def _unit(v):
    n = math.sqrt(sum(c * c for c in v))
    return tuple(c / n for c in v)


def _cross(a, b):
    return (
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    )


def _dot(a, b):
    return sum(x * y for x, y in zip(a, b, strict=True))


def axes_at(origin, u, v):
    """The rigid frame at `origin` (metres) whose x is `u` and whose y
    is `v` made orthogonal to it, its z `u x v`: the frame a
    construction drawn on its own xy stands at. Built by
    `Frame.point_at`, aiming its z and rolling its y onto `v`, so an
    axis-aligned frame's columns are exact."""
    x = _unit(u)
    y = _unit(tuple(c - x_c * _dot(v, x) for c, x_c in zip(v, x, strict=True)))
    z = _cross(x, y)
    eye = tuple(c * m for c in origin)
    target = tuple((o + d) * m for o, d in zip(origin, z, strict=True))
    return Frame.point_at(eye, target, y)


def lifted(z):
    """The translation `z` metres up."""
    return Frame.translation((0 * m, 0 * m, z * m))


def slid(x, y, z):
    """The translation by `(x, y, z)` metres."""
    return Frame.translation((x * m, y * m, z * m))


def _quarter(axis, k):
    """The turn by `k` quarters about `axis` (0, 1, 2), as columns."""
    c, s = ((1, 0), (0, 1), (-1, 0), (0, -1))[k % 4]
    q = ((c, s, 0), (-s, c, 0), (0, 0, 1))
    cyc = [(i + 1 + axis) % 3 for i in range(3)]
    out = [[0] * 3 for _ in range(3)]
    for col in range(3):
        for row in range(3):
            out[cyc[col]][cyc[row]] = q[col][row]
    return out


def _mul(a, b):
    """The product `a b` of two matrices held as columns."""
    return [[sum(a[k][row] * b[col][k] for k in range(3)) for row in range(3)] for col in range(3)]


def moves(doc, frame):
    """**The values putting a copy at `frame`** on its target space's
    axes, as `Placed.unmated` takes them: a chain of spins about `x`,
    `y` and `z` in that order, then the three slides. A frame whose
    axes are the space's up to quarter turns is spelled by exact turns
    (`Formula.turn_over(4)`, `-turn / 4` read off `doc`), any other by
    its `x`-`y`-`z` angles."""
    cols = [list(c) for c in frame.columns]
    chain = None
    for n in range(64):
        x, y, z = n % 4, (n // 4) % 4, n // 16
        r = _mul(_quarter(2, z), _mul(_quarter(1, y), _quarter(0, x)))
        if all(float(r[j][i]) == cols[j][i] for j in range(3) for i in range(3)):
            chain = [("x", x), ("y", y), ("z", z)]
            break
    out = []
    if chain is not None:
        turn = {1: Formula.turn_over(4), 2: Formula.turn_over(2), 3: doc.parse_formula("-turn/4")}
        out += [MateValue(Coord.spin(a), turn[k]) for a, k in chain if k != 0]
    else:

        def r(i, j):
            return cols[j][i]

        cos_b = math.hypot(r(0, 0), r(1, 0))
        beta = math.atan2(-r(2, 0), cos_b)
        if cos_b > math.sqrt(2.0**-52):
            alpha, gamma = math.atan2(r(2, 1), r(2, 2)), math.atan2(r(1, 0), r(0, 0))
        else:
            alpha, gamma = math.atan2(-r(1, 2), r(1, 1)), 0.0
        out += [
            MateValue(Coord.spin(a), Formula.angle_in(t, rad))
            for a, t in (("x", alpha), ("y", beta), ("z", gamma))
            if t != 0.0
        ]
    out += [
        MateValue(Coord.slide(a), Formula.literal(t))
        for a, t in zip("xyz", frame.origin, strict=True)
        if t.meters != 0.0
    ]
    return out


def placed(doc, body, frame, target=None, *, label=None):
    """One copy of `body` placed into `target` (the world when `None`)
    at `frame` on its axes, by the values `moves` spells."""
    space = doc.world if target is None else target
    return doc.place(body, space, Placed.unmated(moves(doc, frame)), label=label)


def against(doc, on, body, frame):
    """One copy of `body` placed into the space the node `on`'s body is
    in, at `frame` on that space's axes: how a construction combined
    with another joins its space."""
    assert isinstance(on, NodeId), f"name the node whose space {on} is in"
    space = doc.space(on)
    assert space is not None, f"{on} builds no body"
    return placed(doc, body, frame, space)


def plane_on(doc, body, origin, normal):
    """The plane through `origin` (metres) with normal `normal`, in
    `body`'s own coordinates, read off `body`'s geometry: one of its
    planar faces parallel to it, flipped to face `normal` and slid along
    it to `origin`. A split's tool at an absolute place."""
    ev = evaluate(doc)
    n = _unit(normal)
    for face in sorted(ev.all_faces(body)):
        if ev.face_carrier_kind(body, face) != SurfaceKind.Plane:
            continue
        pose = ev.face_frame(body, face)
        out = tuple(c if pose.sense else -c for c in pose.axis)
        if abs(abs(_dot(out, n)) - 1.0) > 1e-12:
            continue
        plane = PoseDef.plane(body, face)
        if _dot(out, n) < 0.0:
            plane = PoseDef.flip(plane)
        at = tuple(c.meters for c in pose.origin)
        by = _dot(tuple(o - a for o, a in zip(origin, at, strict=True)), n)
        if by == 0.0:
            return plane
        return PoseDef.slide(plane, RoleRead(plane, "normal"), by * m)
    raise AssertionError(f"no planar face of {body} is parallel to {normal}")


def own_frame(doc, body, origin=(0.0, 0.0, 0.0), x=(1.0, 0.0, 0.0)):
    """`body`'s own frame at `origin`, read off its geometry: the frame
    through its straight edge along its own z that passes through
    `origin`, and its vertex `x` (metres) on its own +x — a frame read
    where a test needs the body's own axes and planes."""
    ev = evaluate(doc)
    edge = None
    for name in sorted(ev.all_edges(body)):
        pose = ev.edge_frame(body, name)
        if pose.u_ref is not None or abs(abs(pose.axis[2]) - 1.0) > 1e-12:
            continue
        at = tuple(c.meters for c in pose.origin)
        if abs(at[0] - origin[0]) < 1e-12 and abs(at[1] - origin[1]) < 1e-12:
            edge = (name, pose.axis[2] > 0.0)
            break
    assert edge is not None, f"{body} has no straight edge along its z through {origin}"
    vertex = [
        v
        for v in ev.all_vertices(body)
        if max(abs(c.meters - w) for c, w in zip(ev.vertex_position(body, v), x, strict=True)) < 1e-12
    ]
    assert len(vertex) == 1, f"{body} has no vertex at {x}"
    axis = PoseDef.axis(body, edge[0])
    if not edge[1]:
        axis = PoseDef.flip(axis)
    return PoseDef.through(axis, PoseDef.point(body, vertex[0]))


def assemble_world_copies(doc):
    """The document's product made the world's copies, as a test that
    places bodies into the world means it: the one copy, or the `⊔` of
    them in document order. Nothing joins a product by being placed
    (`Doc.set_product`), so a document says what it delivers; this is
    the saying, for a scene whose product is every copy it stands in the
    world. Call it once the scene is placed; a document that already
    has a product keeps it. Answers whether it set one."""
    if doc.product is not None:
        return False
    world = doc.output(doc.world)
    copies = [p for p in doc.placements() if doc.space(p) == world]
    if not copies:
        return False
    doc.set_product(copies[0] if len(copies) == 1 else doc.assemble(copies))
    return True
