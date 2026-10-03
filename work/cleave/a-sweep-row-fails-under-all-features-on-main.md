---
id: a-sweep-row-fails-under-all-features-on-main
kind: issue
title: sweep::tilted_sphere_pair_k_rows::a_carved_balls_meridian_fragments_record_no_clearance_charge fails under --all-features on main
status: open
opened: 2026-10-03
priority: P2
cost: E
---


Reported by the `cleave/tangent-interior-refuse` lane (PR 3938, 2026-10-03): the row fails under `cargo test --all-features` on main as well as on that branch, so it is not that PR's. CI's gate does not run `--all-features` for sweep, which is why it is green. Not measured further: reproduce on main, find which feature flips it, and route to the owner of `tilted_sphere_pair_k_rows` (REACH's clearance charge, by name).
