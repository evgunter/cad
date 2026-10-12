---
id: a-sphere-cut-in-divides-a-face-the-carry-reads-by-its-pre-cut-key
kind: issue
title: A sphere cut-in divides a face outside the carry's division rows, so a carried record still names the pre-cut key
status: open
opened: 2026-10-11
priority: P1
cost: M
refs: [4650]
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
