---
id: the-mesh-crates-unit-rows-paste-the-chord-pass-setup-twelve-times
kind: issue
title: mesh's unit rows paste the chord-pass setup (vertex ids, then compute_chords) twelve times beside its one helper
status: open
opened: 2026-11-23
priority: P4
cost: C
---


Filed by TESSLANE's `tesslane/collapsed-side` lane, at a review finding. A unit
row of one face's polygon or chords needs the chord pass a witness run makes:
every vertex point minted into the shared positions with its id, then
`chords::compute_chords` on the witness route table. One helper does that,
`chords::tests::witness_pass` (the pass, the positions and the face bounds),
and `trimmed::collapsed_rows` and `chords`' own `chords_of` read it. Twelve other
test sites in `crates/mesh/src` write the setup out again (the `for (vk, p) in
body.vertex_points()` loop, then `compute_chords`): four in `curved.rs`, six in
`trimmed.rs`, one each in `tessellate.rs` and `walk.rs`. `grep -rn "for (vk, p)
in body.vertex_points()" crates/mesh/src` lists them with the helper's own.

Closing it: each site reads `witness_pass` (or a variant of it, where a site
needs its own route table), and the loop lives once.
