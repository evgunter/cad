---
id: value-discards-read-a-refusal-as-nothing-found
kind: issue
title: About two dozen arms read a refusal as nothing found (Ok(Zero) | Err(_) => None / false), outside the escalation-discards gate
status: open
priority: P2
cost: M
opened: 2026-11-03
---


Found by the dual review of PR 6454 (`onecopy/ladder-climb`), at the
blind spot its gate `scripts/gates/escalation-discards.sh` names. That
gate counts the arms a search moves on by (`Err(_) => continue | break |
{} | ()`). It does not count a **value discard**: an arm that returns a
refusal as "nothing found" (`Ok(Sign::Zero) | Err(_) => None`, `=> false`).
A caller that then reads `None` as "not this candidate" refuses, where
nothing else decides, with a cause that is not the reading that stopped it.
That is the `whole_body_side` defect in another spelling.

Each arm below needs one of two dispositions. Either it is a search's kept
reading (port the search to `geom_brep::ladder::climb`, `Rung::of_reading`
being the adapter for a `Result<Option<_>, Indeterminate>` reading), or its
caller is right to read the refusal as absence (a detector's posture, a UI
probe, a predicate whose `false` is a conservative answer). Say which, in
place. Then either register the shape in the gate, or extend the gate's
register with these dispositions.

Production hits at the time of filing (the gate's code view, `#[cfg(test)]`
items excluded):

- `crates/editor-core/src/mate/unobservable.rs` `pose_move` (line 155): `Ok(None) | Err(_) => false,`
- `crates/editor-core/src/stackup.rs` `stackup` (line 1869): `Ok((_, Chamber::LocalOnly)) | Err(_) => None,`
- `crates/geom-brep/src/certify.rs` `named_by_parallelism` (line 3566): `Ok(_) | Err(_) => None,`
- `crates/geom-brep/src/locus.rs` `plane_cylinder_locus` (line 347): `Ok((Sign::Zero, _)) | Err(_) => None,`
- `crates/geom-brep/src/locus.rs` `cylinders_locus` (line 411): `Ok(CylinderGap::External(_) | CylinderGap::Internal(_)) | Err(_) => None,`
- `crates/geom-brep/src/pair_section/gap.rs` `turn` (line 433): `Ok(Sign::Zero) | Err(_) => None,`
- `crates/geom-brep/src/pair_section/spline_implicit/sweep/critical.rs` `decide` (line 321): `Ok((Magnitude::Positive, _)) | Err(_) => None,`
- `crates/geom-brep/src/pcurve_cache.rs` `derive_class` (line 10053): `Ok(Sign::Zero) | Err(_) => None,`
- `crates/geom-brep/src/props/curved.rs` `sphere_region_cover` (line 4196): `Ok(Sign::Zero) | Err(_) => None,`
- `crates/profile/src/seg.rs` `arc_clear_of_carrier` (line 1769): `Ok(Sign::Zero) | Err(_) => false,`
- `crates/test-utils/src/fuzz.rs` `parse_env` (line 165): `Err(_) => None,`
- `crates/topo/src/boolean/carrier_touch.rs` `clusters` (line 280): `Ok(Some(Unlocalized)) | Err(_) => None,`
- `crates/topo/src/boolean/join.rs` `germ_side` (line 5775): `Ok(Sign::Zero) | Err(_) => None,`
- `crates/topo/src/boolean/mod.rs` `boundary_cuts` (line 8176): `Err(_) => None,`
- `crates/topo/src/boolean/plane_eq.rs` `senses` (line 214): `Ok(Sign::Zero) | Err(_) => None,`
- `crates/topo/src/boolean/solid_contain.rs` `ray_crossings` (line 4488): `Err(_) => false,`
- `crates/topo/src/chart_region.rs` `interior_witness` (line 2057): `Err(_) => false,`
- `crates/topo/src/link.rs` `off` (line 221): `Err(_) => None,`
- `crates/topo/src/link.rs` `at` (line 232): `Err(_) => None,`
- `crates/topo/src/splitting/rules.rs` `cone_axial_window` (line 846): `Err(_) => None,`
- `crates/topo/src/validate.rs` `census_finding` (line 6448): `Ok(SeamCover::Once | SeamCover::MixedSense) | Err(_) => None,`
- `crates/viewer/src/pane/properties.rs` `reading_notes` (line 1638): `Ok(props::Reading::One(_)) | Err(_) => None,`
- `crates/viewer/src/session/probe.rs` `probe_bounds` (line 129): `Err(_) => false,`
- `crates/viewer/src/sketch.rs` `tip_state_at` (line 2167): `Err(_) => None,`
