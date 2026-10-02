# JOIN-1 delta-2 review notes (PR 3790 @ be0732270)

Running notes; restart-safe. Delta under review: 21b7f289..be0732270 net of main.

## Status
- [ ] setup / builds
- harness: crates/sweep/tests/join1_delta2_harness.rs (arc/brick/R2/R1-hex with OPERAND column = union with far brick)
- main tree = git archive origin/main e7c2ffe29 at /home/user/join1-delta2-main (+ harness)
- PR body claims `the_peg_collar_unions_are_operands` "no longer ignored" — on be0732270 it is STILL #[ignore] (join1_delta_probes.rs:481). Premise mismatch.
