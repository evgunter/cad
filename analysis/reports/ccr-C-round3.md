## For Ev

**Short answers**

- **How ugly?** Small, if built as a family. A circle and an ellipse each get two little types, one stored about its centre and one stored about a point of its arc, behind the one `Conic` reading every consumer already uses. Outside `conic.rs` nothing knows which it holds.
- **Why not one form?** Each single form is measured to fail somewhere.
  - Centre only: a cone section toward the parabola reads 5e-8 m off its own cone at `Interval`, even at a 17 m semi-axis. Stored at its vertex it reads 6e-16.
  - Arc point only: two concentric authored circles are no longer provably concentric over a tolerance box.
- **Is it the family pattern?** It should be. What is built is not (below).
- **No cutoff.** Agreed, and none is needed. Each construction always makes the same kind. No number picks.
- **A panic for ill-conditioned input?**
  - For the kernel choosing the wrong kind: yes, that is a kernel bug, and no person can cause it.
  - For a size limit on the centre kind: no. A person reaches any such number through four doors with legitimate geometry (table below). It is also not needed: the centre kind can read as well as the arc kind at any size (measured to 10 km).

**(a) The final state**

- **The pattern in the tree.**
  - Recipe layer: a sketch arc is written as `Bulge`, `Via`, `Center`, `Sweep` or `ArcLen`. Each keeps only its own numbers, and all lower to one `Segment::Arc` that every reader reads (`[recipe.forms]`).
  - Kernel layer: `Curve3`'s kinds each hold their own data behind its evaluators, and `Circle` and `Ellipse` sit behind `Conic`.
  - Common shape: each member is its own type with no unread field, there is one reading, and no consumer asks which member it has.
- **What is built is not that.** It is one struct with a private tag (`Born`), a frame origin that means "centre" or "arc point" by the tag, and a `phase` field documented "unread on an ellipse born with a centre". That is one type meaning two things.
- **What is:**
  - *Centre kind* holds the centre, the axis and seam directions, and the sizes. Its parameter is the angle from the seam.
  - *Arc kind* holds a point of the curve, the directions, the sizes and, for an ellipse, where on the ellipse that point sits. Its parameter is zero at that point.
  - The shared API is `Conic`: point, derivatives, inverse, directions, cover, centre, seam vertex. The centre is the stored value on the centre kind and derived on the arc kind. Only `conic.rs` matches on the kind.
- **What decides the kind:** the construction, the same way every time.
  - A construction whose centre runs off to infinity at a pose it can reach (a cone cut toward the parabola, a cylinder cut toward its axis) always stores a point of the arc.
  - A construction that is handed a centre (an authored centre, a circle about an axis) always stores it.
  - Nothing converts one kind to the other.
- **Each kind reads from a point of the curve.** The arc kind reads from its stored point. The centre kind reads from its seam vertex, summed once from the centre. Measured: this certifies exactly what the arc kind does, digit for digit, from 10 m to 10 km. (likely; the `Interval` side is not measured)
- **Operations:**
  - A *move* maps the stored point and directions. The kind stays.
  - A *reversal* flips the axis. The stored point and the kind stay.
  - A *re-seam* turns a centre-kind conic exactly about its centre. The arc kind has no re-seam, because its parameter zero is its stored point.
  - A *scaling* scales the stored point about the pivot and multiplies the sizes. The kind stays.

**(b) The size limit: drop it.** (likely)

- A limit at the centre door would be the cutoff you ruled out: an authored arc of radius 999 m and one of 1001 m would be stored differently.
- The measurement shows no natural place for it. The centre sum loses gradually (40 of 40 poses certified at 500 m, 33 at 900 m, 28 at 1 km, 7 at 1.5 km). It never gives a wrong answer at ε = 1e-9, at any size.
- What replaces it: the centre kind's seam-vertex reading (40 of 40 at every size), and the fixed choice of kind per construction.

Door by door, can a person reach the centre kind with a far centre before any typed refusal? No door today checks a carrier's size or centre. The session box is asked of where the body is.

| Door | What decides the kind | Reachable by a person? |
|---|---|---|
| Authored `Center` arc | the person wrote a centre: centre kind | Yes: a far centre, or a 3 km radius with the centre in the box |
| Authored `Bulge`, `Via`, `Sweep`, `ArcLen` | the person wrote points and tangents: should be arc kind at the start point | Yes today: lowering keeps only a 2-D centre, which is far as the bulge goes to 0 |
| Circles about a surface's axis (rims, latitudes, sphere and torus circles) | centre kind | Yes: a gently curved wall has its axis kilometres away |
| Scaling a body | should keep the kind | Yes: a shallow arc scaled ×10 |
| STEP circle or ellipse | the file wrote a centre: centre kind | Yes: the file's sizes |
| A move | keeps the kind | Size cannot change, but a centre can leave the box |
| Runaway-pose sections | arc kind, always | No. A centre kind here is a kernel bug |

So a numeric limit cannot be a kernel-bug-only panic. The first five rows are real geometry inside the box, and for them neither a panic nor a refusal is right.

What can be a kernel bug is the last row. Four such mints still store a centre today: the cylinder's tilted ellipse, the equal-cylinder bisector, the oblique end face, and STEP's recognised circle. Each needs a point of the arc. The last three already hold one; the cylinder cut needs a near-vertex formula like the cone's.

**(c) What the measurements changed**

- "Size alone picks the kind" is out: the near-parabolic table above is under 1 km.
- "A box-sized centre form stays within the floor" was too strong. It refuses from about 900 m. The seam-vertex reading removes that.
- The symbolic row lost its zero because the sweep registers the centre's own node, not through arithmetic. Storing the centre keeps the node.
- The 1e7 cover is off the true arc on main too. Confirmed.

