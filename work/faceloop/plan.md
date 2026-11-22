# FACELOOP — a profile drawn on a face holds that face's loops (plan)

The ruling is `docs/DESIGN.md` `[variables.kinds]` (PR 7284); the
agreed state, its corrections and the measured elbow are the parent
row, `a-faces-loop-or-closed-trace-cannot-become-a-profile-loop`. Each
unit's body is its spec.

## The slate, in dependency order

1. **U1** `a-faces-loop-reads-each-joint-off-the-body` (M, open): the
   joint at each vertex of a face's loop, corner or tangent, read off
   the edge between its two walls at every lane. Inert. It settles the
   riskiest unknown, so it goes first.
2. **U2** `a-profile-on-a-face-holds-that-faces-outline-or-a-hole` (H,
   parked on U1): `LoopProgram::Boundary(Outer | Through(edge))`, its
   read, its lowering at each lane, its typed refusals, and the names
   of the walls it sweeps (step id plus source edge).

Off this slate, parked on U2: the click, on VSEAM
(`a-click-on-an-edge-of-the-sketched-face-writes-its-loop`, M), and the
Python spelling, on PYDOOR (`a-faces-loop-has-no-python-spelling`,
P2, E). The parent row is parked on U2 and the click.

## What runs beside it

- **Now, in parallel with U1:** LATHE's
  `a-revolve-refuses-a-profile-on-one-side-of-its-axis-as-across-it`
  (P1, M; needs nothing of this build, and half of the elbow's revolve
  headings refuse until it lands), HELDRECORDS's
  `a-pipe-does-not-union-with-an-elbow-revolved-off-its-own-cap` (P0,
  H, held there), PROFILE's build of the PR 7251 ruling, and the
  designers' weighing of PROFILE's
  `part-of-an-outline-or-a-section-where-no-edge-exists-has-no-profile-form`.
- **Coupled with U2:** PR 7251's build deletes `Step::ContinueTo` and
  `Step::TangentArcTo`, the kernel steps U2's lowering writes at a
  tangent joint today. Neither waits on the other; whichever lands
  second re-spells the other's sites (U2 adds about four), and each
  lane is told of the other while both are live. SELFREF adds document
  reads to the same resolve hop and is told likewise.
- **Re-baselines U2 will meet:** `circle-lowers-to-one-segment`
  (PROFILE, parked) turns a pipe's two-arc rim into one closed edge.
  U2's lowering then writes one `circle` where it wrote two arcs, and
  the source-edge names inside its piece names move with the rim's.

## Open questions (not settled by the ruling)

1. **The static doors** (blocks U2). `ProfileProgram::shape`,
   `partition`, `structure` and `pieces` answer without an evaluation;
   a `SetProgram` that keeps named steps calls `partition`. A
   `Boundary` loop's pieces are known only with the body. Do these
   doors refuse it typed (as the trace forms refuse `Unevaluated`),
   which makes any reshape of a profile with a named `Boundary` piece
   refuse; or treat the step as one run whose members are checked at
   evaluation (N5 `Vanished`)?
2. **Where the hole's edge read sits** (blocks U2): in the loop
   (`LoopProgram` holding a read, a seat per loop), or in `DrawnOn`
   beside the traced faces (a seat per hole). Both persist.
3. **The arc at a tangent joint** (blocks U2). After `.tangent()` the
   lattice admits no centre-given arc (`arc_to(Center)` is in the
   endpoint-full row only, `crates/profile/src/path/program.rs`), so
   the lowering writes `tangent_arc_to(p)`, `arc_to(End { p })` after
   PR 7251's build, and the centre is derived from the inherited
   tangent and the end, not read off the edge's carrier. Is that the
   "ordinary arc piece" the ruling means, and is the derived centre
   checked against the carrier, or does it hold by construction (D1:
   one or the other, never both)?
4. **A drawn smooth joint, swept** (blocks U1 only if its first check
   fails): whether a sweep records a declared tangent joint on the
   lateral edge it mints, where the two walls lie on two surface
   records. That would change what an edge stores.
5. **A hole named through a rim a keyway splits** (blocks U2 only if
   its first check fails): Ev's example has the elbow keyed through its
   bore; if the split rim's name refuses N5, the hole refuses instead.
6. **Click order** (blocks U3): if the program's loop order, not
   containment, decides the outer loop, the click needs a placement
   rule the ruling does not state.
7. **`Through` an edge of the outer loop**: read here as a typed
   refusal naming `Outer` (one spelling per act, `[one-way]`); confirm.

## Exit shape

U1 and U2 merged, and the click on VSEAM. The parent row closes with
the last of them. No exit criteria are set, so no walk is owed.
