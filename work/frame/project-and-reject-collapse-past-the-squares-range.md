---
id: project-and-reject-collapse-past-the-squares-range
kind: issue
title: Vec3::project_onto and reject_from collapse silently where onto's squares leave the format
status: open
opened: 2026-10-21
refs: [normalize-overflow-yields-zero-axis]
priority: P2
cost: E
---

Sibling of `normalize-overflow-yields-zero-axis`, found by its review
and not taken with it. `Vec3::project_onto`
(`crates/geom-core/src/linalg/vec.rs`) divides by
`onto.norm_squared()`, so an `onto` past ~1e154 projects to a silent
zero and one below ~1e-162 blows up; `Vec3::reject_from` divides
`(onto × self) × onto` by the same square, and its own bands (an
`onto` near 1e150 with `|self|` near 1e20 overflows; an `onto` below
~1e-140 rejects an orthogonal `self` to an exact zero) are pinned in
`crates/geom-core/tests/props1_review_rows.rs`
(`p1_reject_overflow_band_is_not_project_ontos`,
`p1_reject_underflow_band_accuracy`).

**Why `normalize`'s prescale is not the same small change here.** Both
results are invariant under scaling `onto` by a power of two, so
`normalizing_scale(onto.norm_witness())` applied to `onto` is exact
over the reals. But `normalize`'s result is bounded by one, so the only
bits its scale-down can drop are of entries already below the normal
range. A projection's entry is `onto_i · (self·onto)/|onto|²`, which
scales with `|self|/|onto|` (up to `2⁵¹³` when the witness is in
`[2⁵¹¹, 2⁷⁶⁷)`), so a component of `onto` that the `2⁻²⁵⁶` arm flushes
can carry a normal entry of the answer. That breaks the rule the
`normalize` prescale was held to: lose no bit the unscaled spelling
got right. `reject_from`'s numerator is `|onto|²·|self|`, so its bands
also depend on `|self|`, and they sit inside `normalize`'s unscaled
range (1e150, 1e-140), where `normalizing_scale` does nothing.

Each needs its own band analysis (the cuts on `onto`, and whether to
scale `self` too), and `reject_from`'s two pinned rows flip with it.
