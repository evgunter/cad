---
id: the-census-cannot-record-a-curved-seat-between-two-solids
kind: issue
title: The at-rest census cannot record a curved seat between two solids: a shaft in a bore reads CensusUndecidable
status: open
opened: 2026-11-10
priority: P3
cost: H
refs: [curved-copies-filling-one-volume-read-as-undecidable]
---


Found by SEAL's lane on `work/seal/a-cut-by-a-curved-seat-keeps-its-two-faces-held-apart-by-the-record.md` (2026-11-10), from a run probe.

Two solids seated along a cylinder: a shaft `cylinder_of_arcs_at(2, 0.4, (1, 1), …)` in a plate `bored_block_of_arcs(2, 2.0, 0.5, 0.4)` bored at r = 0.4. `record_at_rest` returns `CensusUndecidable` ("the kernel cannot yet tell whether curved faces of two parts touch") for every bore-wall/shaft-wall pair, and for the shaft wall against the plate's top and bottom. A planar peg in a square hole records four face–face records.

`curved-copies-filling-one-volume-read-as-undecidable` covers copies of one revolved body filling one volume. This row is two different solids at rest on one cylindrical carrier, opposed senses.

**When this lands,** SEAL's curved-seat row goes live: a cut by such a seat builds a silently wrong body until the held-apart gate reads curved area records. Settle that row's design first.
