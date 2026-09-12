# SCALAR — the scalar lane (plan)

**STATUS: OPEN (2026-09-11).** Opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3).

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
  frame door, and the validated-net door a sweep test rebuilds by hand.

## Territory — none, and why

This program claims **no paths**. `crates/geom-core/src/*` and
`crates/geom/src/*` are PROPS' territory and most of `H5` sits inside
them; `interval-transcendentals/` is in no program's `paths` at all.
Every unit is announced to PROPS, and a row reaching unowned ground
draws that fence in the PR that reaches it. **A row's work reaching an
unowned path is not a licence to edit it; it is a fence that has not
been drawn**, and the PR that reaches the path draws it.

## The slate

| item | class | what it is | where the work lands |
| --- | --- | --- | --- |
| `D290` | **M** | Fix is stated but needs a new KnotVector rescale op in geom-core | `crates/geom-brep/src/edge_nurbs.rs` (`on_carrier_domain`), `crates/geom-core/src/spline.rs` (KnotVector rescale door) |
| `S393` | **M** | Small diff once the door's home, name and roll question are decided | new door in `crates/geom/src/curves/nurbs.rs` or beside `crates/sweep/src/skin.rs`; callers `crates/sweep/tests/common/mod.rs`, `demos/tour/src/skinned.rs` |
| `sweep-test-rebuilds-validated-net-for-v-reversal` | **M** | Fixing the test needs a new geom door or a validated-parts admission decided | `crates/sweep/tests/review_probes_m8_4.rs`, plus a new v-reversal or `from_validated_parts` door in `crates/geom/src/surfaces/nurbs.rs` |
| `D6` | **H** | Ruling schedules a newtype sweep; `sense_sign` spans ~100 sites, many crates | — |
| `D283` | **H** | Ev must rule whether ε-typing reaches `Exhaustiveness::floor`; sets `cell_width` precedent | — |
| `unit-vector-invariants-carried-as-prose` | **H** | Stated design question; a validating newtype is new policy surface needing Ev, plus generic-scalar fallout. | `crates/geom-core/src/linalg/frame.rs:238`, `crates/geom-core/src/linalg/vec.rs`, `crates/profile/src/lib.rs:353` (new `UnitVec3<T>`) |
| `H5` | **H** | 535 refs, 15 files; open representation questions; Q1 revision; certificates re-pinned | `crates/geom-core/src/{real,ring_interval,interval,dual}.rs`, `interval-transcendentals/`, `crates/topo/src/{props.rs,chart_region.rs}`, `crates/geom-brep/src/{pcurve_cache.rs,ssi/enclose.rs,props/quad.rs}`, `crates/mesh/src/nurbs_cert.rs`, `crates/geom-core/src/spline/compose*`, `docs/DESIGN.md` (Q1), ~6600 lines of ring tests, `docs/tess-budget-data/` |

## Order

**The rulings open the program, not the units.** `D6`, `D283` and the
unit-vector question go on one `[ev]` PR together: they are one
conversation (what does the kernel put in a type, and what does it carry
in prose?) and answering them separately has already cost this board two
re-litigations. `H5`'s own gates are partly the same conversation —
`docs/DESIGN.md` Q1 — and partly its own.

The three door rows can run before any of that and do not depend on it:
`D290` mints a `KnotVector` rescale, `S393` decides a frame door's home
and name, and the sweep test's hand-rebuilt net wants a v-reversal or
`from_validated_parts` door in `crates/geom`. All three are "the caller
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
