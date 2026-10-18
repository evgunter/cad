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

from pncad import Frame, PoseDef, RoleRead, SurfaceKind, evaluate, m, rad


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
    return sum(x * y for x, y in zip(a, b))


def frame_at(origin, u, v):
    """The rigid frame at `origin` (metres) whose x is `u` and whose y
    is `v` made orthogonal to it, its z `u x v`: the frame a
    construction drawn on its own xy stands at."""
    x = _unit(u)
    y = _unit(tuple(c - x_c * _dot(v, x) for c, x_c in zip(v, x)))
    z = _cross(x, y)
    # The rotation's axis and angle, from its matrix [x y z] (columns).
    r = ((x[0], y[0], z[0]), (x[1], y[1], z[1]), (x[2], y[2], z[2]))
    cos = max(-1.0, min(1.0, (r[0][0] + r[1][1] + r[2][2] - 1.0) / 2.0))
    angle = math.acos(cos)
    if angle < 1e-15:
        axis = (0.0, 0.0, 1.0)
    elif math.pi - angle < 1e-9:
        # A half turn: the axis is the column of r + I with the largest
        # diagonal entry.
        i = max(range(3), key=lambda k: r[k][k])
        col = [r[k][i] + (1.0 if k == i else 0.0) for k in range(3)]
        axis = _unit(col)
    else:
        axis = _unit((r[2][1] - r[1][2], r[0][2] - r[2][0], r[1][0] - r[0][1]))
    t = tuple(c * m for c in origin)
    return Frame.rotate_then_translate(axis, angle * rad, t)


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
        try:
            pose = ev.face_frame(body, face)
        except Exception:  # noqa: BLE001 — a face with no canonical frame is not a plane
            continue
        if pose.u_ref is None:
            continue
        out = tuple(c if pose.sense else -c for c in pose.axis)
        if abs(abs(_dot(out, n)) - 1.0) > 1e-12 or ev.face_carrier_kind(body, face) != SurfaceKind.Plane:
            continue
        plane = PoseDef.plane(body, face)
        if _dot(out, n) < 0.0:
            plane = PoseDef.flip(plane)
        at = tuple(c.meters for c in pose.origin)
        by = _dot(tuple(o - a for o, a in zip(origin, at)), n)
        if by == 0.0:
            return plane
        return PoseDef.slide(plane, RoleRead(plane, "normal"), by * m)
    raise AssertionError(f"no planar face of {body} is parallel to {normal}")
