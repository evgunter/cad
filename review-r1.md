# DECIDE-10 review — R1 (PR #4929, frozen head `7937b969a8`)

**Verdict: APPROVE-WITH-FIXES.** At the interval lane, every measured count and verdict I re-took matches the PR body, and every re-baseline is a sum (main re-taken beside it). The soundness premise "the arm a certified read takes is the arm the construction takes" is false at an inexact witness. That gap existed on main as `sign_gated`. The atom-key merge adds new false proofs there, and the relabel makes them indistinguishable from a theorem. No shipped verdict path reaches it. Two classes of prose are left stale.

## Findings
- **MINOR-1 (claims 2, 3), sure, by execution.** `signed.rs:27` and `sym.rs:637`/`1810` state that a certified read "answers which arm the construction takes". The read signs the **exact** form. At `Sym<f64>`/`Probe` the construction compares a **rounded** value, and the two can take different arms. Take `d = (x+1e17)−1e17−0.25` at `x = 0.5` over the point box `[0.5, 0.5]`. In f64, `d` is −0.25 and takes `when_le`. Exactly, `d` is +0.25 and the read takes `when_gt`.
  - (a) `select(d, z+w, z) − z` with `w = 1e-15` gives `Ok(Zero)` with `proven = true`, though its value is 1.1e-15. On main it was the same zero, labelled `sign_gated`; it now carries the theorem's bit.
  - (b) **New with this PR, from the atom-key merge (claim 3):** `atan(select(d, z+w, z)) − atan(z)` is `proven` on the PR and `numeric` on main. The gated `atan(z)` atom now merges with the ungated one.
  - (c) With `w = 1e-3` the read's "theorem" is disputed (`theorems_disputed` 1).
  - Reach: `Sym<f64>` and `Probe` get brackets only through `analysis.rs:522` `axis_of` at a point box. That includes the K replay at `drive.rs:2036` (`Sym<Probe>` over a midpoint box, shipped rules), so a wrong `SymbolicZero` can reach K telemetry. No release verdict path is affected that I found: the coincidence replay evaluates with no box. At `Interval` the claim holds.
  - Probe: `geom-core/tests/review_r1_probes.rs::r1_a_read_zero_at_an_f64_witness_…`.
