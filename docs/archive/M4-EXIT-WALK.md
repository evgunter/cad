# M4 exit walk (8c) — criteria vs evidence

Status: assembled at the 8c seam; criterion-10 note: the corpus +
latency rows are MET at 8a (#118); 8b's K-lint row is an item-8
OBLIGATION (M3 addendum), not an exit criterion — its merge rides
in parallel and does not gate this walk.

| # | Criterion | Status | Evidence |
|---|-----------|--------|----------|
| 1 | Recipe doc authored via DocEdits, persisted, round-trips bit-exact + replays bit-identically at 3ε + Interval | MET | PR 6 D6.1 rows (#112) over the 8a corpus (#118): `persistence (eps=1e-6/1e-9/1e-12)` + interval roundtrip rows, hosted matrix; corpus docs authored via Recorder (bit-identical, reviewer-verified) |
| 2 | Die (and boolean-tour bodies) rebuilt as recipe documents | MET | corpus: die 77 nodes exact 7.8359375/26.625; corner_table, heat_sink, crossing_slots, nested islands ×2, kitchen-sink (#118). Boolean-tour montage bodies whose geometry lives in demos (silhouettes, project box etc.) are represented by their capability-equivalent corpus docs — note honestly in the walk |
| 3 | Mid-DAG param edit → downstream-only recompute, counted; unaffected names resolve identically; name-table golden green | MET | latency lane counted-reuse asserts (die 3/77, hand-verified by 8a review); C5 heat-sink demo (recomputed-1/reused-4 + 135/135 names, #98); name-table goldens green through every merge |
| 4 | Flip-inducing edit → typed ResolveError with the correct flipping predicate | MET | PR 4 (#96): D6.1 single-qualifier-flip fixture (exact predicate pinned), diagnosis goldens cross-ε/cross-scalar; qualifier-delta rung (P2) |
| 5 | Rebind + SetTolerance end-to-end with the shared diff engine | MET | Rebind: PR 4 D3 + A1 ladder e2e (#96), appearance-key rewrite (#92/#96). SetTolerance: PR 6 D4 recorded-ε replay + PR 4 population core — the goldened cross-process ε-diff row (#112) |
| 6 | bit_identity zero production consumers; tripwire allowlist empty; debug assertion in place | MET | PR 5 D3 (#102): empty allowlist, smuggled-consumer CI test (reviewer-verified), debug_assert!(same_source ⇒ eq_bits) at migrated sites |
| 7 | 3′ body reused through Declare-carrying boolean certifies at the 3′ gate | MET | PR 5 D6.1 (#102): closure-corpus kiss body certifies exactly when declared, refuses undeclared; corner-table tier 3 GREEN (#102); crossing-slots corpus row (#118) |
| 8 | Appearance survives recompute, retires loudly with names | MET | PR 7 (#92) + PR 4 D9 (#96) enrich/offers; corpus kitchen-sink carries appearance + MetaValue metadata through round-trip (#118) |
| 9 | STEP export of a corpus part imports intact externally | MET | hosted `step import (freecad)` row (since #94) + az.step required-success (#114); planar-only scope honestly narrated (M5 grows it) |
| 10 | Band 4 corpus runs in CI with rebuild-latency reporting | MET | `band 4 corpus (eps=1e-6/1e-9)` + `rebuild latency (reporting)` hosted rows (#118); baseline JSON with provenance + box-relative clause |
| 11 | Solver contract types compile into the document layer with document-semantics tests | MET | PR 4 D5 (#96): WitnessDatum/ReWitness[Bulk]/WitnessBifurcation N5-verbatim, doors typed + pinned; witness bytes persist hex bit-exact (#112); W4 invisibility by re-derivation |
| 12 | New conventions ratified into DESIGN.md at exit | PENDING → 8c | this PR: the D4 inventory below |

## 8c DESIGN.md inventory (from M4-PR8-SPEC D4, all banked texts)
1. F1–F8 outcomes (each fork: decision, landing, deviations — incl. Doc<P> genericity acceptance, JSON format choice, MetaValue D7 two-round ruling).
2. N6 retirement DONE (+ R2 narrowing as designed consequence).
3. Envelope update: operand-internal-declaration entry RETIRED; new entries = REST-contact join gap (crosslap frontier), post-#106/#116 join+mesh residue (coincident-plane class, sub-ε graze, ill-formed faces — from the #106 report's residue enumeration).
4. F5 verified-at-use wording (PR 5 review F5).
5. #101 declared-tangency discipline into the profile section (+ #104 concept pointer + v2 profiles-as-programs commitment).
6. Roadmap: M4 line → done; M5 line gains openers (curved STEP subset, arc-leg fillet sugar, REST join lane, #89 K-revisit, interval-crate adoption decision, PERF/K notes from 8b).
7. NEW conventions worth ratifying from the milestone's lessons: sentinel-free tagged encodings (the disease that bit twice); save/load door symmetry (every Deserialize check has a Serialize twin); watcher full-matrix floors. (Propose; Evan may trim.)

## D5 state-doc trim plan
- M4-LOG final snapshot (supersede interim snapshots as historical).
- memories: cad-project-state → M4 complete; model-ab-experiment → final table + readout.
- A/B READOUT (draft; finalize with row 10): n=10 rows (7 complete at draft). Fable: rows 1,2,3,4,7 + 6 (interval crate). Opus: 5,8,9. Early pattern: Opus rows show 0 substantive MAJORs and top rubric lines incl. the milestone's only zero-fix-pass unit and an upheld evidence-backed dispute; Fable rows carried the two largest builds (PR 6, #101) with more findings but at higher absolute scope. Confounds: difficulty mix differs, n tiny, reviewer variance unmeasured. Honest conclusion shape: "no evidence Opus implementation is worse at this scale; suggestive that it's comparable; continue the experiment into M5 for n."

## Q9 note
Name still open (Evan's call; #107 Tertium shortlist merged by the other agent). 8c does NOT gate on it.
