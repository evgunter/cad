Re-review probes for PR 6385's fix pass (head bbf92247, base 8d59fac2). Not part of the PR.

- `probe_head.rs` / `probe_main.rs`: a module appended to `crates/topo/src/bound_side.rs`
  on each tree; `rerev6385_dump_verdicts` / `rerev6385_dump_translated` print one
  verdict per pose (`--no-capture`), diffed tree against tree and shift against shift (`tr.py`).
- `probe_head_honesty.rs`: witnessed edge verdicts against an independent offset at the arm.
- `mutants.py` + `run_mutants.sh`: the five mutants, topo lib tests at the default eps.
- `round_err.py`: Round's f64 offset against an exact one, 0..1e4 m, radii 1e-3..1e3.
