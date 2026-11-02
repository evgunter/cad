# CARVE — the plan

what a loft or sweep builds from its sections and its path, and the
bodies it builds that it should refuse

Re-cut 2026-10-31, when the slate had grown to 71 budget points: the
extrude and revolve rows went to LATHE, the skin's arc-wall row to
CARVETAIL (`work/carve/log.md`).

## The slate

**30 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `self-overlapping-spines-build-and-validate` | H | a loft or sweep whose spine revisits itself builds a self-overlapping body and every validation tier says Ok |
| P1 | `a-reflected-loft-placement-evades-both-normal-checks` | M | a loft section placed with a reflected frame passes both stacking decides and builds a zero-volume body that tier 3 accepts |
| P1 | `a-wall-pinned-between-two-loft-sections-refuses-at-the-wrong-door` | H +design | a loft section hinged on one of its own edges collapses that wall, and refuses at the attach gate rather than naming the pinned wall |
| P1 | `sweep-frame-is-a-minimal-rotation-from-the-start-tangent` | H +design | sweep_places carries each station by the minimal rotation from the START tangent, so the section spins near anti-parallel |
| P3 | `the-sweep-doors-return-bodies-tier-3-refuses` | M | the sweep doors gate their result by a tier-2 debug assert alone, and extrude returns bodies tier 3 refuses |

## Order

Three lanes can run at once; they touch different decisions.

- **`self-overlapping-spines-build-and-validate`, the embedding
  certificate.** Its weighing is done (the row's 2026-10-06 section),
  and CLEAR has landed all three prerequisites (`topo::clearance`,
  iso-bounded NURBS windows, `topo::injective`). The unit certifies the
  built loft or sweep embedded at its door, retires the per-slab
  stacking fold into its three homes, and BUILDS the two embedded rows
  the far-normal interim over-refuses. Measure the certificate's price
  on the tour's lofts and `m8_14_long_turn_sweep` first. How inside is
  derived is decided in this unit. CARVETAIL's parked
  `loft-between-opposite-turning-joints-reverses-a-seam-between-stations`
  waits on it.
- **`a-reflected-loft-placement-evades-both-normal-checks`.** The one
  row that builds a wrong body silently in a way the certificate does
  not catch: a square lofted to its mirror image is a zero-volume body
  that need not self-intersect. A rigidity decide at the loft and sweep
  doors, plus the sweep of other doors that take a documented-rigid
  `Affine3`.
- **`sweep-frame-is-a-minimal-rotation-from-the-start-tangent`**: the
  designer pair weighs the frame law before anything is built. Its
  answer decides whether a sweep's v becomes its path parameter, and
  CARVETAIL's P0 `a-half-turn-spine-sweeps-only-off-its-exact-tangents`
  waits on it.

`a-wall-pinned-between-two-loft-sections-refuses-at-the-wrong-door`
follows the certificate: a wall collapsed onto a segment is a boundary
meeting itself off its own edges, so whether it should build, and
where it refuses if not, is weighed on the certificate's head.
`the-sweep-doors-return-bodies-tier-3-refuses` is the same door's other
half (a result gate tier 3 reads), and is specced once the
certificate's shape at the door is known.

## The D10 hold

None of these rows reads declared pairs, declared contact, placement
vocabulary or the node vocabulary. The sweep frame is the kernel's own
carrying law along a path, not a user's placement; a weighing that
finds it wants a user-declared frame stops and says so rather than
adding the vocabulary.

## Review posture

The certificate is the dual tier: an architectural decision at every
loft and sweep door, hard to change once bodies are built on it. The
reflected placement gets a single FULL review: a confident wrong
answer if wrong.
