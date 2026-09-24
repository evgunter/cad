# SCALAR — the scalar lane (plan)

**STATUS: OPEN (2026-09-11; six units landed 2026-09-15).** Opened in
the tracker cut of 2026-09-11 (`docs/WORK-TRACKS-2026-09.md` addendum 3).
The first `[ev]` sitting's six units and the three door rows are on
main; what remains is `H5` (the second sitting) and the Curve3 jet door.

Branch prefix: **`scalar/`**. Away-channel tag `(SCALAR orchestrator)`.
A/B ordinal band **SCALAR = 4100–4199**.

## Charter

`Real`, `Dual64`, the interval types and the lane traits over them are
the kernel's generic-scalar substrate, and the rows here are what the
substrate has been deferring. They divide three ways and the division is
the plan:

- **Questions only Ev answers** — `D6` (is the sense sign a newtype?),
  `D283` (does ε-typing reach `Exhaustiveness::floor`?), and
  `unit-vector-invariants-carried-as-prose`, which is an issue in shape
  but a policy question in substance: a validating newtype is new
  surface, not a refactor.
- **`H5`** — the lane traits themselves: 535 references, fifteen files, a
  proposed `docs/DESIGN.md` Q1 revision, and certificates that re-pin
  when it lands. It is the largest single unit the cut placed anywhere.
- **Doors a caller reaches around** — `D290`'s knot rescale, `S393`'s
  frame door, the validated-net door a sweep test rebuilds by hand, and
  the whole-curve order-1 jet door DOOR re-homed here on 2026-09-12
  (same file and same class as `S393`).

## Territory — none, and why

This program claims **no paths**. `crates/geom-core/src/*` and
`crates/geom/src/*` are PROPS' territory and most of `H5` sits inside
them; `interval-transcendentals/` is in no program's `paths` at all.
Every unit is announced to PROPS, and a row reaching unowned ground
draws that fence in the PR that reaches it. **A row's work reaching an
unowned path is not a licence to edit it; it is a fence that has not
been drawn**, and the PR that reaches the path draws it.

## The slate

