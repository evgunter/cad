# M5 S6 — two-tolerance message-unification sweep (binding spec)

Executes consequence (ii) of the ratified two-tolerance principle
(#129; DESIGN.md D4 ¶1 addendum): below ε_input, user messages and
recourse never fork on exactly-on vs in-band — one message, one
recourse (declare / move / lower), margin as payload data. Kernel
semantics keep the distinction. **Message-level only; no predicate,
verdict, or control-flow change anywhere.**

## Scope — the ten audited pairs (pointers verified at audit time)

1. profile `UndeclaredTangency`/`TangentialContact` vs
   `Escalated{SegmentPair}`
2. boolean `UndeclaredCoincidence` vs `Escalated` (adjacent arms on
   one `decide("bool_plane_offset")`)
3. census `UndeclaredContact` vs `CensusEscalated` (one `decide` in
   `gap_is_zero`) — **the escalated arm currently loses the recourse
   sentence entirely via `{:?}` Debug formatting; fix regardless**
4. `SplitParamNotInterior` vs `SplitParamEscalated`
5. split-join `DegenerateSection` vs `Escalated`
6. sweep `DegenerateExtrusion` vs `ExtrusionEscalated`
7. sweep `DegenerateAngle` vs `AngleEscalated`
8. sweep `DegenerateAxis` vs `AxisEscalated`
9. sweep `VertexCrossesAxis` vs `SliverRadius`
10. certify `NotTransverse` vs `Escalated`; props `DegenerateFace`
    vs `Escalated`

## Rules

- **Variants stay.** Typed payloads are kernel data; this sweep
  unifies the *Display text*, not the type structure. If a pair
  looks collapsible (the two single-decide sites especially), note
  it in the report — do not collapse here.
- **One carrier.** The shared `Indeterminate` Display string in
  predicate.rs is the single source of the unified recourse text;
  per-site Display impls compose it (site context + margin payload +
  shared recourse) rather than hand-rolling near-duplicates. Follow
  the `merge_coplanar_faces` precedent for tone/shape.
- **Recourse text names all three levers**: declare the coincidence,
  move the geometry, or lower the tolerance — phrased once, in the
  carrier.
- **The census `{:?}` bug** (recourse sentence lost through Debug
  formatting) is fixed as part of pair 3.
- **Out of scope** (verbatim from #129): predicate/verdict semantics;
  `TangencyContradicted`/`StaleContactDeclaration`/index-domain
  errors; editor-core `Diagnosis`; the far-vs-band pairs
  (`ResidualExceeded` vs `Escalated` etc.) — different user
  situations, leave alone.

## Acceptance

- For each of the ten pairs: both arms' Display output is the same
  user situation described the same way — same recourse sentence,
  differing only in honest payload data (margins, ids). A test per
  touched module pins the shared recourse fragment (grep-style
  `contains` on the carrier const, not full-string pins that rot).
- Existing message-pinning tests updated, not deleted; any full-text
  pin that resists the carrier refactor is reported.
- No public enum variant added/removed/renamed; no `decide` call
  changed; bit-battery unaffected by construction (messages only).

## Process

Branch `ev/m5-s6-messages` from main. Standard battery (workspace
default + interval lanes, clippy both, fmt, doc), push per unit,
adversarial e2e review + fix pass, PR by orchestrator.
