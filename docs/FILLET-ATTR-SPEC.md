# FILLET-ATTR — every refusing crossing, named (spec)

**Program:** FILLET (`work/fillet/plan.md`), unit
`fillet-refusal-describes-unbracketed-crossing`
(`work/fillet/fillet-refusal-describes-unbracketed-crossing.md`; Ev's ruling on
PR 1734: report the whole list). **Track:** kernel change — the standard v6
unit (binding spec, drawn implementer arm, cross-model dual review, union fix
pass, record-at-merge; §Review below). **Pre-draw fields, logged before the
draw:** difficulty **M**, task-class **STRUCTURAL**.

- **M** — a public enum reshapes (`PathError`), two channels feed it (the
  arc-carrier resolve and the straight pair), and the Python tag surface
  follows; every consumer that matched the old variants moves.
- **STRUCTURAL** — no predicate, band or margin changes; the one float the
  unit touches is a sort key for presentation order, and a sort is not a
  decision (ties break on enumeration order, so the result is a function of
  the inputs — D9 holds).

## The claim

**A refusal about a corner names the corner, and a refusal about a pair names
every corner it tried.** Today `path::arc_fillet::resolve`
(`crates/profile/src/path/arc_fillet.rs`, `build_refused` at `:498`,
`:645`–`:656`) builds every derived corner of the carrier pair, keeps the
FIRST construction refusal, and reports it alone when no corner joins; when
both crossings refuse, the sentence describes whichever was enumerated first —
the wrong corner in 12 % of BLEND-7's sweep, up to 0.83 m from the one the
anchors bracket. Three variants ride that channel and inherit the defect:
`NoCornerForFillet` (`path.rs:744`), `AnchorOutsideTrimmedExtent` (`:756`),
`FilletEnclosesLegCarrier` (`:807`); `FilletEnclosesLegCarrier`'s Display
(`:1333`) already hedges its deixis to "a corner of these carriers" and cites
the issue.

The unit replaces the pick with the list:

```rust
/// No corner of the carrier pair takes a fillet of the requested radius:
/// every derived corner that was tried, with its own reason, nearest the
/// bracketing anchors first.
NoCornerOfPair { radius: T, corners: Vec<CornerRefusal<T>> },

pub struct CornerRefusal<T> { pub at: Point2<T>, pub reason: CornerReason<T> }

pub enum CornerReason<T> {
    /// The corner's gates: it lies behind the incoming ray's origin, or not
    /// behind the arrival anchor (today's `BehindIncomingRay` /
    /// `BehindArrivalAnchor`).
    OutsideAnchors(/* which */),
    /// No tangent circle of the radius at this corner (today's
    /// `NoTangentCircle(NoCornerReason)`).
    NoTangentCircle(NoCornerReason),
    /// The trim would eat a side's anchor (today's
    /// `AnchorOutsideTrimmedExtent`'s payload, verbatim).
    AnchorOutsideTrimmedExtent { side, carrier, setback, available },
    /// The radius swallows a carrier (today's `FilletEnclosesLegCarrier`'s
    /// payload, verbatim).
    EnclosesLegCarrier { side, carrier_radius, offset_radius, largest_tangent_radius },
}
```

What stays where it is: the PAIR-level reasons that have no corner to name —
`CarriersParallel` and `CarriersDoNotMeet` — remain `NoCornerForFillet`
(with `PathNoCornerReason` shrunk to those two); `FilletOffsetLeverTooShort`
stays its own variant (it ABORTS the resolve, `arc_fillet.rs:641`, by design
— a lever the band cannot support at one corner is not a fact about the pair;
do not change that). The straight-pair channel (`path.rs:2339`, `:2354`,
`map_fillet_err` at `:2186`) has exactly one derived corner and produces a
one-entry envelope — the same type, so a consumer matches one shape.

**Display**: one sentence per entry, "at the corner near (x, y): …", each
entry's sentence being today's for that reason (the words survive; the
deixis becomes "this corner", which is now true), the envelope's header
naming the radius and the count. Entries ordered by the sum of distances to
the two bracketing anchors, ascending, ties in enumeration order — the first
sentence is the corner the author most plausibly meant, and the order is
presentation, not a claim. The recourse is per entry where the reason has
one (`FILLET_NO_CORNER_RECOURSE`, `FILLET_FIT_RECOURSE`,
`FILLET_ENCLOSING_RECOURSE` — unchanged text; FILLET-E2's pins follow
them through the new shape).

