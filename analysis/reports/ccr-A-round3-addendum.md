## For Ev

**Short answer: yes, it can be a kernel bug, if three things are done first.**
- **The rule.** A conic stored at its centre must have no semi-axis longer than the run's *rounding reach* R. R is the length the admission rule already defines in "Float rounding": the longest lever over which a short sum's rounding still fits inside the tolerance. It is about 5,000 km at ε = 1e-9 and about 5 km at ε = 1e-12.
- **The rule does not choose a form.** Each door stores what it holds. The rule only checks that a door holding nothing but a centre never makes a conic too long for one. Breaking it is a panic, because the kernel chose wrong.
- **What has to be true before that panic is a bug and not a user error:**
  1. The kernel's own section doors that can make a long conic always store a point of the arc. That is by door, not by size, because their sizes are unbounded.
  2. Every door where a person sets a conic's size refuses past R with a typed error, before it builds the conic.
  3. Both of the above read R through the one door where the tolerance is read.
- **Moves, reversals and re-seams cannot break it:** none of them changes a size.
- **Not a weird cutoff:** it is the admission rule you already have, read as a length, and asked in one place. The new measurements show no step at the box edge, so the session box is not what to ask here.

**Terms.**
- *Centre-stored*: the variant of the conic's placement that holds its centre.
- *Arc-stored*: the variant that holds a point of the arc it serves, its parameter zero there.
- *R*: the rounding reach, `box·(K − 1)·ε / ADMISSION_FLOOR`, read through `Tolerance`.

**Why size, and not the centre's position.**
- A person can move a part, and an accepted move keeps the edge in the box but can carry a centre anywhere. An invariant on where the centre sits could therefore be broken by a move, with no typed refusal first. That would make the panic reachable by a person.
- A size does not move. With the arc in the box and every semi-axis at most R, the centre is within box + R of the origin. Summing the vertex from it then rounds within the headroom R is defined by.

