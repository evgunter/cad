---
id: TCOST-1
kind: unit
title: gate a suite to the code it tests, and re-take the set nightly
status: dispatched
pr: 1612
branch: tcost/1-per-file-gate
opened: 2026-09-02
refs: [1613]
---

Charter lever 3, the per-file gate: an in-file marker names the source paths
a suite covers; `scripts/ci-filter.py` reads the markers and the diff and
emits a nextest filterset that skips a gated suite whose named paths and own
file are untouched; both CI halves consume it; it fails OPEN on tier `all`,
on any unresolvable marker and on any parse error; the nightly runs the
gated set ungated. First users are the existing fuzz rows and randomized
sweeps. Self-merged with a full writeup, reviewed retroactively (Ev's
ruling, 2026-09-02). Spec: `docs/TCOST-1-SPEC.md`. PR 1613 is its
do-not-merge evidence PR (the gate demonstration).

`work/tcost/log.md`, "Opening state (2026-09-02)" (the gate mechanism's
shape); the PR is the dispatch record.
