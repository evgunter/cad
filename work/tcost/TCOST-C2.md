---
id: TCOST-C2
kind: unit
title: rustdoc gate: excluded roots and pass 3 to the nightly; workspace pass scoped to the closure on PRs
status: dispatched
opened: 2026-09-03
branch: tcost/c2-rustdoc-roots-nightly
---

CI-posture unit (Ev's ask). The rustdoc gate's six excluded roots
(`demos/tour` and `demos/wild` each recompile the whole kernel to
document themselves) and its third pass move to the nightly; the PR
job keeps the workspace pass, scoped to the change filter's closure the
way the build is. A broken intra-doc link persists in the tree, so this
is the persistence case. Billed-minute delta recorded from the PR's own
run against F6's addendum.
