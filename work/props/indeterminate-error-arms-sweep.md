---
id: indeterminate-error-arms-sweep
kind: issue
title: The ~40 Indeterminate-carrying error variants the escalation channel makes unnecessary to match on: a deletion sweep
status: review
pr: 2928
branch: props/escalation-channel
opened: 2026-09-05
refs: [k-stats-escalation-channel-and-redo, escalation-channel-misses-op-minted-indeterminates, 1969]
---

## What

`grep -rn 'source: Indeterminate\|cause: Indeterminate\|(Indeterminate)' crates/*/src`
finds roughly 40 error-enum variants across `sweep`, `topo`, `profile`,
`geom-brep` and `editor-core` that carry a funnel escalation out of an
op (`ExtrudeError::ExtrusionEscalated`, `LoftError::StackingEscalated`,
`RevolveError::AxisEscalated`, `BlendError::Escalated`,
`ChartRegionError::Escalated`, `PlaneEqError::Escalated`,
`StructureRefusalKind::Indeterminate`, `NodeErrorKind::Escalated`, …).
PR #1969's escalation channel (`NodeValue::escalations`,
`NodeError::escalations`) makes MATCHING on them unnecessary for the
question "did a predicate escalate, and on what margin": the
subdivision driver reads the log first (`crates/editor-core/src/drive.rs`,
`classify_replay`).

## What this item is