## Phase 1 — measure before touching anything

Re-run BLEND-7's attribution sweep (its fixtures are in
`crates/profile/tests/blend7_review_probes.rs` and `arc_fillet.rs`) at the
merge base and record, in the PR body: the share of refusals where the
reported corner is not the anchors' nearest; the distribution of list
lengths the envelope would carry (how often two entries, how often one); and
whether any refusal today mixes reasons across the two crossings (the case
the list exists for). Nothing in the sweep decides anything; it sizes the
change and is the before-cell of C2.

## Phase 2 — the change

1. The types above in `crates/profile/src/path.rs`; `arc_fillet.rs`'s
   resolve collects `(corner point, refusal)` per derived corner instead of
   keeping the first, sorts as stated, and returns the envelope when no
   corner joins; the straight pair wraps its one corner. The
   `FilletOffsetLeverTooShort` abort is untouched (a row pins that it still
   aborts first).
2. `PathErrorKind` (`path.rs:1120`) and the Python tag table
   (`crates/pncad-py/src/tags.rs:96`–`:99`): `no_corner_of_pair` joins;
   `anchor_outside_trimmed_extent` and `fillet_encloses_leg_carrier` retire
   as top-level tags; the per-entry reason is reachable from Python (the
   entries' reasons carry their own kind, exposed the way `pncad-py` exposes
   nested payloads today — read `errors.rs` and follow its shape). The tag
   inventory test moves with it, its claim kept by name.
3. Consumers: every `match` on the three old variants across the workspace
   (`rg -n 'NoCornerForFillet|AnchorOutsideTrimmedExtent|FilletEnclosesLegCarrier'
   crates demos pncad-py --type rust --type py`), each moved to the
   envelope; hit list and disposition in the PR body. Docs:
   `crates/profile/README.md`'s refusal family, present tense; the item's
   `#1281` deixis comment at `path.rs:1333` goes.
4. Rows (`crates/profile/tests/`): the BLEND-7 fixture where both crossings
   refuse now reports BOTH with their points, nearest first, and the first
   is the anchors' corner (the case the item was filed on); a fixture where
   only one crossing sits in the windows reports one entry (no noise added);
   the straight-pair anchor-fit refusal is a one-entry envelope with the
   same payload as before; `FilletOffsetLeverTooShort` still aborts alone;
   each `CornerReason` arm reachable through the public path door with a
   real profile; the Python tag row; determinism — two calls, identical
   envelopes, and the order is a function of the anchors (move the anchors
   past the midpoint and the order flips — a row shows it).

## Constraints, binding

- No gate re-ranked, no window widened, no predicate touched (BLEND-7's
  fence stands).
- Payloads move verbatim: every number a retired variant carried is carried
  by its `CornerReason` arm under the same name.
- The gate-shaped-vs-author-shaped taxonomy note stays a note (Ev, PR 1734).

## Acceptance

The Phase 1 table; the envelope with every arm rowed; the consumer sweep's
hit list; the Python tag inventory updated with its row; hosted CI green (the
profile suite is scalar-generic, and since 2026-09-04 every run gates the
interval lane with nothing asked for — the `CI-Config: lane=interval` this
clause used to require is deleted; count twelve `test (…)` jobs and say so).

## Out of scope

The refusal taxonomy's shape beyond this (the note); the lattice ladder; any
`sweep` refusal.

## Review

v6 dual on the frozen head, claims to falsify:

- **C1** No corner is ever reported alone when another refused: construct
  pairs where both crossings refuse for DIFFERENT reasons and check both
  entries carry their own reason and point.
- **C2** The first entry is the anchors' nearest corner on BLEND-7's sweep
  (re-run it on the head; the before-cell is Phase 1's).
- **C3** Every payload number of the three retired variants survives under
  the same name (diff the field lists).
- **C4** `FilletOffsetLeverTooShort` still aborts the resolve before any
  other corner is tried.
- **C5** The Python surface: the new tag is reachable and the retired ones
  are gone, with the inventory test asserting the set by name.
