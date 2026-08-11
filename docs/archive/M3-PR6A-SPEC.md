# M3 PR 6(a) binding spec — tier-3′ validator + touching corpus

Orchestrator-authored binding spec (2026-07-23). Deviations must be
REPORTED in the implementer's summary, not improvised. Charter sources:
M3-PLAN item 6 (the non-docs parts), M3-LOG "Accumulating PR 6
obligations", F1/F2 (M3-PLAN, ratified — F2 carries Evan's
explicit-intent condition verbatim). Branch: `ev/m3-6a-tier3prime`.

## Scope

IN: `validate_pseudomanifold` (tier 3′) + declared-contact
certification; contact carriage across result minting (descendant
map); honest `Intersection` descriptions on split/boolean minted
edges (retire the test-only upgrade helper); below-copy minting at
BOB pinch vertices (retire the negative-side `DegenerateSection`
refusal); saddle fixture for the 15.11 pairing guard; the
touching-configuration corpus at rest; closure stress tests;
mass-properties/STL runs on 3′ bodies; k_stats name-tagging of every
new predicate.

OUT (goes to 6(b)): DESIGN.md ratification text, tier-table update,
K-telemetry snapshot WRITE-UP (the data is produced here), voids
documentation, PERF-PLAN fold-in.

## D1 — validator shape (binding)

`pub fn validate_pseudomanifold<T: Decide>(body: &Body<T>, contacts:
&ContactRecords) -> Result<(), Vec<ValidationError>>` in
`crates/topo/src/validate.rs`, structured as:

1. Coarse-gate on tier 2 (as `validate_geometric` does).
2. All of tier 3's local checks (surface implementedness, carrier
   re-certification + description-adjacency, planar residuals,
   dihedral + prefer-intrinsic, boundary containment) — shared via
   extracted helpers, NOT copy-paste; `validate_geometric` keeps
   identical behavior.
