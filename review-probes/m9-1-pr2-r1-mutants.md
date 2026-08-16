# Review probes — m9-1-pr2-r1 (PR #552, head dc9dbdee)

Two mutants executed against the shipped rows (applied, run, reverted;
recorded here so the runs are reproducible):

## Mutant 1 — `declare_node` re-defaults the class
`crates/editor-core/src/names/flush.rs`, declare_node:
`(f.pair.clone(), f.class)` → `(f.pair.clone(), ContactClass::Rest)`.
Result: `m9_1_declare_classes::declare_node_preserves_the_findings_class`
**PASSES** (1 passed / 0 failed). The detector emits only `Rest`
findings today (Tangent arm deferred), so the preservation row cannot
distinguish "preserved" from "re-defaulted". The literal pre-fix code
(`f.pair.clone()` alone) fails to COMPILE (E0277) — the "structurally
cannot pass" claim is a type-system fact, not an assertion the row
enforces.

## Mutant 2 — the op door ignores the authored class
`crates/editor-core/src/eval/wire.rs`, resolve_declarations:
`FacePairDeclaration::new(fa, fb, class)` → `...new(fa, fb, ContactClass::Rest)`.
Result: `m9_1_declare_classes::a_wrong_class_declaration_refuses_at_the_op`
**FAILS** ("a Tangent declaration on a conformal pair must refuse") —
the end-to-end threading row genuinely guards the door.

## Constructibility probe
`let _ = ContactClass::Fit;` in an editor-core test → E0599. No third
variant is constructible in this build, so `declare_pairs_wire`'s
serialize-direction refusal arm is compile-time-unreachable here; it
is cross-build defense only, and is untested by construction.
