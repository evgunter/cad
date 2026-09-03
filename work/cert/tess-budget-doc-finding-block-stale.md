---
id: tess-budget-doc-finding-block-stale
kind: issue
title: docs/TESS-BUDGET.md's headline finding block is a hand-transcribed census of a sweep it no longer describes
status: open
opened: 2026-09-03
---


## What

`docs/TESS-BUDGET.md`'s *"The finding"* block (the fenced block under that heading, `:244-255` at `a4eb03a`) is a census of the tour sweep, hand-transcribed into prose, and every figure in it disagrees with the committed baseline it is supposed to describe (`docs/tess-budget-data/tess-budget-baseline.csv`, cut `6e434ffdebe0` 2026-09-01, commit `a4eb03a`). Re-derived from that file, columns `face`/`triangles` and the sizing block's presence:

| the doc says | the committed baseline says |
|---|---|
| 1025 faces | 1306 |
| 1,149,528 triangles | 1,416,410 |
| 64 NURBS faces (6.2% of faces) | 64 (4.9%) |
| carrying 782,104 triangles (68.0% of the mesh) | 164,710 (11.6%) |
| 390,100 grid cells used | 46,019 |
| 95,090 at the cheapest split | 94,154 (`opt_cells`) |
| 154,129 sized per knot-span cell | 44,446 (`span_opt_cells`) |

The NURBS-triangle figure is off by a factor of ~4.7 and the grid-cell figure by ~8.5, and the first is the one the document's whole argument rests on — the claim that a small number of Hessian-sized faces carry most of the mesh. On the committed baseline they carry about an eighth of it.

**Not edited here** because `docs/` is outside this lane's fence, and because the right fix is a judgement call this lane cannot make: whether the block is stale against a re-cut (in which case re-derive it) or whether it describes a *different* sweep — a full deviation sweep at an earlier head, before per-knot-span sizing landed — in which case it needs to say which sweep and when, or stop being a number.

## Finding

**One mechanism, and it is the one Track K's `C15`/`D201` correction just closed one instrument over.** A census over a committed artefact, transcribed into prose, drifts at the rate the artefact is re-cut and nothing catches it. `tools/tess-lint`'s module docs carried the same shape (the face-identity census) and it had been wrong since 2026-09-01 with nothing to fire; the cure applied there was to give the census one executable home (`tools/tess-lint/tests/baseline_census.rs`) that re-derives it from the committed file on every `cargo test`, and to leave pointers everywhere else. The same cure is available here: the baseline is committed, the derivation is four columns of arithmetic, and `tools/tess-lint`'s own report already computes every one of these figures — `main.rs`'s report header prints faces, NURBS faces, their percentage and their triangle share. The document could cite that output rather than restate it.

**The document's own defence does not cover these figures.** `tools/tess-meter`'s module docs say `docs/TESS-BUDGET.md`'s deviation columns come from a `--deviation` run nothing re-takes and instruct a reader to *"read its sizing columns as live and its deviation columns as dated"*. Every figure in the table above is a SIZING figure — faces, triangles, grid cells, `opt_cells`, `span_opt_cells` — so it is the half the document is asserted to keep live that has drifted, and by factors of 4.7 and 8.5. `tools/tess-lint`'s own report header already prints the first four of them from the committed baseline, in one command, and disagrees with the document line for line.

**Confidence:** sure for the arithmetic (re-derived twice, once in Python over the raw CSV and once through `tess_lint::parse`); unsure about which sweep the doc's numbers came from, which is why this is an issue and not a diff.

**Raised by:** the Track K census re-derivation lane, as the sweep for *"other hand-transcribed counts over the committed baseline"*.

## Was

unrowed — found by a sweep, not placed by a track.
