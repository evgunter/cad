---
id: real-rs-seat9-paragraph-describes-a-second-impl-block-that-is-gone
kind: issue
title: real.rs's SEAT-9 paragraph describes a second verbs/run.rs impl block that no longer exists
status: open
opened: 2026-09-21
priority: P4
cost: E
---


## What

`geom-core/src/real.rs`'s `Bounds` trait doc — the home of the
compound-bound allowlist's SEAT-4 entry — carries a SEAT-9 paragraph
opening *"The file now carries TWO compound headers, and the second is
the answer to the paragraph above rather than an exception to it"*
(`crates/geom-core/src/real.rs`, the paragraph before `# Semantics`).
It argues, in the present tense, that `crates/verbs/src/run.rs` holds
`run_shell` in a second `impl` block bounded `Decide + CertifiedBounds
+ AtRestPolicy`, and that *"They are two `impl` blocks, each asking for
exactly what its callee asks for"*.

`crates/verbs/src/run.rs` has one compound header. LANE-3 made the
shell's certification right a VALUE — `topo::ShellDoor<T>`, whose one
constructor is bounded on those rights — so `run_shell` moved onto the
seat's general `impl<T: Decide + Bounds + PcurveFittedLane +
AtRestPolicy>` block and takes the door as a parameter. The allowlist
entry's count moved 2 → 1 in that same change
(`scripts/gates/bounds-allowlist.sh`, the `crates/verbs/src/run.rs`
row, which states the move); the `real.rs` prose did not, because
`real.rs` is outside LANE-3's fence and the spec told the lane to stop
at the count rather than widen.

## What it needs

The paragraph re-worded to the present: the file carries one compound
header, and the second seam SEAT-9 argued for has nothing in it today.
Whether that RETIRES the SEAT-9 allowance or merely records that it
covers nothing is the judgement the row wants made — the allowance was
written by an implementer lane's own commit (`c4bd40a29f`, S9-1) and
`git log -S` finds no Ev ratification behind it, so it is not text that
has to wait, only text somebody has to decide about.
