---
id: mate-fault-subject-spelled-in-three-crates
kind: issue
title: Which mate a MateFault is about has no home on the enum, and two consumers spell it per-arm
status: closed
opened: 2026-09-04
refs: [1769]
closed: 2026-09-19
pr: 2896
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

## Evidence added 2026-09-15 (CHROME, `chrome/band-refusal-badging`)

Measured while covering `MateFault::Band` in `viewer::tree`. Three
facts about the subject question that the row's two consumers both
re-derive, and that a `subject()` on the enum would state once:

1. **Two arms name no mate, and they are not the same case.** `Band`
   (`crates/editor-core/src/mate.rs`, `MateFault::Band { error }`)
   carries a `BandError` and nothing else; `PosesOfAnotherDocument`
   carries two `DocumentId`s. `PosesOfAnotherDocument` is raised by
   `SolvedPoses::placement` (`crates/editor-core/src/mate/solve.rs`,
   ~`:142`) and **never inserted into a solve's fault map**, so no node
   result can carry it — while `Band` IS recorded against nodes and a
   user does meet it. A `subject()` returning `Option<RecipeNodeId>`
   would answer `None` for both and lose that asymmetry; the two
   consumers' comments carry it today, in prose, twice.

2. **`Band`'s fan-out is the DOCUMENT's, not a cluster's.**
   `solve_document` (`crates/editor-core/src/mate/solve.rs`, the
   `Band::linear` arm at the top of the function) inserts the one
   cloned fault against **every** `Node::Mate` and **every**
   `Node::InstantiatePart` in `doc.order()` and returns before it reads
   a single mate — so the refusal reaches instances in singleton
   clusters that no mate touches, and reaches them before `has_mates`
   or `read_mates` is consulted. Any consumer wording that scopes this
   refusal to "the cluster" names the wrong set. Pinned from the viewer
   side by `crates/viewer/tests/tree_badges.rs`'s
   `a_band_refusal_reaches_the_whole_document_and_blames_no_row`.

3. **The arm is reachable, and only along one path.** At a tolerance
   `Band::linear` refuses, the evaluator's own band door
   (`crates/editor-core/src/eval/wire.rs`, `fn band`) refuses most
   nodes with `NodeErrorKind::Band` first, and the authoring door
   refuses a profile outright — so `MateFault::Band` is met only by a
   document of instances and mates carrying no geometry of its own,
   which is what a saved document commits when its stored ε is read
   back through `Tolerance::init_document_eps`. That is the user path
   the badging complaint
   (`work/chrome/band-refusal-still-badges-every-row.md`) sits on, and
   it is worth stating on the enum rather than rediscovered per
   consumer.

No kernel change is proposed here and none was made.

**What this evidence asks of the row, so it is a schedule and not a
record.** It sharpens the decision the row already says is the open
one — *"whether two non-citing consumers, one of them already
exhaustive, is worth a kernel method at all"* — into a question with a
concrete answer shape, and **MSOLVE is the owner** (`crates/editor-core/
src/mate.rs`):

*A bare `subject() -> Option<RecipeNodeId>` is now the WRONG signature,
and adopting it would be a regression rather than a tidy-up.* It
answers `None` for both subject-less arms and so erases the asymmetry
in (1) above — one arm a user meets and one that cannot reach a row —
which is the single fact both consumers' comments exist to carry. Two
next steps, and whoever takes the row picks one:

- adopt a richer answer that keeps the distinction (an enum over
  *names a mate* / *names none and reaches rows* / *names none and
  cannot*), and re-point both consumers at it; **or**
- close the row with the sentence the row itself already sanctions —
  a note on `MateFault` naming its two consumers — and state the
  asymmetry there, so it has one home instead of two comments.

Either way the `Band` half is CHROME's to badge and MSOLVE's to
describe; `work/chrome/band-refusal-still-badges-every-row.md` is the
badging half and does not wait on this row.

## Closed (2026-09-19, PR 2896)

By the form the row sanctions and `plan.md` item 15 ruled: no
`subject()`. One sentence on `MateFault`'s own doc
(`crates/editor-core/src/mate.rs`) names its two consumers by path —
`viewer::tree::blamed_mates`, `pncad_py::MateFaultPayload` — and
states the asymmetry once: `Band` names no mate and reaches every row
of the document; `PosesOfAnotherDocument` names no mate and reaches
none, being raised by `SolvedPoses::placement` and never inserted in a
fault map. The two consumers' comments are one-line pointers at it
(`crates/viewer/src/tree.rs`, `crates/pncad-py/src/mate_payload.rs`),
so the fact has one home instead of two comments.
