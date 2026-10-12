---
id: a-curved-boolean-refusal-cites-the-germs-recorded-face-after-a-mef
kind: issue
title: The join's CurvedBooleanUnsupported cites the germ's recorded face, which an earlier segment's mef may have divided, not the face that holds the segment
status: closed
opened: 2026-10-04
priority: P4
cost: E
branch: flush/e-tail
pr: 4714
closed: 2026-10-11
---

Found by PR 4008's sweep (cause 2's class: a face key recorded before
surgery, read afterwards as a region) and left unfiled there; review r2
of that PR asked for a file.

## What

`crates/topo/src/boolean/join.rs` `section_segments`: the arm for a
germ pair with no wired join (`(a_s, b_s) =>`, the
`BooleanError::CurvedBooleanUnsupported { operand, face, kind }`
refusal) cites `germ.a_face` or `germ.b_face`, the face the germ was
recorded against at insertion. An earlier segment's `mef` can divide
that face, so the refusal may name a fragment the segment's halves have
left. The kind is right either way (a `mef` fragment keeps its parent's
surface), so only the cited face is wrong.

`wall_region` in the same file reads the face the halves sit on, within
the recorded face's lineage; the refusal could cite that face instead.
No pose is known to reach it after a division: unmeasured.

## Evidence (PR 3985, 2026-10-04)

`boolean::join`'s `wall_region` is gone: it chose the region whose
azimuth window the planar-side chord read, and PR 3985
(`reach/arc-from-pairing`) retired that window — the chord takes the
arc its pairing names. `rest::fragment_holding` and
`chord_join::lineage` remain.

## Measured (branch `flush/e-tail`, 2026-10-11)

The site moved: the refusal is `bool_connect`'s `no_arm` closure in
`boolean/join.rs` (there is no `section_segments` any more), and it
still cited `germ.a_face` / `germ.b_face`. A probe beside it compared
the recorded face with the face holding the segment's entry half
(`chord_join::he_face`) at every refusal, over the 19 `sweep` files
that assert `CurvedBooleanUnsupported` (release, 131 tests): 379
refusals, every one citing the same face either way. No measured pose
reaches it after a division.

## Fix

`no_arm` cites the face that holds the segment's entry half on the
cited operand now (`he_face` of `ea` or `eb`), which lies in the
recorded face's lineage; a stale half refuses as the join's own
`Corrupt`. The kind is read from the recorded face's surface as
before: a `mef` fragment keeps its parent's.

## Closed (2026-10-11, PR 4714)

Fixed in the E-tail batch, PR 4714 (single FULL review, APPROVE-WITH-FIXES; every fix taken). The PR body has a section for this row.
