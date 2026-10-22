"""Shared helpers for the suite: frames as values, and poses read off a
body's own geometry.

A body has no location: a construction lies in its own coordinates,
and where it stands is its placement's. These helpers spell the two
things a test most often needs on that footing — a rigid frame from an
origin and two axes (the pose a placement takes), and a plane at an
absolute place in a body's own coordinates, read off one of its faces
(a split's tool).
"""

import math

from pncad import Frame, PoseDef, RoleRead, SurfaceKind, evaluate, m


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


def frame_at(origin, u, v):
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
