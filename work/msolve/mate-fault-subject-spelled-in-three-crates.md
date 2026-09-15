---
id: mate-fault-subject-spelled-in-three-crates
kind: issue
title: Which mate a MateFault is about has no home on the enum, and two consumers spell it per-arm
status: open
opened: 2026-09-04
refs: [1769]
---

Found by CHROME's style lane on PR 1769, as a class. **Re-homed to
MSOLVE on 2026-09-15 by the CHROME orchestrator, with its premise
corrected** — see "What this row got wrong" below. The id is unchanged;
the title is not, because the old one asserted a count that is false.

`crates/editor-core/src/mate.rs` is MSOLVE's ground, and the fix this
row wants is a method on `MateFault`. CHROME could not write it: the
kernel was fenced by CHROME's `keep_out`, which is why PR 1769 wrote a
viewer-local copy instead. That fence is why the row exists at all.

## The finding

**`MateFault` has no `subject()`.** `grep 'fn subject'
crates/editor-core/src/mate.rs` is empty, so "which mate is this fault
about" is answered by a hand-written match over the enum's arms at each
consumer that needs it, rather than once beside the arms themselves.

Two consumers spell it today:

- `viewer::tree::blamed_mates` (`crates/viewer/src/tree.rs`), added by
  PR 1769. This one is `match`-exhaustive, so a new kernel arm breaks
  its build.
- `MateFaultPayload` (`crates/pncad-py/src/mate_payload.rs`), which is
  LIB's ground.

Neither cites the other.

## What this row got wrong, and what corrects it

The row was filed claiming **three** spellings in three crates. Two of
its three citations no longer say what it claimed, and one never did:

1. `crates/pncad-py/src/py/mate.rs:568-581` — **gone.** `MateFault::mate`
   now reads `self.payload().mate.map(NodeId)`, and the single per-arm
   destructuring moved to `MateFaultPayload`. LIB landed that closing
   `work/lib/mate-fault-accessors-wildcard-into-silence` on 2026-09-08.
2. `crates/pncad-py/src/tags.rs:394-402` — **was never this finding.**
   That is `tags::mate_fault_tag`, which answers **which arm fired**
   (it returns a `&'static str` tag per variant), not which mate the
   fault is about. `mate_payload.rs`'s own header draws exactly this
   line: *"`crate::tags::mate_fault_tag` answers WHICH mate refusal
   fired. This module answers what the arm CARRIES."* So the original
   filing counted a different question as a third copy of this one.
3. `crates/viewer/src/tree.rs:286-292` — still live, now at
   `blamed_mates`; the band moved with the split.

So the population is **two consumers in two crates**, not three, and
the duplication has halved since filing rather than grown.

**The cost argument therefore needs restating before anyone acts.** The
row's stated cost was *"the next consumer writes a fourth copy"*, which
was arithmetic on a count that was wrong. What survives it is the
argument that does not depend on the count: there is no home on the
enum, so each consumer re-derives the answer, and only one of the two is
exhaustive — `MateFaultPayload` should be checked for that property,
which this row never did.

Whoever takes this should decide whether two non-citing consumers, one
of them already exhaustive, is worth a kernel method at all. **That is a
real possible outcome and it closes the row with a citation rather than
a change** — the honest form of which is a sentence on `MateFault`
naming its two consumers, not silence.

## Territory

The fix lands in `crates/editor-core/src/mate.rs` (MSOLVE). The second
consumer is `crates/pncad-py/src/mate_payload.rs` (LIB's ground) and the
first is `crates/viewer/src/tree.rs` (CHROME/VIEW) — so adopting a
`subject()` is a three-program change and wants announcing, not a
unilateral edit.

Signed: (CHROME orchestrator), re-homed 2026-09-15
