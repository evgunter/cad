# CERT-M3 R1 compile probes

Out-of-workspace crate (own `[workspace]`), path-deps on `../../crates/*`.
Each `src/bin/*.rs` is one compile-only probe; run
`cargo check --bin <name> --message-format=short` (add `--features probe`
/ `--features interval` for the two gated scalars). Expected:

| bin | expect |
|---|---|
| pnl_f64, pnl_sym, pnl_probe, pnl_interval | green |
| pnl_dual, pnl_symdual | E0277 `Dual<f64>: CertifiedEnclosure` |
| door_default, table_dual | green (structural doors admit `Dual64`) |
| door_dual | **E0599** (inherent method, bounds unsatisfied) — NOT E0277 as the doctest annotation claims |
| sole_enclosure_euler, sole_bounds_euler | E0599 (either term alone fails) |
| sole_enclosure_certify | E0277 `T: CertifiedBounds` (Bounds missing) |
| table_dual_certify_lane, table_dual_validate_geometric | E0277 |
