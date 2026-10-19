# Arrival-constructed arcs — designer report (round 5: the two details)

## For Ev

**Recommendation (likely).** The verb is `arc_onto()`, no arguments, on a point tip. It opens the fillet family's ordinary arrival side — the LINE side, bound by the same binders every fillet's line arrival takes — with a third kind of incoming: a bare point, the tip, where the arc starts untrimmed with no tangency asked. The arc from the tip is tangent to the arrival line at the anchor, which is both the anchor and the tangent point (no `r` exists, so the DOF the trim would take is closed by pinning the tangency at the anchor). Everything else is the family's existing grammar, unchanged, which is what settles both details: the other designer's spellings are right on both, and mine are withdrawn.

**Definitions.** *Arrival side*: the far side a fillet verb's binders author, a real on-path anchor plus a direction. *Far-end form*: `.to(anchor)` on a bound arrival direction — the side ENDS at the anchor (§2a item 4). *Resolution kinds*: the family resolves an arrival as `Continues` (`.at`: the tip stays Directed on the side), `EndsAtAnchor` (`.to(p)`: a directed point, next direction free) or `Seam` (`.to(Start)`), one kernel (`resolve_fillet`).

### The cells — the whole set

| step | result |
|---|---|
| `arc_onto()` from a plain point (incl. the entry `Open.at(p)`) | opens the arrival side; at the entry the derived departure becomes the entry direction, as `arc_to(Bulge)` does |
| from a directed point (leg end) | opens it; the derived departure (the arrival direction reflected across the chord) is junction-checked when the side binds: definitely sharp proceeds, in-band tangent refuses `JunctionTangent` (recourse `.tangent().tangent_arc_to(p)`), reverse refuses `JunctionCusp` |
| from a Directed tip, `Angle`, `Open` | ill-typed (the bound direction would have to value-match the derived one — the `Center@Directed` rule; a half-bound tip has no position) |
| `.at(p)` + `.angle(θ)` / `.toward(dx, dy)`, either order | the arc from the tip to `p`, tangent there to the line through `p` along the direction; the tip stays Directed along it and the next leg continues on the side (`Continues`, a constructed tangent joint at `p`), exactly as after `fillet(r).at(p).angle(θ)` |
| `.angle(θ)` / `.toward(..)` then `.to(p)` | the same arc; the side ends at `p` with no run (the exact-fit rule: the side IS the arc), the tip a directed point with incoming = the direction, departure free (`EndsAtAnchor`). This is the `B`'s lower bump away from the seam, and the sharp corner with the next leg free |
| `.to(Start)` from the unbound side | the tangent closer: the arc arrives along `Start`'s direction by reference, joint 0 constructed, no token — the seam fillet's own spelling (issue 1578's closer) |
| `.angle(θ)` / `.toward(..)` then `.to(Start)` | the sharp seam with its arrival direction authored: the side ends at the entry vertex, the seam checked against `Start`'s direction; a zero turn refuses `SeamTangent`, recourse `.to(Start)`. The far-end `Start` form §2a item 4 left open "if a case appears": the `B` is the case, and it lands on the shared arrival state, so every fillet verb's line arrival gets it (at the entry, with no side open, it refuses as `FarEndAnchorWithoutFillet` does) |
| `.at(Start)` | ill-typed: `at` binds a position and leaves the side running PAST it, which at the entry vertex means nothing (the closed `at-cannot-bind-start` row: Ev's `at(Start)` was the seam fillet mis-spelled) |
| `.at(p)` alone, then an arc leg | inherited: a plain point whose direction the next arc supplies backwards (`Center@Point`), two arcs meeting tangent by construction |
| degeneracies | direction along the chord ahead: the straight segment; behind: `DegenerateArcChord`; anchor at the tip: degenerate chord — `tangent_arc_to`'s mirrors through its kernel |
| as `spec₂`, as a fused incoming, with an arc arrival spec | none: no `r`, nothing to trim, not an `arc_to` mode; an arc-arrival variant ("tangent to the circle about `c` at `p`") is a different verb if ever asked |
| `Start.arrives_tangent()`, `Via`/`Center` | unaffected: the token belongs to target slots; this verb closes with `.to(Start)`; the `Via`/`Center` half of the PR stands |

**Detail 1 (the sharp seam).** The family's grammar is the far-end form, not `.at`: a side whose natural end is a vertex is spelled `.angle(θ).to(p)`, and §2a item 4 already names `.angle(θ).to(Start)` as the well-defined `Start` case awaiting a need. Spelling it `.toward(d).to(Start)` adds one row on the state every fillet verb shares; `.at(Start)` would add a position binder with a special meaning on one verb. Sure.

**Detail 2 (`.at(p)` + director).** The binders bind a SIDE, and this verb's side is the unmarked line (its arc is the fillet-position arc, tangent to that line at the anchor); arc arrivals are the ones that carry a spec. So line grammar applies: `.at` continues, `.to` ends — the `Continues` / `EndsAtAnchor` kinds the one resolution kernel already distinguishes. My "lands as an arc arrival" would have needed states of its own and given the `B` no sharp corner; the other designer's argument answers it. Likely.

**Name.** `arc_onto()`: it names the arc and the side it lands on, and reads at every cell (`.arc_onto().at(p).toward(d)`, `.arc_onto().toward(d).to(p)`, `.arc_onto().to(Start)`). `arrive()` named only the end and read oddly before `.at`; `half_fillet` says fillet of a verb that takes no `r` and authors a vertex.

**`B`, entered at the foot:** `Open.at(foot).toward(0, 1).line(h).toward(1, 0).tangent_arc_to(mid).arc_onto().toward(-1, 0).to(Start)`. Entered elsewhere: `.arc_onto().toward(-1, 0).to(foot).toward(0, 1)…`. D with a tangent cap at the entry: `.line_to(tip).arc_onto().to(Start)`.

**Confidence.** Both details, as above. Verb and cell set: likely. Token half unaffected: sure.

## For the orchestrator

- Read this round: B-r4, §2a item 4, `ArrivalKind` and `resolve_fillet`'s three kinds (`crates/profile/src/path.rs`), the `FarEndTo` row (on the shared `Angle` state), `Pending { Ray, Arc }` (`verbs.rs`), the closed `at-cannot-bind-start` row. Read-only; nothing built or committed.
- Implementation shape: a third `Pending` kind (a bare point) plus its resolver (the circle through the tip tangent to the arrival carrier at its anchor); the arrival states and their `At`/`Angle`/`Toward`/`FarEndTo`/`CloseTo` rows are reused, so `.at` → Directed falls out by construction. One genuinely new row: far-end `Start` on the `Angle`-with-pending state (a `FarEndTo`/`CloseTo` overload taking `Start`), which also admits `.fillet(r).angle(θ).to(Start)` for every fillet verb — §2a item 4's "Open, deliberately" paragraph is the ratified text that changes (agent text from the G1 round; provenance not run).
- Both my round-4 details were wrong against the code; no residual disagreement with B on the cells. The one open line is B's inherited `.at(p)`-then-arc cell: coherent, but the PR should state it rather than let it arrive silently.
