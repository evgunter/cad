---
id: the-role-name-builders-reach-only-the-outer-profile-loop
kind: issue
title: the role-name builders reach only the outer profile loop, so a hole's band is still hand-spelled
status: open
opened: 2026-09-09
refs: [no-facade-door-mints-a-revolves-role-names]
---


`band`, `band_pi`, `band_rim` and `meridian_vertex`
(`crates/editor-core/src/names/role.rs`, re-exported at
`pncad::select`) take a bare `u32` and fix `loop_index: 0` — the
signature Ev ruled, and the one every consumer that converted was
already spelling. A profile with HOLES has loops `1..n` and its bands
are named the same way, so a caller wanting one still writes the
`StableName` field by field:

- `crates/editor-core/tests/ring_r1_names_probe.rs:96` —
  `RoleSeg::Band(pe(1, s))`, an annulus revolved.
- `crates/editor-core/tests/m4_pr3_names.rs:425` — the same shape
  under a loop over `l`.

Both reach it through that file's own `name1`/`pe` helpers, so
neither is a five-field spelling today; what is missing is a door at
the vocabulary. Three shapes would close it and each costs something:
a second parameter on all four (`band(node, loop, seg)`, which reads
as two anonymous integers at every call site), a locator argument
(`band(node, ProfileEdgeRef { .. })`, which un-shortens the outer-loop
call the builders exist for), or `impl Into<ProfileEdgeRef>` with a
`From<u32>` that means "the outer loop" (one door, at the cost of an
implicit conversion that invents a field).

Not urgent: no consumer outside those two test files names a hole's
band, and the vocabulary itself stays reachable for the ones that do.
