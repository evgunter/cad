---
id: geom-curve-test-frames-hand-roll-the-helper-axis-cone
kind: issue
title: test-side frame helpers hand-roll a world-axis cone instead of calling the kernel's basis door
status: open
opened: 2026-09-12
priority: P4
cost: E
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

## The class is wider than the `0.9` literal (2026-09-15, S393's fix pass)

The sweep that raised this row keyed on `0.9`. The shape does not need
that constant: **an if/else between two world-axis constants feeding a
`cross`** is the same hand-rolled basis whatever the threshold, and
re-run that way the pattern finds more of this suite's neighbours. The
test-side members, all TINT/TCOST ground, are:

- `crates/geom/tests/curves/boxes.rs` (`unit_frame`) — `|z| < 0.9`;
- `crates/geom/tests/curves/n3r2_probes.rs` (`frame`) — `|z| < 0.9`,
  then a further in-plane rotation by `phi`;
- `crates/topo/tests/review_mate9_r2_probes.rs` (`plane`) —
  `|normal.x| < 0.5` picking `+X` or `+Y`, then a double cross to land
  the reference in the plane;
- `crates/geom-brep/tests/intersect_table.rs` (`circle_samples`) —
  `|axis × x̂| > 0.5` picking `x̂` or `ŷ`, the same choice written as a
  magnitude test on the cross product itself.

`Vec3::orthonormal_basis` answers all four. One member of the class was
**inside S393's fence and is folded**:
`crates/sweep/tests/review_fillet_h6_r2_probes.rs`'s `cap_plane` now
takes its `u_ref` from that door. The two `src` members the same
pattern found are on their owners' slates —
`crates/topo/src/boolean/join.rs` (BOOL,
`join-probe-charts-hand-roll-the-across-axis-reference`) and
`crates/viewer/src/datums.rs` (VIEW,
`datums-basis-hand-rolls-the-least-aligned-axis-basis`).

**Where**: `crates/geom/tests/curves/boxes.rs` (`unit_frame`),
`crates/geom/tests/curves/n3r2_probes.rs` (`frame`),
`crates/topo/tests/review_mate9_r2_probes.rs` (`plane`),
`crates/geom-brep/tests/intersect_table.rs` (`circle_samples`).

**Confidence**: sure (every helper reads as quoted).

**Verdict:**
