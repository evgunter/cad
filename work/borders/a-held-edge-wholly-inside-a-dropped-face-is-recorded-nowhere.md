---
id: a-held-edge-wholly-inside-a-dropped-face-is-recorded-nowhere
kind: issue
title: A kept face's edge that lies wholly inside a dropped covered face, both ends at vertex-on-face contacts, is held by no discard row
status: closed
opened: 2026-10-02
closed: 2026-10-20
pr: 5368
priority: P3
cost: M
design: true
blocked_on: [coincident-faces-are-not-imprinted-on-each-other-before-the-lump]
---


## What

Found while building PR 3753 (emit lane, 2026-10-02). The record is `DiscardRow::held` (`crates/topo/src/boolean/discard.rs`). It is read only where `recl::recl_sectors` meets a covered pair at a vertex both operands hold. The discarded fragment is identified by holding that vertex, or a null-edge copy of it (`HeldInto::stretches`).

At a vertex-on-face contact (`vtxfac::classify_vertex_on_face`), the dropped face holds no vertex at the contact. That leaves nothing topological to tell its fragments apart, so the contact records no held edge.

An edge of the kept face that reaches the dropped face's boundary is recorded where it does. An edge whose two ends both lie inside the dropped face is recorded nowhere. An example: the floor of a pocket in the kept body, lying inside the dropped face, with walls rising from it.

`Obstacles` then sees the dropped face's lost region as bordering nothing along that edge. The obstacle may fail to join a neighbouring discard, so a piece may refuse (`a piece of a face held as several borders no recorded discard between them`) or be named apart from other orders.

No corpus case reaches this. PR 4129's review 5 reached it in
editor-core: the plate less the union of holes that notch its edge
(`topo::test_support::meeting::notch_rows`, through
`union_pinch_member_order.rs`'s `tilted_holes`) refuses
`Naming(Emission)` "a piece of a face held as several borders…" for
three notches, two notches and a wedge, and two notches and two
wedges. Those P − U rows are not in editor-core's suite; topo builds
them sound. The notch the slab cuts in `b`'s wall in `near` has its bottom edge at two vertex-on-face contacts, but the notch's x 0.501 edge is recorded at a vertex–vertex contact, and that joins the obstacles.

## Next

1. Build the pocket case and measure it.
2. If it refuses or names by order, decide how a fragment is told at a vertex-on-face contact, for example by the pierce ring vertex where surgery inserts one. That is a design question if topology alone cannot say.

## A second residue: attaching by the entry vertex alone

Found in review of PR 3753 (re-review, 2026-10-02).

**The gap.** `discard::HeldInto` gives a held stretch to any discarded fragment that holds the edge's entry vertex `at`, or a null-edge copy of it. A fragment can touch `at` without bordering the stretch.

**Where it breaks.** At a reflex vertex, a kept sector of the dropped face can lie between the covered sector and a lost one (covered Q1 | kept Q2 | lost Q3). The lost Q3 fragment would then join the covered region's obstacle through the point `at` alone. That is the M1 failure, in miniature.

**Why nothing hits it today.**
- With axis-aligned boxes, at most three sectors meet at such a vertex, so the rule is exact.
- The breaking shape tried was an L-shaped `a`, `b` = [.5,1]×[.5,1]×[0,1], and `H` = [.3,.7]×[.3,.5]×[.5,2]. The kernel refuses it with `ClassificationInvariant` ("edge-edge membership disagreement"), the reflex-wedge limit documented on `resolve_edge_edge`.

**When to revisit.** Before that limit is lifted, or before non-axis-aligned planar covered pairs are admitted, the attach rule should test "borders the stretch", not "touches `at`". The probe source is in the reviewer's notes: `probe_l_vertex.rs`.

## A corpus case reaches it (INTENT stage 4 PR E, 2026-10-10)

PR E glues an undeclared flush contact. With that change, the probe
corpus's `cross` (`a`, `b` flush, the slab `g`, and the bar
x −1..2, y 0.9..1.1, z 0.5..3) refuses this `Emission` in
`[0, 1, 2, 3]`, `[1, 0, 2, 3]`, `[1, 2, 0, 3]` and `[2, 1, 0, 3]` of
`emit_shared_rim_several::no_order_of_the_probe_corpus_refuses_several_shared_rims`.
Before PR E, those orders refused `UndeclaredCoincidence`. The test
admits the refusal by name (`RESIDUES`), so this issue now has a
corpus fixture to measure against.

