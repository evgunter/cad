---
id: tol-and-band-travel-as-a-redundant-pair-past-the-offset-fit-doors
kind: issue
title: (tol, band) travel as a redundant pair through 65 signatures past the offset-fit doors, so a band from a different eps than the witness type-checks
status: open
opened: 2026-09-28
priority: P3
cost: H
design: true
refs: [tol-and-band-travel-as-a-redundant-pair-on-the-offset-fit-doors]
---


Filed by the ENCL lane that closed
`work/encl/tol-and-band-travel-as-a-redundant-pair-on-the-offset-fit-doors.md`
(branch `encl/seed-grid-and-tol-band`). That row made the pair a single
value on `geom_brep::offset_fit`'s five `Tol` doors, the
`geom_brep::OffsetFitLane` doors and `topo::transform`'s `map_surface` /
`map_approx`: each takes the `Tol` witness alone and the fit door derives
`Band::linear(tol)` inside (`offset_fit::run_band`). The same pair
travels much further, and this row is that wider class.

## The class

A signature takes the run's ε twice, as the `Tol` witness and as a
`Band`. Nothing ties the two together, so a band built from a different
ε (`Band::new`, `Band::linear_at`) type-checks beside the witness. The
callee then decides some predicates at one ε and reads `tol.eps()` (or
hands `tol` to a door that does) at another.

## Sites

The sweep: every `fn` whose parameter list has both a `: Tol` and a
`: Band` parameter (a Python scan over every `.rs` file in the tree,
paths qualified or bare). There are 65 hits after the offset-fit doors
left the list:

- `crates/editor-core/src/clearance.rs`: `verify_witness`
- `crates/editor-core/src/mate/solve.rs`: `mate_coset`
- `crates/sweep/src/blend/surgery.rs`: `blend_surgery`, `attach_contact`
- `crates/sweep/src/extrude.rs`: `sweep_loop`, `side_surface`, `upgrade_rim`
- `crates/sweep/src/revolve/full.rs`: `build_full`, `build_lamina`, `build_wire`
- `crates/sweep/src/revolve/partial.rs`: `build_partial`, `finish_partial`, `sweep_loop`
- `crates/sweep/src/revolve/upgrade.rs`: `upgrade_intersection`
- `crates/topo/src/boolean/finish.rs`: `classify_shell`, `select_solid`, `setopfinish`
- `crates/topo/src/boolean/join.rs`: `bool_connect`, `resolve_roles_geometric`
- `crates/topo/src/boolean/ops.rs`: `volume_backstop`, `describe_minted_edges`, `classify_shells`, `fallback`, `finish_fallback`
- `crates/topo/src/boolean/reduce.rs`: `sweep_direction`, `curved_face_arm`, `vertex_on_curved_face`, `vertex_on_face`, `split_other_at_point`
- `crates/topo/src/boolean/rest.rs`: `try_rest_union`
- `crates/topo/src/boolean/solid_contain.rs`: `point_in_solid`, `point_in_solid_of`, `point_in_solid_faces`, `point_in_faces`, `cast_ray`, `at_infinity_side`
- `crates/topo/src/boolean/vtxfac.rs`: `classify_vertex_on_face`
- `crates/topo/src/census.rs`: `census_and_certify`, `census_traces`, `census_traces_planted`, `census_with`, `sweep_cross_solid_backstop`
- `crates/topo/src/offset_axial.rs`: `offset_charts_together`
- `crates/topo/src/offset_together.rs`: `offset_planes_together`
- `crates/topo/src/props.rs`: `mass_properties_with`, `sign_walk`, `assembled`, `mass_properties_closed_form`, `mass_properties_closed_form_of`, `mass_properties_impl`, `face_flux`, `cut_face_rounds`, `nurbs_face`, `trimmed_face`
- `crates/topo/src/replace_face.rs`: `replace_face_offset`, `replace_faces_offset`, `mint_offset`, `plan_reanchors`
- `crates/topo/src/splitting/finish.rs`: `describe_section_boundary`
- `crates/topo/src/splitting/join.rs`: `split_connect`
- `crates/topo/src/validate.rs`: `plus_v_by_sign`, `tier3_local_checks`, `tier3_local_checks_marked`, `nesting_words`, `check_9_words`

