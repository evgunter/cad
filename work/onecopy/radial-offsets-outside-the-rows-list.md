---
id: radial-offsets-outside-the-rows-list
kind: issue
title: About fifty more spellings of a point's offset from an axis, (p − o) − a((p − o)·a), outside the radial-offset row's list
status: open
opened: 2026-11-03
priority: P2
cost: M
---


Found by the sweep of the frame-readings unit (`the-frame-carries-every-placement-reading`), which repointed the sites `a-points-radial-offset-from-an-axis-is-spelled-inline` listed at `geom::Line::offset` (the offset reading, beside `Line::foot` and `Line::param`). The same shape occurs outside that list.

Sweep: an inline `(p − o) − a·((p − o)·a)`, and `let w = p − o;` followed within four lines by `w − a·(w·a)` (or `w − a·(w·a / a·a)`), over `crates/*/src` and `demos`, test files and row modules excluded, as of the unit's merge with main. Each hit is a point's offset from an axis line, the same fact as `Line::offset`:

- `crates/editor-core/src/eval/measure.rs` (line 916): `let rel = point - origin; then (rel - axis * rel.dot(axis)).norm()`
- `crates/editor-core/src/mate/coset.rs` (line 637): `let d = p - origin; then let off = d - u * d.dot(u);`
- `crates/editor-core/src/pose.rs` (line 208): `let v = p - line.foot; then (v - d * v.dot(d)).norm()`
- `crates/geom-brep/src/certify.rs` (line 5726): `let d = p - origin; then let w = d - axis * d.dot(axis);`
- `crates/geom-brep/src/chart_singularity.rs` (line 1039): `let w = p - self.anchor; then (w - self.axis * w.dot(self.axis)).norm()`
- `crates/geom-brep/src/curve_pair/pose.rs` (line 818): `let w = q - o; then (w - u * w.dot(u)).norm()`
- `crates/geom-brep/src/extent.rs` (line 1028): `let off = *at - hinge; then let off_perp = off - a * off.dot(a);`
- `crates/geom-brep/src/intersect/section_bounds.rs` (line 368): `let v = y - origin; then (v - axis * v.dot(axis)) * two`
- `crates/geom-brep/src/pcurve_cache.rs` (line 10649): `let w = c - center; then let w_r = w - axis * w.dot(axis);`
- `crates/geom-brep/src/props/curved.rs` (line 1821): `let w = p - origin; then w - axis * w.dot(axis)`
- `crates/geom-brep/src/props/curved.rs` (line 1888): `let w = center - origin; then Margin::norm3(w - axis * w.dot(axis)),`
- `crates/geom-brep/src/props/curved.rs` (line 2491): `let w = p - apex; then w - axis * w.dot(axis)`
- `crates/geom-brep/src/props/curved.rs` (line 2747): `let r = p - apex; then let rho = (r - axis * r.dot(axis)).norm();`
- `crates/geom-brep/src/props/curved.rs` (line 2774): `let w = p - apex; then w - axis * w.dot(axis)`
- `crates/geom-brep/src/props/curved.rs` (line 2891): `let w = p - apex; then w - axis * w.dot(axis)`
- `crates/geom-brep/src/ssi/enclose.rs` (line 3247): `let q = p - center; then let rho2 = (q - axis * q.dot(axis)).norm_squared();`
- `crates/geom/src/curves/on_carrier.rs` (line 394): `let w = q - origin; then (w - u * w.dot(u)).norm()`
- `crates/mesh/src/cert.rs` (line 115): `let w = p - o; then Point3::origin() + (w - d * w.dot(d))`
- `crates/mesh/src/trimmed.rs` (line 871): `let d = p - origin; then d - axis * d.dot(axis)`
- `crates/step-import/src/normalize.rs` (line 431): `let d = center - axis_point; then let off_axis = (d - s_axis * d.dot(s_axis)).norm();`
- `crates/step-import/src/normalize/singularity.rs` (line 1043): `let w = p - origin; then let r = w - axis * w.dot(axis);`
- `crates/sweep/src/blend/reach.rs` (line 364): `let v = x - cell.c; then (v - a * v.dot(a)).norm()`
- `crates/sweep/src/blend/reach.rs` (line 849): `let v = p - center; then Some(center + (v - a * v.dot(a)).normalize() * radius)`
- `crates/sweep/src/blend/reach.rs` (line 1222): `let off = foot - st.p; then let n = (off - tau * off.dot(tau)).normalize();`
- `crates/sweep/src/blend/surgery.rs` (line 3147): `let off = x - o; then let m = (off - d * off.dot(d)).normalize();`
- `crates/topo/src/boolean/contain.rs` (line 913): `let w = q - origin; then let radial = w - axis * w.dot(axis);`
- `crates/topo/src/boolean/ruled_region.rs` (line 778): `let w = y - line.start; then (w - line.dir * w.dot(line.dir)).norm()`
- `crates/topo/src/boolean/section_cert/cone_pair_search.rs` (line 835): `let w = k.apex - origin; then let apex = ((w - d * w.dot(d)).norm() - radius).abs();`
- `crates/topo/src/boolean/surface_group.rs` (line 56): `let e = c - origin; then (e - axis * e.dot(axis)).norm()`
- `crates/topo/src/bound_side.rs` (line 2176): `let v = q - origin; then (v - axis * axis.dot(v)).norm() - radius`
- `crates/topo/src/cell_region.rs` (line 266): `let w = station - center; then clear(SINGULAR, (w - n * w.dot(n)).norm())`
- `crates/topo/src/injective.rs` (line 4201): `let w = p - o; then (w - axis * w.dot(axis)).norm() - r`
- `crates/topo/src/injective.rs` (line 4225): `let w = p - o; then let radial = w - axis * w.dot(axis);`
- `crates/topo/src/injective.rs` (line 4240): `let w = p - apex; then let radial = w - axis * w.dot(axis);`
- `crates/topo/src/offset_axial.rs` (line 333): `let v = p - self.origin; then v - self.dir * v.dot(self.dir)`
- `crates/topo/src/offset_axial.rs` (line 2464): `let shift = p_start - old.eval(t0_old); then let delta = shift - *dir * shift.dot(*dir);`
- `crates/topo/src/offset_axial.rs` (line 2777): `let v = x - *center; then v - n * v.dot(n)`
- `crates/topo/src/offset_axial.rs` (line 3194): `let v = p - origin; then v - a * v.dot(a)`
- `crates/topo/src/offset_axial.rs` (line 3482): `let v = p - rim.axis_origin; then v - a * v.dot(a)`
- `crates/topo/src/offset_axial.rs` (line 3632): `let v = p - q; then v - a * v.dot(a)`
- `crates/topo/src/pcurves.rs` (line 4215): `let w = vertex - origin; then (w - axis * w.dot(axis)).norm()`
- `crates/topo/src/sector_face.rs` (line 394): `let w = point() - *origin; then (w - *axis * w.dot(*axis)).normalize(),`
- `crates/topo/src/validate.rs` (line 10430): `let d = p - *origin; then Some((d - *dir * d.dot(*dir)).norm())`

Blind spot: an offset written through a local helper (`depth_gap::split`, `reach`'s `off_axis`), a rejection whose subtraction spans more than four lines, and the cross-product form `|(p − o) × a|`. A second pass on `split(` and `off_axis(` finds `census/depth_gap.rs` (`split` serves a free vector as well as point offsets) and `sweep/src/blend/reach.rs`.

Not included: a free vector's rejection (`v − a·(v·a)` where `v` is no point's offset), which is a vector's component square to `a`.

Fix: each repointed at `geom::Line::offset` on the axis it reads (the surface's `Frame::axis_line` once the kind structs land), re-baselining what moves: `Line::offset` divides by `dir·dir`, so a unit-axis site's bits can move in the last place.
