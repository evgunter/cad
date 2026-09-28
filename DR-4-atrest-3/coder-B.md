# ATREST-3 review (PR #3191, frozen head 97b51065e)
Verdict: APPROVE-WITH-FIXES. No MAJOR. No tier-3/3' disagreement on check 7 at the same lane; every continuation measured bit-identical to mass_properties; D-C holds at a Dual.
Executed: probe in crates/sweep/tests, removed after; default ε only.
## Correctness
- MINOR-1 (sure, read): new StepImportError::EnclosureUncomputable re-creates the filed "refusal drops the enclosure" defect in a public type: gate3 maps every refine_to_target error to it, throws away the certified sign bracket, doesn't tell budget from escalation/poison; = work/encl/budget-refusal-drops-the-enclosure-the-caller-needs.md; pncad-py validate_geometric_measured and the tour's continued keep the bracket and classify the budget case; gate3 a third spelling, public; ENCL row not updated.
- MINOR-2 (likely): variant sits uneasily with ratified DESIGN.md:756-770 import step 4 ("holds no idea of validity of its own"); after the PR tier 3' admits the 1e11·ε prism but import refuses it; enclosure: Result/Option would honour DESIGN; disclosed deviation with no schedule.
- MINOR-3 (sure; behaviour confirmed by probe): stale premises: validate.rs:240-245 module docs ("Tier 3' is not this ... can still be refused there on quadrature budget"); step-import lib.rs:155-159; k-lint src/lib.rs:363 and tests/predicate_roster.rs:505 cite sign_certified; docs/K-REPORT.md:789. Class: grep prose for removed names.
- MINOR-4 (sure, executed): plus_v_by_sign doc "same scalar" overclaim; at f64 validate_geometric admits, validate_geometric_structural admits (NotMade), validate_pseudomanifold_structural refuses VolumeUncomputable{Unimplemented}; holds per lane.
- NOTE-1 (sure, executed): claim 1 and D-C: f64 pairs 1e5+5e7, 5e7+1e5, 1e5+1e11, 1e11+5e7, thin 5e7, 5e7 offset 30-3000 widths: both cert doors identical certificates/refusals; 1e11+5e7 and 3000-width offset both refuse correctly. Dual<f64> rational pair: structural doors refuse typed VolumeUncomputable on each solid's first face. Interval: arc prism refuses QuadratureUnsupported at every door.
- NOTE-2 (sure for covered): claims 2 and 3: f64 multi-solid refine_to_target = mass_properties all four fields bitwise through both doors, incl. one part converged at round 0 and the other continued 5-10 verdicts; refusals match first face in arena order; counts gate+continuation = one measurement (16+5=21, 16+10=26); refusing bodies gate 16 vs mass_properties 12 (disclosed). Structural certificate at Dual continued = mass_properties_structural on closed-form two-box pair; no in-tree row. Not covered: parts whose walks stopped at different rounds (couldn't build).
- NOTE-3 (likely): per-window setup regression disclosed but not priced; perf row P4.
- NOTE-4 (likely, read, predates PR): import takes two check-7 derivations per solid on multi-instance files (gate per instance then gate3); sweep blind spot.
- NOTE-5 (likely): undisclosed behaviour change: a hard refusal only in a round after the sign settled used to refuse tier 3', now admitted and becomes EnclosureUncomputable at import.
## Style
- S-1 near-duplicate hooks reporting_hook/round_hook; QuadLane two entries over one pointer (likely).
- S-2 assembled discards parts' quad; "share by construction" prose only (sure).
- S-3 relaxed impls: no row continues a _structural/Dual certificate; no tier-3' row continues a cert with an open face (sure).
- S-4 SignCertificate::enclosure "certified volume bracket" reachable at Dual (likely).
- S-5 stale path work/perf -> work/encl in tour continued comment and pncad-py value.rs:~428 (sure; predates).
- S-6 (Vec, Option) pair + every_solid_derived, unreachable! guard (unsure).
- S-7 module header lists _structural among certifying tiers (unsure).
- S-8 sign_certified_plus_v.rs and tcost_k3_certificate.rs:254 keep old name (sure).
Isolation: no glimpse.
