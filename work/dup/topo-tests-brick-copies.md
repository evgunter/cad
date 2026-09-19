---
id: topo-tests-brick-copies
kind: issue
title: Sixty-six box spellings in crates/topo/tests, 23 of them named `fn brick`, all one line over common::prism_z
status: closed
opened: 2026-09-15
branch: dup/topo-brick-copies
closed: 2026-09-16
pr: 2720
---

## Finding

- **Where**: `crates/topo/tests/` — 24 `fn brick` declarations across 22 suites
  (`review_m3_pr4.rs`, `m3_pr5_boolean_ops.rs`, `review_s6_probe.rs`,
  `shell_roles.rs`, `merge_skip.rs`, `void_door.rs`, `seat3_flush_detector.rs`,
  … ). Five are byte-identical at the signature
  `fn brick<T: Decide>(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<T>`;
  the rest vary only in the bound (`+ Bounds`, `+ CertifiedBounds +
  PropsQuadLane`) or in the argument spelling (`m4_pr2_transform.rs` takes
  `(x, y, h)`; `geom_origin_rows.rs` takes nothing).
- **Importance**: medium
- **Confidence**: sure that they are copies; unsure that the remedy below is
  the one Ev wants
- **Raised by**: the `S52` lane (SUITE), 2026-09-15

`S52` homed the axis-aligned box as `sweep::test_support::brick<T: Decide>`
and collapsed the copies in `sweep`, `mesh`, `step-export` and `stl` onto it.
`topo`'s are the largest remaining population of the same fixture and were
**not** touched, because reaching the home from `crates/topo/tests/` needs
`topo` to carry `sweep = { path = "../sweep", features = ["test-support"] }`
in `[dev-dependencies]`, and `sweep` depends on `topo`. Cargo permits that
cycle — a dev-dependency edge is not followed for the library build — but
nothing in this tree does it yet, and whether it should is a decision about
the crate graph rather than about a fixture. It is worth stating plainly that
the cost is not only conceptual: it makes `cargo test -p topo` build `sweep`,
which is the whole extrude/revolve/blend stack.

The alternatives, so whoever takes this does not have to re-derive them:

1. **The dev-dependency edge**, above. One manifest line, 24 fixtures gone,
   and the rule "an item lives at the narrowest home all of its consumers can
   reach" is satisfied because `sweep`'s own `src` pins name the family.
2. **A second home in `topo`** — `crates/topo/src/test_support_impl.rs` already
   exists with the right gate, but `topo` cannot extrude, so a `brick` there
   would be hand-built through the Euler operators and would NOT be the same
   body the rest of the tree calls a brick. That is a new spelling, not a
   shared one.
3. **`crates/topo/tests/common/mod.rs`**, which already exists and is reachable
   from every suite in the aggregated binary. Same objection as (2): `topo`'s
   tests cannot call `extrude` without the edge from (1), so this only moves
   the hand-building to one place. That is still 24 → 1 inside `topo` and it
   needs no decision from anyone, which may make it the right first move
   whatever happens to (1).

`S52`'s own sweep and its blind spots are in that PR's body.

Related, on this slate: `work/tint/sweep-boolean-suite-brick-and-prism-copies.md`,
whose `brick` half `S52` closed and whose `prism(pts, h)` half is still
open against the same home.

## Claimed by S-DUP (2026-09-16), and the premise above corrected

**Census re-taken at `95bb4ba0`** (`git grep -n "fn brick"`, no path
argument — the only form that makes no path claim):

- **23 declarations across 23 files** in `crates/topo/tests/`, one per
  file. The finding above says 24 across 22 suites, and both halves are
  wrong in different directions: the file count is 23, and the 24th
  "declaration" is `review_m3_pr4.rs`'s `brick_with_torus_face_at`,
  which is a different fixture (a brick with one face relabelled a
  torus) that *calls* `brick`. It survives an unbounded
  `git grep "fn brick"` and not a `git grep -E '\bfn brick\b'`; the
  orchestrator's dispatch carried the unbounded figure and the lane
  caught it. **Corrected 2026-09-16 from the lane's count, verified at
  the merge base.**
- Four more live in `crates/sweep/tests/`
  (`common/cavity.rs`, `blend3_r2_probes.rs` at the two-`Point3`
  signature, and two `use sweep::test_support::brick` importers) — the
  `sweep`-side population belongs to
  `work/tint/sweep-boolean-suite-brick-and-prism-copies.md` and is not
  touched here.
- One is the home, `crates/sweep/src/test_support.rs`.

**The alternatives above rest on a false premise.** Options 2 and 3
object that a `topo`-side brick "would be hand-built through the Euler
operators and would NOT be the same body the rest of the tree calls a
brick". Every one of the 23 copies **already is** that body: each is a
one-line delegation to `common::prism_z`, which already lives in
`crates/topo/tests/common/mod.rs` and is already imported by every one
of these suites. They differ only in the trait bound
(`Decide`, `+ Bounds`, `+ CertifiedBounds + PropsQuadLane`) and in the
argument spelling (`m4_pr2_transform.rs` takes `(x, y, h)`;
`geom_origin_rows.rs` takes nothing). So this is not a new spelling
being minted — it is 23 copies of a wrapper over a shared builder that
is already in a home they can all reach.

