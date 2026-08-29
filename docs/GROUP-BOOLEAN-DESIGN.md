# Group boolean in the recipe layer — D2 + F4 (ratified: A′)

STATUS: **RATIFIED — option A′ ("a Pattern that fuses"), Evan's 👍
on the #496 thread (comment 5303065667), 2026-08-15. IMPLEMENTED by
the LIB program: `Node::PlacedUnion` at #571 (schema v12), its
Python/audit slice at #604.** The option analysis that led here
(A: heterogeneous BooleanGroup; B: sugar; C: n-ary kernel op; the
Pattern-kind and balanced-tree alternatives argued on the #496
thread) is preserved in this document's git history and the PR
conversation.

## The problem, measured (unchanged)

`Node::Boolean` is strictly binary (node.rs:606). The die tour's
21-shell cutting tool costs twenty union nodes, and the fold's
accumulator side nests: the FIRST ball's cavity faces end up 21
role segments deep. Three costs: authoring (twenty nodes for one
thing); naming/selection (`NamePat` matches exact
segment-for-segment paths, so "any face of ball 7" is unwritable);
and F4 — `body_operand` (wire.rs:345) refuses a Pattern's
`Instances`, which is why the tour heatsink's union-to-one-solid
lives in demo code. The pips are Transforms of ONE ball with
bit-identical StableNames, so any flat grouping must mint a
per-instance discriminator or alias.

## The ratified shape — `PlacedUnion`

A new node kind — NOT a `PatternKind` (Pattern's N-bodies-unfused
output contract stays untouched; forking a node's result type on a
variant is the silent-dispatch-trap shape D3 forbids):

- **Contract**: one prototype input (a body-denoting node), a
  placement rule, ONE body out — the union of the prototype placed
  at each placement.
- **Placement rule = `PatternKind`**, reusing the existing rule
  vocabulary, plus one NEW kind `Explicit(Vec<Placement>)`
  (absolute frames; order is data, index is D8-structural). The
  heat-sink document says `PlacedUnion(fin, Linear{..})` (corpus
  `heat_sink_fins`) — F4's out-of-document union retires wherever
  it is re-authored. The die's
  whole tool is ONE node: all 21 pips are the same ball —
  `PlacedUnion(ball, Explicit(21 frames))`.
- **Naming: the vocabulary does not grow.** Per-instance
  discrimination reuses the ratified `RoleSeg::Instance { i, of }`
  (A8/N1) — already wired through `SegTag`, the hasher,
  `sub_names`, the Python mirror, and `walk_names`. No new
  segment, no new tag. Instance names are append-stable (adding a
  placement changes no existing index). A one-row selector
  addresses "ball i's cavity face".
- **Disjointness is CERTIFIED, not declared**: one BVH on the
  prototype, rigidly transformed per placement, pairwise
  separation certificates — built once, cheap per pair. The check
  is sufficient-not-necessary, so the posture is
  **certified-disjoint or typed refusal** (a
  genuinely-disjoint-but-BVH-touching configuration refuses
  honestly, Budget-class, refinable later). This is stronger than
  the graft door's declared-disjoint boundary (#382's asterisk)
  and is the fail-loud reading of "identical objects make
  non-overlap easier".
- **Lowering (corrected per the #571 design-owner adjudication)**:
  the certified-disjoint case goes through
  `graft_disjoint_all_onto_keyed` in one call — the door that
  reproduces the pairwise chain's one-solid/N-shell UNION shape,
  so the result stays a legal boolean operand. (This sentence
  originally named `graft_disjoint_all_keyed`, the pre-existing
  N-ary door; its N-SOLID output is ASM's instancing currency,
  which `setopfinish` correctly refuses as an operand — relaxing
  that refusal would have been the real fork, weakening a gate
  every pairwise consumer relies on. The added door is the
  faithful elaboration of the ratified union semantics.) No new
  kernel naming record; `BooleanNaming` stays two-operand where
  seams actually happen.
- **Evaluation**: new content-key node tag (next free verified at
  implementation time), memo key covers the rule + placements,
  deterministic order = placement order (D9).

## Staged, not riding

- **Face-tied placements** (Evan's refinement note: pip locations
  tied to a FACE for consistent depth/alignment, not numerically
  coincident): a placement variant carrying a `StableName` face
  reference + in-frame offsets, with the Declare-style
  name-reference semantics (references, not DAG edges; N5 dangling
  rules) as the precedent. Its OWN follow-up design rung — the
  base node ships with absolute frames first.
- **Heterogeneous groups** (union of different shapes in one
  node): given up knowingly — nothing measured needs one; pairwise
  `Boolean` chains remain; a future operand-list widening would be
  its own conversation with this node's semantics as prior art.
- **An operand-list edit** (`DocEdit` arm to add/remove a
  placement): inherits today's delete-and-reauthor posture like
  every node; a structural-param edit arm is a separate future
  conversation.

## Costs, eyes open

- **ONE schema version** (claimed as v12): the node variant + the
  new `PatternKind` = ONE vocabulary change, one version
  (persist/mod.rs's one-meaning-per-version rule). It costs the
  standard pattern: golden + SchemaTooOld/UnknownSchema rows both
  directions, version-pin asserts, Python mirror.
- New arms at every `Node` dispatch site (inputs/slots/run_op/
  content_key/appearance/diff/edit validation/Python constructor),
  compiler-guided per D3.

## Acceptance (pinned at `editor-core/tests/lib_placedunion.rs`)

The die tour's tool collapses to one node; the first ball's cavity
faces go from 21 segments to ≤3; a `select_where`/name row
addresses "ball 7's cavity face" directly; the heatsink's
out-of-document union moves INTO the document (the F4 note retires
at its origin, both workarounds deleted per the demo doctrine);
`diefillet` exports stay byte-identical (same bodies, same order
through the graft door); the disjointness gate demonstrably
refuses a constructed overlapping placement AND a
BVH-touching-but-disjoint placement (each with its typed posture);
ε/valence: no new geometry gates.
