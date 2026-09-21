---
id: a-dispute-names-no-predicate-on-the-receipt
kind: issue
title: a theorems_disputed count names no predicate: a receipt reader cannot tell which decision disputed
status: open
opened: 2026-09-21
---


**Disclosed by SYM-11 and found again by both of its reviews** (R2 m4,
R1's unscheduled note), on PR #3028.

`SymCounts::theorems_disputed` counts decisions, not sites. A run at an
inexact lane scalar that reports `theorems_disputed: 3` says three
decisions had a form claiming zero under a definite numeric sign, and
says nothing about WHICH three: not the predicate that asked, not the
margin, not the node. Every other loud column on the receipt has the
same shape, but this one is the only one whose whole purpose is to send
a reader looking — a dispute is a claim that the value channel and the
form disagree, and the first question it raises is "about what".

**Where the claim is made today.** `crates/geom-core/src/sym.rs`, the
column's own doc (which says so) and `count_theorem_disputed`; the
charge in `impl<T: Decide> Decide for Sym<T>::sign_within`;
`crates/editor-core/src/drive.rs`'s `serialize` and `render`.

**What it would take.** The shape report already carries the predicate
per decision (`sym::report`'s `DecisionShape::predicate`), and a
disputed decision records `ShapeOutcome::Definite(sign)` there — the
same row any other definite decision records, so the report cannot tell
them apart either. Naming the dispute means a NEW `ShapeOutcome` row
(say `DisputedDefinite(sign)`), which is a change to the report's
vocabulary: `sym/discharge_pins.rs`'s seam-2 roster grows a member and
has to classify it as not-from-a-discharge, and every consumer of the
report learns the new row. SYM-11 declined that as out of its scope (no
change to the K token vocabulary or the receipt's wire format) and
disclosed it instead.

**The ergonomics both reviews measured.** Driving the far-placement
rows, a reader who sees a count has to re-run the document under the
shape report and diff the definite rows against a clean run to find the
site — which is the work the column exists to save.

**Not urgent, and say why**: no shipped lane replays at an inexact
`Sym` (the driver replays at `Sym<Interval>`, where the column is zero
by construction), so today this costs a unit-test author minutes rather
than a user anything. It becomes real the day a fixture-scale row wants
to attribute disputes across a document.
