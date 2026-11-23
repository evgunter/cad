---
id: geom-brep-recourse-roster-red-on-main-cone-torus-apex-names
kind: issue
title: geom-brep's recourse roster is red on main: cone_torus.rs's APEX_FORM and APEX_ON_TORUS are not declared carriers
status: open
opened: 2026-11-23
---


Found by HELDRECORDS' elbow-seams fix pass (2026-11-23), on
`heldrecords/elbow-seams` with origin/main merged in (the merge of 2026-11-23); the
two files are main's byte for byte. Filed on SECTCONE, whose
`pair_section/cone_torus.rs` names them.

## Measured

`geom-brep::all recourse_roster::every_decided_name_is_read` (slow set)
panics at its `roster_violations` assert:

```
a site names the const `APEX_FORM`, not declared a carrier: pair_section/cone_torus.rs: APEX_FORM
a site names the const `APEX_ON_TORUS`, not declared a carrier: pair_section/cone_torus.rs: APEX_ON_TORUS
```

## Owed

Both consts declared carriers in `crates/geom-brep/tests/recourse_roster.rs`
(`CARRIERS`), or routed through one.

## Reproduce

    cargo nextest run -p geom-brep --test all --run-ignored all -E 'test(/recourse_roster::every_decided_name_is_read/)'
