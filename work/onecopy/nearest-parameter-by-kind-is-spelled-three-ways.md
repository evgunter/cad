---
id: nearest-parameter-by-kind-is-spelled-three-ways
kind: issue
title: A point's nearest parameter by kind is spelled three ways: Curve3::nearest, Curve3::param_near's line and circle arms, and geom::Line::param
status: closed
opened: 2026-11-04
priority: P2
cost: E
refs: [6483, 6482]
branch: onecopy/feet-and-params
closed: 2026-11-06
pr: 6540
---



Found by the FULL review of PR 6483 (`onecopy/foot-certificate`), which added `Curve3::nearest(q, near)`.

"The parameter of `q`'s nearest point, where the kind makes it closed form" now has three spellings in `geom`:

- `Curve3::nearest` (`curves/on_carrier.rs`): `Some` on a line and a circle, through the foot (`Foot::nearest`), and `None` elsewhere.
- `Curve3::param_near` (`curves.rs`): the line's `Line::param`, the circle's and ellipse's angle read near `near`, the spiric's minor angle, and `None` on a spline. Its line and circle arms are the same reading as `Curve3::nearest`'s. Its ellipse and spiric arms are angle reads, which are not nearest (an ellipse's unit-coordinate angle is no foot; `on_carrier` module docs).
- `geom::Line::param`, which both read on a line.

PR 6482 (`onecopy/angle-reads`) rewrites `param_near`'s conic arms over one anchored angle read (`Azimuth::near`) and retires `on_carrier`'s `angle_near`. It does not touch `Curve3::nearest`. After it lands, the circle's foot angle and `param_near`'s circle arm are one read (`Azimuth::near` about the circle's frame), but they remain two doors.

Fix, once PR 6482 is in: give the kind-nearest reading one door. Either `Curve3::nearest` reads `param_near` on the kinds where it is nearest (line, circle), or `param_near` is split into the nearest read and the angle read its ellipse and spiric arms are. Then repoint `Curve3::foot`'s line and circle seeds onto the same door. The callers of `param_near` (33 outside tests, counted by PR 6483's sweep) say which of the two they mean.
