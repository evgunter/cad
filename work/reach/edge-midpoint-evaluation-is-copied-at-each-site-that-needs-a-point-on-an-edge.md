---
id: edge-midpoint-evaluation-is-copied-at-each-site-that-needs-a-point-on-an-edge
kind: issue
title: The point halfway along an edge is re-derived at each site that needs one, and the sites disagree on which carriers count as curved
status: open
opened: 2026-09-25
priority: P1
cost: D
---


Filed by CONTACT-2's review fix pass (PR 3250). That PR gave the
evaluation one home, `geom_brep::EdgeCurve::mid_point`
(`crates/geom-brep/src/certify.rs`), and used it at its own two sites
(`boolean/join.rs` `curved_edge_midpoint`, `chord_join.rs`
`between_edge_in_plane`). The rest were left for this row so the PR
did not sweep territory it does not own.

## The copies (carrier evaluated at the parameter midpoint)

On an `EdgeCurve` — these should call `mid_point`:
- `crates/topo/src/boolean/ops.rs` — the seam-edge witness (`curved`
  = every carrier that is not a `Line`).
- `crates/topo/src/splitting/finish.rs` — the section-edge witness
  (`conic` = `Circle | Ellipse` only; `Spiric` and `Nurbs` fall to the
  endpoint `lerp`).
- `crates/topo/src/chart_iso.rs` `mid_azimuth` (`f64`, same formula).

On a bare carrier and parameter pair (no `EdgeCurve` yet — a spec
under construction), which want a `Curve3` home or the spec's own:
- `crates/topo/src/chord_join.rs` — the conic chord's `witness` in
  `chord_spec` (`s1..s2`) and in `bool_planar_chord_spec`
  (`t_start..t_end`), plus the `rim_run` test fixture.
- `crates/topo/src/replace_face.rs` (three sites),
  `crates/topo/src/offset_together.rs`, `crates/topo/src/offset_axial.rs`
  (two sites) — spelled `(t0 + t1) · ½`, which differs from
  `t0 + (t1 − t0) · ½` in the last bit.

## The disagreement

`boolean/ops.rs` and CONTACT-2's `curved_edge_midpoint` call every
non-`Line` carrier curved and evaluate it; `splitting/finish.rs` calls
only `Circle` and `Ellipse` curved and takes a `Spiric` or `Nurbs`
edge's chord midpoint — a point off the edge, fed to
`classify_dihedral` as its witness. Both gates refuse those kinds
today, so it is latent; the taker decides it once, at the home.

## Sweep that found them

`rg 'eval\((t0 \+ \(t1 - t0\)|\(t0 \+ t1\)|s1 \+ \(s2 - s1\)|t_start \+ \(t_end - t_start\))'`
over `crates/topo/src` and `crates/geom-brep/src`. It cannot match a
midpoint spelled another way (a named `mid` parameter, a `sample_param`
call); not searched further.
