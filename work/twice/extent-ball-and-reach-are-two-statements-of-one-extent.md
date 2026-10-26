---
id: extent-ball-and-reach-are-two-statements-of-one-extent
kind: issue
title: ExtentBall and Reach are two types for the consumed extent
status: open
opened: 2026-10-07
priority: P4
cost: M
design: true
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

## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Design signal.** **Cluster C, the scale a margin is charged at.** Ev (2026-10-26): every point of the representation must be within ε of the real shape, so a decision's charge should be derived from that one contract, not chosen per site. There are three extent types: `ExtentBall`; `geom_brep::Reach` (Span/Measured/Ball); and topo's `Reach`. Which extent a door levers from changes verdicts. No canonical-places row names "the extent a carrier verdict is levered at".
