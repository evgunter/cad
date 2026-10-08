# FORK-S3M — what a placement is, what a mate reads, what a multi-body instance copies (designer B)

## For Ev

**Recommendation** (likely). A placement becomes one node, `Place { bodies, mates }`. It reads bodies that are all in
one space, and it defines one copy of each, all moved by the same rigid motion. It **owns** its mates. Each mate
equates two poses of one kind: one pose in the copied bodies' space and one in the space the copy lands in. The kind
is the primitive, so a mate pins the copy up to exactly that kind's `Subgroup`. That is your "literally shared" taken
literally. A mate holds no number. Every number is a scalar variable inside a pose definition. An instance of a
multi-body part keeps FORK-1's one `Body` per world copy. Those bodies share one space of their own, and the instance
gains one `Frame` port: the part's world, as a pose in that space. Placing several of them in one `Place` keeps their
relative poses. No world coordinate is read to do it.

**Premise check.**
- The three questions are one question: what thing holds a relation between two spaces? Today nothing does. A mate is
  symmetric, so the solver chooses which side moves (A11 (3)–(4)), and nobody can see the choice.
- Once a placement is a node that owns its mates, the other two follow:
  - a mate needs only two poses (which one moves is fixed by which side of the placement it sits on);
  - a multi-body copy is just a placement that reads more than one body.
- One part of the brief needs correcting. "Placing each output separately loses their relative poses" is true only
  when nothing says the outputs share a space. If the instance states that they do, nothing is lost and no new port is
  needed to keep it.

**Terms.**
- A *space* is a set of things related to one another. Each pose and each body belongs to exactly one space, derived
  from what it reads.
- A *source space* is the one shared space of the bodies a `Place` reads.
- A *target space* is the space the copy lands in: the space of the poses the mates pin it to.
- *Carried(copy, p)* is pose `p` of the source space as that copy carries it, which makes it a pose of the target
  space.
- The *own frame* is a space's root frame (the spec's seed, FORK-S3-1).

### 1. A placement
- **Node.** `Place { bodies: [Body | Bodies], mates: [Mate] }`.
  - All the bodies it reads must be in one space, as for a boolean. Otherwise it refuses.
  - It defines one output per body read, of the same kind, and all of them under one rigid motion.
  - Each mate has a minted id, so faults, labels and findings can name it.
  - A mate's **`a`** side is a pose in the source space. Its **`b`** side is a pose in the target space.
  - Every `b` must lie in one space. Otherwise the placement refuses `SpacesDiffer`.
  - The source and target space may be the same space. That case is what a `Transform` was.
