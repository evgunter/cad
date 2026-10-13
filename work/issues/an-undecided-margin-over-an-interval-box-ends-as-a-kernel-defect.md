---
id: an-undecided-margin-over-an-interval-box-ends-as-a-kernel-defect
kind: issue
title: An undecided margin over an interval box ends 'this is a kernel defect; report it': recourse::Reading has no box-replay reading
status: open
opened: 2026-10-12
---

## What

`CertifyError::Escalated` is how certification reports a margin that
landed in its band undecided. Its ending is read as
`RefusedArm::Undecided` at a `Reading`, and at `Reading::Build` it
ends: "There is no way through: this is a kernel defect; report it".
The code:
- `crates/geom-brep/src/certify.rs`: `CertifyError::Escalated`, and
  the arm mapping it to `RefusedArm::Undecided`;
- `crates/geom-brep/src/recourse.rs`: `Reading`, `defect_ending`,
  `Unsized::recourse`;
- `crates/geom-core/src/predicate.rs`: the ending text.

That sentence is true at `f64`, at a point. It is false over an
interval leaf. There, an undecided margin is the E6 driver's cue to
bisect, and when the leaves run out the driver prices what it could not
reach as `Budget`. `Reading` has only `Build` and `AtRest`, so nothing
can say "undecided over this box; a smaller box, or the symbolic tier,
may decide it".

The driver itself never reads that ending: the escalation-log read
answers first (`drive.rs`, `classify_replay`). A one-leaf replay does
print it, though. SYM-17's Phase 1 quoted exactly this sentence:
`demos/tour/src/chaintol.rs`'s probe row, at `carrier_endpoint_start`
on the chain's first transform, with the rigidity checks skipped. It
then judged the refusal "worse than today's", and that judgement was
about the sentence, not the kernel.

Under the rigid-motion witness SYM-17's `[ev]` question recommends,
this is the sentence the plain lane's one-leaf chain row would print.

## Class

The class is every ending rendered from a `Reading` on an undecided
(in-band) arm, reached by a box replay. Sweep the
`Unsized::recourse(RefusedArm::Undecided(..), reading)` callers and the
`defect_ending` callers, and record which can be reached from an
`Interval` or `Sym<Interval>` leaf.

## Found by

Both SYM-17 designers, independently (2026-10-12). Neither ran
anything; the citations above were re-read on `main` by the
orchestrator.

## Measured under the witness (SYM-17 Phase 2, PR #4197)

The prediction above holds. With `topo::transform_rigid` taking a
`RigidMotion`, the chain's plain-lane one-leaf row
(`demos/tour/src/chaintol.rs`, σ = 0.01 rad, default ε) refuses at
the first transform with this ending, at one link and at four: the
start-endpoint residual of `EdgeKey(1v1)` encloses `[0, 8.9988e-5]`
(`carrier_endpoint_start`), "There is no way through: this is a kernel
defect; report it". The cause is SHELF's
`transform-rigid-recertifies-images-enclosed-apart`, not a defect.
