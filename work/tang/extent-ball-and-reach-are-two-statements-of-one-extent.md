---
id: extent-ball-and-reach-are-two-statements-of-one-extent
kind: issue
title: ExtentBall and Reach are two types for the consumed extent
status: open
opened: 2026-10-07
priority: P4
cost: M
---


Found by PR 4118's fifth full review (S3).

## What

`crates/geom-brep/src/extent.rs` carries two types for where a carrier
verdict is consumed:

- `ExtentBall`: a ball. The carrier doors (`topo::boolean::carrier_eq`,
  `rest::pair_extent`) and the tangent-locus witness read it.
- `Reach`: what the section classifiers read their axis rows across. It
  has three variants: `Span`, `Measured`, and `Ball`, which wraps an
  `ExtentBall` for the witness.

Each has its own `foot_on` and `lever_from`. The witness converts at
its door (`Reach::Ball`), and the carrier doors still lever from a
ball's far side. That is the reading PR 4118 found over-long for the
section classifiers' callers on two-sided served rows.

## The shape of a fix

Decide whether the carrier doors and the witness should hand a `Reach`
measured from their consumed points (each face's boundary, as
`rest::pair_extent` already reads it), so that one type states the
extent and the rule in `extent.rs`'s module docs governs every reader.
Each door's ball lever is a verdict-moving change and wants its own
differential.