3. NEW global coincidence census (the injectivity pass tier 3 defers):
   detect all cross-entity position coincidences among DISTINCT
   entities — vertex-vertex, vertex-on-face (interior), vertex-on-edge
   (interior), and the segment-granularity overlaps reconstructed per
   D3. Quadratic sweeps are acceptable and documented (same convention
   as the boolean edge×face sweep and mesh's CDT note). All
   comparisons through named trilean predicates (D5).
4. Certification diff, per F1/F2(iii): every census finding must be
   BACKED by a declared-contact record (directly or via D3
   reconstruction) — else a typed hard error (`UndeclaredContact`
   ValidationError variant, carrying the entity pair and position
   witness). Every declaration must be geometrically CONFIRMED by the
   census — a declaration with no witness is also a typed error
   (`StaleContactDeclaration`); silent tolerance of either direction
   is forbidden. The validator never blesses discovered contacts
   (F1: no scan-to-bless) — discovery ≠ declaration, ever.
5. `contacts` empty ⇒ the census must find nothing; then 3′ ≡ tier 3
   (plus the census actually run). Pin this equivalence with a test
   on a tier-3 body (e.g. the holed box / an extrude).

Indeterminate census predicates escalate per the standard trilean
discipline (typed escalation, never a silent skip).

## D2 — validity-class carriage (binding interpretation of F1)

No mutable validity field on `Body` (validity stays checked-on-demand;
raw-insertion disclaimers unchanged). The validity class rides the
RESULT WRAPPER: `BooleanBody` already carries `contacts` — that IS the
3′ claim. Add rustdoc making the contract explicit: a `BooleanBody`
with non-empty contacts is 3′-grade currency and
`validate_pseudomanifold(&b.body, &b.contacts)` is its at-rest gate;
empty-contact results remain tier-3 currency. 6(b) ratifies this
wording into DESIGN.md; do not edit DESIGN.md in this PR.

## D3 — segment reconstruction rule (design, then pin)

Contact records are vertex-granularity (`VvContact`, `VfContact` —
`boolean/mod.rs:138,147`). The census will find CONTINUOUS overlaps
(edge-on-face, coincident-edge segments). Derive and pin the
reconstruction rule: a census segment-overlap between distinct edges
(or edge-on-face) is certified iff its BOUNDING vertex events are each
backed by declared records (vv records at shared endpoints; vf records
for a vertex resting on the face) and the interior overlap is the
convex closure of those bounded events on both carriers. Document the
failure mode: a segment overlap with a missing bounding record is
`UndeclaredContact` (hard error) — never inferred. Write the
derivation into the PR body and the validator's module docs. If the
derivation shows the rule needs MORE than bounding-vertex records
(a genuine counterexample), STOP and report — that is a design fork,
not an improvisation site.

## D4 — vertex-on-edge absence (derive or add)

There is no vertex-on-edge record type. Derive whether reduction's
`split_edge` insertion guarantees every vertex-on-edge(-interior)
contact is refined into vv records before records are emitted. If yes:
pin with a fixture (operand vertex resting on an operand edge
interior) asserting the emitted records + 3′ certification; document
in module docs. If no: add the missing record type + emission +
certification, and report the deviation.

## D5 — contact carriage across mints (binding: descendant map)

Obligation (PR 5 review R5): the remap (`ops.rs:365-395`, `KeyView`)
keeps only direct key lineage; geometric coincidence can persist via
seam-zip/merge-stage minted descendants while its record drops.
Re-derivation-at-the-gate is REJECTED: certifying a surviving
coincidence from a re-derived (undeclared) source is scan-to-bless
under F1. Therefore: carry records across mints. Extend the existing
`GraftMap` pattern into a descendant map covering seam-zip and
`merge_coplanar_faces` entity replacement (old key → surviving key),
and route `remap_contacts` through it so a record drops ONLY when the
coincidence itself is consumed (entity gone, not renamed). Pin with a
fixture where a recorded entity is minted-over yet the touching
persists (e.g. a corner-kiss surviving a coplanar merge) — record must
survive and certify. If a genuinely-consumed-coincidence case is
found where the record must drop, test that the census agrees
(nothing to certify).

## D6 — split upgrade path (binding: native Intersection minting)

Retire the review-only `upgrade_edges_to_intersections`
(`tests/common/mod.rs:291`). Split-minted section edges and
boolean-minted seam edges get honest `EdgeGeometry::Intersection{s1,s2}`
descriptions AT MINT TIME (both parent surfaces are known there —
certified-by-construction per D2/M2). Split results must then pass
`validate_geometric` directly; delete the helper and rewire its
callers. If the boolean seam side turns out to be structurally harder
than the split side, land split-side + report before improvising.

## D7 — below-copy minting at BOB pinch (committed, Evan #61)

Implement below-vertex copy minting so negative-side pinches work
symmetrically; `SplitJoinError::DegenerateSection`'s
orientation-dependent refusal (`join.rs:94-107,628`) disappears for
the pinch class; flip-and-swap workaround docs removed; the
orientation-table fixtures (`review_m3_pr3_bob.rs:91,126`) flip to
asserting symmetric SUCCESS with equal-volume oracle both ways.
`certify_section_area` stays for genuinely zero-area sections (if any
class remains reachable, document it; if none, delete and say so).

## D8 — saddle fixture for the 15.11 guard (PR 4 review obligation)

Build a saddle-vertex fixture (non-convex neighborhood where
A-consecutive ≠ B-consecutive pairing is geometrically realizable) and
either (i) witness the F12 guard firing (`insert.rs:104,206` →
`PairingMismatch`) or (ii) prove the configuration unreachable for
tier-2 operands — a written argument in the test's module docs, plus
the fixture demonstrating the nearest reachable behavior. Either
outcome is acceptable; silence is not.

## D9 — corpus, closure, exports (acceptance)

New acceptance suite `crates/topo/tests/m3_pr6_tier3prime.rs` (+
sibling files as needed; generic-over-T scenarios; explicit Interval
lane per existing convention):
- PROMOTION: corner-kiss (vv), wedge-touch/tangent-edge, skew touching
  edges, vertex-on-face kiss, flush/corner-flush pillar rests — each
  boolean RESULT validated at rest with `validate_pseudomanifold`:
  green with its carried contacts, AND red (UndeclaredContact) when
  the contacts are withheld — both directions tested.
- NEGATIVE CONTROLS: a tampered declaration (wrong vertex key) ⇒
  StaleContactDeclaration; a hand-built genuine self-intersection
  (proper crossing, no declaration) ⇒ UndeclaredContact.
- CLOSURE (documented-unproven): feed 3′ results back through
  union/intersect/subtract against a generic mover and against a
  second toucher at the same locus. Every case must end in EITHER a
  correct certified result (volume oracle where computable) OR a
  typed refusal — assert exhaustively, no silent wrongness. Tabulate
  close/refuse outcomes in the PR body (feeds 6(b)).
- EXPORTS: mass properties + tessellation/STL on 3′ corpus bodies.
  Watertight admesh verification where export succeeds; typed,
  documented refusal where the touching class is not exportable.
  (The die e2e already covers the generic boolean-result export.)
- ε matrix {1e-6,1e-9,1e-12} × Interval lanes throughout.

## D10 — process (binding)

- OUTPUT DISCIPLINE per M3 conventions: ≤~150 lines per tool call;
  skeleton first; chunked reads; derivations to scratchpad; final
  report ≤150 dense lines.
- All new topology-determining comparisons via
  `geom_core::k_stats::decide` with distinct names (pattern:
  `validate.rs:1319`); list the new predicate names in the PR body
  (the 6(b) K-snapshot consumes them).
- Commit in reviewable stages (suggested: helpers extraction → census
  + certification → descendant map → D6 → D7 → fixtures/corpus);
  push after EVERY commit.
- Gate: full `./local-scripts/ci-local.sh` green on the branch merged with
  current main, run SERIALIZED (no concurrent gate runs; if
  `local-scripts/gate.sh` exists on main by then, use it).
- Open the PR with a full writeup (per-decision D1–D9 outcomes,
  derivations, closure table, deviations). Do NOT merge — adversarial
  review follows.
