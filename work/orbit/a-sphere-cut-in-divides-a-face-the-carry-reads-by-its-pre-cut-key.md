---
id: a-sphere-cut-in-divides-a-face-the-carry-reads-by-its-pre-cut-key
kind: issue
title: A sphere cut-in divides a face outside the carry's division rows, so a carried record still names the pre-cut key
status: closed
opened: 2026-10-11
priority: P1
cost: M
refs: [4650]
closed: 2026-11-24
pr: 7593
branch: orbit/cut-in-carry
---


Unmeasured, read. Found by the review r1 of FLUSH's PR 4650
(`a-box-and-a-cylinder-through-its-corner-edge-intersect-past-tier-3-prime`).

## What

PR 4650 has `boolean::ops::carry_rows` read a vertex-on-face record's
face through the op's face divisions (`Descendants::holder`, over
`Descendants::fragments`: the join's chord-mef rows and the pinch
welds'). The sphere scan's cut-ins are not among them.
`boolean::ops::apply_cut_ins` cuts a sphere face along a meridian
half-plane before the op re-enters `boolean_op_recut`: the cut keeps
the face's key on one piece and mints the other, and neither the cut
nor its new key reaches the division rows. An operand's carried record
names the pre-cut key, so after the re-cut it is read by key alone,
which lands it on whichever piece kept the key: stale where the vertex
rests on the other piece, or lost with a discarded key piece.

## Owed

A pose: an operand carrying a vertex-on-face rest on a sphere face that
the scan cuts in, the rest on the minted piece. Measure it; then log the
cut-in's divisions with the op's own (`Connected`'s fragment rows, or a
row the re-cut pass hands the carry), so `holder` reads them.

## 2026-10-13: names retired by PR 4820 (FUSE)

PR 4820 (CONTACTHOLD's one carry) retired `Descendants::holder`. A
row the reduction found on a divided face now goes through
`boolean::ops::place_found`, and an operand's carried row through
`boolean::ops::carried` (`operand_fate`, then `records::landings`).
Both read a face's pieces through `face_pieces`, which still walks
`Descendants::fragments` only. A sphere cut-in writes no fragment row,
so the gap this row names is still there under the new names (read,
not measured).

## 2026-10-13: the naming record carries the cut-ins (EMIT, branch `emit/naming-refusals`)

`boolean::ops::apply_cut_ins` now returns each operand's cut-in
divisions and meridian edges. `through_the_join` puts the divisions
ahead of the re-entered op's `BooleanNaming::face_fragments_a`/`_b`
rows and lists the edges in `BooleanNaming::chart_cuts`, so naming
chases a cut piece to the face it divided
(`work/emit/a-sphere-cut-in-cap-fails-its-names-missing-upstream`).
That covers naming only. The re-entered op's own
`Descendants::fragments`, which `face_pieces` walks for the carry,
still holds the join's rows alone. Passing the cut-in rows into the
re-entered op, so both read one list, would close this row's gap and
make the after-the-fact prepend unnecessary (read, not measured).

## Closed (2026-11-24, PR 7593)

The cut-ins' divisions seed the re-entered op's own division list (`Descendants`: `cut_fragments` beside the join's `fragments`, and `arcs` for the rim arcs). So the carry, `lowered_faces` and naming read one list. Carried records read cut-in rows and join rows; found rows read the join's alone (`Keys`). The edge sibling (a vertex-on-edge record on a cut-in arc) is fixed in the same PR. Review: one full review, APPROVE-WITH-FIXES, then a verify, APPROVE. The silent drop the review found in `records::landings` is HELDRECORDS' `a-read-record-is-consumed-by-any-removed-remainder-so-a-lineage-gap-drops-it-silently`.