**Door by door** (the lane's survey, checked against the tree where noted)

*Doors where a person sets the size or the centre:*
- **A sketch arc placed by extrude, sweep or revolve** (`sweep/src/swept.rs`).
  - Stores: centre-stored. The profile layer keeps a centre for every arc spec, and the rim's symbolic identity is registered on that centre.
  - Already guarded: profile validation refuses a far, flat arc with a typed `ArcBelowSceneResolution`, before any conic exists (`profile/src/validate.rs`). The bound it reads is `K·ε/(n·2⁻⁵²)` against the centre's magnitude, close to R but not R.
  - Needs: that check restated as "radius at most R" through the same door. The `ConstructedProfile` path skips these checks, so it needs the same one. (likely)
- **Revolve latitudes** (`AxisFrame::latitude`).
  - Stores: centre-stored, on the axis. That is what keeps "a latitude over a centre box is exact".
  - Not guarded today: the radius is the profile vertex's distance from an axis the person places. A partial revolve about a far axis keeps the body in the box, so no refusal fires before the conic is built.
  - Needs: a typed refusal of an axis distance past R at the revolve door.
- **STEP `CIRCLE` / `ELLIPSE`** (`step-import/src/entities.rs`).
  - Stores: centre-stored, from the file's placement.
  - Not guarded: sizes are checked only for being finite and positive.
  - Needs: a typed import refusal past R. An alternative is storing a point of the arc at the edge's vertex, which the file supplies.
- **`recognize_curve`'s circle** (import). It holds sample points of the arc, so it stores arc-stored and needs no check.
- **Scaling a body.** I found no uniform-scale door in the editor; its moves are rigid. The one scaling in the tree is offset's `replace_face::homothety` (radius·k), which a person reaches through the offset distance.
  - It keeps centre-stored, with the centre mapped exactly.
  - Needs: a typed refusal when radius·k passes R, before it builds.
- **A move, a mirror, a pattern.** Sizes are unchanged, so the rule is safe with no check.

*The kernel's own section doors* (no person sets the size directly):
- **Must always store a point of the arc** (each can pass any length; none holds an arc point today, so each needs one computed from local data):
  - the plane–cylinder tilted ellipse;
  - the plane–cone circle across the axis;
  - `bisector_ellipses`, which can take the node it computes just after the mint;
  - offset's sphere and torus seam circles, where `p_start` is already in scope.
- **Already arc-stored:** the plane–cone tilted ellipse (its near vertex), `transport_curve` (its mid point), and blend's oblique end face (its feet).
- **Bounded by an operand's own size, so centre-stored is safe** if that operand obeys the same rule one level up (the quadric-datum row): plane–sphere and sphere–sphere sections, coaxial cone, torus and cylinder circles, torus parallels, the cylinder's rim circle, and blend's trims and surgery circles.
- **Unsure:** `junction::scaled` (blend). Its radius comes from a caller I did not trace; it needs a check or an arc point.

**Which doors cannot make it a kernel bug today:** revolve, STEP import, offset's `homothety`, the `ConstructedProfile` path, and possibly `junction::scaled`. A person can reach each of them before any refusal has had its say. Each needs its one typed refusal past R; the section doors need their arc points. With those in place, the panic marks only kernel mistakes.

**The rest of the answer stands.**
- One placement type with two variants, each holding only its own numbers, behind the one `Conic` reading. This is the family pattern of `Curve3`, and of the sketch arc's specs in `[recipe.forms]`.
- Each variant sums its points by short chords from a point of the curve.
- Every reader states the curve through the conic.

**Sentence for "Float rounding"** (`crates/geom-core/README.md`):
> "A stored form that loses digits is re-minted in a well-conditioned form: a conic stores the point its mint holds, its centre or a point of the arc it serves with its parameter zero there, as one of a family of placements behind the one `Conic` reading, and every point is summed by chords from a point of the curve. A conic stored at its centre has no semi-axis past the rounding reach `R`; each door a person sizes refuses past it typed, and a mint that breaks it is a kernel bug."

**Confidence.**
- The panic can be made a kernel bug: likely, once the five doors above are guarded.
- Size, rather than the centre's position, is what survives a move: sure.
- R is the right length: likely. It matches the measured headroom; the exact constant is the lane's to pin.
- The door list: likely. It is the lane's survey, re-checked against the tree for profile validation and editor moves only.

## For the orchestrator

**What moved me**
- **Ev's comment brings back a limit as an assertion, not as a choice of form.** My round-3 position ("no limit; certification catches a far centre's shift") becomes the backstop rather than the rule.
- **I state the limit on size against R, not on the centre's position.** A position invariant is broken by an accepted rigid move, because `framed.rs`'s image check asks the session box of the region only. So it cannot be a kernel bug.
- **R, not `SESSION_BOX_EXTENT`.** The size table shows no conditioning step at the box. R is the admission rule's own length. The geom-core README already makes it "read through one `Tolerance` door, never a site's own constant".

**Checked in the tree** (on `origin/oneround/anchored-conic`)
- `profile/src/validate.rs`: `ArcBelowSceneResolution`, headroom `K·ε/(n·2⁻⁵²)` less the check's magnitude, read at the centre's magnitude ("a flatter arc stores its centre farther out"). It is the one existing typed refusal that precedes a conic mint. It is close to R but neither the same constant nor the same door.
- The module docs say a `ConstructedProfile`'s arcs "were verified at their construction and skip" these checks. Whether that construction runs the resolution check is unverified.
- `editor-core/src/eval/framed.rs`: a move asks `topo::session_box_at` of the image, which is a region test only. I found no scale door in `editor-core/src/eval` or `topo` beyond offset's `homothety`.

**Not checked**
- Revolve's axis-distance handling.
- `entities.rs`'s exact checks; I took "finite and positive" from the survey.
- `junction::scaled`'s caller.
- Whether `Circle::new` and `Ellipse::new` can read the run's R. They take no tolerance, so the assertion may have to sit at the mint doors, or read a session-scoped tolerance. That is for the lane.

**For the lane**
- ε-dependence: R is about 5e6 m at ε 1e-9 and about 5e3 m at ε 1e-12, both with K = 10. So one document can pass at one ε and refuse at another. That is consistent with the admission rule, and Ev should hear it.
- The quadric-datum row (`a-quadric-surface-is-stored-by-a-datum-that-may-lie-far-from-its-face`) is what keeps the "bounded by operand" doors safe. The same R rule on a surface's own size closes it.

**Housekeeping:** read-only; I read only the files named above and the survey. Nothing was edited or run.
