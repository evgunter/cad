---
id: escalations-forwarded-whole-are-untriaged-for-a-declarations-object
kind: issue
title: chrome: ~60 error variants forward Indeterminate whole, and which of them offer a declaration their door has no object for is untriaged
status: closed
opened: 2026-09-28
priority: P2
cost: M
closed: 2026-09-29
---

(ENCL implementer, from the §5 sweep of PR 3347; the review asked that
this residue be filed rather than left in the PR body.)

## What

`geom_core::Indeterminate`'s own `Display` ends in
`COINCIDENCE_RECOURSE`, unlabelled: "declare the coincidence, move the
geometry, or lower the tolerance". A field-type grep found about 60
error-enum fields that carry an `Indeterminate`, and many of them
render it whole:

    grep -rnE '^\s+(source|cause|diag|inner|escalation): (geom_core::)?(predicate::)?Indeterminate,?$' crates --include=*.rs

The hits, at 7ab8ead, are spread across `sweep` (revolve 7, extrude 4,
loft, tube, blend), `topo` (validate 6, splitting 5, contact 3,
boolean 5, merge_faces 2, chord_join 2 and others), `editor-core`
(eval 4, names 3), `geom-brep` (pcurve_cache 2, certify, props, offset,
nurbs_iso, newell, offset_meters), `profile` (validate, path) and
`geom` (curves/compose).

Each one that forwards the whole `Display` hands the viewer two
problems:

- an unlabelled repair, which `test_utils::refusal::recourse_markers`
  counts as zero (see item 8 of
  `the-refusal-shape-guard-has-blind-spots`);
- a "declare" lever, which is advice only where the door takes a
  declaration.

Nobody has checked the sites one by one.

## Already done or filed

- The routed blend arms render the payload view with their own
  recourse.
- `PathError`'s non-junction arms render the payload view with their
  own recourse.
- `MeterError::Escalated` does the same (PR 3347).
- Four geom-brep carriers whose escalations `topo::validate` ends
  through their decisions' endings (`CertifyError::ending`,
  `PcurveCertifyError::ending`), or with no lever where the decision is
  not carried (`PropsError::Escalated`), are filed on their owners'
  slates:
  - `work/encl/certify-escalation-renders-the-coincidence-menu-unlabelled.md`
  - `work/iso/plane-nurbs-escalation-renders-the-coincidence-menu-unlabelled.md`
  - `work/pcert/pcurve-certify-escalation-renders-the-coincidence-menu-unlabelled.md`
  - `work/props/props-escalation-renders-the-coincidence-menu-unlabelled.md`
- Extrude and revolve offer the menu on purpose, even on their definite
  arms, so for those two the question is their owners'.

## The triage

For each remaining hit, say which door raises it and whether that door
takes a declaration. Where it does not, the site wants the
`payload()` + routed `Recourse:` shape, and a row filed on the owner's
slate.

**The field-type grep cannot see a tuple variant (ENCL, 2026-09-28).**
It matches `cause: Indeterminate`, not `Escalated(Indeterminate)`, and
about twenty error enums carry their escalation that way:

    grep -rnE '^\s+[A-Z][A-Za-z]*\((geom_core::)?(predicate::)?Indeterminate\),?$' crates --include=*.rs

At 793b5c4 (outside `tests/`) it hits `topo` (validate 3, carrier_eq
2, splitting/containment, sector_shape, replace_face, face_normal,
chart_region, boolean rest/join/contain), `profile` (sugar 2,
structure, seg, path/arc_fillet), `geom` (curves 2), `geom-core`
(linalg/unit_vec) and `geom-brep` (ssi, intersect, edge_nurbs,
dihedral). `ssi::SsiError::Escalated` and
`intersect`'s `Escalated(diag)` render `{diag}` whole under a comment
that says so on purpose (S6); `edge_nurbs`'s is the
`work/iso/plane-nurbs-…` row above. The rest are for the triage.

## The shape guard counts the forwarded menu (CHROME, 2026-09-29)

`test_utils::refusal::recourse_markers` now counts the shared
unlabelled repairs (`BARE_RECOURSES`: `COINCIDENCE_RECOURSE` and the
two no-declaration forms), so an `Indeterminate` forwarded whole counts
as one recourse, and a second recourse beside it is red. The first
problem this row names is closed by that. The second, a "declare"
lever at a door that takes no declaration, is untouched, and the
triage stands.

