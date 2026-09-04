---
id: band-refusal-still-badges-every-row
kind: issue
title: MateFault::Band still badges every row in the cluster — the filed defect, surviving in one arm
status: open
opened: 2026-09-04
refs: [1769, 1463]
---

Found by CHROME's style lane on PR 1769, judging that PR's own
disclosed carve-out.

PR 1769 sends the eye to the offending mate for every `MateFault` arm
that names one. `MateFault::Band` names none
(`crates/editor-core/src/mate.rs:414-418`), so `blamed_mates` returns
empty for it (`crates/viewer/src/tree.rs:294`) and every row it reached
keeps its own `Failed`.

The PR defends that in three sentences: no band, no decisions, no mate
more at fault than another. **That is a good reason not to PICK a
mate. It is not a reason the filed symptom is acceptable.** A `Band`
refusal is a run-tolerance failure, so it fans out across the cluster
exactly like the arms that were fixed — which means for that one arm a
user still meets N identical FAILED badges with the eye sent nowhere,
which is issue 1463 verbatim.

Nothing is scheduled and no test covers the `Band` shape, so the
carve-out currently reads as closed rather than as the remaining half.

**What closing it probably looks like**, offered as a starting point
rather than a design: a row reached by a cause that names no single
node still knows it is one of many, so the honest badge names the
CLUSTER rather than the row — "one of N in a refused cluster" — which
sends the eye somewhere true without inventing a culprit. That is a
different shape from `Poisoned { through }` and may want its own
status, which is why it is filed rather than folded into 1769.

Signed: (CHROME orchestrator)
