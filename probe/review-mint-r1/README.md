Review probes for PR 7488 at 4c17b542 (never merged).

- `crates/profile/tests/review_mint_r1.rs`: profile probes.
- `crates/editor-core/tests/m10_3_driver_interval.rs`:
  `review_probe_what_the_drive_says_about_the_riser`.
- `mutants.patch`: env-switched mutants (`REVIEW_MUT=M1|M3|T1|T2|T3|U1|U2|U10|U11|U12|A|QTIP`),
  applied to the working tree only; the `*.log` files are the runs.