- **Fold.** By coset intersection: `Trivial` pins the copy in the target space; an empty bundle puts it in a new space
  of its own (#3441's rule, kept); anything else refuses `Under`.
- **Reads, not a solver.** A `b` reading `Carried(C, …)` reads copy `C`, so placements are ordered by what they read,
  a cycle refuses at the door like any definition cycle, and no tree, root or declaring role is left.
- **The product** is every copy whose space reaches the world, in the order of the placements, and in each placement
  the order of its bodies.
  - The world is an undeletable `Frame` that only a mate's `b`, or export, may read.
  - A construction never reads it, so a construction space is never the world's space.
- **Ownership and deletion.** A mate belongs to the placement it moves.
  - **Deleting a mate** raises that placement's fold: `Under` refuses at evaluation; an empty bundle moves the copy
    to its own space and out of the product. No edit records a frame or refuses for the placement it removes.
  - **Deleting a `Place`** leaves its readers unresolved and typed (D10). Its readers are booleans, selections and
    other placements whose `b` carried its copy.
  - **Deleting a body** leaves the `Place` that reads it unresolved.
- **Stage 2's world placement.** `PlaceInWorld { body, pose }` becomes `Place { [body], [Frame: own frame ≅
  Offset(World, pose)] }`. The façade's `place(body, at=…)` is the one-mate sugar.
- **Why it is right.** The user chooses, visibly, which copy moves by which placement they edit; an overconstraint
  (FORK-S3-4) belongs to one bundle, decided there; and "the placement isn't a hole on the part" holds.
- **Other final states.**
  - *(R1) Symmetric mate nodes, with the moving side derived from document order.* It keeps your symmetric wording,
    but it is A11 (4)'s invisible choice under a new rule. Rejected (sure).
  - *(R2) The world itself placed once, by one frame in the document, and the product being the world's space.* It
    would give "one number", but it fails. Once `Transform` is a `Place`, copies made to feed a boolean sit in a
    construction space. If that space were the world's, they would enter the product. Rejected (likely).
  - With this recommendation, one number is a habit: the user places one copy against the world and the rest against
    it. E's per-space computing frame keeps the world off every bit.

### 2. What a mate reads
- **Shape.** `Mate { a: Pose, b: Pose, sense: Sense }` for a `Plane` or an `Axis` (later a `Direction`);
  `Mate { a, b }` for a `Frame` (later a `Point`). The door refuses two kinds that differ.
- **The kind is the primitive.** `Frame`–`Frame` coincides, `Axis`–`Axis` is coaxial (leaving `Cylindrical`),
  `Plane`–`Plane` rests (leaving `Planar`). The residual is the kind's symmetry, so neither a cross-kind mate nor a
  primitive with no row in the table can be written.
- **The sense.**
  - It is a slot only where flipping is canonical up to the kind's symmetry. For a plane or an axis, any half-turn
    perpendicular to it gives the same flipped pose.
  - On a `Frame`, "opposed" says which axis it turns about, which is a roll. So it is written as a `turn/2` rotation
    in an `Offset`, not as a convention hidden in the mate. Today, `Opposed` on a frame silently keeps x.
- **What retires.**
  - **`MatePrimitive`** retires into the kinds.
  - **`MateFrame`** becomes `Offset { base, by }`, whose steps are scalar variables.
  - **`FrameBase::Face`** becomes `FaceFrame(face, spin)`, or that frame's `Plane` or `Axis` projection.
  - **`FrameBase::Part`** becomes the instance's `frame` port (§3).
  - **`AxisSense`** becomes `sense`, or a `turn/2` step on a frame (above).
  - **`PlanarRest.offset`** of `h` becomes an `Offset` along the normal by a `Length` variable holding `h`.
  - **The clocking rider** becomes a `Frame` mate whose angle is `FaceFrame`'s `spin`, or a rotation step in an
    `Offset`.
  - Each number becomes a variable that can be named, driven or given a tolerance. Nothing is left on the mate.
- **One construction, not two.** Picking a face on a copy and carrying the source's face pose through the copy give
  the same pose. A face selection on a copy is evaluated as `Carried(copy, FaceFrame(…))`, so it has one canonical
  form and is computed by one path.
- **Other final states.** Allowing two different kinds in one mate (a point on a plane) needs incidence subgroups that
  are no kind's symmetry. That breaks the shared `Subgroup`, and no mate today needs it. Not now (likely).

### 3. A multi-body instance
- **The signature.** FORK-1's ports stay, one `Body` per world copy of the part. It gains `frame: Frame`, the part's
  world as a pose. All of them are in **one space owned by the instance**, the same as a copy with an empty bundle.
- **What is kept.** The bodies' relative poses are kept by sharing that space.
  - A `Place` reading several copies them as one rigid group; one reading one copies only that one; separate
    placements place them separately (a kit: a bolt and its nut).
  - Inserting a part from the GUI writes one `Place` over all of its bodies.
- **Why `frame` is not a world read.**
  - An instance consumes what the part exports, which is its product in its world frame. So the part's world reaches
    the parent as a pose of what was exported, never as coordinates.
  - The parent's own world still reaches only its export.
  - Today's mates on a part's frame need this port to migrate.
- **Rejected.** The alternative adds a `world: Bodies` port beside the per-copy ports. That is two spellings of the
  same copies, and DM3's index would be a third. Rejected (likely).

### Migration (a one-time conversion of saved documents)
- **Gauges.** A gauge becomes a named `Frame`, `Offset(parent frame or World, placement)` (the turntable's is driven
  by its angle). An instance on a gauge becomes `Place { its bodies, [Frame: i.frame ≅ Offset(gauge, offset)] }`.
- **Mates.** A tree mate moves into the bundle of the instance it determined. A declaring mate is stage 4's (until F,
  verified as the spec's interim does). A non-root offset is dropped, and the conversion reports it.
- **`Transform`** becomes `Place { [b], [Frame: own ≅ Offset(own, chain)] }` in its own space.
- **Bit-equality** needs the `Offset` evaluation to give exact matrices at whole fractions of a turn. Otherwise each
  `turn/2` that replaces `opposed()` moves an ulp.

### Ratified text that changes
- **D10**
  - "possibly none: an assertion or a mate defines none" becomes "…an assertion defines none". A mate is part of a
    placement.
  - "A world placement is an operation reading one `Body` and defining its copy" becomes "A placement is an operation
    reading bodies of one space and defining a copy of each under the one rigid motion its mates pin; the product is
    the copies whose space reaches the world".
  - "an instance of a part defines one `Body` variable per world placement of the part" gains "…and its world as a
    `Frame`, all in one space of its own".
  - "A **placement** is the bundle of mates that pins one copy" stays.
- **A3** is rewritten as the node vocabulary: `Place`, two-pose mates, and `InstantiatePart` without a gauge.
- **A10**: its `PlaceInWorld` sentence is replaced, as above.
- **A11** (1): "Each primitive pins the pair's relative pose to a coset" becomes "a mate between two poses of one kind
  pins the copy to a coset of that kind's symmetry". (2), (3) and (4) retire; (5)'s member walk becomes `Carried` reads.
- **Provenance.** The D10 and A10 sentences were written by the agent's FORK-2b and FORK-1/3 commits (`dacdc06650`,
  `8d63e344f4`) after your #4220 comments. They are not in your own words, so they carry less weight.

## For the orchestrator
- **Not read:** §11 of the spec, as asked. My answer is close to the spec's candidate. It differs in four places:
  `Place` reads a list and there is no `world: Bodies` port; no `sense` on a `Frame` mate; a face selection on a copy
  is `Carried`; and the `frame` port is argued as the part's export frame.
- **Assumed:** FORK-S3-1 gives every construction space a readable root frame, which I call the "own frame". Both the
  `Transform` conversion and the `PlaceInWorld` conversion depend on it.
- **Bit-equality risk:** `opposed()` (`mate/solve.rs`) is an exact matrix, but a rotation by π in floating point is
  not. B's unmoved-poses check needs the `Offset` evaluator exact at rational fractions of a turn, or an exact
  half-turn step.
- **Ev's transcript:** the 2026-10-03 transcript is only in deeper history (`git fetch --deepen`). I quoted "one
  number rather than a profusion" (Ev, 2) from it.
- **Off-question:** the spec's per-kind `sense` on a `Frame` keeps today's hidden "half-turn about x" convention.
  Worth a line in B whichever way the fork goes.

## Round 2

**1. What `Place` reads: I move to one shape (`Body` or `Bodies`).** D10 already has a kind for "several bodies, ordered,
in one space", so a list slot would be a second spelling of `Bodies`, and a slot holds its own kind. My only reason for
a list was point 2's per-body ports, and that reason goes there. Reversal cost: low, since a list is sugar over a
`Bodies` construction.

**2. The instance: I move to `bodies: Bodies` + `frame: Frame`, with one amendment.** The part's product is one thing
the part said once, so one port is right. The per-body ports would duplicate it, which was my own objection to
`world: Bodies`. My amendment is about picking one body: it should name the **part's placement** (by its id, as a face
is named by `StableName`), not a position. Under DM3's position index, an `UpdateReference` that reorders or deletes
the part's placements silently re-points the pick to another body, against DM6 ("no edit infers a re-point"). Named,
the same update leaves the pick unresolved and typed. So I keep FORK-1's identity (one body per part placement) and
drop its ports. This is a small fork on DM3 that Ev could weigh; reversal cost is low, and the named pick is the one
that cannot lie.

**3. A `frame` output on `Place`: I hold no port.** A's `P.frame` is exactly `Carried(P.copy, <the source space's own
frame>)`. A frame of an instance's copy is `Carried(copy, I.frame)`. So the port is a derived value given a second
spelling, which has to be canonicalised back at the coincidence door. On the sibling question: the port is opaque
only if the door treats it as an atom. Read as sugar for `Carried`, it is transparent. The rewrite the door wants (a
pinned copy's `on` pose carried equals its `to` pose, modulo the kind's subgroup) needs the reference to name the
placement and a source-space pose, and `Carried(copy, pose)` names both directly. With a port, every read must first
be un-sugared, or a door built on the atom proves only "same copy", which is the FORK-S3O concern. A's "another one,
10 mm over" is `to = InFrame { Carried(P.copy, seed), 10 mm }`, so no use is lost. Is A's argument answered? Yes:
"where the instance sits" is `Carried(copy, I.frame)`. Not a choice for Ev unless A holds. Reversal cost: adding a
port later is additive, while removing one strands its readers.

**4. Sense: I move to A's `Flip`.** Equality-only matches D10's "an incidence is a construction over one variable,
never a check between two". `Flip` is undefined on a `Point`, so the sense gap becomes a type error instead of a door
rule. And a `Sense` slot on two kinds only is a per-kind special case. My worry about two spellings (`Flip(a) ≅ b` vs
`a ≅ Flip(b)`) is answered by canonical form, because `Flip` is an involution and the door normalises it to one side.
We already agree on an explicit `turn/2` for a `Frame`.

**5. A clocked coaxial: I move; mine was wrong.** A `Frame` mate pins the slide. Today `Coaxial` + rider leaves
`Prismatic` (slide free), so my spelling would change what the mate leaves free and the migration's solved poses. A's
`Axis`–`Axis` + `Direction`–`Direction` keeps `Prismatic`. Cost: `Direction`'s subgroup (dim 4: translations plus spin
about it) enters the fold table in stage 3 B, as one arm of the shared `Subgroup`. The spec's §3 line has the same
error.

**Proposed text.**
- D10 Operations, replacing "…one `Body` variable per world placement of the part" and "an assertion or a mate defines
  none":
  - "…an instance of a part defines the part's product, one `Bodies` whose members are named by the part's
    placements, and the part's world as a `Frame`, both in one space of the instance's own; possibly none: an
    assertion defines none, and a mate is a clause of a placement, not an operation."
- D10 Operations, replacing "A world placement is an operation reading one `Body` and defining its copy": "A placement is an operation reading one shape (`Body` or `Bodies`) and a bundle of mates, and defining the copy,
    of the shape's kind. The product is every copy whose space is the world's."
- D10 Spaces, replacing "the bundle of mates that pins one copy of a part relative to others":
  - "the bundle of mates that pins one copy of a shape: each mate equates, modulo its kind's symmetry, a pose of the
    shape's space with a pose of the space the copy is placed into; a pose of another copy is read as that copy
    carries it."
  - The world becomes "one undeletable valueless frame that only a mate's target side and export read".
- A10: "A world placement (`PlaceInWorld`) is an operation reading one `Body` and defining its copy as an output" →
  "A placement (`Place`) reads one shape and defines its copy"; "the copies the document's world placements define" →
  "every copy in the world's space, in placement order".
