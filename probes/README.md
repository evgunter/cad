# R2 review probes — MESH-4 / PR 1517 (issue 881)

Additive review artefacts for the independent R2 review of PR 1517 at
frozen head `29d70b04ba9621cac49b336d82988f0d05d949d0`. Nothing here is
wired into any cargo target; each file is a standalone `rustc` program.

- `r2_band_edge_differential.rs` — bitwise differential of the six
  ported terminal ε reads against the merge base `ba0a90a08`'s bare
  spellings, over a boundary battery centred on the band edge (exact
  band, +/-1 ulp, +/-0.0, subnormals, +/-inf, NaN, a 2000-point spread)
  at eight bands including 0.0 and NaN. 242040 checks, 0 mismatches.

- `r2_redfirst_simulation.rs` — the three new `sizing::tests` rows
  transcribed and run against the mutations the PR claims it verified
  red-first against, plus one it does not claim.

Run:

    rustc -O probes/r2_band_edge_differential.rs -o /tmp/r2probe && /tmp/r2probe
    rustc -O probes/r2_redfirst_simulation.rs   -o /tmp/r2red   && /tmp/r2red
