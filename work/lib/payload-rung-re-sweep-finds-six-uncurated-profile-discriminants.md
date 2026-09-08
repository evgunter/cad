---
id: payload-rung-re-sweep-finds-six-uncurated-profile-discriminants
kind: issue
title: six uncurated discriminants of prelude-carried profile types, from LIB-CUR6's re-sweep
status: open
opened: 2026-09-08
refs: [LIB-CUR6]
---


LIB-CUR6 settled the five rows LIB-CUR5's re-sweep filed and re-swept
the class at its own merge base, as the discipline requires. The
re-sweep found the banked set empty and six hits that are NEW — none
of them named by CUR3's table, CUR4's, CUR5's, or the five rows CUR6
closed. All six are in `profile`, and CUR5's run reported none of
them.

## The pattern, and what it could not match

For every name the three curated façade lists (`document.rs`,
`select.rs`, `prelude.rs`) introduce that resolves to a `pub enum` or
`pub struct` declared in the façade's own path-dependency set, take
every type identifier appearing in the declaration's body and report
those declared in that same set and on no curated list. Crate-aware,
so `viewer::blend::BlendError` cannot shadow `sweep::blend::BlendError`
(CUR4's methodology note). **402 curated names, 812 declared types,
137 raw hits**; narrowed to the CUR3/CUR4 shape — an ENUM (a
discriminant a caller branches on) that is not itself an error type
and is declared in the same crate as its carrier — leaves **19 rows
over 18 names**, of which twelve rows are already argued or are false
positives (below).

**This run's numbers do not reconcile with CUR5's** (525 curated, 796
declared, 115 raw, 22 narrowed) and its narrowed SET differs in both
directions. The pattern is described in prose and re-implemented each
time it is run, so two implementations of one description disagree;
that is a methodology finding of its own and is recorded here rather
than reconciled. What is checked is that every row below resolves to a
real declaration at a real `file:line`.

Blind spots, restated with the two this run closes and the one it
adds:

- (a) struct payloads — CLOSED one rung by CUR4 and by this scan,
  which indexes `pub struct` as well.
- (b) a payload named only through a type ALIAS or an associated type.
  Live instance, unchanged from CUR5: `CarrierRelation` reads as an
  uncurated payload of `FlushEvidence` and is the SAME TYPE the
  prelude carries as `PlaneRelation`
  (`crates/topo/src/boolean/plane_eq.rs:58`).
- (c) refusals the façade DECLARES rather than re-exports; their
  payloads never enter the compared set.
- (d) a generic parameter instantiated to an uncurated type.
- (e) macro-minted types — NARROWED here. A `pub enum` written
  literally inside a `macro_rules!` body IS indexed by a
  declaration-level scan, and that is how `Step` below was found
  (`crates/profile/src/path/program.rs:399`). What (e) still misses is
  a type whose NAME is minted by a macro — every `slotmap` key in a
  payload position, which no `pub enum` / `pub struct` line spells.
- (f) private fields — CLOSED here. A `pub struct`'s body is read
  through a bare-`pub` field filter (`pub(crate)` is not public), which
  removes CUR5's four false positives — `Provenance`, `NullFacePair`
  and `CurveGeom` against `Body`'s arenas, `UnitQuantity` against
  `UnitDef` — and `Entry` against `NameTable`, whose two fields are
  private too. `ParamValue` survives the filter: `ParamEnv::bindings`
  really is `pub`, and the row below records it where it was already
  argued.
- (g) NEW — the scan reads every identifier in an ENUM's body, VARIANT
  NAMES INCLUDED, so a variant whose name collides with a type
  declared in the same crate reads as a payload it is not. CLOSED here
  by dropping each arm's leading identifier before the scan; before
  that filter this run reported three: `Certified`
  (`CheckKind::Certified` against `editor-core`'s `measure::Certified`),
  `Unavailable` (`Subject::Unavailable` against `stackup::Unavailable`)
  and `MeasureUnavailable` (`UnevaluatedReason::MeasureUnavailable`
  against `analysis::MeasureUnavailable`).
- (h) NEW, OPEN — the scan is crate-aware and not MODULE-aware, so two
  types with one name in one crate are indistinguishable to it. No
  false positive this run traces to it; it is stated because (g)'s
  three instances are the same failure one level down and the fix for
  (g) does not touch it.

## The hit list

| hit | carrier | disposition |
| --- | --- | --- |
| `ContactKind` | `ProfileError::SelfIntersects` (`crates/profile/src/validate.rs:480`, carrier at `:448`) | **OPEN.** `Crossing` / `Overlap` / `Touch`: three different self-intersections with three different repairs, off a prelude-carried refusal. |
| `EscalationSite` | `ProfileError::Escalated` (`crates/profile/src/validate.rs:564`) | **OPEN.** Which stage escalated — one segment, a segment PAIR, a loop, or the fillet — and the recourse differs by stage. Carries `SegmentRef`, itself uncurated, so this one has a rung under it. |
| `FilletLeg` | `CornerReason::AnchorOutsideTrimmedExtent` (`crates/profile/src/path.rs:605`) and `PathError` (`:1007`) | **OPEN.** Incoming leg or outgoing: which side of the corner the fillet did not fit, which is the one thing a caller shortens. |
| `FilletLegCarrier` | `CornerReason::AnchorOutsideTrimmedExtent` (`crates/profile/src/path.rs:611`) | **OPEN.** Line or arc, which decides whether the reported setback is a linear distance or an arc length — the units of the number beside it. |
| `NoCornerReason` | `CornerReason::NoTangentCircle` (`crates/profile/src/path.rs:596`) | **OPEN, and the sharpest.** The prelude already carries `PathNoCornerReason`, a DIFFERENT type with the same shape one door over, so the surface carries one of a pair and not the other. |
| `Step` | `ClosedLoop::program` (`crates/profile/src/path/program.rs:1739`; declared at `:399`) | **OPEN.** The recorded authoring program a closing verb hands back, whose element type is a macro-declared `pub enum` this list does not name — so a caller can hold a `ClosedLoop` and not read the program it recorded. |
| `Attr`, `AttrKind`, `MetaValue` | `DocEdit`, `EditError` | the appearance and metadata families in `NOT_CARRIED`. |
| `BandField` | `BandError::InvalidValue` | argued non-carriage, CUR4, in `prelude.rs` with its falsifier. |
| `MarginDiag` | `Indeterminate::margin` | argued non-carriage, LIB-CUR5, in `prelude.rs` with its falsifier. |
| `MappedCurve` | `EdgeDescription::Scaffold` | argued non-carriage, LIB-CUR6, in `prelude.rs` with its falsifier. |
| `Diagnosis`, `RecipeEditRef` | `ResolveError` | deliberately interior at LIB-CUR5 (LB17): the telemetry half. |
| `Qualifier` | `RoleSeg` | naming interior; `RoleSeg` is itself the inside of a name. |
| `ParamValue` | `ParamEnv::bindings` | `NOT_CARRIED`, "types whose curated face is a different shape". |
| `CarrierRelation` | `FlushEvidence::relation` | **false positive**, blind spot (b): the same type the prelude carries as `PlaneRelation`. |

## What a unit closing it would decide

Whether each of the six is a discriminant a caller BRANCHES on, which
is the CUR3 test, under LB17's carrier rule for the census placement.
Four of the six ride ONE refusal family (`ProfileError` /
`CornerReason` / `PathError`), so they are likely one stanza rather
than six; `Step` is the odd one and is a read-back value, not a
refusal. `NoCornerReason` should be decided beside the
`PathNoCornerReason` the prelude already carries, since carrying one
of a pair and not the other is the inconsistency, not the reach.
