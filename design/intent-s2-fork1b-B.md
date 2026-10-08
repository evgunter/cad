# FORK-1b — the pose kinds as a lattice (designer B)

## For Ev

**Recommendation (likely).** Your instinct holds, with one correction: the kinds form a *partial order*, not a lattice. Combining two values gives a kind that depends on where they are, so a combination is always an explicit construction with a fixed output kind, never an inferred "join".
- The revolve reads its axis **in the profile's own sketch coordinates**, so "lies in the profile's plane" cannot be false and nothing checks it.
- The revolve **defines its world axis as an output**. A circular pattern, a tube or a coaxial mate reads that output.
- Axis-first is your construction: `frame_on(axis, point)` gives a frame whose x-axis is the axis. A profile drawn on that frame and revolved about its x-axis is structurally on the original axis.
- Neither `AxisInPlane` nor a definition check survives.

**Premise check.**
1. *"A profusion of names."* The value kinds are fine as D10's five: `Point`, `Direction`, `Axis`, `Plane`, `Frame`. The profusion is elsewhere:
   - pose information spelled as loose scalar triples (a plane's normal, a transform's rotation axis, a linear pattern's direction, a tube's `u_ref`);
   - one stored shape read as two kinds.
2. *Your aside: does an axis have an origin?* As stored, yes: `Datum::Axis` holds an origin and a direction.
   - As a kind, no. D10 compares axes "modulo slide and spin along itself", and a circular pattern reads only the line.
   - The tube reads that origin as its centre, so two tubes on one line with different centres would compare equal by D10 and still build different bodies.
   - The tube really reads a `Frame`: its centre, its spine direction and its `u_ref`.
   - `Datum::Plane` is the same: it stores an origin that the kind forgets. (sure)
3. *"A lattice."* The join is value-dependent. An axis plus a point gives a full frame when the point is off the axis, and only a point on a line when it is on it. So kinds cannot have a join operation. A combination has to be a named construction that refuses the degenerate case. (sure)

**1. The structure (likely).** A pose kind is a frame known only up to a group of rigid motions, its *symmetry*. That symmetry is exactly what D10's coincidence paragraph forgets.

| kind | forgets (symmetry) | dof |
|---|---|---|
| `Frame` | nothing | 6 |
| `Plane` (oriented) | in-plane slide, spin about the normal | 3 |
| `Axis` (oriented line, no origin) | slide and spin along itself | 4 |
| `Point` | every rotation about it | 3 |
| `Direction` | every translation, spin about itself | 2 |