- **MINOR-2 (claim 5), sure, by execution.** `door_zero` (`sym.rs:4420`, called at `sym.rs:5150`) no longer excludes zeros the read reached. Whenever the registry is non-empty, any door-walk zero under a definite sign raises `registrations_contradicted`, including one from the read or the algebra with no registration in it. With one unrelated true registration in the session, the read zero above counts contradicted 1 plus disputed 1 on the PR, and 0 plus 1 on main. Algebra zeros were already conflated this way on main, so this widens an existing conflation to read zeros. The docs at 4403–4419 and 4196–4209 still say the column counts "a registered zero". The PR's own claim-5 row (`sym11_…_not_a_disputed_theorem`) is sound, and I planted `discharge(id) || door_zero(id)` in `contradicts`: the row went red. Probe: `r1_a_read_zero_counts_as_a_contradicted_registration`.
- **MINOR-3 (Q4, claim 4), sure, by inspection.** Rule C is deleted, but prose still describes it as live, often dial-off:
  - `sym.rs:105-109` ("sound, unit-pinned … stays dial-off");
  - `sym.rs:343`, `sym.rs:1065` (`signed`'s module doc: "Rule C: … the clause-3 fold");
  - `sym.rs:1737`: `canonical_root`'s doc says "Its fourth source is a certified read and rides rule C's dial", which `root.rs` now contradicts;
  - `sym.rs:2439`: "the three value reads [`signed`] owns";
  - `sym.rs:4002`, `4561`, `4578`;
  - `editor-core/src/analysis.rs:520`, `sweep/src/swept.rs:511`.
  - `Rung`'s doc at `sym.rs:4177-4179` still argues "a stronger claim … re-labelled as a weaker one: `discharge`'s docs argue it". Neither half is true now.
  - The body's sweep patterns (`signed_root` and the like) could not match the prose "rule C", so this is a class: sweep `rule C|clause[- ]3|sign read`.
- **NOTE-1 (claim 1; corrects the brief's premise), sure, by inspection.** `sym::Rung` is `pub`, not private (`sym.rs:4185`). `Rung::Door` is the registry, and `profile::DecisionRecord::answered` (`profile.rs:279`, `pub` under `sym-profile-testing`) carries it per decision, as do the profile ledger's `Door/Decision` lines. So a test-feature reader can still tell "the registry answered". No decision type carries it, so the ruling's "by type" holds for decisions.
- **NOTE-2 (claim 2, registry), sure, by execution (the PR's own row).** `sym.rs:125` says a registered zero is E12's claim with "the axiom checked at the witness". The next paragraph of ERROR-DESIGN E12 (`ERROR-DESIGN.md:624-631`) says the witness "cannot tell an identity from a coincidence". The PR's `sym_root_rows.rs:877` registers `x·x ≡ x` over [0.9, 1.1] and asserts `(x·x − x)·min(x, 3)` is `proven`, although it is non-zero on most of that box. That is Ev's accepted risk. But the receipt is now the only place a registered zero showed apart from a theorem, and this unit removes it, so the premise as written overstates.
- **NOTE-3 (claim 8), likely, by inspection.** The E12 "TAKEN" rewording (`ERROR-DESIGN.md:611-615`) is a description moved by Ev's ruling on #4901, not a new design. `b773363cc1` is an agent commit (checked via the API). Its new text names "the K token `symbolic_zero`" for the column `symbolic`: see Style S1.
- **NOTE-4 (claim 4), sure, by execution.** Rule C was off in every shipped set: `shipped()`, `default`, the coincidence replay (`replay.rs:89`), `plane_eq`, `swept`, and the retry masks, which are AND-masked. Rule G's source 4 read only under `signed_root`, so rule G decides the same in every shipped configuration. Every count I re-took agrees.
- **NOTE-5 (claim 7), sure, by execution.** The link's refusal at `pcurve_envelope` with the door shut is a measurement. Main gives the identical (831 + 0 symbolic, 712 numeric, frozen 1318) and refuses at the same node, `31:4dfc805388a7caa1`. The door is load-bearing there on main too.

## Style
- **S1 (Q1), likely.** "Proven" has at least six spellings: `MarginDiag::proven`, `Zero`/`Pending`/`topo::Discharge::Proven`, `SymCounts::symbolic`, `ShapeOutcome::Symbolic`, the K token and `SampleOutcome::SymbolicZero` (`symbolic_zero`), and `theorems_disputed`. The serialized surfaces disagree: the drive line says `symbolic=`, while K, k-lint's summary (`main.rs:187`) and `sym.rs:1383` say `symbolic_zero`.
- **S2 (Q7), likely.** `rungs(…, door: bool)` (`sym.rs:4363`) always equals `decision`: the first attempt passes `decision` and retries pass `true` only under `decision`. One distinction is carried by two flags.
- **S3 (Q3), likely.** One lib row guards rule C's retirement (`a_sign_of_the_box_proves_nothing`; I planted an abs-by-read fold and it went red). No row covers the read's premise at an inexact witness (MINOR-1). The gate bit returning to the digest is guarded only by C6 in `what_only_the_read_answers`, by inspection; on main my `atan` probe is numeric. A kind re-introduced is guarded by type plus k-lint's retired-token row.
- **S4 (Q7), unsure.** `max(r, −r) − r` over `r ∈ [1e-3, 2e-3]` is `proven` under shipped and `all()`, while `abs(r) − r` is numeric (probe `r1_a_sign_of_the_box_proves_through_max`). That is consistent with E12's wording (max branches, abs does not), but the line between "a sign of the box" and "a decision" is a spelling, and the row's name `a_sign_of_the_box_proves_nothing` claims more than it pins.
- **S5 (Q4), unsure.** Dated history in the retired vocabulary: `sym_9_retry_interval.rs:415-481`, `m10_derived_frame_tilted_interval.rs:1325,1488`, `m10_bulge_interval.rs:255`, and `sym.rs:477-552,882`. These may be intended as records.
- **S6 (Q2), likely.** `door_zero`'s doc (4403-4419) asserts an invariant ("a REGISTERED zero") that the code no longer enforces (MINOR-2).
- **S7, unsure.** Four separately filed items for the same 1e-12 red (`work/*/a-ring-serving-two-pierces-*`): duplicates outside this unit.

**Counts:** 0 MAJOR / 3 MINOR / 5 NOTE, plus 7 style. **Ratings (1–5):** correctness 4, tests 3, docs and prose 3.

**Questions exercised:** Q1, Q2, Q3 (three plants), Q4, Q5, Q6 (body tables against rows), and Q7. Q8 was partial: `sym.rs` header lines 1–1100 skimmed and greps over the rest, not read end to end.

## Gate verification
Run 38328595199 is on `7937b969a8`. `test` is the only red job, and the only red row is `coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven` at ε = 1e-12. The default and 1e-6 rows are 13594/13594 green. Locally on main `f7bb2bb7c` that row **fails at 1e-12 and passes at 1e-9**, so it reds on main too.

## Run locally (debug, `CARGO_INCREMENTAL=0`; the PR's numbers in parentheses where it gives them)
- PR head, 16 rows (`m10_10_pins_interval` 5, `m10_9_pins_interval` 7, tilted family 4): all pass in 183 s. Main: the same set, 15 rows, all pass.
- Bracket at `certifies_at`. Main: 1386 / 5 / 57 / 1004, frozen 1869. PR: 1448 / 1004 (1448 / 1004). Door shut: main 1386 / 5 / 0 / 1061, PR 1391 / 1061 (same). Read shut: main 1386 / 0 / 57 / 1009, PR 1443 / 1009 (same).
- Pad. Main: 1306 / 2 / 64 / 1284, frozen 3294. PR: 1372 / 1284 (1372 / 1284). Door shut: PR 1308 / 1348 (same). Read shut: PR 1370 / 1286, main 1306 / 0 / 64 / 1286.
- Link. Main: 814 / 0 / 48 / 663. PR: 862 / 663 (862 / 663). Door shut: refuses at `pcurve_envelope` on both, 831 / 712.
- Plate 1048 / 654 and annulus 566 / 399: equal on both.
- Tilted family. Main: 572 / 400 / 0 / 514, frozen 1380, `[32,16,0,0]` / `[24,24,0,0]`. PR: 972 / 514, `[48,0]` / `[48,0]` (same). Read shut on the PR: end cap and FlipZ 573 / 300, refusing at `newell_plane_residual`; the start cap is 972 / 514 and certifies (same).
- Not re-taken: the pad's release leaf at 1e2·ε and the leaf cost, best of 3. I used the CI-profile rows instead.
- Plants, each reverted:
  - `contradicts` asking the door: `sym11_a_contradicted_registration_is_not_a_disputed_theorem` goes red.
  - Abs folded by the read: `a_sign_of_the_box_proves_nothing` goes red.

**Probe commit:** `decide/10-review-r1` at `8455ac50f` (probes on top of `7937b969a8`), with this report committed on top. On main the probes run with `.theorem().is_some()` in place of `.proven()`.
