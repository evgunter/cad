IN PROGRESS

# Review r2 — PR #4036 at 266264cc

Probe: crates/sweep/tests/review_r2_vv_probes.rs (shapes × cube edge/corner × grid/tilt/psi/hex sweeps).
Early: hex sweep (box corner vs cube corner sharing a cone axis, n=6 survivors) 648/648 SOUND on head.
Interim (head, my probe): 0 BAD in grid/psi/hex. tilt: 105 BAD at eps=1e-6 = my f64 oracle's error (exact-rational oracle agrees with kernel on all checked).
F12 PairingMismatch fires on head 1422 times (notch, notch5, reflex315 vs cube edge/corner), all at n=6 distinct germs, tie fallback never fired. Main comparison pending.
