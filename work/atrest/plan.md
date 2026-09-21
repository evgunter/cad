# ATREST — the plan

what tier 3 accepts and refuses: the at-rest validator's holes in both
directions

Opened 2026-09-20 by TOPO's priority-seam cut (`work/README.md`, Track
size).

## The slate

**26 budget points** of dispatchable work against a ceiling of 30 —
about one sitting, which is what the cut was for. Nine rows arrived;
one is parked on a trigger outside this program and does not count.

| pri | item | cost | status | title |
|---|---|---|---|---|
| P0 | `an-inside-out-part-passes-tier-3-because-only-the-body-total-volume-is-pinned` | D | open | A multi-solid body whose one solid is inside-out (negative signed volume) passes tier 3 when the body's total volume is positive — validate pins only the total, never per solid |
| P0 | `tier-3-does-not-check-shell-roles-per-solid` | D | open | tier 3 accepts a solid whose shells classify to two Outer boundaries — shell-to-solid grouping is unchecked |
| P0 | `check-9-nesting-is-line-bounded-only` | H | open | check 9's nesting half is silent on every ARC-BEARING outer loop: an annular rim between two circles (every shelled vessel of revolution) still accepts a ring outside its outer loop |
| P0 | `sense-inversion-is-invisible-to-tier-3-on-arc-capped-lofts` | H | open | inverting every face's sense on an arc-capped loft leaves tier 3 green with an unchanged positive enclosure |
| P0 | `tier3-prime-still-couples-plus-v-to-the-reporting-target` | D | open | tier 3' still couples its +V check to the reporting target, so it refuses bodies tier 3 admits |
| P0 | `validity-refuses-an-interior-chart-singularity` | H | **parked** | Validity refuses a revolution-chart face whose loop is rims only (a pole or apex interior to the face) |
| P1 | `validate-tier3-curved-boundary-containment` | H | open | validate tier 3 — face-boundary containment on curved surfaces (the last unmarked deferral in the not-yet-checked list) |
| P3 | `quadric-datums-unchecked-at-rest` | D | open | Check 1 names no analytic surface whose stored frame or datum fails to describe a locus — they escalate elsewhere, by accident |
| P3 | `declared-opposite-orientation-refusal-is-unreached-by-any-row` | E | open | three planar-door consumers are reached by no row with a reversed planar face — merge_faces's declared-pair rung, join's ring_run_ccw, rest's face_carrier — so dropping the sense at the door survives the suites there |

## What is gated, and by what

`validity-refuses-an-interior-chart-singularity` is **parked on
`work/exch/import-normalizes-the-rim-only-cap`**, which states the
order outright: *"this before TOPO's validity rule, or import starts
refusing files it accepts today."* Both rows carry Ev's 2026-09-18
ruling that a chart singularity inside a face is a vertex of it; ours
is the refusing half and cannot land first. It was carried as `open`
through the cut, which made the slate read 31/30 and read this row as
dispatchable; it is neither.

`check-9-nesting-is-line-bounded-only` is gated in **two of its three
thirds** rather than as a row. The `ArcParity` and `NoWalk` classes
both wait on the general arc-aware parity walk
(`work/tang/arc-aware-point-in-loop`, #1076, open). The `Disc` class —
every edge an arc of one circle, which is the annular rim of every
shelled vessel of revolution, i.e. the shape the row is named for —
waits on nothing but a visibility change: `boolean::contain`'s
`disc_side` decides it exactly and is private to that module, which is
**CONTACT's** ground since REACH's cut. That widening is announced on
CONTACT's board by the unit that lands it.

## Order

The two admit-holes that any shelled vessel of revolution hits before
the refuse-holes, because a checker that ADMITS something invalid is
trusted by every program downstream while one that refuses something
valid is merely in the way.

1. **The per-solid pair**, as one unit:
   `an-inside-out-part-passes-tier-3-because-only-the-body-total-volume-is-pinned`
   and `tier-3-does-not-check-shell-roles-per-solid`. They are the
   same defect stated twice: tier 3 never asks which SOLID anything
   belongs to. Both want `props::classify_shells`, which `validate.rs`
   calls from nowhere, and both want the same per-solid walk. Split
   across two lanes they would mint that walk twice in one file and
   conflict over it; together they are one piece of machinery closing
   two P0 admit-holes.
2. **`tier3-prime-still-couples-plus-v-to-the-reporting-target`**, in
   parallel with the above — it lives in check 7's hook
   (`PlusVCheck` / `lane_certificate`), a different region of
   `validate.rs` from the per-solid pass, and it closes the one place
   where the two doors disagree on a valid body.
3. **`check-9-nesting-is-line-bounded-only`**, narrowed to the `Disc`
   class, once CONTACT has been told what the widening is for. Its
   `ArcParity`/`NoWalk` residue gets its own file at the moment the
   unit discloses it (`work/README.md`: disclosing a residue is not
   scheduling it).
4. **`sense-inversion-is-invisible-to-tier-3-on-arc-capped-lofts`**,
   which opens as MEASUREMENT and not as a change — its own body lists
   the three things to measure first, and
   `memories/refusal-text-is-not-cause.md` makes that a checkpoint,
   not a preamble.
5. `validate-tier3-curved-boundary-containment` is the last unmarked
   deferral in the not-yet-checked list and closes the set. The
   P3 pair ride whichever unit lands beside them.

## Review posture

**Protocol v7** (`docs/MODEL-AB-LOG.md`, Ev 2026-09-19): the dual runs
on triaged-in units only; everything else is opus-implemented and
opus-reviewed outside it. The question was recorded OPEN at the cut,
on the PROPS-cut precedent, and is answered here for this program
rather than inherited.

**Triaged IN** (v6 dual, ordinal drawn from 6400):

- `tier3-prime-still-couples-plus-v-to-the-reporting-target` — it
  moves a PUBLIC return type
  (`validate_pseudomanifold_certificate`'s), it restructures a hook
  generic over `PropsQuadLane` with four callers, and a previous lane
  declined the same restructure for the same reason. Architectural,
  and hard to change later.
- `check-9-nesting-is-line-bounded-only` — it decides which walk may
  express a loop's region and crosses another program's privacy
  boundary to do it. Broad impact, and the wrong answer here refuses
  valid bodies.

**Outside the protocol**, opus implementer and opus reviewer, with a
**FULL** review (correctness claims alongside the style questions)
rather than style alone: every remaining row changes what the
validator ADMITS or REFUSES, and this is the one door every other
program reads as proof. A new check that is subtly wrong is a false
refusal delivered to every downstream lane, which is more than reading
the diff can settle.

No row here is mechanical, so none merges on the orchestrator's read
alone.
