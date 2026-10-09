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

## Round 3

**1. Reading Ev's sentence.** "None of these should be 'moving' at all; at most one of them already has a placement."
I read it as rejecting `Transform` as a motion: an operation that takes a positioned thing and repositions it. A body
has no location (Ev, 2026-10-03), so there is nothing to move it *from*. What a person means by "transform the ball
21 times" is 21 relations, each saying where a copy of the ball sits relative to something else. In each pair (the
ball, a pip), the ball has no placement and the pip has exactly one. That is "at most one". Read against the cases:
- **The die's pips.** Each pip is a copy of the ball, pinned against a pose of the die. The ball is not moved, and no
  pip was ever anywhere else.
- **A part moved within its own document before a boolean.** It is a copy related to the poses it should sit
  against, not a moved body. The original is unchanged and stays readable.
- **`Transform` over a placed copy `C`.** "Moving" `C` would give it a second placement, which is the contradiction
  Ev named. It is either a new copy related to `C` (a `Place` reading `C`, whose bundle reads `Carried(C, …)`), or a
  further mate in `C`'s bundle, which is an overconstraint.

So `Transform` is **neither a construction nor its own operation**: it retires, and its uses are placements. A
`Transform` construction (the sibling of `Pattern`, reading a body and a chain) would be exactly the motion Ev
rejects. It would also be a second way to say where a copy of a body sits, beside `Place`, so I lean against the
S3P designer who proposed keeping it.

**2. Final state.**
- **Each pip** is `Place { shape: ball, mates: [Frame: BodyFrame(ball) ≅ Offset { <a pose of the die>, by: chain } ] }`.
  - The chain is scalar variables, today's frame numbers.
  - Its space is the space of what it reads: the document's construction coordinates, where the die and the ball
    were built.
  - It is **not** in the product, because construction coordinates are never the world's space.
