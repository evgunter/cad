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
- `H5` is gated on the **second `[ev]` sitting**: `RingInterval` vs an
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
