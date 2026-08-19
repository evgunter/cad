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

**Calibration.** Expect findings counts to rise, and the docs column to widen
downward. Per Protocol v5 that is the instrument changing, not implementation
quality. A style lane producing nothing on most PRs is under-calibrated, not
clean — though that expectation is inferred from the scan's hit rate on merged
code, not measured on single diffs, so revisit it after a few rows.

**What this lane must not become.** A second amnesty channel. §C2/§C7 found
that disclosure currently functions as immunity — a disclosed deviation scores
as a *positive* on the "silent devs" column with no counter-metric asking
whether it was acceptable. The reviewer brief's Q6 exists to close that; do not let a `## Style` section
become the place where known problems go to be recorded and forgotten.
