---
id: sweep-suites-re-spell-the-nurbs-walls-predicate
kind: issue
title: Five sweep suites re-spell common::approx::nurbs_walls' non-placeholder NURBS face predicate inline
status: open
opened: 2026-09-26
---


Filed by the ENCL tight-ε lane's fix pass (PR 3272), whose new suite
`crates/sweep/tests/encl_curved_loft_shell.rs` first re-spelled this
predicate and now calls the door.

## Finding

`crates/sweep/tests/common/approx.rs` has `nurbs_walls(body)`, the
one home for "the body's faces whose surface is a real (non-placeholder)
`Surface::Nurbs`", returning each face key with its surface. Suites in
the same test binary spell the same body-face walk inline instead.

Searched by shape, not symbol: `Surface::Nurbs(<x>)) if
!<x>.is_placeholder()` across `crates/` on this branch.

| site | shape | disposition |
|---|---|---|
| `crates/sweep/tests/offd_r1_probes.rs`, `the_fitted_obstruction_holds_on_a_curved_fit` and the row above it (two sites) | `.faces().find(...)` over the predicate | a `nurbs_walls(&body).first()` |
| `crates/sweep/tests/cert8_r1_probes.rs`, its wall walk | `filter_map` over the predicate | `nurbs_walls` |
| `crates/sweep/tests/r2_probe_cert8.rs`, its wall walk | the same | `nurbs_walls` |
| `crates/sweep/tests/m9_2_chart_region_loft.rs` (two sites) | the same | `nurbs_walls` |
| `crates/sweep/tests/verbs_offd.rs`, one `matches!` inside a face filter | the predicate alone | `nurbs_walls` if the walk is the same one |
| `crates/sweep/tests/common/approx.rs`, `nurbs_walls` itself | the door | the home |

Not this row: `crates/geom/tests/*` and `crates/geom/src/surfaces.rs`
assert the predicate on ONE surface (no body walk, a different
question); `crates/step-import/src/adopt.rs` is production; the two
`crates/step-import/tests/` sites (`recognize_pins.rs`,
`cert_n2r2_consumer_probes.rs`) are in a different test binary with no
access to `sweep`'s `common`, so they are a candidate for a shared home
only if one is made above both crates.

**Blind spot:** a walk that tests the kind through `SurfaceKind`, a
`match` with the placeholder test in the arm body rather than a guard,
or `n.is_placeholder()` read without the `!` (keeping placeholders) is
not matched. The last shape was checked by the same grep without the
`!`; its hits (`r2_probe_cert8.rs`'s `if p.is_placeholder()` branch)
are a different question and not copies.
