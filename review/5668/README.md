# Review of PR 5668 (F2) — probes

- `instrument.patch`: env-gated (`RAWDOOR_LOG=path`) logging in topo's `raw_door`: per call site
  (`#[track_caller]`), whether `validate_geometric` passes the measured body (f64 only).
  Note: typed doors route through `raw_door`, so with the env set the instrument also runs tier 3
  inside counted regions (it reds shell_census / tcost / sign_walk / thread-count rows — artefacts).
- `rawdoor-verdicts-*.txt`: per raw-door site, count of PASS / REFUSE bodies (1e-9 file = fast run + full sweep run, so doubled).
- `gate-hole.patch`: an unreasoned `classify_unvalidated(&*body)` (a finished, valid body) added
  under `m5_pr9c_sphere_doors.rs`'s `rev` reason with the count bumped: `raw-measure-door-sites.sh` passes.
- `m6_k_funnel_probe.rs` (+ `probe_root.rs`, a `[[test]]` target): m6_surgery with a probe counting what the
  K-FUNNEL bracket records once the gate runs inside it.
