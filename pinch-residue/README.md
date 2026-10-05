# Probe evidence for `join/pinch-uncrossed-residue`

Not for merge.

- `crates/sweep/examples/r2_pinch_probes.rs` is review r2's battery for
  PR 4038, with an `R2P_CHI` per-body Euler count added.
- `crates/sweep/examples/r1b_pinch_probes.rs` is review r1's battery
  for PR 4038.
- `crates/sweep/examples/r2_holed_probe.rs` is PR 4026's review r2
  `r2_holed_battery`, ported to `AtRestBody`.
- `instr.patch` holds the env-switched instruments, measured on main
  `4cf3f8b9`:
  - `PX_OUTERLOOP`: an outer loop may cross;
  - `PX_DUMP`: the pinch vertex's orbit at the refusal;
  - `PX_SPLICE`: splices the crossed face's loops after the zips;
  - `PX_MFKRH`: promotes the ring instead;
  - `PX_RINGFACE`: the outer × ring two-face crossing, which the lane
    built;
  - `OUTCOME_WIDE`: lifts `outcome`'s 110-character cut.
- `diffcls.py` takes two output files and prints the per-line moves.