- **The boolean** `Union { die, pips }` is legal under "reads lie in one space", because the die (a construction) and
  the pips (copies pinned against the die's poses) are in that one space. The union's result is placed in the world
  by its own `Place`.
  - This needs S3P's "a space is a set of copies" to also count the document's construction coordinates as a space,
    whose members are its constructions and the copies pinned against their poses. Otherwise no boolean of a body
    and its displaced copies can be written. Flag this to S3P.
- **21 pips as one node.** `Pattern` becomes the multi-copy form of `Place`.
  - It reads one shape and a family of target poses (`Linear`, `Circular`, an explicit `Vec<Frame>`), and defines a
    `Bodies` of copies, each pinned by one `Frame` mate from `BodyFrame(shape)` to its member of the family.
  - So a pattern also relates copies and never moves anything.
  - The die's frames (six `Explicit` in the file) become six `Frame` variables.
- **`Transform`** (with `wire_transform`, `placeable_operand`, the walk through it) and **`PlacedFrom`** retire: every
  copy is a `Place` or `Pattern` output; the chain moves into the pin, so stage D's `transform_rigid` call is unchanged.

**3. Text, and (i)/(ii).**
- **D10 Operations** gains: "Nothing moves a body: a body has no location, and a copy of it is placed. A copy is
  defined by one placement (`Place`, or a `Pattern`, which places one copy per member of a pose family), never moved
  after. A further statement of where it sits is an overconstraint."
- **D10 Spaces** gains, after "a part is born in its own space": "A document's constructions share its construction
  coordinates, which form a space; a copy pinned against their poses is in that space, and a boolean may read both."
- **A10** gains "`Pattern` places as `Place` does".
- **D10's last paragraph** adds `Transform` and `PlacedFrom` to the retirements.
- **(i)/(ii)** is unchanged by this. Retiring `Transform` decides how a copy is defined, not how many shapes one
  placement reads.
  - A `Pattern` defining a `Bodies` fits (ii), where `Place` reads one shape. Under (i), a pattern's output would
    have to be read as one shape, or exploded into a list.
  - So the result leans further towards (ii), with my round-2 amendment: an instance's single body is picked by the
    part's placement id, not by position.

## Round 4 — corrections against Ev's 2026-10-03 words

**Ev's principles cited below** (Ev 2 and Ev 3, verbatim in `5f7a1c71e3`):
- **E1:** "parts don't sit at (0,0,0) in their own space; they just don't have a location (or orientation) … never
  anything real."
- **E2:** "you can't Transform an already placed part … the placement isn't a hole on the part to be filled, it's a
  description of how two inherently-unplaced parts relate."
- **E3:** the world "looks like a part … other things can relate to it … but it actually just sets the coordinates";
  "one number rather than a profusion."
- **E4:** none of the computing-coordinate choices depend on the world node.
- **E5:** "constraints that fall back to being assertions seems worse than either."
- **E6:** a boolean "require[s] that they're already related."
- **E7:** "different spaces are just implicit in whether parts are constrained relative to each other. there is no
  canonical main space."

Where my earlier rounds read otherwise, Ev's words win. The corrections:

1. **Withdrawn: the instance's `frame: Frame` port (R1, R2).** It bends E1 and E3. It made a part's world frame a pose
   that the parent mates against, which makes a part's location real. An instance defines the part's world-named
   bodies only, in one space (their relations are the part's, computed per E4), and no pose of the part's world. A
   mate on an instance reads poses of its bodies (faces, or poses the part's construction defines over them). The old
   `FrameBase::Part` mates, which named the part's coordinates, have no faithful image. The migration re-anchors each
   on a pose of the body it read, or drops it and names it.
2. **Withdrawn: "a document's construction coordinates form a space" (R3).** It is the "document's own coordinates"
   that Ev caught on #4324, and it bends E1 and E7. Replacement: a space is relatedness, nothing else. Each pip is a
   `Place` of the ball whose mates read poses **of the die's body** (e.g. a face frame, `Offset` by scalar
   variables). The pip is then related to the die, so the union of the die and the pips reads one space (E6). Ev's
   "at most one of them already has a placement" is literal here: the die has none, and each pip has one.
3. **Withdrawn: a mate side read as `BodyFrame(body)` / "own frame" (R1, R3).** It is a frame standing for the
   coordinates, which bends E1. A mate's `on` side is a pose defined over the shape's own geometry or construction,
   never a frame that names its coordinates. What a part's first construction frame is belongs to S3P. I depend on
   S3P answering it without a blank frame.
4. **Withdrawn: "until F, a mate beyond the pin is verified as the spec's interim does" (R1).** That is E5's
   "constraints that fall back to being assertions". It also recreates placing-or-declaring by position, which is
   the problem A11 (4) caused. From C onward, a mate that does not lower its bundle's fold refuses. Today's declaring
   mates migrate to explicit assertions (stage 4/5's `Assert`) or are dropped, and each is named. If that has to wait
   on stage 4's lint, C waits; the stage does not land a fallback.
5. **Revised: the product and the world (R1's R2 rejection reversed).** E3 asks for one number. I rejected "the world
   related once" only because pip copies would enter the product, and that reason came from treating a space as
   coordinates. Revised:
   - a **world placement names** one body for the product and defines no copy (copies are `Place`'s alone; the pose
     stage 2 put there was "transient", Ev, #4220);
   - every named body must lie in one space, or the gather refuses, naming the spaces;
   - the world is related to that space by **one** mate, its own and undeletable, whose `to` is a pose of a named
     body;
   - only export reads it.
   So pips that are not named stay out, the union is named directly, and no identity `Place` exists just to export.
6. **Held:** a `Place` owns its mates (E2); `Transform` retires (E2: nothing moves); mates hold no number; `Flip`;
   an axis plus a direction for clocking. `Pattern` as a family of placements holds only where its target poses are
   poses of a body. The die's six explicit frames re-anchor on die poses, or the migration names them.
7. **Text.**
   - D10 Operations: "A world placement names one `Body` for the product; the bodies it names lie in one space, which
     the world is related to by one mate of its own, read by export alone. A placement (`Place`) reads one shape and
     defines its copy, pinned by its mates against poses of other bodies; nothing else defines a copy or moves one."
   - D10 Spaces: "A **space** is a set of bodies related by placements; nothing else, no coordinates, makes one."
   - FORK-1: "an instance of a part defines one `Bodies`, the bodies the part's world names, in one space."
   - **(i)/(ii):** the `frame` port leaves both options. Picking a single body stays by the part's world placement id.

## Round 5 — final, under Ev's #4324 answer ("a sketch exposing only its lines … reuse the same profile in multiple positions")

**Round 4 holds unchanged in substance.** Ev's answer strengthens three of its points.
- **A mate's `on` side** is a pose read off the shape's geometry: a face frame or a face's plane, an axis, their
  `Offset`s and named combinations, or `Flip`. A profile has no frame, so there is nothing else to read. This is E1:
  a body has no location, only geometry.
- **The die's pips need no copy at all.** The ball's profile is 2-D content, and a revolve reads it through a frame
  the reader supplies. So each pip can be a construction that reads that one profile through a frame read off the die
  (an `Offset` of a die face frame by scalar variables). The pips are then built where they are: nothing moves, and
  nothing is placed (E2, and Ev's "none of these should be 'moving'"). The union reads the die and its pips, all of
  one construction (E6).
  - `Place` and `Pattern` remain only for **copies of a body**, such as a part's bodies through an instance, or a
    finished, filleted body reused.
  - `Pattern` is a family of placements whose target poses are read off a body. A repeated *feature* is a
    construction reading one profile through a family of frames, so it needs no copy.
- **The world placement** stays as round 4 has it. It names bodies for the product and defines no copy. The named
  bodies lie in one space. The world is related to that space by one mate of its own, whose `to` is a pose read off a
  named body's geometry (E3: "one number"). Only export reads it (E4). It needs no frame from nothing.
- **Dependency on S3P:** the first body's frame is S3P's to settle. Mates never read that frame as a "part frame",
  because a mate reads geometry only. So whatever S3P settles changes nothing here.

**D10 Operations and placement text, final:**
- "A placement (`Place`) reads one shape (`Body` or `Bodies`) and a bundle of mates, and defines its copy. Each mate
  equates, modulo its kind's symmetry, a pose read off the shape's geometry with a pose read off another body's
  geometry in the space the copy joins. A mate is a clause of its placement, not an operation, and holds no number.
  Nothing moves a body: a copy is defined by its one placement, and a further mate that lowers nothing refuses.
  `Pattern` places one copy per member of a pose family. A world placement names one `Body` for the product and
  defines no copy. The bodies it names lie in one space, which the world is related to by one mate of its own, read
  by export alone. An instance of a part defines one `Bodies`, the bodies the part's world names, in one space."
- Spaces: "A **space** is a set of bodies related by construction or placement; nothing else makes one."

## Round 6 — mates and values on equal footing (Ev, #4326, #4325)

**1. Yes; I withdraw "a mate is a clause".** A placement's constraint list holds two kinds of constraint, on equal
footing:
- A **mate** equates two poses of one kind, read off geometry.
- A **value**, `Value { mate, coord, by }`, sets one coordinate that mate leaves free to a `Length` or `Angle`
  variable (Ev 2: "type suits its slot").
  - `coord` is one generator of the mate's residual subgroup: after `Plane`, `SlideX`/`SlideY`/`Spin`; after `Axis`,
    `Slide`/`Spin`.
  - The axes are the frames the mate's two sides project from, each a `Frame` read off geometry (its reference read
    off an edge, an axis or a second face). Otherwise the value refuses, typed `Uncharted`.
  - A planar face's default u-reference is derived from computing coordinates, so it is not admitted. No raw
    coordinate names a degree of freedom (E1).
  - Zero is where the two frames coincide, and each subgroup has one chart (translate, then spin about the target's
    normal or axis).

**2. `Offset` as a mate target retires; values are the kernel form.** `Frame: a ≅ Offset(F, dx, dy, θ)` equals
`Plane: a ≅ F` plus three values. Only the values can leave a coordinate loose: a slot's slide with the spin free has
no coset form. Stand-off, roll and full offset become values, and the façade's `at=` writes them. Since no
construction reads a frame (S3P round 8), nothing else reads `Offset`, so it goes (Ev 1: "many ways to say these
things").

**3. Stated symmetry.** One rule covers both kinds: each constraint must lower the residual's dimension **modulo the
shape's stated symmetry**, or it refuses (#4325, E5).
- A round pin's `Spin` after an `Axis` mate lies within that symmetry. The pin is pinned without it, and a `Spin`
  value refuses as fixing nothing.
- Anything free beyond the symmetry with no value leaves the copy **loose**, per Ev's #4325 hope. A loose copy's pose
  is display state only (E1). Export, measures that need its pose, and booleans (E6) refuse, naming the free
  coordinates.

**4. Text.**
- D10 Operations: "A placement (`Place`) reads one shape and defines its copy, constrained by mates and values on
  equal footing. A mate equates, modulo its kind's symmetry, a pose of the copy with a pose of what it joins, both
  read off geometry. A value sets one coordinate the mates leave free, in the chart of the frames the mate reads, to
  a scalar variable. Each constraint must fix something the rest leave free, modulo the shape's stated symmetry, or it
  refuses. A copy with nothing free outside that symmetry is pinned; otherwise it is loose, and nothing that needs its
  pose reads it. Nothing moves a body."
- Spaces: "A **space** is a set of bodies related by placements; nothing else, no coordinates, makes one."
- Relied on: E1, E2, E5/Ev 4, E6, Ev 1, Ev 2. Bent: A11 (`Under` becomes loose), and my own earlier "clause" wording.

## Round 7 — reconcile: I concede to one constraint kind

1. **Yes, and `Value` goes.** Round 6 kept `Value` for one reason: a set of values that leaves some coordinates free
   (a slide set, the spin free) has no coset form. That is true, but the need behind it is not real.
   - Every **pinned** copy is reachable with mates alone: `Frame ≅ Offset(F, chain)` covers any values whatever.
   - A partial set only distinguishes **loose** copies, and a loose copy has no pose. All that partial set changes is
     display state, which is "never anything real" (E1).
   - So `Value` would be a second way to say what mates say (Ev 1), and it goes.
   - **A correction to A's summary:** a perpendicular `Plane` mate after a `Plane` mate leaves `Prismatic`, so it fixes
     the slide **and** the spin together, not "a slide alone". That is harmless for the same reason: the copy is
     either pinned (where the spin also had to be fixed) or loose.
2. **Yes, `Frame ≅ F` would have equalled `Plane ≅ F` plus three zeros.** Under one kind, no zero is written, so Ev's
   rule on zeros never arises.
   - Several mate sets can still fold to the same coset (`Frame` vs `Plane` + `Plane` + `Direction`). That is not two
     mechanisms. It is one mechanism with a canonical form, the fold, the same way two formulas can be equal.
   - Where a number appears, it is a scalar variable in the target pose's definition, never a constant standing for
     an omission.
3. **Yes, my chart's "translate, then spin" was a chosen convention.** Ev would rightly see it as arbitrary.
   - In A's form, each number sits in a construction the user wrote: an `Offset` along a direction read off
     geometry, or a `Rotate` about one.
   - The mates fold by coset intersection, which does not depend on order. So nothing is ordered by the kernel.

**Final D10 sentence.** "A placement (`Place`) reads one shape and defines its copy. Each of its mates equates, modulo
its kind's symmetry, a pose of the copy with a pose of what it joins, both read off geometry. Every number is a
scalar variable in those poses' definitions (`Offset`, `Rotate`). A mate that fixes nothing the others leave free,
modulo the shape's stated symmetry, refuses. A copy with anything free outside that symmetry is loose and has no
pose: nothing reads it that needs one. Nothing moves a body."

## Round 8 — the order-free chart, and the one condition values need

1. **Yes, the order-free chart holds.**
   - On `Planar`, (x, y of the copy frame's origin in the target frame, the angle between their references) are global
     coordinates of SE(2). On `Cylindrical`, (the axial coordinate, the angle about the axis) are global coordinates.
   - Each is a function of the relative pose alone. None composes another, so no order is chosen. That answers my
     round-7 objection, and I withdraw my round-7 concession: Ev's #4325 hope is per freedom, and the values carry it.
2. **I accept, with one condition. Here is the case that fails without it.**
   - Take `Plane ≅ F`, then `SlideX = dx`, then a `Point` mate (a copy point `p` at offset `r` from its origin, to a
     partner point `q`).
   - The point fixes the position `t = q − R(θ)r`. The value then demands `(q − R(θ)r)ₓ = dx`, which is a cosine
     equation in θ with two roots, or none.
   - So a value's level set is not a coset. `{tₓ = dx}` holds rotations about every point of a line. Once a mate folds
     against it, the solve leaves coset intersection, and the two-branch answer it yields is decided by no structure
     (A11 (1), and E5's "no cleverness").
   - **Condition:** values chart the residual of the bundle's mates, folded first, because the constraints are a set,
     not a sequence. A value's `coord` must be a generator of that residual, and is otherwise refused.
     - The door refuses an added mate that would remove a free coordinate a value already sets, and names the value.
       The solve refuses the same for a state the door never saw.
     - So in the case above, the `Point` mate refuses at insert, naming `SlideX`. Its own residual, `Revolute` about
       `q`, has no x-slide.
   - **What the condition keeps.** Ev's case is a mate, then values on what it leaves, then a later mate that removes
     only the coordinates no value sets. That still works. The copy is loose until mates and values cover every
     coordinate outside its stated symmetry, and is then one point, with no branches.

**D10 sentence.** "A placement constrains its copy by mates and values on equal footing: a mate equates, modulo its
kind's symmetry, a pose of the copy with a pose of what it joins, both read off geometry; a value sets one coordinate
of what the mates leave free, charted by frames read off geometry, to a scalar variable. A constraint that fixes
nothing the rest leave free, modulo the shape's stated symmetry, refuses, and so does a mate that would take a
coordinate a value sets. A copy with anything free outside that symmetry is loose and has no pose."