## Closed (CHROME triage, 2026-09-29)

At `ec6bf25`, the two greps hit 88 sites outside `tests/`: 62 field-shaped
and 26 tuple-shaped. Every one is classed below. A door's
declaration is `BooleanDeclarations` (`crates/topo/src/boolean/mod.rs`
near :378). In the recipe it is the `declare` input of `Node::Boolean`
and `Node::Union` (`crates/editor-core/src/node.rs` near :2036 and
:2144), and no other node carries one. A profile's declaration is its
tangent joints (`ProfileLoop::tangent_joints`). The viewer draws
`NodeErrorKind`'s `Display` (`crates/viewer/src/tree.rs`).

Classes: **(a)** the door takes a declaration, so the menu is right;
**(b)** it does not, so the site owes `payload()`, a subject and a
routed `Recourse:`; **(c)** it is never shown, because it is caught and
re-wrapped (the wrapper is cited); **(d)** it is done or filed.

Counts: **a 12, b 26, c 26, d 24.** None of the (b) sites is on
CHROME's ground (`crates/viewer`). They are filed on seven programs'
slates and on `work/issues/`:

- `work/carve/carve-refusals-short-of-the-shape-guard.md`: 13
- `work/shell/shell-refusals-short-of-the-shape-guard.md`: 3
- `work/wire/wire-refusals-short-of-the-shape-guard.md`: 3
- `work/paths/paths-refusals-short-of-the-shape-guard.md`: 1, plus a
  question on `SHARED_CLAUSE_ONLY`
- `work/issues/unowned-viewer-refusals-short-of-the-shape-guard.md`: 2
- `work/topo/topo-escalations-offer-a-declaration-the-door-cannot-take.md`: 1
- `work/trim/trim-escalations-offer-a-declaration-the-door-cannot-take.md`: 1
- `work/curved/curved-escalations-offer-a-declaration-the-door-cannot-take.md`: 1
- `work/linalg/linalg-escalations-offer-a-declaration-the-door-cannot-take.md`: 1,
  plus `FrameError::Degenerate`, which the second pass found

### The table