Nine rows opened the program. Seven are closed on main (2026-09-15): the
three door rows `D290` (PR 2461, #201), `S393` (PR 2466, #203) and
`sweep-test-rebuilds-validated-net-for-v-reversal` (PR 2627, #204), and
the three rulings `D6`, `D283` and `unit-vector-invariants-carried-as-prose`,
answered in the first `[ev]` sitting (PR 2457) and executed as the six
units below. Two are open:

| item | class | what it is | where the work lands |
| --- | --- | --- | --- |
| `curve3-eval-and-deriv-at-one-t-run-two-basis-passes` | **M** | Whole-curve order-1 jet door: `Curve3::eval` + `deriv` at one `t` run two span locations and two basis passes on the `Nurbs` arm; no `ders1` exists above the span level | new door on `NurbsCurve3` in `crates/geom/src/curves/nurbs.rs` and an exhaustive arm set on `Curve3` in `crates/geom/src/curves.rs`; seven production pair sites listed in the row |
| `H5` | **H** | 535 refs, 15 files; open representation questions; Q1 revision; certificates re-pinned | `crates/geom-core/src/{real,ring_interval,interval,dual}.rs`, `interval-transcendentals/`, `crates/topo/src/{props.rs,chart_region.rs}`, `crates/geom-brep/src/{pcurve_cache.rs,ssi/enclose.rs,props/quad.rs}`, `crates/mesh/src/nurbs_cert.rs`, `crates/geom-core/src/spline/compose*`, `docs/DESIGN.md` (Q1), ~6600 lines of ring tests, `docs/tess-budget-data/` |

## The ratified units (2026-09-15)

The first `[ev]` sitting (PR 2457) answered `D6`, `D283` and the
unit-vector question; the three rows are closed and their work was six
units, all landed (blocks SCALAR-B2 and SCALAR-B3, full v6 duals):

| unit | from | PR | sample | block / slot |
| --- | --- | --- | --- | --- |
| `sense-sign-doors-take-the-bit` (SENSE-DOORS) | `D6` | 2649 | #206 | B2 / 0 |
| `sense-sign-multiplies-fold-onto-outward-normal` (SENSE-FOLD) | `D6` | 2668 | #211 | B3 / 0 |
| `unit-vector-witness-in-geom-core` (UNITVEC) | the unit-vector row | 2646 | #205 | B2 / 1 |
| `frame-witness-and-the-tube-door` (FRAME-WITNESS) | the unit-vector row | 2675 | #210 | B3 / 1 |
| `rate-pair-in-geom-core` (RATE-PAIR) | `D283` | 2657 | #207 | B2 / 2 |
| `exhaustiveness-receipt-carries-its-lane` (EXHAUST-LANE) | `D283` | 2667 | #208 | B3 / 2 |

Each unit's `## Closed` section is its record; the specs are deleted per
`docs/DOC-LEDGER.md`. One tally candidate across the six (RATE-PAIR's
M1, OPUS arm); the other eleven arms' MAJORs were absent or bilateral.
What the units handed on to other programs is on their rows (TRIM's
angular arms and limb-3 guard, TOPO's opposite-orientation class, BOOL's
sketch-plane affine, INSTR's funnel-name corpus gap, BLEND's
`perp_unit` axis, PROPS' rod residual).

## The H5 units (2026-09-21, from the second sitting's rulings)

Nine units in two chains, from `H5`'s `## RATIFIED`. The ring chain
and the no-trait chain are independent until N1 (the props doors read
the ring), so they interleave; each is announced to PROPS (every one
reaches `crates/geom-core/src/*` or `crates/topo/src/props.rs`) and to
the programs named. Full v6 dual on every unit; Fable specs on RING-2
(it changes certified bounds) and on RING-3.

| unit | what | class | ground | certified bounds move? |
| --- | --- | --- | --- | --- |
| LANE-0 | the offset-fit hook: `recertify_approx`/`approx_offset_surface`/`remap_certificate` become one `Option<OffsetFitLane>` argument on `tier3_local_checks_marked`, `mint_offset`, `map_approx`; the `Some` is read at ONE per-scalar seam, `AtRestPolicy::offset_fit_lane` (DL3's policy home), and the doors between it and the passes carry `AtRestPolicy` as a bound | E | TOPO (`validate.rs`), SHELL (`replace_face.rs`, `transform.rs`), unowned `topo/src/props.rs`, TRIM (`pcurve_cache.rs`), WIRE (`eval/wire.rs`), `crates/verbs` | no |
| RING-0 | the two differentials: `ring_interval_differential.rs` asserts `poison ⇔ dec < Def` per op and prints the disagreement classes; a scratch newtype-over-`DInterval` run whose red rows name every dependent site (RING-2's dry run; nothing of it merges but the assertion) | M | TCOST/TINT (`crates/geom-core/tests/*`) | no |
| RING-1 | `geom_core::interval` compiles unconditionally; `interval-transcendentals` a normal dependency; the feature keeps gating only the instantiation until RING-3; `ring_interval.rs:11-13` and Q1's phrase re-worded | E | PROPS (`crates/geom-core/src/*`, `Cargo.toml`); CIW announced; GUARD (`test-features-dev-only.sh`) | no |
| RING-2 | `RingInterval` = newtype over `DInterval`, poison = `dec < Def`, surface kept as inherent methods; every certificate re-pinned with the cause named; INSTR's `tess-budget-data` re-taken | **H** | PROPS (`ring_interval.rs`, `props/*`, `offset_fit.rs`, `patch_bound.rs`, `spline/*`, `geom/src/*`), TRIM (`pcurve_cache.rs`), MESH (`chords.rs`, `nurbs_cert.rs`), SHELL (`offset_meters.rs`), INSTR, the unowned `topo/src/props.rs` and `ssi/*`; TCOST/TINT for the 36 test files | **yes** — Fable spec |
| RING-3 | the newtype dissolves into `Interval`: the ring's refusal surface lands on `Interval` as named doors with the ring's bodies, `!is_certified()` the predicate, `from_certified` to a sole `CertifiedBounds` bound so `crossing_bracket` goes; `Enclosure`, its blanket impl and DL4's `Enclosure` plants deleted; the endpoint census re-keyed; C9 and DL4 re-written (Ev's text — `[ev]` PR); 829 `src` refs in 25 files (survey 2026-09-24) | M | PROPS, SSI, ENCL/OFFSET/SHELL, CHORD+TESS, GUARD, TCOST/TINT, the unowned `topo/src/props.rs`, Ev | no (bit-preserving, refusal-preserving) |
| RING-4 | the `interval` feature deleted (fail-loud): ~69 code cfg sites, ~251 gated test files, 16 manifests; CI's lane axis collapsed onto the interval lane, backend and oracle jobs kept; Q1, `DESIGN.md:266`, C9's feature sentence, `GUI-DESIGN.md` re-worded (Ev's text — `[ev]` PR); a cost gate first (split from RING-3's row by the orchestrator, 2026-09-24) | M | CIW, TCOST/TINT, MIRROR, GUARD, LIB/BIND, CLEAR/PROPS/EDIT/STACK/WIRE, CHROME, Ev | no |
| LANE-1 | `PropsQuadLane` deleted: `mass_properties`/`classify_shells*` at `Decide + CertifiedBounds` naming `quad_lane::cut_face`, `mass_properties_closed_form` public, the `_structural` twins take the hook's `None`; `datum_lo` → `Bounds::lo`; the identity test deleted | M | TOPO, unowned `props.rs`, LIB (`pncad-py`, prelude), demos, TINT | no |
| LANE-2 | `ChartRegionLane` → `Option<RegionLane>` on `census_and_certify` … `pair_region_verified`; the certified twins supply the door | M | CURVED (`census.rs`), TOPO, unowned `chart_region.rs` | no |
| LANE-3 | `ShellLane` folded into `AtRestPolicy`; DL3's wording moves (a re-wording, not a decision) | E | WIRE, PROPS | no |
| LANE-4 | `PcurveFittedLane` → `FittedLane` hook on ~57 signatures in five crates plus `EvalScalar::fitted_lane`; `lane_name` → argument; `certify_at_dual` row rewritten | **H** | TRIM, BOOL/PIN, BLEND, SHELL, WIRE, TINT, S-TINT | no |

Order: LANE-0 and RING-0 first (both dispatchable now; LANE-0 and RING-1
are block SCALAR-B4's slots 1 and 2 — their pre-draw class was recorded
as `H5`'s plan class H before the byte, and the cut came out lower; the
block record says so), then RING-1 → RING-2 → RING-3, LANE-1 → LANE-2 →
LANE-3, LANE-4 last (it collides with every open lane on the booleans,
blend and wire ground). The editor-core six (`Lane`, `MinClearanceLane`,
`SectionScalar`, `AxisScalar`, `SeedScalar`, `ChartCoherenceLane`) are a
census row on WIRE's and PROPS' slates, not SCALAR units.

## Order

**What is next (2026-09-15).** Two items remain and they are not the
same kind of work:

- `curve3-eval-and-deriv-at-one-t-run-two-basis-passes` is a door row of
  the `S393` shape (class M) and is dispatchable now as a single unit.
  Under the block protocol it opens **block SCALAR-B4 as slot 0**; the
  block's other two slots are `H5`'s first sub-units, so the block stays
  branch-side until those land. Its ground is `crates/geom/src/curves/*`
  (PROPS) and seven production sites across TOPO, BLEND and CERT ground;
  announce before dispatch.
- `H5`'s second `[ev]` sitting (PR 2701) is answered; its units are
  cut in §The H5 units below and run in that order. The paragraph
  that follows is the pre-sitting record.
- `H5` was gated on the **second `[ev]` sitting**: `RingInterval` vs an
  always-on `Interval` (S1, with the decoration-channel obstacle the
  steelman found), the `Dual` question (S2 / M10), the lane-trait
  collapse (S3 / S44's open half) and `Enclosure`'s fate (S55). The
  sitting cuts `H5` into its sub-units; nothing here re-cuts it. The
  sitting's PR is prepared from `H5`'s body — the questions, the
  steelman verdicts and the DESIGN.md Q1 revision it proposes — and
  waits for Ev.

The original order, kept for the record:

**The rulings open the program, not the units.** `D6`, `D283` and the
unit-vector question go on one `[ev]` PR together: they are one
conversation (what does the kernel put in a type, and what does it carry
in prose?) and answering them separately has already cost this board two
re-litigations. `H5`'s own gates are partly the same conversation —
`docs/DESIGN.md` Q1 — and partly its own.

The three door rows can run before any of that and do not depend on it:
`D290` mints a `KnotVector` rescale, `S393` decides a frame door's home
and name, and the sweep test's hand-rebuilt net wants a v-reversal or
`from_validated_parts` door in `crates/geom`, and the Curve3 jet row wants
a whole-curve `ders1` beside the span-level one. All four are "the caller
reaches around a missing door" and share a reviewer's question: is the
door the right shape for the OTHER callers?

`H5` goes last and is not one unit. Its own body carries the
decomposition; nothing here re-cuts it.

## Review posture

Full v6 dual with Fable specs on `H5` and on any unit that changes a
certified bound. The door rows take a style review with a correctness
arm. No A/B row on a ruling — a ruling is never work.
## How the class column is read

`E` / `M` / `H` is a **dispatch estimate**, made on 2026-09-11 by reading
each row against the tree, and it is the axis this program's order runs
on. It is not a verdict on the finding and it is not in any header: no
field carries it, `work.py` does not parse it, and this table is the only
place it lives. A lane that finds the estimate wrong says so in its PR
and this table is corrected in the same PR.

- **E** — the fix is written in the row or obvious from it: one or a few
  files, no design question, no ruling, small diff.
- **M** — multi-file, or a small design call (where a shared home lives,
  what a door looks like), or a census or instrument to build first.
- **H** — cross-cutting, numeric or algorithmic, gated on a ruling, or
  spanning several programs' territory.

The cut that opened this program is `docs/WORK-TRACKS-2026-09.md`
addendum 3; it is a survey, and this plan supersedes it as the charter.
