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

## 1. Why this exists (dispatcher context)

`docs/SMELL-SCAN-2026-08.md` §C established, from twenty structural scans and
seven postmortem passes over the merged history, that this project's reviews
are **exceptionally strong at soundness and structurally blind**. The same
reviews that ran 8000-matrix SVD differentials, re-derived a meters conversion
by hand, and found a certificate excluding true 2π by ~1111 widths produced
**zero** findings on: a mode switch on `is_empty()`, a two-ε signature, a file
holding four quadrature engines, three parallel CDT pipelines, a second surface
enum, or a body-wide accessor in the wrong crate.

The cause is not effort. It is that the protocol is **claims-driven**:
reviewers falsify the claims they are handed, and they do it well. A code-free
module, a 449-line accumulated header, and a duplicate type name across a
façade **assert nothing**, so nothing points a reviewer at them.

**The failure mode to avoid is turning the reviewer brief into a checklist.** Reviewers here answer the questions they are given. Ten crisp
yes/no items will produce ten crisp ticks and no judgement. Every question
in the reviewer brief is phrased to require taste, and its stance exists to make "I'm not
sure, but this looks off" a *complete and welcome* review finding — which the
adversarial-falsification lane, with its high confidence bar, actively
discourages.

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
