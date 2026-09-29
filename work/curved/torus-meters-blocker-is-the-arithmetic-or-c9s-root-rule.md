---
id: torus-meters-blocker-is-the-arithmetic-or-c9s-root-rule
kind: issue
title: CURVED-SPIRIC-DESIGN says an outward-rounded sqrt in the certification scalar retires the torus; Interval has one now, so is the torus blocker the arithmetic or C9's no-root rule?
status: open
opened: 2026-09-29
priority: P3
cost: M
design: true
refs: [ring-3-residue-outside-its-fence]
---


## The question

`docs/CURVED-SPIRIC-DESIGN.md` names the torus meters conversion's
blocker as a missing operation: Q3(ii) (§1, "revise C9 to admit an
outward-rounded `sqrt` in `RingInterval`, which retires the torus
meters conversion") and §4's block note (`ssi/enclose.rs`'s
`implicit_enclosure` refuses `Surface::Torus` because the residual's
`ρ = |w|` is "a square root the C9 ring does not provide",
`ring_interval.rs`'s module doc).

RING-3 (`ring-3-ring-dissolves-into-interval`) dissolved `RingInterval`
into `Interval`, which HAS an outward-rounded `sqrt` (`Real::sqrt` on
the backend). The torus arm still refuses (`enclose.rs`'s
`implicit_enclosure` answers `Interval::refused()` for
`Surface::Torus`), and the doc's premise — the operation is missing —
no longer holds as written. Two readings of what blocks it now:

- **the arithmetic**: the certification doors
  (`geom_core::interval::certification`) reach no `sqrt`, and
  `scripts/gates/certification-doors.sh`'s REAL rule keeps `Real`'s out
  of a certification file, so the missing piece is a root door on
  `Certification`;
- **C9's rule**: certification arithmetic is transcendental-free and
  takes no root by design (`crates/geom-brep/README.md` C9;
  `ssi/certify.rs`'s `composite_form`, "a certified root, which
  certification arithmetic does not take"), so Q3(ii) is a revision of
  C9 itself, whatever the scalar can compute.

Which it is, and so what Q3(ii) asks Ev, is CURVED's to say; the
design doc is not edited until it is (its lines also still name
`RingInterval`, `RingInterval::poison()`, `ring_interval.rs` and
`Bounds`/`Enclosure`, which retire with the answer).

`work/curved/plan.md`'s exit shape already carries "the C9 ring `sqrt`
question is opened as an `[ev]` conversation (on TANG if unasked at
the walk)"; this row is that question restated for the tree after
RING-3, so the conversation asks the right one.

## Found by

SCALAR-HYGIENE (`ring-3-residue-outside-its-fence`'s CURVED bullet),
2026-09-29.
