# The style lane — dispatcher notes

The reviewer's document is `docs/prompts/reviewer-style-lane.md`. **Read it
once, then point reviewers at it by path — do not paste it.**

**What a dispatch still owes, on top of the pointer:**

- the explicit claims to falsify (`memories/orchestration-model.md`);
- any per-lane emphasis — which questions carry this particular review, and
  why;
- a reminder that the report must name the questions exercised and carry the
  confidence vocabulary.

---

## 1. The failure mode to avoid

**Do not turn the reviewer brief into a checklist.** Reviewers answer the
questions they are given: ten crisp yes/no items will produce ten crisp ticks
and no judgement. Every question in the brief is phrased to require taste, and
its stance exists to make "I'm not sure, but this looks off" a *complete and
welcome* finding — which the adversarial-falsification lane, with its high
confidence bar, actively discourages.

---

## 2. Calibration, and what this lane must not become

**Calibration — measured 2026-08-20, over Track D's six units.** The
expectation above was inferred from the scan's hit rate on merged code and
asked to be revisited after a few rows. It has been, and it holds: **no style
review in the wave produced nothing.**

What they found, on units that had already passed their author's own checks:

- **D18** — three MAJORs, including two rotted premises in `euler.rs`'s module
  header, the file the PR edited most and two thousand lines above its diff.
- **D2** — a new type that was `topo::EntityId` with two arms removed and a
  byte-identical `Display`, minted inside the PR closing a duplication finding,
  with the worked precedent recorded one screen away in the same document.
- **D17** — a guard that failed open in exactly the case its own message names,
  in the half that PR added; and a fifth statement of the unit's own premise,
  live in another programme's issue, that no wording of the lane's sweep reached.
- **D8** — an eighth copy of the scanned-for pattern, in a fifth crate, plus the
  observation that the PR had already performed the extraction that would have
  prevented it, for the *other* primitive, in the same diff.
- **D7** — a dead `pub` accessor ten lines above the deleted struct, in the file
  the diff opened, which the unit's own disclosed blind spot said it could not
  see and did not go look for.

**Two shapes recur and are worth dispatching against.** First, *the fix
reproducing the defect it closes* — three of the six. Second, *a disclosed blind
spot treated as a discharge*: the reviewer brief's stance makes "I could not
tell" a complete finding, and the correlate is that a lane's own "my pattern
could not match X" is a **work order**, not an absolution. Both are invisible to
the falsification lane, which is asking whether the claims hold rather than
whether the shape is right.

**Two of the errors corrected in this wave originated in orchestrator briefs**,
not lanes — a graft-door argument built on a door that cannot raise the error
its docs advertise, and an instruction to publish a *ceiling* where the true
figure was a floor. Reviewers correcting the dispatcher is a working lane, not a
malfunction, and the dispatcher should say so in the brief.

**What this lane must not become.** A second amnesty channel. §C2/§C7 found
that disclosure currently functions as immunity — a disclosed deviation scores
as a *positive* on the "silent devs" column with no counter-metric asking
whether it was acceptable. The reviewer brief's Q6 exists to close that; do not let a `## Style` section
become the place where known problems go to be recorded and forgotten.
