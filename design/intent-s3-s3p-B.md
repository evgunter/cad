# FORK-S3P — what a free pose is, and what a space's member is (designer B)

## For Ev

**Recommendation** (likely).
1. **A free pose is a `Frame` with no value: a space's own frame.** Every other pose is defined.
   Its numbers are scalar variables in a definition (coordinates in a frame, a rigid offset of a
   frame), and they carry its tolerances. A document may hold any number of free frames, each a
   separate construction space. The world is the one undeletable free frame, and no construction
   reads it.
2. **The computing frame is chosen per operation, not per space.** An operation computes in the
   frame of the earliest *member* its reads reach. A member is a free frame other than the
   world, or a copy (a placement's output). Spaces remain: they decide what may combine, what
   the product is and what the census compares. They carry no frame. When a new mate relates two
   spaces, no computed bit moves. Only the world map that export composes changes.

**Premise check.** Part 1 follows from D10's own definition of a pose ("a frame known up to its
kind's symmetry"), carried to its limit. A frame known up to *every* rigid motion has no value.
That is Ev's "they just don't have a location (or orientation)", and it is the only free pose
there is. A pose that carries coordinates is not free: the coordinates are in some frame, so it
is a definition over that frame. Part 2 is where the defect is. "Each space computes in the
frame of its earliest member" makes an operation's bits depend on something it does not read:
which member is earliest in its whole space. That contradicts D10's "Reading is the only
dependency". It also breaks D9 in exactly the case the brief names, two spaces becoming related.

### Part 1 — the free pose

- **What it is.** `Free` for a `Frame` holds nothing: no value, unit or distribution. It is a
  *free frame*, the frame its space's poses are written in. A free `Plane` would be a frame
  known up to everything, which is not a plane, so only a `Frame` may be free.
- **Tolerance.** A free frame has no value, so it has nothing to tolerance. A pose's uncertainty
  is the pushforward of its definition's scalars (VR3 and VR8 unchanged): a datum's position
  tolerance is a band on its coordinates. A zone tolerance (a joint distribution over several
  scalars) is a later question.
- **How many.** A document is born with one. "A new space" mints another, as an ordinary
  variable that can be named and is deleted only when nothing reads it (VR7). Bodies built from
  different free frames are unrelated. An operation whose reads reach two spaces refuses, as a
  boolean does. A body enters another space only as a copy placed there. This is Ev's first idea
  of 2026-10-03: build the tool beside the part in its own space, place it, then do the boolean.
- **The world** is a free frame that no construction reads and that is never a member. Export
  reads its coordinates, and moving it is one mate edit.
- **Rejected:**
  - (a) No free poses, with the seed as a non-variable "space" entity. This is coherent, but
    slots (a sketch on "a new frame") and placements ("the copy's frame") must read the seed.
    Making it a variable reuses reading, naming and VR7.
  - (b) A free pose as a frozen value in its space's frame. That is `InFrame` with coordinates
    that no tolerance or sharing can reach (VR4, VR8): the absolute datum, one level down.

### Part 2 — members and the computing frame

- **Members** are the free frames other than the world, and the copies. A construction is not a
  member. A copy is the rigid image of its body, and its frame is its body's free frame as the
  copy carries it.
- **The rule.** An operation computes in the frame of the earliest member (by mint order) that
  its reads reach. The walk stops at a free frame or a copy, and never goes through a copy into
  its source. The choice reads the recipe only, never a value and never the world. So the frame
  is a function of the reads and is keyed in the memo like everything else. A body built on `F`
  computes in `F`.
- **What it makes true that the per-space rule cannot.**
  - An edit moves exactly the bits of the operations downstream of it in reads. D9 holds without
    needing the word "unrelated".
  - A copy's face reads bit-identically to its source body's face (`transform_rigid` maps
    `u_ref` covariantly), so a sketch on it turns as on the original. DM1's zero spin comes from
    the minting operation's frame, which is a function of the recipe.
- **Two spaces related by a new mate.** A copy with an empty bundle (its own space) gains a mate
  into another space. It and every copy placed against it join that space. No existing operation
  changes what it reaches, and the relative poses inside the joined group do not depend on where
  it sits, so nothing recomputes. The census gains the new pairs, the product may gain copies,
  and the world map changes. Spaces rooted at free frames never merge: a placement copies a body
  into a space and leaves the source where it was.
- **Worked example.** Copies `Q1` and `Q2` are mated only to each other. A sketch sits on a face
  of `Q1 ∪ Q2`. `W0`, minted earlier, is in the world. Now `Q1` gets a world mate.
  - *Per space:* the world space's earliest member is now `W0`. The union recomputes in `W0`'s
    frame and its bits move. Where the union minted the face's carrier (`orthonormal_basis`,
    `topo/src/boolean/join.rs`), the u-reference can change branch and the sketch turns
    (likely).
  - *Per operation:* the union reaches `{Q1, Q2}`, stays in `Q1`'s frame, and nothing moves.
