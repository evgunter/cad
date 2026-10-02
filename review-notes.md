# JOIN-1 delta-2 review notes (PR 3790 @ be0732270)

Running notes; restart-safe. Delta under review: 21b7f289..be0732270 net of main.

## Status
- [ ] setup / builds
- harness: crates/sweep/tests/join1_delta2_harness.rs (arc/brick/R2/R1-hex with OPERAND column = union with far brick)
- main tree = git archive origin/main e7c2ffe29 at /home/user/join1-delta2-main (+ harness)
- PR body claims `the_peg_collar_unions_are_operands` "no longer ignored" — on be0732270 it is STILL #[ignore] (join1_delta_probes.rs:481). Premise mismatch.
- confirmed: describe_minted_edges Scaffold arm now = recorded_skip(f1,f2) over merged.skipped (curved only per MergeCoplanarOutcome doc). ops.rs:1882
- STYLE: join1_mechanisms.rs:52-58 walls_and_caps doc cites `BooleanReduction::continuation` (dropped at merge) + "REST zip instead" stale; and duplicates mate2_common::wall_decls which already adds continuations (double declaration).
- STYLE/possible MINOR: vtxfac.rs ~640 flips only start-on-arrival; insert.rs mint_directed handles 4 cases incl end-on-departure. Asymmetric siblings.
- insert.rs structural match: contradictory germs (gf arrival + gt arrival) silently first-wins.
- peg_in_socket_union_holds (mate7a) asserts soundness whenever it builds at any eps; accepts JoinDesync|Join(_) only when eps > DEFAULT_EPS (1e-9). Not weakening in unsound direction; need eps ladder.
