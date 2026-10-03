---
id: a-mate-frame-is-written-in-the-reading-instances-coordinates
kind: issue
title: A mate's literal frame is written in the coordinates of the instance it reads, so split and inline must refuse where its meaning would change
status: parked
opened: 2026-10-01
priority: P2
cost: M
design: true
blocked_on: [3990]
---


Filed by the EDIT orchestrator off `[ev]` PR 3505 (fork-log row 21),
where the designers named it M1 and Ev took the narrow rule for now on
the understanding that this row lifts it ("if M1 will fix this then A
is fine!", 2026-10-01).

**The finding.** A mate side's frame is a literal origin, axis and
clocking reference "in that instance's own part coordinates"
(`MateFrame`, `crates/editor-core/src/mate.rs:120`). For a face deep in
a sub-assembly h, those are h's part coordinates, not the coordinates of
the document that owns the face. So the same numbers mean something else
whenever the instance the mate reads changes:
- **Split and inline** change it. Under ASSEMBLY.md A4, as ruled on
  PR 3505, a mate crosses the seam only when its coordinates do not
  change: the instance it reads is its group's root, on the part's
  world, at the empty chain. Otherwise it refuses, named. Inline of a
  mate-placed instance whose part's root sits at an offset refuses for
  the same reason (choice 1 = A).
- **Outside split and inline**, a literal frame on a sub-assembly's
  inner face stays put when that inner part moves inside the
  sub-assembly (for example when its offset or the sub-assembly is
  updated), so the mate drifts off the face it names.

**The proposal (M1, both designers of row 21).** Write a mate frame in
the coordinates of the document that mints its face, and have the solve
compose the inner part's pose inside h, which h's evaluation already
holds (`SolvedPoses` rides in the evaluation context, `eval/wire.rs`).
Split and inline then never touch an alignment, the frame-rule
refusals in A4 and inline's choice-1 refusal lift, and a frame on an
inner face follows that face.

**Why it is a design row.** It changes A3 (what a mate's frame means),
MSOLVE's ground, and the saved meaning of every literal frame on a
non-root inner face (a migration question: re-express existing frames,
or version the field). It goes to a designer pair and an `[ev]` PR.

**The face arm already has M1's property** (`[ev]` #3888, built by
P2-face, `docs/doc-ledger/edit-placement-spec.md` § P2-split rulings 7 and 8): a
`FromFace` side names no face, its frame is its own head's face read in
the member's part, so it crosses split and inline with its head under
A4's condition (b) alone, and follows that face wherever the head
reads it. What this row asks is the same for the authored arm.
