Review probes for PR 4551 at 18b57227 (not for merge).

- `src/main.rs` + `check.py`: `between` (200k cases) and `blossom`/`restrict` (30k cases, p = 1..8)
  emitted as f64 bits by Rust, checked against exact rationals (Fraction) in Python.
  Run: `cargo run --release -- between > between.txt`, `... blossom > blossom.txt`, then `python3 check.py between|blossom`
  (paths in check.py are /root/*.txt).
- `bench/`: `canonical_composite` timing, built once against head and once against base.
- `mutant-*.diff`: the step mutations and the frame_box_on revert the review ran the suites under.