- **Candidate B, per space as written.** Its gain is one coordinate system per space, shared by
  the census and the viewer. Its costs are those above. Choosing "the space's root" instead of
  "earliest" fails for the world's space, whose root, the world, is not a member. I lean against
  it (likely). The choice is reversible: only the frame each operation picks differs, so
  switching moves bits and no representation.

### Migration

- **Absolute datums** become `InFrame { the first free frame, the same scalar variables }`. That
  frame is today's coordinates, so every construction is bit-equal. A part's world placement
  becomes a copy mated to the world at its pose.
- **`Space::Own`** (an unplaced group at its earliest instance's identity) becomes copies whose
  earliest has an empty bundle. Operations that reach that copy compute as today. A census pair
  that does not reach it computes in the earlier of the pair's frame, so it moves by rounding
  with no verdict change (likely).
- **Placed groups** leave world coordinates. Their digests move by each copy's world map, as
  spec unit E expects.

### Ratified text that changes (quoted)

- **D10 Variables.**
  - "the poses (…), which may be free or defined" → "the poses (…), of which a `Frame` may be
    free and the rest are defined".
  - "A variable is **free** — a value, its written unit (D6) and optionally a distribution — or
    **defined**" → "A scalar or discrete variable is **free** — a value, its written unit (D6)
    and optionally a distribution. A free `Frame` has no value: it is a space's own frame.
    Otherwise a variable is **defined**". This is agent text from the FORK-1 `[ev]` PR
    (`ae1f6464e7`).
- **D10 Spaces.**
  - "A **space** is a set of copies related to one another; a part is born in its own space" →
    "A **space** is a set of members (free frames and copies) related to one another; a document
    is born with one free frame".
  - "a document builds in a frame of its own" → "… in free frames of its own".
  - "The kernel computes each space in the frame of its earliest member, chosen from the recipe
    and never from values or from the world, so an unrelated edit moves no bit (D9)" → "The
    kernel computes each operation in the frame of the earliest member its reads reach, never
    the world's, so an edit moves no bit of an operation whose reads it leaves alone (D9)".
  - This is agent text from the D10 draft (`84404cdbf8`), approved with #3990. Ev's own words
    were "chosen near the geometry and depends on nothing about the world node" and "i would've
    expected there to still be some freedom". Both are kept.
- **D10 Booleans:** "A boolean's operands must already be in one space" extends to "An
  operation's reads lie in one space; otherwise it refuses".
- **Consequential re-wording:**
  - VR3: a `Frame`'s `Free` arm carries no payload.
  - A11 (2)'s "with its earliest instance at that frame's origin" retires into D10.
  - DM1's "the frame the carrier stores is `orthonormal_basis`'s" gains "in the frame its
    minting operation computes in (D10)".

Confidence: a free pose is a valueless `Frame`: **sure**. Per-operation over per-space:
**likely**. The claims about carriers minted by booleans and about census rounding: **likely**.

## For the orchestrator

- **Spec unit E under this answer.** E becomes the per-operation choice, keyed in the memo by
  the earliest reached member.
  - `world_of` survives as composition along the bundle tree, for export and display only.
  - Test 15 becomes "a copy's body digest equals its source's": a copy is a body plus a pose.
  - Test 16's "reordering `Place` nodes" names no edit (mint order is fixed); it needs a delete
    and re-add.
  - Test 17 holds by construction.
- **A bundle's mates must read one space.** Their targets are expressed in the earliest target
  member's frame. A bundle reaching two unrelated spaces refuses. The spec does not say this.
- **`InFrame` of kind `Frame` vs `Offset`** (spec §1) are two spellings of "a frame relative to
  a frame", against "one way". Ask before unit A; I did not weigh it.
- **FORK-S3-5:** a part's world must not become the assembly's computing frame. The instance's
  copies are members, and its `frame` port is read only by mates. I assumed this.
- Ev's verbatim transcript (`5f7a1c71e3`) needs `git fetch --unshallow`.

## Round 2

**1. Per-operation rule: I hold mint order and move towards A's anchor.** Under F's one-space
bundles, and since spaces rooted at free frames never merge, each space has at most one free
frame. So the rule becomes: *an operation computes in its space's free frame if its reads reach
it, otherwise in the frame of the earliest minted copy it reaches; never the world.* This adopts
A's "a construction computes in its free frame, nothing to choose" without exceptions. It closes
my gap where a construction on a later free frame could have computed in an earlier copy's
frame. Mint order rather than operand position decides it for me, for three reasons:
- **Union as a set.** Under FORK-S4U a union's members are a set folded in mint order, so "the
  first copy it reads" has no meaning there except mint order.
- **Same bits for structurally equal operations.** With operand position, `A ∪ B` and `B ∪ A`
  compute in different frames and their bits differ. D10 calls the two one construction.
- **A's mixed case.** When a boolean reads a construction and a copy, A's rule needs an extra
  clause; mine has the free frame outrank the copy.
Position's one merit is that the person can steer the frame by reordering operands. The frame is
invisible and moves only bits, so that freedom buys nothing. My earlier argument is not answered
by A, and A's locality argument holds under both rules. If Ev picked position instead, nothing
representational changes: one function picks a different member and bits move. The choice is
reversible either way at bit-move cost only, so it is a minor call for Ev.

**2. D9: I move to A.** D9 says "same build + same inputs → bit-identical outputs", which is
determinism and not locality. Locality is D10's own promise, a property of each operation, and
D9 is cited only for determinism. I also adopt A's D6 re-word: "Every free continuous variable's
value" becomes "Every free scalar's value".

**3. The world: there is no representational difference, but A's door rule needs fixing.**
- Both reports make the world one free `Frame` marked undeletable. "Never a member" is just my
  rule's statement that nothing computes in the world, which A also says.
- A's "no definition may read it" is too strong. A copy's world mate reads `Offset { world, pose
  }` (spec C's migration of `PlaceInWorld`), and that is a definition reading the world.
- The rule I propose: *a pose reaching the world is read only by a placement's mates*. That is
  "construction never reads the world" as one door check over reads.

**4. Points the reports raise separately.**
- Bundles read one space: we agree. **A definition reaching two free frames refuses at the door:** I adopt it. It is my "reads lie
  in one space" rule placed at the definition door, with the operation door as the
  evaluation-time twin.
- `InFrame` vs `Offset` as one thing spelled twice: still an orchestrator question.

**Proposed D10 text.**
- *Free variable:* "A variable is **free** — for a scalar or a discrete kind, a value, its
  written unit (D6) and optionally a distribution; for a `Frame`, nothing: a free frame is a
  space's own frame, which everything written on it is measured from — or **defined** …"
- *Poses:* "… the poses (…), of which a `Frame` may be free and every kind may be defined."
- *Computing frame:* "Nothing computes in the frame of a space. An operation computes in its
  space's free frame if its reads reach it, else in the frame of the earliest-minted copy they
  reach, and never in the world's, so a bit moves only under an edit on the path the operation
  reads; D9 makes that result the same on every build."

## Round 3

**1. Ev's principle bears on the frame rule: the claim "the frame moves only rounding" is false.**
Several carriers take their reference from `orthonormal_basis` in the computing axes, which is a
convention and not rounding: sweep cap planes (`sweep/src/swept.rs`), Newell planes
(`geom-brep/src/newell.rs`), and a split's one-corner section (`section_loops.rs`, the fallback
in `chord_u_ref`). If the frame changes, the reference changes branch, so a zero-spin sketch on
that face (DM1) turns by a finite angle. Verdicts can also flip at the edges of the sliver band.
Both are rare but visible, so the frame choice is arbitrariness that reaches the person, and
Ev's rule applies: don't hide it. (Correction to round 1: the `join.rs` site I cited is test
code.)

**2. The three candidates.**
- **Mint order hides an order.** It is invisible and not editable, and a delete-and-recreate
  silently changes which member decides. It is the case Ev rejected for the union.
- **The operands' common frame is order-free, but it exists only when every read reaches one
  member.** That covers a construction on one free frame, or a split of one copy, but not a
  boolean of two copies. Its fallback is an order again.
- **The first member reached in the operation's stated order is the author's visible
  statement.** Stated order means signature order, or a list's listed order. It coincides with
  the common frame wherever one exists, so it is order-free exactly where an order-free answer
  exists and says its order where none does. It is also the order that D10 Booleans already
  makes visible ("keeping one fixed operand's description"), and the union's list (#4323).

**3. I move from mint order to stated order.** Locality holds, because the deciding member is a
read. The D10 sentence I would now propose:

> Nothing computes in the frame of a space. An operation computes in the frame of the member
> its first read reaches, in the order it states (its signature, a list's listed order): a free
> frame other than the world, or a copy. Where every read reaches one member, no order enters.
> A bit moves only under an edit on what the operation reads or how it is ordered; D9 makes
> the result the same on every build.

**For the orchestrator.** The deeper fix: carriers that mint a reference by `orthonormal_basis`
derive it from their inputs instead (a cap from the path frame, a section from the cutting
plane's reference). The frame then moves only rounding. File it as a topo issue whichever rule
Ev picks.
