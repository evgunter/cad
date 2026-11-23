---
id: geom-brep-recourse-roster-red-on-main-fifteen-sites
kind: issue
title: "geom-brep's recourse roster is red on main: four undeclared const carriers, ten unlisted indirect sites and one stale listing in intersect, pair_section, ssi, pcurve_cache and carrier_resolution"
status: open
opened: 2026-11-23
---


Found by HELDRECORDS' elbow-seams fix pass (2026-11-23), on
`heldrecords/elbow-seams` with origin/main `dc263f8553` merged in. Filed
on CIW, the owner of `crates/geom-brep/tests/recourse_roster.rs`
(`work.py territory`); the sites lie on GERM/SECT/TANG (`intersect.rs`),
SSIARITH/SSIEDGE (`ssi/`), SECTCONE (`pair_section/`), and ONEROUND
(`carrier_resolution.rs`) ground.

## Measured

`geom-brep::all recourse_roster::every_decided_name_is_read_and_every_site_is_listed`
(slow set) panics at its `roster_violations` assert. The lines below
are the ones on files the branch shares with main byte for byte, so
they are main's (the branch's own lines, `curve_face/end_root.rs`'s
`END_ROOT_*`, are HELDRECORDS' and owed on PR 7463):

```
a site names the const `EXTERNAL`, not declared a carrier: intersect.rs: EXTERNAL
a site names the const `INTERNAL`, not declared a carrier: intersect.rs: INTERNAL
a site names the const `PC_PARALLEL_GAP`, not declared a carrier: intersect.rs: PC_PARALLEL_GAP
a site names the const `TANGENCY`, not declared a carrier: pair_section/ruling.rs: TANGENCY
an indirect site not listed: carrier_resolution.rs: a_far_centred_section_stands_at_the_scale_its_form_is_stored_at: &cut
an indirect site not listed: pair_section/cones.rs: against_wall: ConeDecision::Radius.name()
an indirect site not listed: pair_section/cones.rs: decide: a
an indirect site not listed: pair_section/cones.rs: read: decision.name()
an indirect site not listed: pcurve_cache/spline_incidence.rs: span_incidence_as: name
an indirect site not listed: ssi/certify.rs: analytic_limbs: arm.sup
an indirect site not listed: ssi/certify.rs: analytic_limbs: arm.value
an indirect site not listed: ssi/enclose.rs: implicit_gradient_enclosure: parts
an indirect site not listed: ssi/enclose.rs: over: ConeParts::over(mid, apex, axis, angle)
an indirect site not listed: ssi/enclose.rs: over: parts
a listed indirect site the census no longer finds: pcurve_cache/spline_incidence.rs: spline_incidence_as: name
```

The nightly has not shown it: main's last scheduled nightly (run
44989179403, 2026-11-22) failed its "full suite, build once" job, so
the eps rows that carry the slow set were skipped.

## Owed

Each const declared a carrier (`CARRIERS`) or routed through one, and
each indirect site listed in `INDIRECT` with its disposition (the
`pcurve_cache` site renamed `spline_incidence_as` → `span_incidence_as`).
Bisect which merges brought each, so the lanes that caused them know.

## Reproduce

    cargo nextest run -p geom-brep --test all --run-ignored all -E 'test(/recourse_roster::every_decided_name/)'
