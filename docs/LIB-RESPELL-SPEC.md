# LIB-RESPELL spec — the §2c fillet family, implemented (binding)

Mandate: implement the RATIFIED PATHS-DESIGN §2c (fifteen rounds
with Evan, merged #419) — the fused fillet family on the
pure-verb kernel, retiring the §2b compound register. Read §2c
IN FULL first; it is the contract. This spec adds only
sequencing, fences, and acceptance.

## 0. Discipline (absolute)

docs/LIB-PYG1-SPEC.md §0 verbatim and binding (foreground builds
one at a time via local-scripts/with-build-slot.sh, no parking +
kill-your-own-waiter, commit+push per chunk, NO Co-Authored-By,
no model names, merge-main-before-open + re-merge on movement,
checks STARTED, cold clippy CI scope all three lanes incl.
`-p pncad-py --features python`, k-lint discipline, comments
state the INVARIANT).

## 1. Two PRs, sequenced

**PR-1 — the kernel algebra re-spell (crates/profile):**
1. The pure-verb kernel per §2c round 12: verbs as pure
   functions over bare state values in a SEALED module; the
   chain threads values and applies EMISSIONS. The axiom
   (round 11) is the acceptance property: a verb reading
   anything beyond its input state + args must be UNWRITABLE
   (demonstrate: describe in the report why the module boundary
   makes the old carrier-aware wall inexpressible).
2. The transition table per rounds 13–15: ONE declaration,
   macro-expanded (lean (a)) into typed methods + replay arms +
   Step variants + tags. The V2 differential census RETIRES to
   one smoke row (keep exactly one; delete the rest with the
   table as their replacement — state the mapping).
3. The family: `fillet(r)`, `fillet_arc(r, spec)`,
   `arc_fillet(spec, r)`, `arc_fillet_arc(spec, r, spec₂)`,
   `arc_to(spec)`; the COMPLETE ArcData family (round 9: every
   admissible (state, mode) pair per the DOF matrix, tested;
   inadmissible pairs unrepresentable via the trait matrix;
   endpoint inside the endpoint-full variants). Ray-extension
   semantics (round 10): bare `fillet` after ANY leg extends
   the tangent ray as a REAL line leg; `FilletCarrierUnsupported`
   and `ArcCarrierSpelling` RETIRE (no carrier-keyed refusal can
   exist under the axiom); `NoCornerForFillet` survives.
   `at_on`/`to_on`/`at_toward` RETIRE.
4. Geometry invariance: the resolve machinery is UNCHANGED —
   every §2b-register construction re-spelled to the new family
   must produce a BIT-IDENTICAL lowered loop (the re-spell
   differential: old-spelling fixtures recorded pre-deletion,
   new spellings replay to identical bits). Rocker's oracle and
   census hold untouched.
5. Step vocabulary re-spells (the fused steps + ArcData enum at
   the wire); SCHEMA BUMPS (the v3 precedent; take the number
   after main's at your final re-merge — the LBRET/ASM-2A
   double-claim lesson: re-check at the LAST re-merge, the
   constant conflict is the coordination point). Goldens and
   fixtures re-bless; SchemaTooOld/UnknownSchema rows both
   directions.
6. §2/§2a/§2b/§3 REWRITE to the ratified surface (§2c's
   sequencing note): the directed-point definition, the leg
   list, the surface table; §2b's register text compresses to
   its historical note; §2c re-titles from proposal to ratified
   record. The conversation history stays (git + the #419
   thread) — the doc states the RESULT.

**PR-2 — consumers (after PR-1 merges):**
7. Corpus + demos re-spell (every arc_to/at_on/to_on/at_toward
   site; rocker; the guide's Rust blocks); Python bindings
   re-spell (the lattice classes' verbs + ArcData bound + stubs
   + ty fixtures — the PYG1 house pattern; suite delta stated);
   audit absence rows updated; the chat examples from the #419
   conversation land as doctests.
8. **The test-support shim DELETES here** (re-sequenced from
   RETTAIL by ruling on #431: its ~42 surviving callers are
   at_on/to_on chains this unit re-spells — migrate them to the
   new family and delete profile/src/test_support.rs; the
   deletion-horizon register entry closes). Also adjudicate
   #433 (lattice/validate collinearity) in PR-1's §4 rewrite —
   propose a disposition in the PR body for Evan.

## 2. Fence

OUT: NURBS-leg vocabulary changes, U4 anything, G5/G14, new
predicates or geometry beyond the ratified ray-extension
construction, CI structure. Anything missing: REPORT.

## 3. Acceptance

- PR-1: the re-spell differential (bit-identity vs recorded
  old-spelling fixtures) green; one table smoke row; grep
  proves at_on/to_on/at_toward/FilletCarrierUnsupported/
  ArcCarrierSpelling absent from the public surface; batteries
  -p profile -p editor-core -p pncad green; schema rows; cold
  clippy all lanes; zero new [[test]] binaries.
- PR-2: python suite green (delta stated); audit arithmetic
  re-derived; hosted CI green both PRs.
- Pre-draw fields at dispatch: difficulty L, task-class NUMERIC
  (the ray-extension construction is new geometric behavior).

## 4. PR discipline

Reports ≤150 lines each to
~/.local/share/cad-work/lib-respell-pr{1,2}-report.md with
per-phase figures. Open, do NOT merge. Final message per PR:
number + report path + ≤10-line summary. Forks: report,
smallest faithful reading, flag.
