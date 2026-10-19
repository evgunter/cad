Probes for the second delta check of PR 5119 at 0ab07e42. Not wired into
any crate: drop `r2_shell_role_probes.rs` into `crates/topo/tests/` (plus
its `#[path]` line in `all.rs`) and `r2_enclosure_probe.rs` into
`crates/topo/src/props/` (plus `#[cfg(test)] mod r2_enclosure_probe;`
after `mod quad_lane;` in `props.rs`). `enc_oracle.py` reads the
enclosure dump (`--ignored --nocapture r2_enclosure`, lines `R2E`/`R2F`)
and checks it in exact rationals.
