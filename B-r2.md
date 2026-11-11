# Fork `clear-enclosure` — designer B, round 2

Shown: designer A's round-1 report.

## For Ev

**Recommendation now (likely).** Drop the separate cell-bound type. Take the other report's three changes, and add one of mine: the engine itself keeps its cell readings out of the driver's log, so no caller has to remember to.

1. **One crossing, owned by the lane.** A stored value enters as the reals it stands for: itself at f64 and Probe, its bracket at `Interval` and `Sym`. The shell's `lift` parameter goes.
2. **No second reading.** Where the engine decides that something is exactly Zero about an edge or a vertex, it takes the body's own verdict instead of re-measuring it over a cell.
3. **A refusal names which width blocked it:** the family's (only a narrower parameter box answers) or the cell's (refinement or budget).
4. **The sweep owns its log disposition.** Cell readings never reach the node's log, by the engine's door. The one reading that is the family's leaves as the typed refusal of change 3, and the caller logs that.

**What moved me, and why.**

- **The Zero places are real, and they are the sharper fault.** I checked three of the four in `topo::injective`:
  - `self_intersection_on_shared` (`is_zero`): decides that a face's shared side lies on a separator, Zero at the band.
  - `self_intersection_apex`: decides that a vertex is the cone's apex, by distance Zero.
  - `self_intersection_cusp`: decides that two normals are opposed, Zero.

  Each is a fact about an edge or a vertex that holds by construction. Enclosed over a family as two independent evaluations it widens with the box and stops reading Zero. The fourth, a meeting's separation, is already read off topology (`Enclosure::separation`'s own doc gives exactly this reason), so the tree has made this fix once.
- **My design did not address it.** A separate cell-bound type would still have read these identities numerically over the family. My first report said `Sym` earns "at the point reading" but did not find these sites. Change 2 is needed.
- **The newtype objection holds.** I assumed the cell bound need not be a full scalar. It must be. The engine's values are 3-vectors of intervals (about 80 typed sites across the engine's files), and the kernel's vector type gives sums, dot, cross and norm only to a `Real` scalar. So a cell-bound type would be a second interval scalar by delegation. That is the shape you retired on PR 2701.

**What I keep.** The fault I named is real, and the other report lists it under "one width, two levers" as kept by hand. But change 3 alone leaves it kept by hand: each caller still wraps the sweep in an unlogged scope.

- `wire_clearance` wraps it (audit row F30). The shell's `cavity_embedded` runs the same sweep with no scope (read, not run).
- F29 reads each margin twice to avoid logging.
- The at-rest gate would be the next caller to remember.

Change 4 moves that scope inside the engine's door. It is a door, not a type, so an engine site could still call the lane's `decide` directly. If you want that closed too, a gate on the engine's files in C9's style would do it. I lean no gate: one door is enough.

**One caveat on change 2 (likely).** Not every Zero place has a matching tier-3 verdict.

- `self_intersection_on_shared` also runs against separators the engine builds itself (a plane through straight shared edges, the plane of a conic). Tier 3 certifies an edge on its face's carrier, not on those planes.
- The spec should say, for each separator kind, which of two things applies:
  - tier 3's verdict is read; or
  - the separator is built from the shared edge, so it vanishes there by construction and nothing is decided.
- A numeric re-reading should remain nowhere.
- The apex and cusp places do match tier 3 readings (chart singularities as vertices, and the material wedge).

**Unchanged from my first report.**

- f64 enters as itself, and the band is in the reading, not the crossing.
- `Sym` fits the engine but decides nothing over a cell. Its whole benefit arrives through change 2.
- The `[topology.tiers]` rewording: the band decides at every scalar, and the stored values stand for the body itself at a point scalar and for the family at `Interval` and `Sym`.

**Confidence.**

- Recommendation: likely.
- The three Zero places are second readings: sure, read from the code.
- Dropping the type: sure.
- Change 4 over caller-kept scopes: likely.
- The caveat on engine-built separators: likely.

## For the orchestrator

- **Converged with A** on changes 1–3 and on rejecting the range type. Designer B's remaining addition is change 4, which A's report describes as hand-kept but does not change. The one open detail is whether a gate backs it; designer B leans no.
- **Checked this round.**
  - `injective.rs`: `is_zero` (line ~702, used at ~4376, 4422, 4432), the apex decide (~1675), `cusp` (~4818).
  - `clearance.rs`: `Enclosure::separation` (~1273).
  - `geom-core/src/linalg/vec.rs`: every `Vec3<T>` impl is bounded `T: Real`.
- **Not checked.**
  - Whether the gate can hand tier 3's per-edge verdicts to `boundary_of`. A flags the same.
  - Whether the cusp site's quantity (the normal sum times the ball's reach) is the same number as tier 3's wedge reading, or only the same fact.
- **Still stands from round 1.** The shell's missing unlogged scope is a live defect at `Interval` and `Sym<Interval>` (read, not run). It belongs in `work/clear/`. Designer B did not file it, since designer B may not push.
- **Nothing was built.** No target directory exists.
