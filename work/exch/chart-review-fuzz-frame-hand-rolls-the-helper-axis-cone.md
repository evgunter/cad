---
id: chart-review-fuzz-frame-hand-rolls-the-helper-axis-cone
kind: issue
title: chart_review_fuzz's frame() hand-rolls the |x| < 0.9 helper-axis cone instead of calling Vec3::orthonormal_basis
status: open
opened: 2026-09-12
---


## Finding

**Raised by SCALAR's S393 lane**, from the class sweep that unit ran
(`docs/S393-SPEC.md` §4: a hand-built frame whose axes come from a
world-axis cone rather than from a kernel door).

`crates/step-import/src/chart_review_fuzz.rs`'s `frame` builds the
fuzz harness's `(axis, u_ref)` pair by picking the helper axis off a
hard cone — `if a.x.abs() < 0.9 { +X } else { +Y }`, then
`a.cross(h).normalize()`. `Vec3::orthonormal_basis` is the kernel's
door for that pair, and `geom_core::linalg::frame`'s module docs name
the `abs() < 0.9` form as the magic-constant dodge the kernel's
band-decided policy replaces.

Not a wrong answer today: the pair is orthonormal either way. What it
costs is that the fuzz harness's frames are not the kernel's, so a
change to the kernel's basis policy — PROPS is making one, and its
`u_ref` seam is already announced on this program's log (2026-09-06) —
does not reach the frames this harness fuzzes over.

The sibling sites of this shape are two helpers in
`crates/geom/tests/curves/`, which are S-TINT's; they have their own row,
`geom-curve-test-frames-hand-roll-the-helper-axis-cone`.

**Where**: `crates/step-import/src/chart_review_fuzz.rs`, `frame`.

**Confidence**: sure (the helper reads as quoted).

**Verdict:**
