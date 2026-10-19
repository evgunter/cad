# Arrival-constructed arcs — designer report (round 6: the question beneath)

## For Ev

**Recommendation (likely, with the deciding argument in §5): the arc IS the arrival side — an ARC arrival, ending at its anchor.** The verb is `arrive()`, no arguments, on a point tip (plain point, leg end, or the entry point). It opens an arc-arrival builder: `.at(p)` and a director (`.angle(θ)` / `.toward(dx, dy)`) in either order build the arc from the tip to `p` arriving tangent to the direction, and the tip lands as a directed point at `p` — incoming the direction, departure free — exactly as `Radius`/`Via` arrivals land today. The seam: `.to(Start)` is the tangent closer (`Start`'s direction by reference, joint 0 constructed, no token; issue 1578), and `.at(Start)` with a director is the sharp seam with its arrival authored (the `Via { q, p: Start }` close's own shape, as a binder), classified as every seam is, a zero turn refusing `SeamTangent` with `.to(Start)` as recourse. Start cells, departure classification, degeneracies, exclusions (Directed/`Angle`/`Open` tips; `spec₂`; fused), and the `Via`/`Center` token half are as both designers wrote in rounds 4–5 and are unchanged. No split.

**Definitions.** *Line arrival*: a fillet's far side given as a line through an on-path anchor; the arc's tangent point on it is DERIVED and the side runs on past the anchor. *Arc arrival*: a fillet's far side given as an arc ending at its anchor; the anchor is the arc's end and the tip lands there with its departure free.

### 1. What the author means
"Arrives along `d` at `p`" states the arc's own end tangent. Whether anything then runs along `d` is the next verb's fact, not this one's: in the `B`, the spine follows the lower bump at a right angle, and nothing runs west from the foot. The author is describing the arc's end, which is the arc-arrival reading. Sure.

### 2. The two builders
The line grammar's two cells are the arc grammar's one cell plus `.tangent()`: `arrive().at(p).toward(d).tangent()` continues along `d` exactly as the line reading's `.at` would, and the line reading's `.to(p)` is the arc reading's `.at(p)` under another name. Nothing is reachable under the line reading that the arc reading lacks; the line reading reaches one thing twice. Not a capability, a re-spelling. Sure.

### 3. The seam row
`.at(Start)` with a director on an arc-arrival builder adds one row with an existing meaning (`ViaArrivalStart`'s: anchor at the entry, director pending, the close classified). The far-end `.toward(d).to(Start)` would instead rule §2a item 4's open form for every fillet verb's line arrival — a close in the middle of a side — which `at-cannot-bind-start-in-the-path-form` left open on purpose and no case asks for. The `.at(Start)` row is the "radius-arrival close" that row named as the vocabulary addition it would be; whether `Radius` arrivals also take it is that separate item, not this one. Likely.

### 4. The name
`arrive()`. `arc_onto` names a side to land on, which is the line-side reading; `half_fillet` says fillet of a verb with no `r` that authors a vertex.

### 5. The deciding argument
What decides it is the anchor's role, and it is a fact about the ratified grammar, not a preference. In every line arrival the anchor is "a real on-path point" ON the side and the arc's tangent point is derived from `r` and the carriers — the two are distinct, which is why `AnchorOutsideTrimmedExtent` exists (a trim that would eat the anchor refuses) and why every continuation "runs PAST" the anchor (§2a item 4). This verb has no `r`: its arc is determined only if the tangent point IS the anchor. A line-arrival spelling would therefore be the first line arrival whose anchor is forced to be its own tangent point, its every `.to(p)` the exact-fit degenerate case by construction, and its `.at(p)` a tangent joint no rule in the line family derives. It would borrow the line rows while obeying none of their semantics. In the arc-arrival grammar every existing rule applies unchanged: the anchor is the arc's end (a hard anchor, §2c dissolution), the tip lands as a directed point, the builder holds its director pending, and the entry-anchored close with a pending director already exists. One reading needs a new rule; the other needs none. Likely.

**`B`, entered at the foot:** `Open.at((0, 0)).toward(0, 1).line(2).toward(1, 0).tangent_arc_to((0, 1)).arrive().at(Start).toward(-1, 0)`. Entered elsewhere: `.arrive().at((0, 0)).toward(-1, 0).toward(0, 1)…`. D with a tangent cap at the entry: `.line_to(tip).arrive().to(Start)`.

**Ratified text:** §2 Fillet / §2c family list (the arc-arrival builder standing alone, start pinned at the tip), §3's table, a new §2a item; §2 Closure / §6 PQ4 name `arrive().to(Start)` beside the seam fillet and carry the token's check-or-supply wording. §2a item 4's far-end `Start` form stays open.

## For the orchestrator

- Evidence this round is already in hand from rounds 1–5: `radius_complete` / `via_complete` return the `NoAng` directed-point state; line `At` rows on `Angle` return `DirectedPlain`; `AnchorOutsideTrimmedExtent` and §2a item 4's "runs PAST" sentence; `ViaArrivalStart`. No new reads; read-only; nothing built or committed.
- The two designers' round-5 crossover is resolved on B's round-5 (= A's round-4) cells; the one new row is `At(Start)` on the new builder states (`At` takes `Point2` today, so a Start-accepting row or a target-typed argument), with the viewer's `at` row gaining a target picker on those states only.
- Residual to state in the PR, not to decide here: whether `RadiusArrival` takes the same `.at(Start)` close (the "radius-arrival close" the closed row named).