**(d) The sentence** ("Float rounding")

"A stored form that loses digits is re-minted in a well-conditioned form: a conic is one of a family behind the one `Conic` reading, stored about its centre or about a point of the arc it serves with its parameter zero there. Its construction fixes which, the same at every size. Each evaluates from a point of the curve, the centre form from its seam vertex. Every reading of where a conic is goes through the conic."

**Confidence**
- Two kinds as a family, chosen per construction: likely.
- No limit: likely.
- The seam-vertex reading in the centre kind: likely at f64, unsure at `Interval`.
- The door table: by reading, not run.

## For the orchestrator

**What moved me, and to what**
- **From one struct with a placement pair to two types per kind.**
  - The pair `(0, 0)` is an in-band value standing for a kind.
  - `[recipe.forms]` says "one enum whose variants each keep exactly the numbers", and that is what Ev asked for by name.
  - The pair also needed `0·x` to fold at every scalar, which nobody has checked.
- **From "the centre door refuses past the box" to no limit.** Three things together:
  - Ev's second comment.
  - The size table's graded loss, which I predicted last round, with no step at the box.
  - The door check for the addendum: the limit is reachable by a person at every outside-fed door.
- **The chord reading moves from "at the reader" into the centre kind's evaluator.** The size table's "reader-chord" row is this arithmetic, and it is what makes a limit unnecessary. PR 7194 rejected it as "every bit moves for 5×". The size table shows 28 → 40 at 1 km and 0 → 40 at 10 km, and bit moves are not weighed.

**Held, and now supported**
- Mint's datum over size alone: `small-cone-sections-at-interval/README.md`, the near-parabolic table.
- The cover and the symbolic mechanism: `two-facts.md`. My round-0 "different expression" was wrong in mechanism, as I conceded in round 1.

**Facts behind the door table (by reading `origin/oneround/anchored-conic`)**
- No session-box or size check exists in `crates/profile/src`, `crates/sweep/src/swept.rs` or `crates/topo/src/replace_face.rs` (grep for `SESSION_BOX_EXTENT`, `session_box`, "past the box").
- `sweep::swept` mints a sketch arc with `Curve3::circle_on` from `Arc2`'s centre and `u_ref = (q − c).normalize()`. Its seam vertex is the start vertex `q_from` in exact arithmetic, so the seam-vertex reading starts at the arc's own start.
- `replace_face::homothety` rebuilds `Curve3::circle(map(center), axis, radius·k, u_ref)`. It reads `.center()` and goes through the centre door, so it turns an arc-kind circle into a centre kind. Under the family it maps the stored point instead.
- The survey's "holding only a centre" list is the lane's delegated read. I did not re-check each line.

**What each door needs**
- **Authored arcs.** The kind should follow the spec the person wrote: `Center` gives the centre kind, the others the arc kind at the leg's start. `Segment::Arc` carries only `Arc2 { centre, radius, sweep }`, so the spec is erased at lowering and the 2-D centre is itself far as the bulge goes to 0. This is the conic's question one level up, in `profile`. Not filed. Until it lands, every sketch arc is a centre kind and a tiny-bulge arc's carrier sits about u·R off its vertices.
- **Circles about a far axis.** These are `a-quadric-surface-is-stored-by-a-datum-that-may-lie-far-from-its-face`. No conic-level rule fixes them. The conic reads consistently with itself; the mismatch against the surface is the surface's.
- **The four runaway-pose mints.** One row each, or one row for the four. These are the kernel bugs in Ev's sense, and the only place a panic could mean one. A debug assertion would have to sit in each mint, since nothing numeric at the centre door tells a computed centre from a given one.

**Unchecked, and open details**
- **The centre kind's seam-vertex sum at the enclosure scalars.**
  - `(c + u·a) + u·a·(cos θ − 1)` holds the size twice: up to 3·w(a) on the far side, against w(a) for `c + u·a·cos θ`.
  - The size table is f64 only.
  - If it costs certified `Interval` builds, the centre kind keeps `centre + size·direction` at the enclosure scalars, as `Ellipse::quarter_turned_where` already reads differently by `T::WITNESS`.
  - I would want the lane's number before the sentence says "each evaluates from a point of the curve" without qualification.
- **Circles have no re-seam door.** A centre-kind circle's far side reads at u·ρ under the seam-vertex sum, the same order as the centre sum. `a-far-circle-has-no-re-seaming-door` stays open.
- **The size table's family cannot separate size from centre position**, as its README says. Nothing in my answer depends on that separation now.
- **At ε = 1e-12 the anchored form shows "wrong" cells at ρ = 1e4.** The README attributes them to the check (an f64 near 2π is 8.9e-12 m of arc apart at that radius). That is the parameter-grid argument for "parameter zero at the stored point", seen in a measurement. It applies to the centre kind at 10 km too. A centre-kind conic that large can hold an edge's ends only near its seam, so `seamed_near` stays load-bearing for large centre-kind ellipses.

**Text**
- `[recipe.forms]` says a form joins a family "only because it is how someone authors the value, never for the kernel's convenience". The kernel-layer family is stated only by Ev on PR 6329. The proposed sentence makes the conic the first written instance. Whether DESIGN should state the kernel-layer pattern is a separate, smaller question for Ev.
- The README's "the session box … never a carrier's stored terms" now stands untouched.

**Process**
- Read-only. I read the round-3 message file and the tree. Nothing was run.
- I treated the addendum with Ev's third comment as part of the question, as the coordinator's message directed.