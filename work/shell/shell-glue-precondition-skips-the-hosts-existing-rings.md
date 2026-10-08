---
id: shell-glue-precondition-skips-the-hosts-existing-rings
kind: issue
title: shell_open's glue precondition compares the new ring with the host's outer loop only, not with the rings the host already holds
status: open
opened: 2026-10-07
priority: P3
cost: E
refs: [check-9-refuses-only-a-ring-meeting-its-outer-loop]
---


## What

Found by the sweep on branch `join/tier3-pinch-checks`. Before
`kfmrh(host, guest)` makes the guest's outer loop a ring of `host`,
`crates/topo/src/shell.rs` (the glue in the open-face arm, around
`ring_outer_contact(&out, host_outer, guest_outer, band)`) checks that
loop against the host's OUTER loop, and each promoted pair against its
partner. It does not check it against the rings the host already
holds. A designated face that already carries a hole could take a new
ring that touches it.

No body ships that way: check 9 now refuses two rings of one face
meeting (`ValidationError::RingMeetsRing`), and the verb's closing
`validate_geometric` runs it. But the refusal then arrives as the
verb's generic at-rest report, not the named
`ShellError::OpenFaceRimNotExpressible` the precondition gives for the
outer loop. Not measured: no fixture here was built to reach it.

## The shape to give

Run the same `ring_outer_contact` against each of `host`'s rings before
the glue, refusing `OpenFaceRimNotExpressible` with its own `what`, and
pin it on a designated face that holds a hole near the mouth.