**And option 1's blocker is not a crate-graph decision either.**
`crates/topo/Cargo.toml` `[dev-dependencies]` already carries `mesh`,
`stl` and `step-export`, with the comment *"dev-dependency cycles are
cargo-legal and never reach production dependents"*. A `sweep` edge
would be a fourth of the same kind. What is left of option 1 is a
build-cost argument only: none of those three pulls `sweep`, so
`cargo test -p topo` does not build the extrude/revolve/blend stack
today and option 1 would make it.

## What this row now is

**Move A only: the named copies → 1 inside `topo`, at the home the
builder already sits in.** One generic `pub fn brick<T: Decide>` in
`crates/topo/tests/common/mod.rs` beside `prism_z`; the copies deleted.

The class turned out larger than the row again, in the other direction
from the count: 23 named `fn brick` **plus 11 renamed or inline box
spellings** the name-shaped census could not see (`distant_brick`,
two `bx` twins, `box_at`, `corner_table`'s `top`/`leg`, two `bx`
closures, and six inline pairs). **34 spellings removed.** Method item
2 — grep the construction, not the name — is what found them.

*(Superseded: 34 was the figure when this section was written. The
reviewer's differently-shaped sweep then found 26 more. The closing
table below is authoritative at 66, and the progression 23 → 34 → 66
is left standing because it is the row's evidence for method item 1.)*

The remaining half — whether `topo`'s Euler-built brick and `sweep`'s
extrude-built one are one fixture or two, and where the shared one
lives — is its own row,
`work/dup/brick-has-two-constructions-and-two-homes.md`, and it does
**not** need the `sweep` dev-dependency: `topo`'s
`src/test_support_impl.rs` is a home every crate above `topo` already
reaches. Move A is compatible with either answer and turns that later
decision from a 24-site edit into a one-line one.

Sequencing ratified by Ev in chat, 2026-09-16: "A now, B next unit".

## Closed (2026-09-16, PR #2720)

**66 box spellings removed**, cross-checked mechanically rather than
counted: the diff removes 67 `prism_z`/`prism` call openings under
`crates/topo/tests/` and adds one, the one inside `brick`.

| | |
| --- | --- |
| named `fn brick` | 23, across 23 files |
| renamed or inline, found by the construction sweep | 17 |
| let-bound `Prism`s read only for `.body`, found by the reviewer's differently-shaped sweep | 26 |

The row's own figure was 24 across 22 suites. It was wrong **high** on
the pattern (`brick_with_torus_face_at` is a different fixture) and
wrong **low** on the population by a factor of nearly three, and the
two errors had different causes: an unbounded grep, and a name-shaped
census over a class whose members are mostly not named.

**X4 fired, and was caught by the reader rather than the author.** The
first pass proved `brick ≠ geometric_cube` and stopped, in a file with
**three** cube doors; the style review found that `mapped_cube` →
`cube_into` also ends with `describe_as_intersections`. Executed on the
fix pass with the same instrument, `mapped_cube(Point3::new)` and
`brick::<f64>((0.0,1.0),(0.0,1.0),(0.0,1.0))` are **arena-identical** —
vertices, edges, faces, half-edges, loops, surfaces, curves and points
all equal. So the unit's own new door was a fourth spelling of a
builder already in its file, and the doc it shipped with asserted a
two-door partition that was false. Both are fixed; the finding that
`geometric_cube` and `cube_into` are one ninety-line sequence written
twice is filed, not fixed, because it reconciles rather than merges.

`review_m3_pr6.rs`'s `mapped_cube(Point3::new)` + `cube_into(…)` pair
was left standing on the orchestrator's ruling: the pair is one
deliberate idiom seeding a second shell in the same body, converting
half of it would split that, and it now reads honestly against a doc
that says the two builders agree.

### Residue, each with its own file

- `work/tint/topo-tests-geometric-cube-and-cube-into-are-one-sequence-twice.md` — full-review unit
- `work/tint/topo-tests-review-m2-pr7-rederives-the-shared-cube.md`
- `work/tint/topo-tests-unit-cube-has-fifty-spellings.md` — 49 spellings, and the three nullary-wrapper conventions this unit's own three decisions created
- `work/tint/topo-tests-notched-beams-fixture-is-verbatim-in-two-files.md`
- `work/tint/topo-tests-straddle-seat-hand-copies.md`
- `work/tint/topo-tests-self-declared-fixture-copies-census.md`
- `work/tint/topo-tests-scaled-probe-box-five-spellings.md` — widened in place to its class rather than duplicated

And on another program's ground, evidence added to an existing row
rather than a second file: `work/mesh/nurbs-face-bound-unsound-on-a-random-rational.md`.
