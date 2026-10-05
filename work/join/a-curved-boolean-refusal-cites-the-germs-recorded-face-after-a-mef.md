---
id: a-curved-boolean-refusal-cites-the-germs-recorded-face-after-a-mef
kind: issue
title: The join's CurvedBooleanUnsupported cites the germ's recorded face, which an earlier segment's mef may have divided, not the face that holds the segment
status: open
opened: 2026-10-04
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
