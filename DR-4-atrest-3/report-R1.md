# ATREST-3 / PR #3191 @ 97b51065e: review
Verdict: APPROVE-WITH-FIXES. No MAJOR. Core change correct; probes confirm claims 1-3 at f64. Fixes: doc overclaims, stale citations, a test gap, an unscheduled seam.
Isolation: read nothing from any other lane; with-build-slot printed other slot holders' pids, not inspected.
Executed: temp probes in crates/sweep/tests and crates/step-import/tests; worktree restored clean.
## Correctness
- Claim 3 (assembling parts stopped at different rounds) HOLDS, sure, EXECUTED: fine arc loft (1e9·ε, refining past round 0) + 1e5·ε prism both orders; arc loft + 5e7·ε prism both orders; exhausting 1e11·ε loft + 1e5·ε prism both orders; arc loft + exhausting loft. Every pair: refine_to_target equals mass_properties in all four fields as raw bits, or same typed QuadratureBudget naming the same face; gate+continuation count = one measurement (16+8=24, 16+13=29); tier 3 and 3' same verdict.
- Claim 1 HOLDS, likely (read + executed pairs). plus_v_at_target checked against old target read.
- D-C HOLDS, sure (read; f64 half executed): round_hook(None)/reporting_hook(None) return Ok(None); structural cert door refuses VolumeUncomputable same face/source as mass_properties_structural; Dual cannot construct QuadLane::certified().
- Claim 2 at other scalars not measurable, sure, EXECUTED: at Interval arc lofts refuse QuadratureUnsupported at round 0 through all doors; rests on pointer pin + reading.
- MINOR-1 (sure, EXECUTED): gate3's new doc "the import pays one read of each face" false for assemblies: per-instance gate (validate_geometric) then gate3 walks every face again; two-instance assembly: 8 props_quad verdicts through import_step vs 4 for gate3+continuation and 4 for mass_properties. Sweep pass 1 could not match it. tcost_k3_import_certificate single-solid by design. NOTE (likely): since ATREST-1 per-solid check 7, the per-solid gate's stated reason no longer applies to check 7; retiring it changes ratified DESIGN.md text -> [ev].
- MINOR-2 (sure): multi-solid round-splitting path pinned by no committed row (a_multi_solid_certificate_reads_each_face_once settles at round 0; face_list_door_tests closed-form corpus).
- MINOR-3 (sure): new public StepImportError::EnclosureUncomputable on EXCH's ground, no scheduled follow-up; on merits (likely) truthful cause beats TierInvalid, but it is a reader-side refusal of a body the kernel admits ("the gate has grown an opinion").
- MINOR-4 (sure): stale step-import docs lib.rs:155-159 (gate refuses exhausting wall) and import_step # Errors missing the variant.
- NOTE-1 (sure): plus_v_by_sign doc overclaims "same scalar"; at f64 validate_geometric admits arc prism, validate_pseudomanifold_structural refuses (executed); "at one lane" is true.
- NOTE-2 (likely): cost wording right for round-0 settlers only; generally one setup per round before settle + one for continuation (predates PR).
- NOTE-3 (likely): refusal source can name a different face when an earlier face exhausts budget and a later hard-errors in a later round.
- NOTE-4 (sure, EXECUTED, out of fence): topo::mass_properties on 1e5·ε arc prism translated 4e9·ε panics at geom-brep/src/props/quad.rs:2456 (A2 area tripwire, perimeter arm, 1.33x ceiling) on a rayon worker.
- Claim 6 (likely): nothing guaranteed lost. Claim 4 (likely): return type carried.
## Style
- Stale citations of removed/renamed symbols (sure, class): k-lint src/lib.rs:363, tests/predicate_roster.rs:505, docs/K-REPORT.md:789 (sign_certified); work/chord/approx-face-mesh-certifies-against-fit.md:21 (quad_lane::cut_face); work/atrest/tier-3-does-not-check-shell-roles-per-solid.md:52 (props::sign_certified).
- Near-twin hooks reporting_hook/round_hook (likely).
- assembled drops parts' quad/band/tol on prose "share it by construction" (likely).
- Coupled tuple (Vec, Option) + every_solid_derived, None=>nonempty only by unreachable! (unsure).
- Relaxed SignCertificate impls: structural/Dual continuation untested (likely); Dual attempt failed (geometric_cube::<Dual64> refuses ScaffoldAtRest).
- Module doc overreach validate.rs:8-12 (_structural under "each certifying tier"; "every tier-3 door makes check 7 the same way") (likely).
- Stale comment step-import/tests/wild.rs:533 (sure).
- Long justifications: tour continued GAP comment, bounds-allowlist argument (unsure).
