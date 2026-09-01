# BOOL-8 — issue 433 half (i): the line-continuation junction and `line(len)` off a directed point

**Binding at dispatch** (S-BOOL program, `docs/S-BOOL-PLAN.md`;
difficulty logged pre-draw: **M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
This unit executes a RATIFIED ruling — the Q1 ruling and its
second-round extension in `docs/S-BOOL-PLAN.md` §Rulings (Evan,
in-chat, 2026-09-01) are the primary specification, together with
the §4 amendment text quoted below, which Evan reviewed in-chat.
Issue 433 does NOT close here (it closes when BOOL-9 also lands).

## Situation

`junction_check` (`crates/profile/src/path.rs`, ~:1502) refuses
EVERY zero-turn forward junction as `JunctionTangent` — the
undeclared-tangency doctrine over-applied. For line-onto-line the
turn is carrier IDENTITY, which `validate`'s `tangent_joints`
semantics already treats as legal undeclared data. Per §2c of
`docs/PATHS-DESIGN.md` a directed point is `(position, tangent)`
and NOTHING else, so a plain `line` leg after a `line` leg IS the
straight continuation: the line through the point along its tangent
is the whole carrier, no extra data, no identity decision, no new
predicate. The ruling: spell it `line(len)` off a directed point —
NO new verb (`line_continue` was drafted and dropped) — with the
tangent inherited bitwise so consecutive legs are exactly parallel.
The use case is named: lily's loft vertex budget (the named-gap
comment in `demos/tour/src/lily.rs`, ~:870–905), the use case the
`ArcContinueNeedsArcCarrier` doc's "no use case" premise denied.

## Deliverables

1. **`line(len)` accepts a directed-point start**: the verb-table
   transition gains `directed point → directed point`. The leg
   departs along the point's own intrinsic tangent, inherited
   BITWISE (consecutive legs exactly parallel even where derived
   vertices round). Binding bits only; there is no junction (no
   authored direction exists to classify) and nothing is declared.
   The minted vertex is a structural subdivision of the carrier.
2. **`junction_check`'s zero-turn-forward arm narrows**: the
   structural continuation is not a junction and is never
   classified (nothing reaches the check — preferred if the
   construction gives it no authored direction to classify), while
   an AUTHORED direction landing in the tangent band still refuses
   `JunctionTangent` with the recourse naming the structural
   spelling. Curved zero-turn junctions, cusps, and
   `SameCarrierJunction` (declared identity, #101's rule) all keep
   refusing — untouched, with rows proving it.
3. **PATHS §4 amended** with the reviewed text, verbatim (design
   surface — see Hard rules):
   - the verb table's `line(len)` row transition becomes
     `Directed → Point; directed point → directed point`, with the
     row text: "off a directed point, the straight continuation:
     the leg departs along the point's own intrinsic tangent.
     Binding bits only; there is no junction (no authored direction
     exists to classify) and nothing is declared. The minted vertex
     is a structural subdivision of the carrier — the loft
     vertex-budget shape.";
   - §4 item 1's recourse sentence extends: "…if intended as
     tangency onto a new carrier, use `.tangent()`, which makes it
     exact by construction; if intended as a straight continuation
     of the same line, spell it `line(len)` off the directed point
     — no junction exists there; otherwise move the geometry (or
     lower the tolerance).";
   - §4 item 4 clarifies: same-carrier junctions refuse when
     DECLARED — `.tangent()` onto the incoming carrier is identity,
     not tangency (#101's rule). The structural continuations —
     `line(len)` off a directed point, the post-fillet extension —
     are not junctions at all: the departure is the point's own
     tangent by construction, so nothing is checked and nothing is
     declared.;
   - the OPEN (#433) block becomes a ruling record: the lattice and
     `validate` agree — a subdivided straight run is well-formed as
     data (validate, unchanged) and expressible structurally
     (`line(len)` chaining); an authored direction landing in the
     tangent band still refuses, recourse as above. Per the ruling's
     second-round extension, `arc_continue` is NOT kept as the
     axiom's exception: it is scheduled for REMOVAL (BOOL-10), its
     subdivision need re-spelling as declared subdivision on the arc
     leg itself. Companion: `RawLoop` is not an authoring door — the
     vertex table is the materialized form (BOOL-9). #433 closes
     when both land.
4. **Lily migrates onto the lattice** in the same unit — the demo is
   the ruling's demonstration and the named-gap close: the
   subdivided straight run spelled as `line(len)` chaining off the
   first leg's directed point, through the public surface. The
   named-gap comment retires. The loft's vertex COUNT is the need;
   derived endpoints are the honest form (an authored collinear
   target is exactly the value-coincidence the ladder refuses). If
   the render changes, measure and state the delta in the PR (the
   render lanes gate).
5. **The `ArcContinueNeedsArcCarrier` doc's retired premise
   rewritten** — the "no use case" sentence is false since
   LIB-RETTAIL and the ruling retires it; the DOC is this unit's
   (the verb's removal is BOOL-10's — do not touch the verb, its
   eval arm, or the wire format).
6. **Rows**: the continuation chain accepted end-to-end (red on
   main — today's refusal names `JunctionTangent`); exact
   parallelism pinned at the bits; the vertex-count/structural-
   subdivision shape pinned (the lily shape as a fixture); curved
   zero-turn still refuses; cusp still refuses; declared identity
   (`SameCarrierJunction`) still refuses; an authored in-band
   direction still refuses with the new recourse text.
7. **ε posture** (issue-1356): the continuation itself is exact by
   construction (bitwise tangent), so state which reads remain
   band-sensitive in the touched arms (the tangent-band membership
   for AUTHORED directions) and pin per-band where they are.
8. **Class sweep** (discipline §5): every zero-turn / tangent-band
   comparison in `junction_check` and its callers; the three
   #433-stance prose sites updated to the ruling.

## Acceptance

- The continuation-chain row red on main, green under the change;
  lily builds through the public surface with the named-gap comment
  gone; all keep-refusing rows green; PATHS §4 carries the amended
  text verbatim.
- Hosted CI green on the final head; gate record per head in the PR.

## Hard rules

- NO `Co-Authored-By` trailer, no model names in commits. "issue
  433" spelled out, no closing keywords (the issue does not close
  here anyway).
- **The PR does not merge on green.** PATHS §4 is design surface:
  the PR carries the amended junction semantics for Evan's eyes
  before merge (the text was reviewed in-chat 2026-09-01; the PR
  body must quote the one paragraph that differs from the reviewed
  draft — the ruling record's arc_continue sentence, which follows
  the ruling's own second-round extension). The orchestrator holds
  the merge for the sign-off.
- Scope fence: `crates/profile/src/path.rs` (junction_check, the
  line verb's state acceptance), profile suites,
  `demos/tour/src/lily.rs`, `docs/PATHS-DESIGN.md` (§4 + the verb
  table row + the `ArcContinueNeedsArcCarrier` doc site if it lives
  there; the in-code doc site in `path.rs` too). NOT: `RawLoop` or
  any lib.rs façade change (BOOL-9), `arc_continue`'s verb / eval /
  wire (BOOL-10), `validate` semantics (unchanged by the ruling),
  schema, `docs/MODEL-AB-LOG.md` / `docs/S-BOOL-*.md` / SMELL
  edits. `crates/profile` is SMELL track V fence ground — if the
  work reaches V's rows, stop and report.
- Re-merge main before opening the PR.
