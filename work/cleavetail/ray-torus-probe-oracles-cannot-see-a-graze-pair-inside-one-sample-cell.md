---
id: ray-torus-probe-oracles-cannot-see-a-graze-pair-inside-one-sample-cell
kind: issue
title: the ray x torus probe oracles count sign changes only, so a graze pair inside one sample cell is invisible to them
status: open
opened: 2026-10-10
priority: P3
cost: E
---


## Found (CLEAVE, the sweep of `ellipse-torus-graze-certifies-six-roots-where-the-true-distance-crosses-four`, 2026-10-10)

That row's false red was a sampled oracle that could not see a graze
pair inside its end sample cell. The ray × torus probes' oracles are
cruder: they count sign changes of a dense sampling and refine no
extremum at all, so a graze pair inside ANY one cell reads as no
crossing.

- `crates/topo/src/boolean/r1_probes.rs`, `oracle_roots` (400,000
  samples over the ray's reach): read by the `Tally::compare` rows. A
  door that certifies a true graze pair reads as a `COUNT` failure (a
  false red); a door that answers `Miss` on such a graze passes, the
  oracle seeing nothing (its `gap > 1e-3` escape aside).
- `crates/topo/src/boolean/r2_probes.rs`, `bisect_roots` (20,000
  samples), read by `r2_adversarial_rays_agree_with_the_independent_counter`
  and `r2_lattice_certified_counts_agree_with_the_independent_counter`;
  and the inline scanner in
  `r2_extreme_radii_ratios_keep_the_certified_roots_honest` (400,000
  samples).

Not measured: whether any of their poses grazes within one cell today.
They are fixed poses and pass, so nothing is red; the gap is that these
rows cannot catch a wrong `Miss` on a graze. The fix is the shape
`crate::boolean::conic_oracle::crossings` has (refine each sampled
extremum of `|F|`, the end cells included) or an exact oracle for the
line × torus quartic.

## Slate triage (2026-10-26, CLEAVE)

- Moved to CLEAVETAIL (`work/cleavetail/`) by `git mv`, id and body unchanged: the track opened for CLEAVE's P3 and P4 tail: refusal stories, guards that cannot go red, latent soundness and hygiene. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
