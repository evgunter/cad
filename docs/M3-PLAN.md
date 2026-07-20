# M3 Work Order — Splitting, Booleans, Cross-Shell Surgery

**Status: DRAFT for ratification** (the #24 pattern: forks below carry firm
recommendations; nothing merges without Evan's sign-off; the ratified plan
becomes the binding record and PRs cite it). **Fork resolution so far (the
#42 conversation, 2026-07-20)**: F2 resolved with Evan's explicit-intent
condition (invariant text under F2); F5 resolved — curved intersections
deferred to M5 as a unit, M3 planar-only, no speculative curved-readiness
abstraction; F8's ∅-as-typed-success confirmed; F1/F3/F4/F6/F7/F9 treated
as ratified-with-the-plan per Evan (method commitments / forced by ratified
principles). Awaiting final 👍 on this updated text.

Read `DESIGN.md` first (D1 incl. tiers, D2, D4, D9, the Banked principles —
especially coincidence "structural or declared, never inferred from values",
non-manifold-results-as-typed-errors, voids-born-only-from-booleans, SSI
completeness contract). M3's goal (Roadmap): **plane-splitting of solids;
boolean set operations (∪, ∩, ∖); the cross-shell Euler surgery they need** —
the milestone where two independently built bodies first interact.

Primary sources, all read and distilled into
`<main-checkout>/references/notes/`: ch. 14 (`mantyla-ch14-splitting-algorithm.md`),
ch. 15 (`mantyla-ch15-boolean-set-operations.md`), and — new for M3 —
**`m3-grounding-synthesis.md`**, the second-witness cross-examination of both
chapters against the TOG 1986 journal version (Mäntylä, *Boolean Operations of
2-Manifolds through Vertex Neighborhood Classification*). Implementer/reviewer
prompts cite the notes and the synthesis, never the scan. Headlines the plan
stands on:

- **The paper settles the normal-convention question in our favor**: TOG §2/§6.1
  state CCW-from-outside + outward normals — exactly our ratified convention.
  Ch. 14's rule (a) is correct as printed *for us*; ch. 15's Program 15.7
  side-code labeling (`IN=+1`) is the suspect half of the known ch.14/ch.15
  sign inconsistency. Nothing is ported on faith regardless (fork F3).
- **The paper contradicts the book's rule (b)** (ON-edge reclassification): the
  two symmetric cases (ABOVE-ON-ABOVE / BELOW-ON-BELOW) carry swapped verdicts
  between witnesses. Neither table is trusted; F4 adjudicates from first
  principles.
- **The paper supplies the unprinted on-edge machinery** (`srecledges`): Tables
  II/III + the edge-edge angular-sort tie rules — and reveals single-on-edge
  handling is **op-dependent**, which the book's printed fragment obscured.
- **The paper proves nothing**: zero code listings, no termination argument;
  completeness is claimed only "under the assumption that all numerical tests
  can be correctly evaluated" — a direct endorsement of the trilean/Q1
  architecture, and a transfer of proof obligations to us (F12).

## Forks to resolve at ratification (recommendations firm, pushback welcome)

**F1 — Tier-3′ (pseudomanifold) validator shape.** Boolean results are
guaranteed only criterion 3′: contacts limited to entirely-coincident-but-
distinct edges, edge-on-face, vertex-on-face, vertex-on-edge/vertex — touching
allowed, proper self-intersection not. *Recommendation*: a distinct at-rest mode
`validate_pseudomanifold` (tier 3′) = tier 3 minus global-position injectivity,
plus **declared-contact records** as the source of truth for where touching is
legal: the boolean pipeline *knows* every contact it creates (they are exactly
the ON-set survivors) and emits them as typed records; 3′ validation certifies
each declaration geometrically and requires no *undeclared* coincidence — never
scans for contacts to bless after the fact. This is the round-8 coincidence
ladder applied to validation. Bodies carry their validity class; tier-3 bodies
remain the default currency.

**F2 — 3′ versus "non-manifold results are typed errors" (reconciling two
ratified principles).** *Recommendation*: ratify the boundary as
representability. Pseudomanifold touching via **distinct** entities (two
vertices at one point, two edges on one segment) is representable in our
half-edge structure, is what the pipeline naturally produces, and is a typed
*success* carrying its 3′ declarations. Genuine non-manifoldness — a single
edge with >2 faces, a shared-entity wedge fan — is unrepresentable and stays a
typed error at the site that would have needed it. The round-9 ratification is
thus sharpened, not revised: "non-manifold" means non-representable, and 3′ is
the honest name for the representable touching class. **Evan's condition
(#42, ratified into the invariant): touching is always backed by explicit
intent.** Concretely: (i) operand coincidences are only ever structural
(shared key) or declared (recipe data) — near-coincidence NEVER silently
becomes contact (escalated typed error instead, per F6); (ii) result-side
touching arises only from those intentional coincidences propagated through
the boolean node, and the result carries machine-checkable declared-contact
records that tier-3′ certifies; (iii) an *undeclared* contact discovered at
validation is a hard error, never blessed.

**F3 — the sign chain (method commitment).** One first-principles primitive
`enters_material(dir, face) := sign(dot(dir, outward_normal))` (< 0 ⇒ into
material), derived under our conventions, then the ENTIRE chain re-derived
against it end-to-end: rule (a) → 15.7 side codes → 15.10 coplanar table →
15.11 IN→OUT null-edge dispatch → the lmev he1/he2 orientation encoding →
scanjoin's A-he1↔B-he2 antiparallel correspondence → lmfkrh ring=IN-copy →
loopglue zip → revert. Every step gets a mirror-check entry; one canonical
two-brick worked example (TOG Fig. 15.4 analogue) is hand-traced in the plan's
PR 4 spec and pinned as a kernel test. Port nothing by sign-copying.

**F4 — rule (b) adjudication (method commitment).** Derive the ON-edge
reclassification table from its stated *purpose* (nonmanifold configurations
must come out as disconnected pieces; no dangling faces/edges) on the two
discriminating fixtures (tangent-edge, touching-wedge), at f64 and Interval;
document which witness was right and why; only then pin the table. The
derivation is part of PR 2's binding spec, reviewed adversarially like any
code.

**F5 — SSI scope: M3 is planar-boundary booleans.** The ch. 14/15 pipeline is
polyhedral; both witnesses state curved faces break the vertex-neighborhood
reduction. *Recommendation*: M3 splits and booleans require all-planar
boundaries (typed `CurvedBooleanUnsupported` on any non-Plane face — precise,
honest, and consistent with fail-loud), with the architecture explicitly not
foreclosing the extension: the reduction step is written against a
`face-intersection` interface whose M3 implementation is plane×plane
closed-form; M2's certified analytic pairs (plane×cylinder, …) slot in at M5
with real SSI + general pcurves, where the marching+interval completeness
contract already ratified governs. Extruded/revolved bodies with arc walls are
therefore not boolean-able until then — the acceptance corpus uses prismatic
bodies (which M2 produces natively from polyline profiles). Alternative
considered: grafting the M2 closed-form pairs into M3's reduction — rejected as
a scope trap (curved ON-neighborhoods reopen every classification table with
none of the book's guidance; that IS the SSI milestone).
**Resolved with Evan (#42, 2026-07-20): curved defers to M5 as a unit.** The
dependency chain is fourfold and entirely M5-shaped: (a) intersection-locus
representation — even a tilted plane×cylinder cut is an ellipse, outside
`Line | Circle`; generic pairs are degree-4+ ⇒ NURBS caches + certified
fitting (M5 "NURBS depth"); (b) general pcurves; (c) second-order sector
classification (the `TangencyLocus` regime — curved sectors tie at first
order exactly when surfaces are tangent); (d) certified marching numerics
(the SSI contract). An `Ellipse`/conics analytic variant is a noted M5-era
option (buys exact plane×quadric cuts under D3's closed-enum growth) but does
not unlock booleans alone — (c)/(d) still gate. **And the inverse commitment:
M3 builds NO speculative curved-readiness abstraction** beyond the thin
face-intersection boundary — no generalized sector types, no curvature
parameter slots; M5 refactors the boundary against real curved requirements
rather than inheriting a guessed one.

**F6 — coincidence discipline in the reduction (applying round 8).** The book
conscripts near-coincident vertices into ON via EPS snapping; the paper's §7
tolerance-ordering hack papers over the resulting inconsistencies.
*Recommendation*: every reduction/classification comparison is a Q1 trilean
predicate — definitely-off ⇒ clean side, exactly-on ⇒ ON, in-band ⇒
**Escalated typed error** (a genuine sliver: the operand pair is
ill-conditioned at this ε), following the round-8 ladder: coincidence is
structural (shared key — impossible across bodies), declared (bit-equal
descriptions arising from shared recipe data DO decide ON exactly), or a typed
sliver error resolved by an explicit repair/adoption op (D7 machinery, M5+).
Consequence, stated honestly: M3 booleans on *independently modeled*
nearly-touching bodies fail loudly rather than guess — which is the design
thesis of this kernel, and what the D7 import/healing story exists for.

**F7 — maximal-faces precondition.** Booleans assume no two adjacent coplanar
faces. *Recommendation*: fail-loud precondition (`NonMaximalFaces` typed error)
plus a separate public normalization op `merge_coplanar_faces` (the explicit
opt-in; per the no-automatic-face-merging M2 ratification, merging is never
silent) — and boolean *outputs* run it as a documented final stage because the
seam zip manufactures coplanar pairs by construction (that stage is part of the
op's contract, not hidden healing: the recipe records one boolean node).

**F8 — containment fallback, empty results, and voids.** When boundaries don't
intersect, classification needs vertex-in-solid (ray parity with the
golden-angle retry schedule, promoted from profile's 2-D machinery to 3-D) and
the results include ∅, A, B, disjoint union — and **the first legitimate
voids** (A∖B with B strictly inside A: the inner shell is born here, exactly as
the voids-only-from-booleans ratification anticipated). *Recommendation*: ∅ is
a typed success value (`BooleanResult::Empty`), not an error — GQ2's per-node
result DAG wants a value; multi-shell results (voids, disjoint unions) are
tier-2-legal multi-shell bodies and the M2 single-shell sweep invariant stays
untouched. `FullRevolveHoles`' error text gains its promised pointer to the
boolean route in M3 PR 5.

**F9 — null-entity orientation as data (method commitment).** The book encodes
which side a null entity faces in half-edge slot position (he1/he2) and list
position (`floops`) — the mirror-bug farm both notes flag. All M3 scaffolding
carries typed attributes instead: `NullEdge { below_end, above_end }`,
`NullFacePair { above_loop, below_loop }` (booleans: `{ in_copy, out_copy }`),
correspondence by explicit key pairs, never index coincidence.

## Working agreements (inherited unchanged)

One implementer + one adversarial e2e reviewer (falsification assignments,
executed programs) + one fix pass per PR; overlapped pipeline, fix pass the
only serialization point; reviewer suites promote as `review_m3_prN*`;
self-merge with full writeups on green CI; genuinely fork-shaped discoveries
wait; branches `ev/m3-<n>-<slug>`; orchestrator log `docs/M3-LOG.md`;
OUTPUT DISCIPLINE header in every agent spec; D9 charter and Q1 predicates
throughout — the K funnel now unified (PR 7), so M3's new predicates (the
richest crop yet: sector classification, containment, coplanarity) are
name-tagged into the same telemetry from birth.

## PR sequence

1. **Euler-inventory extensions + null-entity scaffolding types**
   *(self-merge grade)*. Cross-shell `mfkrh` (ring → new face, shell split —
   the M2-deferred transient becomes constructible) and its glue-side dual
   call site; named `split_edge` op (the lmev edge-split idiom); `revert`
   (orientation reversal: loops + cached planes, D9-deterministic);
   `laringmv` (ring re-homing via trilean point-in-loop); worklist `movefac`
   shell partition. Typed null-entity attributes per F9. Tier 1/2 must
   tolerate (and tier-1 debug-assert through) the new mid-op states: null
   edges (zero length — note this *relaxes* PR 3's zero-length certification
   refusal for the scaffolding lane only, behind the null-edge type, never
   for certified `EdgeCurve`s), 2-loop null faces, multi-shell mid-op bodies
   (component-aware E–P already handles them). `merge_coplanar_faces` (F7).
2. **Split, part 1: reduction + neighborhood classification** *(the
   sign-chain PR)*. Vertex-vs-plane trilean sweep (F6 discipline; no
   snapping); `split_edge` insertion of exact crossings; ON-vertex
   neighborhood orbit (typed sector array; wide/reflex handling re-derived —
   the paper's complement-negation vs the book's convex-subdivision, pick by
   derivation); rule (a) re-derived from `enters_material` (F3); rule (b)
   adjudicated per F4 with the two fixtures IN THIS PR's review; null-edge
   insertion with F9 attributes. The Program 14.7 head-rebinding erratum is
   sidestepped structurally (explicit transition-pair scan, worklist form —
   F12). Acceptance: notched-block (Fig. 14.2 analogue) classified correctly
   at all ε rows incl. Interval.
3. **Split, part 2: join + finish; the `split` public op** *(self-merge
   grade)*. Loose-end pairing (lex sort with a TOTAL comparator — the ε-banded
   sort is engineered out per F13/synthesis; exact-order band like profile's
   canonical form); null-face completion; `lmfkrh` distribution + component
   DFS; both result solids returned functionally (no in-place consumption).
   Slicing (§14.9) falls out as a near-free plane-section query — include it
   (first real sectioning feature). Acceptance: asymmetric solids split at
   generic/vertex-grazing/face-coplanar planes; every mirror-check site from
   the synthesis's ch. 14 list unit-tested; A reassembly oracle (split then
   re-glue ≡ identity on the two-manifold class).
4. **Booleans, part 1: reduction + classification across two bodies.**
   Edge×face sweep with `contfv`/`contfp` case codes as typed trilean
   predicates; the three ON-sets as declared-contact records (F1/F6);
   vertex-vertex sector intersection search; coplanar-sector classification
   via Eq. 15.3 (the book's table — the paper's ∖ row is its own misprint,
   see synthesis §C) behind an oriented-plane-equality predicate replacing
   `vecequal`; on-edge machinery transcribed from TOG Tables II/III into
   typed, unit-tested-per-row decision tables (op-dependence included);
   edge-edge angular-sort with Table-I ties. The 15.7-vs-rule-(a) sign
   inconsistency resolved by the F3 chain derivation. The 15.11
   consecutive-pairing invariant gets its saddle/4-crossing stress fixtures
   here (F12) — proven or replaced with correspondence-keyed pairing.
5. **Booleans, part 2: joining + result generation; `union`/`intersect`/
   `subtract` public ops.** Ch. 14 join reused with A↔B correspondence
   disambiguation (explicit key pairs, not correlated array order — F9);
   `setopfinish` with Eq. 15.1 component selection; `revert` for ∖; seam zip
   via cross-shell kfmrh + loopglue (the ch. 12 machinery's promised second
   consumer); `merge_coplanar_faces` output stage (F7); containment fallback
   + ∅/A/B/disjoint/void results (F8); `FullRevolveHoles` error text updated.
   Acceptance: the canonical two-brick trace (F3) pinned bitwise; Fig. 15.1
   coplanar-overlap intersection; A∖B ≡ A∩revert(B) as an executable oracle
   across the corpus; voids: cube∖inner-cube yields the two-shell body and
   tier-2 passes; disjoint operands yield typed disjoint-union/∅ per op.
6. **Tier-3′ + M3 exit** *(the ratification-sweep PR)*. `validate_pseudomanifold`
   per F1/F2 with declared-contact certification; the touching-configuration
   corpus (tangent edge, touching wedge, vertex-on-face kiss — the rule-(b)
   fixtures promoted to at-rest bodies); closure stress tests (3′ results fed
   back through booleans — documented-unproven, fail-loud on the gaps);
   mass-properties and tessellation/STL run on boolean results (M2's
   machinery meets M3's bodies: watertight exports of a boolean result is the
   milestone's demo); K-telemetry snapshot from the new predicate crop
   (first adversarial-corpus data — the D7 preview Evan predicted); M3-exit
   DESIGN.md sweep (F1/F2/F5/F6/F7/F8 ratifications folded in; tier table
   updated; voids documentation).

## Deliberately not in M3

- Curved-face booleans, general SSI, pcurve machinery — M5 with NURBS depth
  (F5; the interface seam is built, the implementation is not).
- `TangencyLocus` and import adoption (D7) — M5+; the 3′ declared-contact
  records are designed not to conflict with the future adoption ladder.
- Filleting/blends (pre-M5 predicate reification per Banked principles),
  feature DAG (M4), sketch solver (M6), viewer (deferred).
- Boolean performance (BVH/spatial indexing for the edge×face sweep) —
  correctness first; the quadratic sweep is documented like mesh's CDT note.

## Exit criteria

Split and all three booleans on prismatic solids end-to-end through public
ops only, including: the canonical two-brick suite bitwise-pinned; coplanar
overlaps; vertex/edge-touching configurations landing as 3′ bodies with
certified declared contacts; voids born from ∖ and validated at tier 2;
∅/disjoint results typed; A∖B ≡ A∩revert(B) oracle green across the corpus;
tier 1 after every op, tier 2 + tier 3 (or documented 3′) at rest; mass
properties + watertight STL export of boolean results verified externally;
CI green at ε ∈ {1e-6, 1e-9, 1e-12} + interval lane; every mirror-check site
from the synthesis unit-tested; rule-(b) adjudication and the sign-chain
derivation documented in M3-LOG; new conventions ratified into DESIGN.md at
exit.
