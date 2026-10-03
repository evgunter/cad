---
id: mesh-message-less-ordering-asserts-have-no-unit
kind: issue
title: crates/mesh's message-less ordering asserts that are not dominations have no unit
status: open
opened: 2026-09-18
---

## Finding

PR 2848 (`tess/domination-assert-messages`) gave messages to `crates/mesh`'s
bound-vs-truth asserts and deliberately left the message-less ordering
asserts that are NOT dominations, calling them "a different unit". This row
is that unit. A red on any of these prints only
`assertion failed: <expression>` — no value, so the reader reproduces by
hand to learn how far off it was.

In `crates/mesh/src/` (cited by enclosing test):

- `nurbs_cert.rs`: `hessian_hull_dominates_sampled_second_partials`
  (`b.muu > 0.0 && b.muv > 0.0 && b.mvv > 0.0`);
  `planar_bilinear_bounds_collapse` (`hu > 1e3 && hv > 1e3`); the
  `(b.cert(uv) - 0.25 * q).abs() < 1e-15` closed-form row; the
  `bands[1].nvc >= 1` band-schedule row.
- `sizing.rs`, `torus_grid_steps_meet_the_bound_with_equality`: three —
  `hu < sagitta_step(..) && hv < sagitta_step(..)`, the
  `((ratio - 2.0 * (1.0 + beta)) / ratio).abs() < 1e-12` identity, and
  `ratio > 2.0 && ratio < 2.0 + SQRT_2`.
- `chords.rs`: a `count >= 1`. `curved.rs`: `MAX_ANGULAR_STEP <= FRAC_PI_4`
  over two constants — a compile-time fact, so by
  `docs/prompts/implementer-discipline.md` §2 it is documentation and a
  `const` assertion or deletion is the repair, not a message.

In `crates/mesh/tests/`: `mesh10r2_probes.rs` (`worst < 1e-12`, and the
`seam` conjunction beside it); `m7_nurbs_trimmed.rs` (`dev_coarse > 0.0 &&
dev_fine > 0.0`, `tris_fine > tris_coarse`, `dev_fine < dev_coarse`);
`mesh7r1_probes.rs` (the `t_pole` conjunction); `newell_probes.rs` (a volume
tolerance and a `triangle_count > 0`); `common/mod.rs`
(`bp.points.len() >= 2`); `errors.rs` and `review_m2_pr6_errors.rs`
(`count > 16_777_216.0`); and four `signed_volume(..) > 0.0` rows across the
`review_m2_pr6_*` suites.

The extraction found 27 as of `17fc3bfda`; one (`profile_overrides.rs`, a
`>` inside a closure) is a false positive, and PR 2848 gave messages to the
three that were dominations or dust ceilings, leaving the 23 above.
Conjunctions are the worst of them: a red does not say which conjunct
failed.

## Blind spot

Same extraction as
`work/tint/ordering-asserts-outside-mesh-unswept-for-illegible-domination-messages.md`:
`assert!` / `debug_assert!` with an ordering operator in the condition.
Message-less `assert_eq!` rows and bare `assert!(flag)` rows were not
counted.

## Home

Filed on CHORD's slate because most of the list is in CHORD's four files
(`nurbs_cert.rs`, `sizing.rs`, `chords.rs`); the `crates/mesh/tests/` and
`curved.rs` entries are TESS's ground and ride along rather than getting a
second row for one class.
