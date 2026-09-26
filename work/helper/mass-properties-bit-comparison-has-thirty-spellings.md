---
id: mass-properties-bit-comparison-has-thirty-spellings
kind: issue
title: compare a MassProperties by bits is written ~31 ways and they disagree about the pads
status: open
opened: 2026-09-15
priority: P1
cost: D
---


## What

"Compare a `MassProperties` by bits" is written out by hand in about
thirty test files. Sweep (pattern and blind spot below), as of
`f0dd455`, counting OCCURRENCES of a mass-properties field read through
`to_bits()`:

```
8 sweep/tests/sign_walk_plus_v.rs          3 sweep/tests/shellfix1_bitdump.rs
7 pncad/tests/all.rs                            3 sweep/tests/sf2a_r1.rs
4 sweep/tests/tcost_k3_certificate.rs           3 sweep/tests/lib_u3_sections.rs
4 sweep/tests/shell_census_is_thread_count_invariant.rs
4 sweep/tests/reporting_door_bit_digest.rs      2 topo/tests/void_door.rs
4 sweep/tests/mass_props_are_thread_count_invariant.rs
4 sweep/tests/m5_pr11_quad_props.rs             2 topo/tests/shell_roles.rs
4 sweep/tests/continuation_is_thread_count_invariant.rs
4 step-import/tests/tcost_k3_import_certificate.rs
4 step-import/tests/nurbs_import.rs             2 topo/tests/m4_pr2_transform.rs
4 mesh/tests/mesh10r1_probes.rs                 2 sweep/tests/revolve_ring.rs
4 mesh/tests/iso_rectangle_door.rs              2 sweep/tests/review_fillet_h5_r1_probes.rs
4 editor-core/tests/lib_g17_r2_probes.rs        2 sweep/tests/r1_area_gauge_probes.rs
3 viewer/tests/combine_ops.rs                   2 sweep/tests/fillet_h5_hostless_rim.rs
3 topo/tests/m3_pr1_surgery.rs                  2 editor-core/tests/review_m4_pr2.rs
                                                2 editor-core/tests/r1_m10_1_probes.rs
1 viewer/tests/blend_authoring.rs               2 editor-core/tests/m4_pr8_k_probe.rs  (converted)
1 sweep/tests/review_blend1_r2_probes.rs
1 sweep/tests/fillet_h5_r2_probes.rs
```

31 files, ~110 occurrences.

## Why it is a row and not a style note

**The spellings disagree about what the comparison COVERS.** The
certificate has four fields — `volume`, `surface_area`, `volume_pad`,
`area_pad` — and the pads are the certified half-widths a quadrature
face produces. A spelling that reads `volume.to_bits()` alone claims
"the mass properties are bit-identical" while saying nothing about the
enclosure that produced the volume; two of the files above
(`step-import/tests/nurbs_import.rs`,
`sweep/tests/tcost_k3_certificate.rs`) do read the pads, most do not,
and nothing holds them level. That is the "one claim restated in N
places, where the harm is drift" shape this program names.

## Disposition of the sweep

- **Converted (SUITE/D114, this PR):**
  `editor-core/tests/m4_pr8_k_probe.rs` — its local feed is gone and it
  calls `fixture::value_channel::props_digest`, which reads all four
  fields.
- **Not converted, and why:** every other hit. `props_digest` lives in
  `editor-core`'s test tree and is generic over `ValueChannelBits`, a
  trait that tree declares; `sweep`, `topo`, `mesh`, `step-import`,
  `viewer` and `pncad` cannot reach it. Converting them needs a home
  that all six can see — `crates/test-utils` is the obvious candidate —
  and that is a decision about a shared crate's surface, not a thing a
  differential lane should take on the side.

## What the pattern could not match

`volume\.to_bits()|surface_area\.to_bits()|volume_pad\.to_bits()|
area_pad\.to_bits()` over `crates/*/tests/`. It misses: a field bound
to a local first (`let v = m.volume; v.to_bits()`); a comparison
through `Debug` text rather than bits (`fixture/digest.rs`'s feed does
this deliberately); a comparison through an `assert_eq!` on the whole
struct where `MassProperties` derives `PartialEq`; anything in
`crates/*/src` doctests; and the `demos/`, `benches/` and `tools/`
roots, which `crates/*/tests` does not cover.

Filed by SUITE/D114.
