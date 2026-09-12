---
id: geom-curve-test-frames-hand-roll-the-helper-axis-cone
kind: issue
title: two geom curve-test frame helpers hand-roll the |z| < 0.9 helper-axis cone instead of calling the kernel's basis door
status: open
opened: 2026-09-12
---


## Finding

**Raised by SCALAR's S393 lane**, from the class sweep that unit ran
(`docs/S393-SPEC.md` §4: a hand-built frame whose axes come from a
world-axis cone rather than from a kernel door).

Two helpers in `crates/geom/tests/curves/` build a deterministic
orthonormal `(axis, u_ref)` pair by picking the helper axis off a hard
cone:

- `boxes.rs`'s `unit_frame` — `if axis.z.abs() < 0.9 { +Z } else { +X }`,
  then `axis.cross(helper).normalize()`;
- `n3r2_probes.rs`'s `frame` — the same two lines, then a further
  in-plane rotation by `phi`.

The kernel decides this: `Vec3::orthonormal_basis` returns the pair, and
`geom_core::linalg::frame`'s module docs name `if n.z.abs() < 0.9 { e_z }
else { e_x }` as the magic-constant dodge its band-decided policy
replaces. The cost is not a wrong answer today — either construction is
a correct orthonormal pair — it is that these suites' frames are not the
kernel's frames, so a change to the kernel's basis policy does not reach
them and cannot be measured through them. PROPS is changing exactly that
policy (`docs/PROPS-SIGN-HULL-SPEC.md`, announced on several logs), which
is when a suite holding its own copy stops agreeing silently.

The third site of this shape is `crates/step-import/src/chart_review_fuzz.rs`
and is EXCH's; it has its own row,
`chart-review-fuzz-frame-hand-rolls-the-helper-axis-cone`.

**Where**: `crates/geom/tests/curves/boxes.rs` (`unit_frame`),
`crates/geom/tests/curves/n3r2_probes.rs` (`frame`).

**Confidence**: sure (both helpers read as quoted).

**Verdict:**
