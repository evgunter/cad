---
id: shell-curved-clearance-consumer
kind: issue
title: where a curved wall-clearance gate can call the E7 engine from (the shell verb sits below it)
status: open
opened: 2026-09-03
refs: [shell-curved-wall-clearance-window, M10-5, 1055, 1191]
---

## The valve M10-5 left open

M10-5 built the E7 clearance engine and its acceptance fixtures are
issue 1055's own dumbbell: the neck's two facing walls, 0.4 apart, are
reported `Violated` against the 0.6 that two 0.3 walls need, with an
f64-verified witness. The EVALUATOR that issue asked for exists.

What did not land is the consumer, and the blocker is layering, not
effort.

- `topo::shell`'s gate site (`wall_clearance`, `crates/topo/src/shell.rs`)
  is inside `crates/topo`.
- The E7 engine is `editor_core::clearance`, and `editor-core` DEPENDS
  on `topo` (the G1 layering note in `crates/editor-core/Cargo.toml`:
  "editor-core sits ABOVE the kernel … the kernel crates gain NO
  editor-core dependency"). `topo` cannot call it.
- The dependency direction is not incidental to the engine either. Its
  inputs are a `Doc`, a `ParamBox` and a leaf the E6 driver certified —
  document-layer objects that do not exist at `topo`'s altitude. A
  `Body<Interval>`-only engine would be a second subdivision, not a
  call.

So closing 1055 needs a decision about WHERE a curved wall-clearance
gate lives, and that is a VERBS + M10 design question:

1. **A verb-layer gate above editor-core**: `shell` keeps its
   closed-form planar gate, and the curved arm becomes a check the
   document layer runs on the shelled body (the M10-6 reporting lane's
   natural shape — an assertion over a `min_clearance` measure). The
   verb then no longer refuses; a report does.
2. **A `topo`-level engine over `Body<Interval>`**: the same inner
   subdivision without the parameter box, called from `wall_clearance`.
   Correct for the verb's own question, and a duplicate of the cell
   subdivision this unit shipped.

## What it would cost today even with the layering settled

Two measured limits from M10-5's own suite, both worth knowing before
either option is chosen:

- **Box width.** No node's interval replay builds over a parameter box
  wider than a small fraction of ε (issue 1191's class), so a
  parametric curved gate answers over ε-scale boxes only. Option 2 does
  not have this problem — it has no parameter box.
- **Cost, and WHERE it falls.** Measured on the hexagonal prism of
  `crates/editor-core/tests/m10_5_clearance_interval.rs` and on the
  twelve-vertex comb of `m10_5_r2_probes_interval.rs` (R2's fixture):
  a bound the geometry BREAKS costs a handful of cell pairs, because
  the sweep stops at the first verified witness; a bound the tree can
  EXCLUDE costs nothing. What exhausts the shipped
  `DEFAULT_MAX_CELL_PAIRS = 65_536` is the FRONTIER — a bound sitting
  on a pair's own separation, where no cell pair ever resolves either
  way. A shell gate picks its own `c`, so whether it lands in the cheap
  regime or on a frontier is a property of the wall thickness it is
  asked about, not of the engine. It also needs a cheaper pair filter
  than the quadratic adjacency walk the engine does today once the face
  count of a shelled body is in play.

## Home

`crates/topo/src/shell.rs` (the gate site, which cites 1055 by name)
and `crates/editor-core/src/clearance.rs` (the evaluator). Rides with
`work/shell/shell-curved-wall-clearance-window.md`, which is the
issue-1055 record and stays parked until this is answered.

## The question for Ev (SHELL orchestrator, 2026-09-04)

M10-5 and M10-6 have merged, M10-7 is in review, and SHELL-1 (the
`ShellNaming` record LIB-G17 waits on) is dispatched, so every input
this fork needs is on the table. What follows is measured from the
tree at main `cf90f96a`, with a recommendation.

**The invariant, stated once.** The sealed shell hands the void door
`Carried { Positive }` per cavity shell — the construction's own
per-face reach margins. That evidence is sound exactly when the
cavity body is EMBEDDED: two non-adjacent cavity faces at strictly
positive separation. The planar gate (`wall_clearance`) proves that in
closed form for plane pairs; the curved residue is the same question
for curved pairs, and it is E7's global self-intersection question
(`ERROR-DESIGN` E7, "non-adjacent face pairs certified strictly
positive distance") asked of the cavity clone before insertion. No
shell-specific predicate is needed — "walls ≥ 2t apart" and "the
inward-offset boundary is embedded" are the same gate.

**Three facts that shape the answer.**

1. The engine is already body-level below its document skin.
   `editor_core::clearance::min_separation` (M10-6) takes
   `MinSepSelection { body: &Body<Interval>, faces, at, index }` —
   the `at`/`index` fields are attribution, not inputs; the inner
   subdivision (`window_of`, `Cell`, `Sweep`, `split`,
   `verify_witness`, ~1000 lines of `clearance.rs`) reads only
   `topo::Body<Interval>`, `bvh` and `geom-core`. Only the OUTER half
   (`clearance_over`, `LeafFold`, `facet_restrict`) touches `Doc`,
   `ParamBox` and `Evaluation`. `topo` already depends on `bvh`.
2. No scalar remap of a body exists (M10-7 deviation D3: the arenas
   are `SlotMap`s whose keys a rebuild cannot preserve), so an
   `f64`-built shell cannot lift itself to `Interval` to run an
   interval engine. A certified curved gate runs where an `Interval`
   body exists: inside `shell::<Interval>`, or in the driver's leaf
   replay, which evaluates every node at `Interval` anyway
   (`drive.rs`: "a leaf replays the recipe at `Interval`").
3. `topo::shell` already states its contract PER SCALAR — its bound
   `CertifiedBounds` says "a scalar without certification rights
   cannot form the call", and `MinClearanceLane` (`measure.rs`) is
   the per-scalar-capability shape in the same tree: a compile-time
   capability whose `None` is a typed refusal at the site, and only
   `Interval` answers `Some`.

**Option A — the analysis lane asks E7's question of the shell node's
output.** After LIB-G17 gives documents a `Node::Shell`, the driver's
`Interval` replay runs `self_intersection` over the node's cavity
faces (an E10 assertion, or a standing per-node check in the M10-6
reporting lane) and reports `Violated` with an f64-verified witness.
No kernel change; no new engine. The verb never refuses the curved
case at any scalar; a document that does not carry the check never
learns, and a `Body` built through the direct door (every demo and
test today) is unguarded. The layering objection in the body above
is answered by not asking `topo` to call anything.

**Option B — the engine's body-level half moves DOWN into `topo`
(behind `interval`), and the verb runs it at the scalars that can.**
One engine: `topo::clearance` (or a `crates/clearance` between
`topo` and `editor-core`) holds the inner subdivision and
`min_separation`; `editor_core::clearance` keeps the leaf/param-box
outer half and calls down. `shell` takes a lane bound in
`MinClearanceLane`'s shape — `impl for Interval` in `topo`, `f64`
answers `None` — and at a lane that answers, the cavity clone is
checked strictly positive over its non-adjacent face pairs before
insertion; a crossing refuses `ShellError::WallClearance`-typed with
the witness (the same variant the planar gate raises, with the pair
named), a frontier or budget refuses `Escalated`. At `f64` the door's
docs state the residue exactly as today. In the driver the shell node
then REFUSES in the `Interval` replay — a typed node refusal, priced
as mass by E6/E8 — with no assertion to author. Cost: a move of M10's
file while M10 is live (measured: M10-7's PR #1725 does not touch
`clearance.rs`); a pair-filter measurement on the tube and vessel
fixtures before pulling (the engine's BVH admission prices pairs by
`separation_lo`, so a concentric shell's facing walls are the only
pairs that subdivide, but that is a claim to measure, not assume).

**Option C — a shell-specific engine in `topo`.** The issue's option
2 as written. Rejected in advance: a second subdivision beside M10's
is the duplicate the reviewer style lane exists to catch.

**Recommendation: B.** The gate belongs at the door that carries the
evidence, and this tree already spells "certified where the scalar
can, refused typed where it cannot" at that very door. B leaves the
`f64` window exactly where A leaves it — neither option closes it,
because certification is interval work — but B makes the certified
replay refuse instead of requiring a user to ask, keeps one engine
with one home, and turns `shell-curved-wall-clearance-window` into a
row (`shell::<Interval>` on the OFF-D dumbbell's curved twin refuses
with a witness; on the tube it holds). Its honest cost is the move of
a live program's file, which is why this is a joint question rather
than a SHELL unit.

**What closes the items either way.** Under B: the move lands as one
unit (joint SHELL/M10 review), then the gate as a SHELL unit; the
window item closes on the refusing row. Under A: the window item
re-parks on LIB-G17 and closes on a driver row over a curved-neck
document; `topo::shell`'s module docs keep the window sentence
permanently.

**Sign-off affordance: 👍 the PR comment naming A or B**, or say
otherwise; the item and DESIGN.md's roadmap line on M10 clearance
(`docs/DESIGN.md` §validate tier 3, "M10 interval clearance") update
in place with the ruling.

## Ruling (Ev, `[ev]` #1737, 2026-09-04): B

"i think B makes sense?" — recorded as the ruling. Cut into two units:
`SHELL-3` (the engine's body-level half moves into `topo` behind
`interval`, joint with M10) and `SHELL-4` (the gate at the shell door
at certifying scalars). `shell-curved-wall-clearance-window` is
re-parked on SHELL-4. `docs/DESIGN.md`'s tier-3 note names the home.
