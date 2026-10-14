---
id: the-plate-verdict-pin-holds-only-at-the-default-eps
kind: issue
title: demos/tour's the_plate_verdicts_hold_their_bits pins bits computed from tol.eps(), so it fails under CAD_TOLERANCE_EPS=1e-12 (identically on main); the pin is per-ε, not ε-free
status: open
priority: P4
cost: E
opened: 2026-10-10
---


Found by SHELL's unit 17 lane (PR 4525) during its merge-base differential, and filed by the SHELL orchestrator.

`demos/tour/src/tolerance.rs:1046`, `the_plate_verdicts_hold_their_bits` (INTENT stage 2 test 13), builds its studies from `tol.eps()`: `spread = tol.eps() / 64`, and from that `half`, `sigma` and `worst`. It then compares the printed verdict bits with one `PINNED` string. So the row can hold only at the ε the pin was taken at. Under `CAD_TOLERANCE_EPS=1e-12` it fails `"a plate verdict moved"`, identically on merge base 4a4fce2f4 and on 533bfea56. The plate study never reaches the offset door.

The nightly does not catch this. It runs the tour's suite at the default ε only (`.github/workflows/nightly.yml`, "demos tour suite"), and re-takes only `demo-tour certified` at 1e-6 and 1e-12. Anyone running the tour's suite at another ε row, as SHELL's differentials do, gets a red row.

Owed, one of:
- the pin keyed per ε row, i.e. one pinned string per supported ε;
- the row skips, with a stated reason, outside the ε its pin was taken at;
- the inputs expressed so the verdict bits are ε-free.
