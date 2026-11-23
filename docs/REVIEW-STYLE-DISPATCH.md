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

## 2. Two shapes worth naming in the emphasis

**The fix reproducing the defect it closes.** A unit that unifies duplicates can
mint one; a unit that adds a guard can leave it failing open.

**A disclosed blind spot read as a discharge.** A lane's own "my pattern could
not match X" is a work order, not an absolution.

Both are invisible to the falsification lane, which asks whether the claims
hold rather than whether the shape is right.

---

## 3. The dispatcher's own exposure

**Reviewers correcting the dispatcher is a working lane, not a malfunction** —
say so in the brief.

**The rule against enshrining an unchecked causal story binds the dispatcher
hardest** (`memories/review-and-dependency-policy.md`). A lane's unverified
observation, repeated back to it as an instruction, arrives carrying the
dispatcher's authority and is one commit from a ratified doc. Check a lane's
claim before you build a brief on it.

---

## 4. What this lane must not become

A second amnesty channel. §C2/§C7 found that disclosure currently functions as
immunity — a disclosed deviation scores as a *positive* on the "silent devs"
column with no counter-metric asking whether it was acceptable. The reviewer
brief's Q6 exists to close that; do not let a `## Style` section become the
place where known problems go to be recorded and forgotten.