- *Order:* K is above L when every K value determines an L value. A frame determines its origin and its sketch plane; a plane its normal; an axis its direction.
- *Unordered pairs:* `Point`, `Axis` and `Plane` are mutually unordered. That is your observation: an axis and a plane combine only by construction.
- *Kinds and elements.* The kinds a variable can have are these five. Other elements of the order are not kinds until some reader needs one:
  - a point on an axis (today's stored `Datum::Axis`);
  - a line in a plane (`AxisInPlane`).
- *Sketch coordinates.* The sketch plane has the same five-kind order in 2-D. A frame lifts a 2-D value into the 3-D order (a 2-D line in frame F is an `Axis` lying in F's plane). Profile coordinates are already 2-D values.
- *Orientation.* Kinds are oriented. D10 forgets slide and spin, not sense, and a mate's `AxisSense` is authored. `reversed(v)` flips a value.

**2. How kinds combine (likely).** There are two families, and both are definitions (`Expr` over pose variables, as `w * 2` is over lengths), not nodes: they build no geometry.
- **Projections** go down the order and are total. They never refuse:
  - `direction(axis)`, `normal(plane)`;
  - `origin(frame)`, `plane(frame)`;
  - `axis(frame, X|Y|Z)`;
  - `lift(frame, 2-D value)`.
- **Constructions** go up the order or sideways. Each has a fixed output kind and refuses its degenerate input with a margined verdict (Q1): Zero refuses, the sliver band refuses, a definite value builds.

| construction | refuses when |
|---|---|
| `frame_on(axis, point)`: origin at the point's foot on the axis, x along the axis, y towards the point | the point is on the axis |
| `axis_through(point, point)` | the points coincide |
| `axis_through(point, direction)`, `plane_through(point, normal)` | never |
| `plane ∩ plane → Axis` | the planes are parallel |
| `axis ∩ plane → Point` | the axis is parallel to the plane |
| `frame(point, z, x_ref)` | `x_ref` is parallel to `z` |

- **The rule.** An incidence (an axis in a plane, a point on an axis) is never an input condition checked between two values. It holds because both values are projections of one construction. This is D10's "meant to coincide → construct it".
- So **your example builds a `Frame`, not a `Plane`**. A `Plane` output would forget the axis, and "contains the axis" would again be a fact about values.
  - The axis is a projection of that frame. Its plane is another projection of the same frame.

**3. What a slot admits (likely).** A slot holds exactly its kind, so unit B's door stays equality (`SlotVarKind`) and the wire has no subtyping.
- Where the projection is unique (a `Plane` at a `Direction` slot means its normal), the *authored* door lowers a finer value through it and mints an anonymous defined variable (VR4, VR6). That is sugar, like "a node id means port 0".
- Where it is not unique (a `Frame` at an `Axis` slot), the door refuses with `AmbiguousProjection`, naming the choices.
- **Rejected: a slot that admits every finer kind.** The stored read would then carry information the reader ignores, so a spin of a frame would move a token whose geometry it never touches. Every reader would also have to project for itself. With the projection stored, D10's "frame modulo the kind's own symmetry" is written in the document, not re-derived. (likely)

**4. The revolve's axis (likely).** `Revolve { profile, axis: a 2-D line in the profile's coordinates, angle }`. It defines `body: Body` and `axis: Axis`, where `axis` is the lift of the 2-D line through the profile's frame.
- **What it makes true.** An out-of-plane axis cannot be written (the audit's F15 tilt), and there is no check at all, so today's `written_against` frame comparison retires.
- **The profile's closing edge.** An edge lying on the axis can read the same 2-D values, so "the profile closes on the axis" becomes structural rather than a value coincidence.
- **Sharing.** A pattern, tube or mate reads `revolve.axis`.
- **Axis-first worked example.** A shaft axis A exists. Write `F = frame_on(A, p)`, draw the profile on F, and leave the revolve's axis as F's x-axis. Stage 4 proves `revolve.axis ≡ A` by the identity `axis(frame_on(A, p), X) = A`.
- **Against the alternatives:**
  - A 3-D `Axis` with a door check makes every legal revolve from a world axis a value-decided coincidence, which is what D10's lint exists to flag.
  - `AxisInPlane` as a kind still compares two frames.
  - A flag kind (a line in a plane) moves that comparison without removing it. Reversible: a flag kind can be added later as an element if a slider mate needs it.

**5. What it meets.**
- **Stage 4 coincidence (likely).** "Modulo the kind's own symmetry" is the projection to the kind. Canonical forms are normal forms of pose `Expr`s. The new part is a finite table of identities, one per projection–construction pair, such as `origin(frame(p, …)) = p` and `normal(plane_through(p, n)) = n`.
- **Mates, A11 (1) (sure).** The closure `{Se3, Planar, Cylindrical, Prismatic, Revolute, Trivial}` is the list of these symmetries:

  | subgroup | symmetry of |
  |---|---|
  | `Trivial` | `Frame` |
  | `Planar` | `Plane` |
  | `Cylindrical` | `Axis` |
  | `Revolute` | a point on an axis |
  | `Prismatic` | a line in a plane |
  | `Se3` | nothing |

  - A primitive equates the two sides' frames projected to one kind: `FrameCoincidence` is `Frame`, `Coaxial` is `Axis`, `PlanarRest` is `Plane`. Coset intersection is the value-level combination.
  - `Clocking` is refused because a bare angle is no kind.
  - A ball mate (`Point`) or a parallel mate (`Direction`) would need symmetries the closure lacks. That is stage 3's choice, and the order says what each would add.
- **Circular pattern:** reads an `Axis`, as today.
- **Linear pattern:** a `Direction` and a `Length`.
- **Split:** a `Plane`.
- **Tube:** a `Frame`, replacing its axis datum and loose `u_ref`.
- **Sketch on a face (`FaceFrame`):** becomes the definition `frame_of(face, spin)`.
- **`PlaceInWorld`'s pose and the mate frames:** a `Frame`.

**6. Sequencing (likely).** #4222's FORK-1 shape can merge and unit A can be built now. Unit A must carry three things:
1. **No `AxisInPlane` kind.** The diff already leaves it out.
2. **`Revolve`'s signature is `body: Body` and `axis: Axis` from the start.** Then the later move of the axis into the revolve changes only a read, and the output ids are not minted twice. Until then the port is the lift of whatever axis it reads.
3. **`Datum::AxisInPlane` defines an `Axis`.**

The datum nodes stay operations through stage 2. They become free or defined pose variables at stage 3, when the free arm lands, and the constructions above become `Expr` operators then.

**7. Ratified text that changes.** The D10 sentences are agent-written in PR 3990, and you ratified the model; the lists were not separate decisions.
- **D10 Variables.** Current: "the geometric values (`Point`, `Direction`, `Axis`, `Plane`, `Frame`)". New: "the poses (`Point`, `Direction`, `Axis`, `Plane`, `Frame`), each a frame known up to its kind's symmetry, ordered by which determines which. A slot holds its own kind; a finer value is read through its projection, and an incidence between poses (an axis in a plane, a point on an axis) is a construction over one variable, never a check between two."
- **D10 Coincidence**, after "an axis forgets slide and spin along itself", add: "; a projection of a construction reduces to what it was built from". Current: "Coaxiality is one `Axis` variable read twice". New: "Coaxiality is one `Axis` read twice, directly or as projections of one construction".
- **ASSEMBLY A11 (1).** Current: "the closure is `Subgroup::{…}`". New: "the closure is the symmetry groups of the pose kinds a primitive equates, `Subgroup::{…}`".

**Confidence:**

| Claim | Confidence |
|---|---|
| A partial order, not a lattice | sure |
| The tube is mis-typed today | sure |
| The mate closure is the symmetry list | sure |
| The revolve's 2-D axis plus its `axis` output | likely |
| Coercion at the door rather than subsumption | likely |
| Constructions as `Expr` operators rather than nodes | likely |

## For the orchestrator

- **Contamination, disclosed.** `get_comments` on #4222 returned all its comments in one call, so I saw your comment sketching an answer. I disagree with it on two points:
  - it calls the structure a lattice, and the join is value-dependent;
  - it has the revolve read a combined element admitted where an `Axis` is asked. I reject subsumption, and I make the revolve's axis relative to its profile.
  
  Weigh my convergence with it on "axis plus point" accordingly.
- **Provenance.** The checkout is shallow (a graft), so `git log -S` on D10's sentences hits the graft. I relied on the FORK-1 reports' reading of PR 3990.
- **Off the question, worth a row.** The tube uses `Datum::Axis`'s origin as its centre (`eval/wire.rs` `tube_args`), while D10's axis forgets slide. Once stage 4 compares axes canonically, two tubes on one line with different centres become "structurally one axis" but different bodies. File it under INTENT stage 3 (`tube-spine-reads-an-axis-origin`). I did not file it, since the brief said to change nothing.
- **Today's revolve is design A at value level.** `wire_revolve` accepts only `DatumValue::AxisInPlane`, and the circular pattern and tube accept only `Datum::Axis` (`mate/member.rs` `axis_datum`). So today you cannot pattern about a revolve's own axis datum. The revolve's `axis` output fixes that.
- **Spec amendments if taken:**
  - §1 and §2: `Revolve` gets two ports.
  - Q5's port-0 sugar keeps `body` at port 0.
  - Test 3 checks that a revolve's output axis is readable at a circular pattern.
- **Not weighed:** whether `Body | Bodies` admission (FORK-1) should also become door coercion through an explicit pick. It has the same shape, but it is not a pose question.
