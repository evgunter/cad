---
id: a-section-faces-index-is-its-completion-order-not-its-identity
kind: issue
title: A SectionFace { k } is the k-th section a half completed, so a held section name silently denotes another face whenever the half's section count changes
status: closed
opened: 2026-10-18
priority: P2
cost: M
design: true
pr: 5486
branch: wire/section-face-pieces
closed: 2026-10-20
---


Found by FORK-INPLANE's designers (2026-10-18). Measured on the pentagon
of `crates/editor-core/tests/split_pinch_keeps.rs`: with the plane just
below the apex, the upper half holds `SectionFace { 0 }` only. Just above
it, the upper half holds `SectionFace { 0 }` and `SectionFace { 1 }`. So a
held `SectionFace { 0 }` denotes a different face after a recorded flip,
and nothing diagnoses it. N3's "nothing rebinds itself" is honoured at a
merge (every group is `Merged`), but not by the role's own index.

Question: should a section face be identified by what bounds it (for
example, the operand faces its chords cross), with completion order kept
only as a tiebreak? An N5 diagnosis for "a section index moved" would
need that identity read off the finished body, and no record gives it
today.

Signed (WIRE orchestrator).
