---
id: split-band-on-at-a-concave-edge-may-mint-a-pinch-from-near-coincidence
kind: issue
title: Split's ON verdicts use Band::linear(tol), so a plane within tol of a concave edge may produce a pinch half — near-coincidence becoming contact
status: open
opened: 2026-10-02
priority: P3
cost: M
refs: [split-halves-have-no-contact-records-so-no-pseudomanifold-self-check]
---


## What

DESIGN's tier 3' (i) says near-coincidence never silently becomes contact. A plane within tolerance of a concave edge, but not on it, reads the edge's vertices ON and can mint a pinch half whose touching is an artefact of the band. Unverified: whether an in-band ON verdict escalates typed (Q1) or decides. A row: a plane offset by tol/2 from a notch's tip line.

Found by the TQUERY designer pair weighing `split-halves-have-no-contact-records-so-no-pseudomanifold-self-check` (2026-10-02); read from the tree, NOT run — measure first.