editor-core
- `eval/measure.rs:298` `PrimitiveRefusal::Escalated`: c (`eval/wire.rs` near :2438 → `NodeErrorKind::Escalated`)
- `eval/mod.rs:1420` `NodeErrorKind::Escalated`: b, WIRE (direction norms, revolve full turn, measure parallelism; none declares)
- `eval/mod.rs:1580` `NodeErrorKind::UndeclaredContact`: a (Boolean/Union `declare`; payload with its own two-armed recourse)
- `eval/mod.rs:1604` `NodeErrorKind::UndeclarableContact`: d (payload with a no-declare recourse)
- `names/emit.rs:360` `NamingError::Escalated`: b, WIRE/EMIT (no declaration names a naming discriminator)
- `names/geompred.rs:213` `SelectRefusal::InBand`: b, WIRE (reached through `pncad::select` and Python, not the viewer)
- `names/geompred.rs:277` `SelectRefusal::PairInBand`: a (the flush pair is declarable on the Boolean's `declare` input)

geom-brep
- `certify.rs:424` `CertifyError::Escalated`: d (`work/encl/certify-escalation-renders-the-coincidence-menu-unlabelled.md`)
- `dihedral.rs:500` `MustCarryVerdict::InBand`: c (`sweep/src/extrude.rs` near :1044, `blend/surgery.rs` near :4158, `revolve/upgrade.rs` near :183)
- `edge_nurbs.rs:176` `PlaneNurbsRefusal::TransversalityEscalated`: d (`work/iso/plane-nurbs-escalation-renders-the-coincidence-menu-unlabelled.md`)
- `edge_nurbs.rs:180` `PlaneNurbsRefusal::Escalated`: d (same row)
- `intersect.rs:636` `SectionError::Escalated`: c (`topo/src/boolean/join.rs` → `BooleanError::Escalated`; `chord_join.rs` near :751 → `SplitJoinError::Escalated`; `replace_face.rs` near :1926 → `ReplaceFaceError::Escalated`)
- `newell.rs:95` `NewellError::Escalated`: b, unowned (`work/issues/`)
- `nurbs_iso.rs:239` `IsoRowError::Escalated`: b, TRIM (shell op)
- `offset.rs:273` `OffsetError::Escalated`: b, SHELL/OFFSET (shell op)
- `offset_meters.rs:294` `MeterError::Escalated`: d (PR 3347)
- `pcurve_cache.rs:871` `PcurveCertifyError::FittedEscalated`: d (`work/pcert/pcurve-certify-escalation-renders-the-coincidence-menu-unlabelled.md`)
- `pcurve_cache.rs:906` `PcurveCertifyError::Escalated`: d (same row)
- `props/mod.rs:490` `PropsError::Escalated`: d (`work/props/props-escalation-renders-the-coincidence-menu-unlabelled.md`)
- `ssi.rs:394` `SsiError::Escalated`: c (`edge_nurbs.rs` near :685, `pcurve_cache.rs` near :1450)

geom-core
- `linalg/unit_vec.rs:143` `UnitVec3Error::Escalated`: b, LINALG (the viewer's route re-wraps it at `eval::wire::refusal`, but Python's `FrameError` shows it whole through `OrthoFrameError`)

geom
- `curves.rs:265` `EllipseInvalid::Escalated`: b, CURVED (split, via `SplitJoinError::Section`)
- `curves.rs:322` `SpiricInvalid::Escalated`: c (`topo/src/offset_axial.rs` near :2113 → `ReplaceFaceError::Escalated`)
- `curves/compose.rs:174` `ComposeError::SeamEscalated`: c (no product crate calls `compose_chain`)

profile
- `path.rs:1332` `PathError::Escalated`: a (junction arm: `.tangent()` is the declaration; the other routed arms render payload with their own recourse; the fall-through question is on PATHS)
- `path/arc_fillet.rs:204` `GateRefusal::Escalated`: c (`arc_fillet.rs` near :701 → `StructureRefusal`, near :723 → `PathError::Escalated`)
- `seg.rs:102` `SegIssue::Escalated`: c (`validate.rs` near :1908, `path.rs` near :3224)
- `structure.rs:685` `StructureRefusalKind::Indeterminate`: b, PATHS
- `sugar.rs:204` `TrimRefusal::Escalated`: c (`path.rs` near :3099)
- `sugar.rs:443` `ArcTrimRefusal::Escalated`: c (`arc_fillet.rs` near :441, :763)
- `validate.rs:995` `ProfileError::Escalated`: a (tangent joints; the `SHARED_CLAUSE_ONLY` question is on PATHS)

sweep
- `blend/mod.rs:1059` `BlendError::Escalated`: d (PR 3457)
- `extrude.rs:199, :215, :227, :244` (`ExtrusionEscalated`, `CosurfaceEscalated`, `SliverJoin`, `SliverRim`): b, CARVE
- `loft.rs:179` `LoftError::StackingEscalated`: b, CARVE
- `revolve/mod.rs:374, :385, :404, :424, :491, :502, :513` (`AxisEscalated`, `AngleEscalated`, `SliverRadius`, `SliverAxisClearance`, `CosurfaceEscalated`, `SliverJoin`, `SliverRim`): b, CARVE
- `revolve/tube.rs:149` `TubeError::Escalated`: b, CARVE

topo
- `boolean/carrier_eq.rs:82, :95, :103` (`Escalated`, `Undeclared`, `Contradicted`): c (`boolean/mod.rs` near :2250–:2340, `contact_verify.rs` near :184–:189, `flush.rs` near :263–:291)
- `boolean/contain.rs:57` `ContainError::Escalated`: c (`census.rs` near :1397; `validate.rs` `classify_contain`)
- `boolean/join.rs:729` `FrameError::Escalated`: c (`join.rs` near :707 → `BooleanError::Escalated`)
- `boolean/mod.rs:843` `BooleanError::Escalated`: a
- `boolean/mod.rs:857` `BooleanError::UndeclaredCoincidence`: a
- `boolean/mod.rs:873` `BooleanError::DeclarationContradicted`: a (but its `{diag}` is a definite verdict carried as an `INVALID` margin, rendered whole beside the arm's own recourse: `work/topo/declaration-contradicted-renders-an-invalid-margin-and-the-declare-menu.md`)
- `boolean/rest.rs:685` `TangentLocusError::Escalated`: c (`boolean/mod.rs` near :2371, `insert.rs` near :298, `sectors.rs` near :548)
- `boolean/solid_contain.rs:167` `PointInSolidError::Escalated`: a (`BooleanError::Containment`)
- `chart_region.rs:302` `ChartRegionError::Escalated`: c (`census.rs` near :2392, :5688 → `ValidationError::CensusEscalated`)
- `chord_join.rs:184` `SplitJoinError::OrderEscalated`: d (payload and `NO_DECLARATION_RECOURSE`; the Boolean's wrapper adds the declaration back)
- `chord_join.rs:191` `SplitJoinError::Escalated`: d (same)
- `contact.rs:206, :217, :223` (`Contradicted`, `Escalated`, `Undeclared`): d (payload and `CONTACT_RECOURSE` or the contradiction recourse)
- `euler.rs:852` `EulerOpError::SplitParamEscalated`: b, TOPO (split, blend)
- `face_normal.rs:112` `NormalAtError::Escalated`: c (`boolean/vtxfac.rs` near :145)
- `flush.rs:195` `FlushRefusal::PairInBand`: c (no product crate calls `topo::flush::find_flush_candidates`; editor-core's detector goes through `pair_finding` → `SelectRefusal::PairInBand`)
- `merge_faces.rs:419` `MergeCoplanarError::DeclarationContradicted`: a (verifies declared pairs at the Boolean's merge; same defect, same row)
- `merge_faces.rs:471` `MergeCoplanarError::Escalated`: a (same)
- `pcurves.rs:415` `PcurveMintError::Escalated`: b, unowned (`work/issues/`)
- `pcurves.rs:2489` `PinMiss::Escalated`: c (`pcurves.rs` near :2450 → `PcurveMintError::Escalated`)
- `props.rs:1937` `ShellClassifyError::Escalated`: d (PR 3457: payload and `ending`)
- `replace_face.rs:525` `ReplaceFaceError::Escalated`: b, SHELL
- `replace_face.rs:787` `TransportError::Escalated`: c (`replace_face.rs` near :1806)
- `sector_shape.rs:207` `SectorFault::Rung`: c (`boolean/sectors.rs` near :235, `splitting/neighborhood.rs` near :319)
- `shell.rs:520` `ShellError::Escalated`: b, SHELL
- `splitting/containment.rs:134` `PointInLoopError::Escalated`: a (whole only through `PointInSolidError::Loop` at the Boolean; `contain.rs` near :80 and `SplitJoinError::RingHoming` re-wrap it)
- `splitting/containment.rs:489` `ConicArcError::Escalated`: c (`containment.rs` near :995)
- `splitting/finish.rs:181` `SplitFinishError::DescribeEscalated`: d
- `splitting/mod.rs:214, :244, :254` (`CrossingEscalated`, `SliverVertex`, `SliverSector`): d (payload and `SPLIT_COINCIDENCE_RECOURSE`)
- `validate.rs:861, :1008, :1033, :1050, :1360` (`DegenerateTorusEscalated`, `PlanarFaceEscalated`, `PlanarBoundaryEscalated`, `SliverDihedral`, `RingContactEscalated`): d (each decision's own ending)
- `validate.rs:1500` `ValidationError::CensusEscalated`: a (`too_close`: a census contact is declarable)
- `validate.rs:6340, :6809, :6819` (`RingOuterVerdict::Escalated`, `EdgePair::Unsure`, `Window::Unsure`): c (`validate.rs` near :6298 → `RingContactEscalated`; `shell.rs` near :1594 → `ShellError::Escalated`)

### What the greps cannot see

The two patterns match a field or tuple of exactly `Indeterminate`.
They miss other field names, `Box<>` and `Option<>` wrappers, and
multi-field tuples. A second pass over any field or variant whose type
mentions `Indeterminate` found five more:

- `editor-core` `MateFault::Indeterminate` (`mate.rs` near :793):
  payload only, with no menu. That it states no recourse at all is the
  shape guard's business.
- `mate/coset.rs` `FoldStop::Indeterminate`: c (`mate/solve.rs` near
  :839).
- `topo` `SplitFinishError::SectionWindingUndecided` (`finish.rs` near
  :193): d.
- `splitting/classify.rs` `Roots`: internal.
- `geom_core::FrameError::Degenerate` (`linalg/frame.rs` near :277):
  b, filed on LINALG.

The pass still cannot see a type that holds an `Indeterminate` behind
another error type's field (for example `error: UnitVec3Error`). Those
were followed by hand from each hit's type (`OrthoFrameError`,
`DirectionRefusal`).

The classing is by door. Where a door takes a declaration but the
decision has no declarable object (Boolean crossing insertion, profile
names such as `loop_orientation`), the row that lists the site says so.