The deletion sweep the k-stats spec explicitly named as a different
unit: which of those variants still carry information a consumer needs
(the op's own context around the escalation — Display text, the
recourse sentence, the site) and which are pure wrappers a consumer
could read off the log instead. Not the two arms in `classify_replay`:
those stay load-bearing until
`escalation-channel-misses-op-minted-indeterminates` lands (the log
does not carry op-minted `Indeterminate`s or the mate solve's).

## Acceptance

A hit list of the variants with a disposition each (kept: names why;
deleted: its consumer now reads the log), a sweep pattern with its
blind spot stated, and no behaviour change in what a consumer can
learn.

## Measured (PR 2928): 82 variants classified, 0 retired

### The pattern, and what it could not match

The item's own recipe —
`grep -rn 'source: Indeterminate\|cause: Indeterminate\|(Indeterminate)' crates/*/src`
— returns 79 lines, of which **38 are variant declarations**: the
"roughly forty". **Its blind spot is the field name and the type
spelling.** It misses every variant whose payload field is called
`diag`, and every one that spells the type `geom_core::Indeterminate`,
which is **44 more variants of exactly the same class** (`BooleanError`,
`ContactRefusal`, `MergeCoplanarError`, `SplitJoinError`,
`EulerOpError`, `SplitReduceError`, `PointInSolidError`,
`PointInLoopError`, `SplitFinishError`, `ComposeError`, the
`NodeErrorKind` contact arms, …). Widened to
`(source|cause|diag): (geom_core::)?Indeterminate` plus the tuple form,
the class is **82 variants across nine crates** — not five: `sweep`,
`topo`, `profile`, `geom-brep`, `editor-core`, and also `geom`,
`geom-core`, plus the crate-internal ones. What the widened pattern
still cannot match: a variant that carries an `Indeterminate` inside
another type (a `Box`, a nested refusal struct, an `Option`), and a
variant whose payload field has some fourth name.

Every one of the 82 is CONSTRUCTED — the smallest count of qualified
`Enum::Variant` references outside its declaration is 2
(`SplitReduceError::CrossingEscalated`). There is no dead variant here,
so there is no retirement on those grounds either.

### The measurement, and why it retires none

**The escalation log is a per-bracket SIDE channel, not a second copy
of the error.** `Bracket::open` has exactly four shipped call sites —
`editor-core/src/eval/mod.rs` (one per node evaluation),
`editor-core/src/eval/parts.rs` (the part-cache shield) and two in
`topo/src/props.rs` — plus `k_stats::detached` in
`editor-core/src/names/discriminate.rs` and `mesh/src/tessellate.rs`.
`record_escalation` drops what it is given when no frame is open, and
`splice` says the same for a detached run. So for **every other caller
of these ops** — `step-export`, `step-import`, `stl`, `verbs`,
`pncad`, `pncad-py`, `viewer`, `demos/tour`, `demos/wild` and any
library consumer — the log is empty and the error enum is the only
channel that carries the escalation at all. Retiring a variant would
not move information from one channel to another; it would delete it
for every unbracketed caller.

That reason alone keeps all 82. Each also has at least one of:

**(a) It carries a SITE the log has no field for (40 of 82).** The log
holds `(predicate, margin, band)` and nothing else — no entity, no
sample index, no op stage. These name one:

`BlendError::Escalated` (site) · `BooleanError::UndeclaredCoincidence`
(pair, relation) · `CarrierEqError::Undeclared` (relation) ·
`CertifyError::Escalated` (check, sample) · `ComposeError::SeamEscalated`
(seam) · `ContactRefusal::Contradicted` (steer) ·
`EulerOpError::SplitParamEscalated` (edge) ·
`ExtrudeError::CosurfaceEscalated`, `::SliverJoin` (loop, vertex),
`::SliverRim` (loop, segment) · `FlushRefusal::PairInBand` (pair) ·
`LoftError::StackingEscalated` (slab) · `NamingError::Escalated`
(predicate) · `NewellError::Escalated` (vertex) ·
`NodeErrorKind::Escalated` (predicate), `::UndeclarableContact` (row),
`::UndeclaredContact` (finding, merged) ·
`PcurveCertifyError::Escalated` (check, sample) ·
`PcurveMintError::Escalated` (half_edge) ·
`PointInSolidError::Escalated` (face) · `PrimitiveRefusal::Escalated`
(predicate) · `ProfileError::Escalated` (site) ·
`RevolveError::CosurfaceEscalated`, `::SliverJoin`, `::SliverRadius`
(loop, vertex), `::SliverAxisClearance`, `::SliverRim` (loop, segment) ·
`SelectRefusal::InBand` (name, predicate), `::PairInBand` (pair,
predicate) · `ShellClassifyError::Escalated` (shell) ·
`SplitFinishError::DescribeEscalated` (edge) ·
`SplitJoinError::Escalated` (face) ·
`SplitReduceError::CrossingEscalated` (edge), `::SliverSector` (vertex,
face), `::SliverVertex` (vertex) ·
`ValidationError::DegenerateTorusEscalated` (face),
`::PlanarBoundaryEscalated` (face, edge), `::PlanarFaceEscalated` (face,
vertex), `::RingContactEscalated` (face, ring), `::SliverDihedral`
(edge).

**(b) Its `Display` composes the op's own sentence and its recourse
(all 82).** The log carries no text. No arm in the class is a bare
`write!(f, "{diag}")` passthrough — checked by scanning for that
spelling; every one prefixes its op and its situation
(`"contfp: {diag}"`, `"section: configuration trilean escalated — an
ill-conditioned operand pair at this tolerance: {diag}"`). Several
crates GATE that: `every_props_error_arm_names_a_recourse`,
`every_chart_region_arm_names_a_recourse`, and the `recourse_roster`
suites in `profile` and `sweep`, which read `predicate_census` and
require a recourse per predicate name.

**(c) It states a DISTINCTION the log cannot express.** `Contradicted`,
`Undeclared`, `DeclarationContradicted`, `UndeclaredCoincidence`,
`UndeclaredContact`, `UndeclarableContact`, `InBand`, `PairInBand`,
`Sliver*` and `Rung` are not all escalations: several are DEFINITE
facts carrying the classifier's diagnostic as evidence, and for those
the log is empty because the funnel never escalated. Collapsing them
onto "read the log" would report a definite contradiction as an
indeterminacy.

The remaining 42 — payload is the `Indeterminate` alone — are kept on
the universal reason plus (b), and, where the name says so, (c):
`ArcTrimRefusal::Escalated`, `BooleanError::DeclarationContradicted`,
`BooleanError::Escalated`, `CarrierEqError::Contradicted`,
`CarrierEqError::Escalated`, `ChartRegionError::Escalated`,
`ContactRefusal::Escalated`, `ContactRefusal::Undeclared`,
`ContainError::Escalated`, `EllipseInvalid::Escalated`,
`ExtrudeError::ExtrusionEscalated`, `FrameError::Escalated`,
`GateRefusal::Escalated`, `IsoRowError::Escalated`,
`MergeCoplanarError::DeclarationContradicted`,
`MergeCoplanarError::Escalated`, `MeterError::Escalated`,
`MustCarryVerdict::InBand`, `NormalAtError::Escalated`,
`OffsetError::Escalated`, `PathError::Escalated`,
`PcurveCertifyError::FittedEscalated`, `PlaneNurbsRefusal::Escalated`,
`PointInLoopError::Escalated`, `ReplaceFaceError::Escalated`,
`RevolveError::AngleEscalated`, `RevolveError::AxisEscalated`,
`RingOuterVerdict::Escalated`, `SectionError::Escalated`,
`SectorFault::Rung`, `SegIssue::Escalated`, `ShellError::Escalated`,
`SpiricInvalid::Escalated`, `SplitJoinError::OrderEscalated`,
`SsiError::Escalated`, `StructureRefusalKind::Indeterminate`,
`TangentLocusError::Escalated`, `TransportError::Escalated`,
`TrimRefusal::Escalated`, `TubeError::Escalated`,
`UnitVec3Error::Escalated`, `ValidationError::CensusEscalated`.

### `classify_replay`'s two arms stay, and the reason changed

The ruling was not to touch them until the channel carried the op-minted
family. It now does — but they stay, for a reason the item did not have:
the log speaks only where a bracket was open, and **two shipped paths
escalate where none is** — the whole-document mate solve
(`work/msolve/mate-lane-escalations-reach-no-nodes-log.md`) and a rayon
map whose units decide without a frame of their own
(`work/perf/rayon-maps-outside-props-lose-the-funnels-recordings.md`).
Deleting the arms turns those into the `_ => Bisect` fall-through and
silently drops the terminal-sliver test. `drive.rs`'s comment says this
now instead of naming the eight sites.

### What this item was right about, restated

What PR 1969's channel bought is that `classify_replay` does not have to
MATCH on those 82 variants to ask "did a predicate escalate, and on what
margin". That is a fact about this one consumer, and it is worth what it
cost. It is not a fact about the variants: they are the ops' own typed
refusals, and this measurement finds no consumer that could read one off
the log instead.