Struct fields carrying both (the second pass, over every `struct` or
`enum` body): `topo::props::SignCertificate` (`crates/topo/src/props.rs`),
`editor_core::mate::solve::Solve` (`crates/editor-core/src/mate/solve.rs`),
plus one test-local struct each in `crates/pncad/tests/all.rs` and
`crates/editor-core/tests/lib_doors_node_result.rs`.

**What the scan cannot see:** a pair where one half sits inside a struct
or a closure capture and the other is a parameter; a band handed on under
another type (a `(f64, f64)` threshold pair: that is
`band-derivation-has-a-scalar-twin`'s class); and an `eps: f64` beside a
`Band`. The `_at` numeric-target instruments in `offset_fit` are that
last shape by design, since a chosen target is what they are for.

## Two things the offset-fit fix left behind

Status after the same PR's review fix pass:

- **The first is closed.** `replace_face_offset` / `replace_faces_offset`
  now take `Tol` alone and derive `Band::linear(tol)` once, first
  (mapped into a new `ReplaceFaceError::Band`), so the analytic mint,
  `plan_reanchors`, the carrier transport, the iso rows, the nappe and
  apex decisions and the fit lane all classify at one band. `mint_offset`
  and `plan_reanchors` are private and still take `(band, tol)`, but
  their only production caller is that door, so the pair there cannot
  disagree.
- **The second is stated, not closed.** Not every caller of
  `tier3_local_checks_marked` derives `band` from `tol`:
  `topo::n2r1_probes` hands it a fixed `Band::new(1e-9, 1e-8)`. The
  split is written in the function's docs and at the offset arm.

- `topo::replace_face_offset` / `replace_faces_offset` are public and take
  `(band, tol)`, and `mint_offset` passes the caller's band to the
  analytic mint (`geom_brep::offset_surface`) but only `tol` to the fit
  lane, which now meters at `Band::linear(tol)`. If a caller's band
  disagrees with its `tol`, the analytic and fitted arms of one call
  classify at different bands. Every in-tree caller builds its band as
  `Band::linear(tol)`.
- `topo::validate`'s `tier3_local_checks_marked` still receives both. Its
  offset-fit arm now reads only `tol`, while every other check reads
  `band`.

## What is open

The carrier. Most of these sites derive the band once at a door
(`Band::linear(tol).map_err(...)`) and thread it down beside `tol`,
because `tol` is still needed for pcurve mints and `.eps()` reads. So
"derive it where it is read" costs a typed `BandError` arm at every
reader. A type holding both, constructible only from one witness, keeps
the single derivation. Which one reads right differs by crate, which is
why `design: true`.

## A sibling class: "derive the run's band at the door" is spelled per crate

Every door that takes `Tol` alone and needs the band derives it the same
way, `Band::linear(tol)` mapped into the door's own error, and the
derivation is spelled again in each crate:

- `geom_brep::offset_fit::run_band` (`crates/geom-brep/src/offset_fit.rs`)
- `editor_core::names::discriminate::band`
  (`crates/editor-core/src/names/discriminate.rs`)
- `editor_core::eval::wire::band` (`crates/editor-core/src/eval/wire.rs`)
- `profile::path::linear_band` (`crates/profile/src/path.rs`)
- in `topo`, inline: 21 `Band::linear(tol).map_err(` sites across
  `boolean/combine.rs`, `euler.rs` (3), `flush.rs`, `merge_faces.rs` (3),
  `pcurves.rs` (2), `props.rs` (3), `replace_face.rs`, `separation.rs`
  (2), `shell.rs`, `split.rs`, `splitting/finish.rs`, `splitting/mod.rs`
  and `transform.rs`, plus four `match Band::linear(tol) { .. }` forms in
  `validate.rs`.

Each is one line, so the duplication is cheap; what it costs is that the
rendering of the refusal is decided per error type too. The `BandError`
carries repairs addressed to a caller choosing thresholds ("set a finite
positive tolerance", "raise ε or K"). A caller that handed in `Tol`
already holds a valid tolerance, so rendering the carrier whole gives it
recourses that are not open to it. `OffsetFitError::Band` and
`ReplaceFaceError::Band` render the one true recourse (a less extreme
ε); `ShellError::Band` and the other `{error}`-rendering arms above still
render the carrier whole. A carrier type built from one witness (above)
would give this derivation, and its refusal's rendering, one home.