**A three-box reproducer** (both reviews of PR 4527, FORK-DM4 unit 1, measured
on its head and on its base with identical results). `A = [0,1]³`,
`C = [1,2]×[0,1]²` touching `A` at `x = 1` (the `A`–`C` pair declared from
`find_flush_candidates`), and `M = [0.5,1.5]×[−0.5,1.5]²` covering that contact.
A `Union` over the six member orders: `amc`, `mac`, `cma` and `mca` refuse
`Naming(Emission { "a piece of a face held as several borders no recorded
discard between them" })`, while `acm` and `cam` build. So the union decides by
member order, against DM4's "in every member order", on three boxes and with no
corpus document needed. It also blocks any fixture that wants a building union
whose admission rests on a pair the fold never meets (a contact a third member
covers), which is how both reviewers tried to exercise the pairwise verdicts.
Worth re-weighing the priority.

## `cross` no longer reaches it; the three-box reproducer still does (branch `emit/union-order`, 2026-10-13)

`cross`'s 10 refusing orders were not this residue. They were a chase
that stopped at a seam a later fold step had joined away: the join was
not followed, and the joined-away edge's split lineage had left the
finished body. `Obstacles` now follows both
(`work/emit/a-merged-face-pieces-borders-in-a-union-depend-on-member-order.md`,
closed). All 24 orders of `cross` fuse and publish one table, and the
corpus pins (`emit_shared_rim_several`'s `RESIDUES`,
`emit_union_flush_names`'s `KNOWN_REFUSING`,
`union_member_orders_decide_alike`'s `EMIT_RESIDUES`) are gone. No
corpus case reaches this row now.

The three-box reproducer is unchanged on that branch. `[a, m, c]`,
`[c, m, a]`, `[m, a, c]` and `[m, c, a]` still refuse "a piece of a
face held as several borders...", and `[a, c, m]` and `[c, a, m]`
build. Read in `[a, m, c]`, the parent is `a`'s top merged with `c`'s.
The pairwise judgement of `a` and `c` links them. The fold loses the
parent's region to two discards:
- `a`'s top over x 0.5..1, discarded when `m` folds in;
- `c`'s top over x 1..1.5, discarded when `c` folds in.

The two meet along x = 1. Each dies with its own member's rim there,
`a`'s and `c`'s, two distinct edges that no step ever glued, because
`m` covered the contact before `c` arrived. Nothing in the rows joins
the two discards. Each borders one piece, so neither divides.
Linking them needs a reading of coincidence that the fold never
performs: the `(a, c)` judgement's glue of those rims, or geometry.
That choice is this row's open question.


## The pocket case, built (2026-10-14)

`editor-core`
`emit_union_borders::a_pocket_floor_held_inside_a_dropped_face_names_alike`
is `#[ignore]`d; its attribute gives the run command.

**The shape.**
- `a` = the plate [0,3]×[0,2]×[0,1] less the pocket
  [1,2]×[0.8,1.2]×[0.5,2]. The pocket's floor is at z 0.5.
- `b` = [0.5,2.5]×[−1,3]×[0,0.5]. Its top is flush with the floor and
  wider than it, so the floor's four edges lie inside `b`'s top, both
  ends at vertex-on-face contacts.

**Measured.**
- The pair union `a ∪ b` refuses `Naming(Emission)` "a piece of a face
  held as several borders no recorded discard between them".
- The union in order `[a, b]` refuses the same way.
- The union in order `[b, a]` builds. It names the merged parent
  `a:z=0.50+b:z=0.50` as three pieces:
  - {`a:x=1.00`, `a:x=2.00`, `a:y=0.80`, `a:y=1.20`}, the floor;
  - {`a:y=0.00`};
  - {`a:y=2.00`}.

So the row's pocket case is reached in editor-core and decides by
member order, on two members.

**Why this is stopped here, not fixed.** A record-only fix has to say
which fragment of the dropped face takes each floor edge. Along the
floor's outline, `b`'s top is two discarded fragments:
- the covered one, under the floor, whose region the floor holds;
- the annulus inside `a`'s material, which is the region actually lost.

Both are bounded by the outline. "The fragment holding the entry
vertex" (`HeldInto`) would therefore give the edge to both, and only
the annulus borders it as lost region. This part is reasoned from the
shape; the row records were not dumped for it. Telling them apart takes a
rule: the fragment the classification dropped as inside the other
operand, as against the covered copy. That is step 2 of **Next**,
still open, and the three-box reproducer's question above is the same
one.

## The designer pair's answer (to Ev, fork-log row 171)

**The premise above is wrong** (dumped by both designers). In the pair
union and `[a, b]` the kernel does not carve `b`'s top along the
floor's outline: the contacts are tangential and `vtxfac` does no
surgery, so `b`'s top under the plate is **one** discarded fragment
holding nothing. `[b, a]` names only because Eq. 15.3 keeps operand
A's copy of a same-oriented coplanar pair: with `b` as A, `b`'s copy is
kept and carved, and the outline is recorded as `bordered`.

**Recommended:** name it, do not refuse. At a vertex-on-face covered
contact (`vtxfac::classify_vertex_on_face`, where `covered` is already
pushed), the kernel records `HeldEdge`s for the kept face's edges that
lie in the dropped face (`HeldAt::Inside`), attached to the one dropped
fragment that contains the contact point by the kernel's own `contfp`
(`boolean/contain.rs`), refusing typed only in band. Records only; the
emitter does not change.

- With the record emulated, the pocket fixture names alike in the pair
  union, `[a, b]` and `[b, a]`, matching `[b, a]`'s names today.
- The fragment matters: in a tunnel-with-pillar variant, giving the
  edges to every dropped fragment makes the tunnel piece cite the
  pillar's four walls. Attaching only where the dropped face is one
  fragment, and refusing otherwise, refuses a slot variant in two
  orders that `[b, a]` names.
- topo `boolean_covered::a_discarded_face_holds_the_edges_of_the_kept_face_that_runs_into_it`
  re-baselines: it gains the slab notch's bottom edge, the edge this
  row says is recorded nowhere.

No N2 text changes for this one; it asks only whether the record is the
right home.

## Ev's ruling: no stopgap (PR 5153, 2026-10-19)

Ev read the uncarved dropped face as a deeper problem in how the kernel
represents edges lying on planes. Ev declined the `HeldEdge` record as
a stopgap ("no stopgap"). The fix is the kernel's: imprint each
coincident face with the other operand's in-plane edges before the Eq.
15.3 lump. It is filed on FLUSH as
`coincident-faces-are-not-imprinted-on-each-other-before-the-lump`, and
this row waits on it. Once that lands, re-measure the pocket fixture
(un-ignore it) and the three-box reproducer. The emitter is expected to
need nothing, because the outline is then recorded as `bordered` in
every order, as `[b, a]` already does.

## FLUSH's design for the imprint row (2026-10-19)

The designer pair on `coincident-faces-are-not-imprinted-on-each-other-before-the-lump`
converged on the fate-change cut (the design is in that row's `## Design`).
Two consequences for this row:
- **The three-box reproducer is not reached by it.** In the four
  refusing orders, no binary step meets a coincident pair; `M`'s two
  transversal cuts make the two discards. It stays EMIT's n-ary fold
  question.
- **`HeldEdge` / `DiscardRow::held` and the `HeldInto` entry-vertex
  rule retire** when the imprint row lands. The pocket fixture is
  re-measured then, as above.

## Resolution (PR 5368, 2026-10-20)

The fate-change cut landed (`crates/topo/src/boolean/fate.rs`). A dropped
coincident face is cut along the kept face's edges where its fate
changes, so the fragment under the pocket floor is substituted
(`BooleanNaming::substituted`, no discard) and the lost annulus is its
own discard, bordering the floor's outline across the one-sided chords
(`DiscardRow::bordered_through`). `HeldEdge`, `HeldInto` and
`DiscardRow::held` are gone, and with them the entry-vertex attach rule
of the second residue.

**Measured on PR 5368's head.**
- The pocket fixture, `editor-core`
  `emit_union_borders::a_pocket_floor_held_inside_a_dropped_face_names_alike`,
  is un-ignored and publishes one set of names in the pair union, `[a, b]`
  and `[b, a]`.
- The three-box reproducer is unchanged: `[a, m, c]`, `[m, a, c]`,
  `[m, c, a]` and `[c, m, a]` refuse "a piece of a face held as several
  borders no recorded discard between them", and `[a, c, m]` and
  `[c, a, m]` build, as recorded above. No binary step of it meets a
  coincident pair, so the cut does not reach it. It moves to
  `a-union-over-a-stepped-plate-refuses-in-the-orders-that-fold-the-plate-early`,
  this row's nearest open relative.
