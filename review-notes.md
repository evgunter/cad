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

## Claim 1 battery results (main e7c2ffe29 -> head be0732270, release, operand = union w/ far brick)
## r2: ops=30000 head BAD=0 head non-operand builds=0 main BAD=0
  ERR Join -> SOUND: 534   e.g. R2 seed=16 case=49 U
## hex: ops=10584 head BAD=0 head non-operand builds=0 main BAD=0
  ERR Join -> SOUND: 1932   e.g. HEX x=(-1.0, -0.25) y=(-1.0, -0.25) z=(1.0, 3.0) U AB
## arc: ops=7350 head BAD=0 head non-operand builds=0 main BAD=0
  ERR JoinDesync -> SOUND: 37   e.g. ARC half half d=(0.0, 0.0) zb=(1.0, 3.0) decl=true U
  ERR Join -> SOUND: 16   e.g. ARC half half d=(0.5, 0.0) zb=(2.0, 4.0) decl=true U
  ERR Euler -> SOUND: 16   e.g. ARC lens square d=(0.0, 0.5) zb=(1.0, 3.0) decl=false S
  ERR RestZipUnsupported -> SOUND: 7   e.g. ARC half half d=(0.0, 0.0) zb=(2.0, 4.0) decl=true U
  ERR ClassificationInvariant -> SOUND: 2   e.g. ARC tri quarter d=(-0.5, 0.0) zb=(0.0, 2.0) decl=true U
  ERR ClassificationInvariant -> ERR Join: 2   e.g. ARC tri quarter d=(-0.5, 0.0) zb=(1.0, 3.0) decl=true U
## brick: ops=6000 head BAD=0 head non-operand builds=0 main BAD=0
  ERR RestZipUnsupported -> SOUND: 8   e.g. BRICK x=(-0.5, 1.0) y=(-0.5, 1.5) z=(-1.0, 0.0) decl=true AB U
- head ignored join1_r1_*/delta_probes: all green except join1_r1_hex_detail (detail probe, expected). BADs: REFLEX sqQ1 sx=-0.5 sy=0.25 U v=16 (filed P0 work/zip/a-flush-declared-reflex-union-ships-the-wrong-volume, pre-existing on main); SEAM ball S EMPTY WRONG want=5e-8 (oracle polygonisation residue).
- the_peg_collar_unions_are_operands: GREEN with --ignored on head (proud+flush OK SOUND). Still #[ignore] in tree, contrary to PR body.
