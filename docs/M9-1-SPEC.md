# M9-1 — contact vocabulary: records + declaration classes (spec)

Orchestrator work order for M9's first unit (M9-PLAN item 1,
RATIFIED #509). Substrate: fresh exploration 2026-08-15. C3/C4
(CONTACT-DESIGN, ratified #178) are the requirements; this spec
sequences them and rules the eight substrate flags. Scope:
`Rest` + `Tangent`; `Fit{gap}` DEFERRED by ruling (its payload
shape is PINNED below so the deferral stays additive).

## Shape: ONE unit, TWO PRs (the M8-3 pattern — row at PR-2)

The substrate measured the three-home growth at ≥14 kernel files
+ 7 editor-core files + a schema break; the plan's sprawl escape
is taken UP FRONT rather than mid-flight:

- **PR-1 — the kernel home** (topo, geom-brep): record types +
  carriage + verification + failure modes.
- **PR-2 — the recipe + LIB homes** (editor-core, pncad,
  pncad-py) + the SCHEMA BREAK. The schema claim goes to main at
  PR-2's dispatch, not PR-1's.

## Layering ruling (substrate flag 1 — the load-bearing one)

`ContactClass` and the finding vocabulary move DOWN into the
KERNEL: a `topo` (or geom-core) `ContactClass { Rest, Tangent }`
plus a kernel-side finding type carrying pair + class + evidence.
editor-core's `FlushFinding`/LIB re-exports WRAP or re-export the
kernel types — never a parallel enum. Forced by two facts: topo's
`UndeclaredContact { finding }` must carry the same vocabulary
the detector produces (C4 + SELECT-DESIGN §3d, "one vocabulary
end-to-end") and topo cannot depend on editor-core; and ASM R2-b's
`Node::Mate { class }` needs the SAME type. One enum, defined
lowest, re-exported upward.

## PR-1 — kernel (binding content)

1. **Record granularities (C3 verbatim)**: `CurveContact
   { face_a, face_b, witness }` and `PatchContact { face_a,
   face_b }` beside the existing vv/vf records; `CarriedContacts`
   AND `BooleanDeclarations`/`DeclaredPairs` gain the class —
   `DeclaredPairs` becomes a keyed MAP (set → map), and the
   carried V–V/V–F channel carries class too (flag 5; a
   declaration without a class is unrepresentable, not defaulted).
2. **Carriage**: `remap_contacts` chases FACE lineage for the new
   granularities (descendant map, never re-derivation — C4's
   replay rule); `ContactRecords` gains `PartialEq` so
   bit-identical-replay rows can compare records, not just naming
   (flag 6).
3. **Verification, Rest**: kind-generalize the
   `oriented_plane_eq` ladder — a `carrier_eq` family (plane;
   sphere center/radius; cylinder axis/radius), each margin at
   its NAMED lever arm, same three-outcome shape
   (Same-oriented / Same-opposite / Distinct + Escalated), the
   `flush_pair_relation` door staying the ONE shared door.
   Opposed senses exact-bit (S10); aligned coincidence
   CONTRADICTED (containment, not contact).
4. **Verification, Tangent**: reuse the jet machinery
   (`tangent_jet`, `tangent_span_bounds`,
   `tangent_certificate_lane` — the demanded set IS the
   certifiable set; outside it refuses typed, which is C3's
   order-k boundary). C4's bridged residue is exactly the in-band
   κ_rel INCLUDING exact zeros at isolated points (the #175
   G1-chain clause) — deliberately weaker than a jet certificate;
   the spec text carries that sentence so the reviewer attacks it
   as designed, not as an accident.
5. **Failure modes**: validation-tier `ContactContradicted
   { declaration, witness, margin }` and `Escalated { diag }`
   added beside `UndeclaredContact`/`StaleContactDeclaration`;
   fired at USE and at the AT-REST gate both. Recourse text
   reconciled to SELECT-DESIGN §3d's ratified TWO arms (declare
   the named class / move the geometry) — the shipped three-arm
   `COINCIDENCE_RECOURSE` predates that ratification and drops
   its "lower the tolerance" arm at these sites (flag 2). AQ6's
   trilean rides here: value-equal-by-authoring carriers
   (peg/bore radii) get the definite/bridged/contradicted shape
   with recourse text steering designed clearance to `Fit{g₀}` —
   NAMING THE DEFERRAL ("Fit lands with its first consumer")
   rather than a dead pointer.
6. **Certifier boundary (flag 7, stated)**: `PatchContact`'s
   chart-space area/overlap certifier does NOT ship in this unit
   — no trim-region-overlap predicate exists and its home is
   M9-2's census arms. PR-1 ships the record TYPE with its
   certification obligation stated in-doc and a typed
   not-yet-certifiable posture; M9-1 → M9-2 SERIALIZE (the plan's
   M9-2 → M9-3 chain extends one link left). CurveContact's
   per-locus certifier DOES ship (the jet loop exists).
7. **Fit reservation (flag 3)**: pin the payload shape now —
   kernel `Fit { gap: T }` (resolved scalar; recipe-side an
   Expr), noting `ContactClass` loses `Copy` when it lands;
   variant itself still deferred.

## PR-2 — recipe + LIB + schema (binding content)

1. `Node::Declare` pairs gain the class (pair → (pair, class));
   `ValuePayload::Declarations` and `resolve_declarations`
   thread it; content-key feed includes it (keys are
   process-internal — no schema cost from the key, memo
   invalidation accepted).
2. LIB: `ContactClass::Tangent` detector arm in
   `find_flush_candidates`; `declare_node` PRESERVES
   `finding.class` (today it drops it — that's a live bug the
   class payload exposes); prelude/select re-exports become
   re-exports of the KERNEL types per the layering ruling;
   SELECT-DESIGN §3 doc updated in the same PR.
3. **Schema clean break** (v-next claimed on main at PR-2
   dispatch): `deny_unknown_fields` makes the Declare class field
   a wire break — the full lbret_schema_v8.rs pattern (goldens
   v1..v-next, the prior-version refusal fixture,
   SchemaTooOld/UnknownSchema both directions, BOTH version-pinned
   fixtures incl. pncad/tests/plate_param, the version-pin
   asserts, the claim reasoning recorded in the test's doc
   comment). `ContactClass` gains serde.
4. **Persistence boundary stated in-doc (flag 4)**: declarations
   persist as node payload; RECORDS never persist (D9 re-derive);
   ASM-4's interface record stores DECLARATIONS — the R2-b seam
   sentence, cited to ASM-R2-SPEC-DRAFT:41-58 so the door is
   negotiated here once.
5. **Python**: the binding has no `Node.declare` constructor
   (only `boolean(..., declare=)`) — the gap is NAMED in the PR
   body and the error-tag mirror updated; adding the constructor
   is not this unit.

## Acceptance

1. The m4_pr5_declare and crosslap_rest suites extended: a
   declared planar `Rest` still unions bit-identically; an
   undeclared kiss still refuses; a declared pair with the WRONG
   class is `ContactContradicted` naming the margin (new row,
   red-then-green against a class-ignoring mutant).
2. A `Tangent` declaration on a certified-lane pair (cylinder on
   plane) verifies through the jet loop; the same pair with
   definite normal independence at a sample CONTRADICTS; an
   isolated κ_rel zero on a G1 chain BRIDGES (the #175 clause,
   pinned as its own row).
3. AQ6 rows: value-equal-by-authoring peg/bore radii — definite
   verdict beats the declaration both directions; the recourse
   text names `Fit{g₀}` and its deferral.
4. Replay: bit-identical rerun reproduces RECORDS bit-identically
   (the new PartialEq row).
5. Class-less declarations are UNREPRESENTABLE post-break (a
   construction attempt is a compile error kernel-side, a typed
   refusal at the persistence door for old documents).
6. ε-row three-outcome honesty on every new verification row;
   hosted CI fully green both PRs; the two fixture twins
   (topo/tests/common + demos/tour/booleans.rs) stay twins.

## Process

Implementer: block M8-15 slot 1 (OPUS, from the byte-186 draw).
Difficulty pre-dispatch: **L**; task-class: MIXED → records as
NUMERIC per the #409 rule (classified pre-dispatch, this spec).
One blinded reviewer + fix pass per PR or per unit at the
orchestrator's call at PR-2 (the M8-3 precedent: row at the
second PR); review ordinal claimed from the ledger ON MAIN at
review dispatch. Standard brief lines apply (foreground
discipline, no trailers, invariant comments, ε honesty, k-lint
discipline, merge-main + union).
