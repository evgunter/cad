---
id: embedgate
kind: program
title: EMBEDGATE — the at-rest gate that asks the embedding question, and what a finished body carries so it is not asked twice
status: ready
opened: 2026-11-22
group: api/assembly
prefix: embedgate/
tag: (EMBEDGATE orchestrator)
paths: [crates/topo/src/finish.rs]
keep_out: [cut from EMBED on 2026-11-22 after Ev asked in chat whether a chunk could be split off (EMBED at 118 against 30) along the seam between how the embedding question is answered and where it is asked - the rows moved by git mv with ids and bodies unchanged, EMBED keeps the readings (crates/topo/src/injective.rs) and EMBEDWALK the walk under them (clearance.rs), the verdict door embedded() is the seam (PR 6885), validate.rs is TOPO's and RESTFRONT's (the facts tier 3 keeps), census.rs is shared with HELDRECORDS, ATREST and RESTREAD, and HELDRECORDS holds a-finished-body-holds-its-contact-records-at-every-door which EMBED's PR 7124 builds - run scripts/work.py territory on your branch and announce the seam in the PR]
---
**The at-rest gate that asks the embedding question, and what a
finished body carries so the question is not asked twice.**

The unit at the top is `the-at-rest-gate-certifies-embedding` (P0). Its
`## Spec` orders lanes A to E; A and B (now) landed under CLEAR and
EMBED (PRs 6885, 6905). What is left is this track's:

- **Lane C, the gate.** Built on `origin/embed/gate`, measured on
  `origin/embed/gate-measure`. It does not land ahead of the readings:
  no gate that refuses valid bodies lands
  (`the-f64-cavity-gate-refuses-thirty-two-valid-shells`).
- **The carry** (`a-finish-carries-the-embedding-facts-of-unchanged-face-pairs`,
  Ev on PR 7101): a finish trusts other finished bodies' answers for
  face pairs an op did not change.
- **An op over an assembly** (`an-op-over-an-assembly-answers-only-for-what-it-made`,
  Ev on PR 7068).
- **Lanes D and E** follow C.

Slate, order and what each waits on: `plan.md`.
