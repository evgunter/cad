---
id: graze-side-tighten-offer-at-k-under-two-recurs-in-band
kind: issue
title: split: the conic graze side's tighten offer at K < 2 lands the belly read in band again, so the refusal recurs with a smaller offer
status: open
opened: 2026-10-10
priority: P3
cost: M
---


(Filed by PR #4571's fix pass, under the orchestrator's ruling.)

## What

`splitting/classify.rs` `conic_plane_candidates` reads three things in
order: the conic's reach `R` (`split_conic_plane_parallel`), the graze
`R − |D|` (`split_conic_belly_graze`), and the graze's side `D`
(`split_conic_graze_side`, through `decide_nonzero`). Where the side
decides zero, the gate's rejection carries `D`. The Boolean
(`EdgeOnPlane`, sized `AnySign`) then offers "tighten the tolerance below
|D|/K".

The offer can be followed, and it converges, because |D| ≥ (K−1)ε > 0.
But at K < 2 it does not get through in one step.

**Worked case:** K = 1.5, R = 1.8ε, D = ε.

| ε′ | band (ε′, 1.5ε′) | `R` | `R − \|D\|` = 0.8ε | `D` | result |
|---|---|---|---|---|---|
| ε | (ε, 1.5ε) | Positive | Zero | Zero | rejected; offers ε′ < ε/1.5 ≈ 0.667ε |
| just under 0.667ε | (0.667ε, ε) | Positive | **in band** | — | `BellyGraze` escalates again; offers below 0.8ε/1.5 ≈ 0.533ε |
| below 0.533ε | (0.533ε, 0.8ε) | Positive | decides Positive | — | two roots, the crossing answers |

So the refusal recurs once, with a smaller offer, before the operation
passes.

## Open

- Whether the graze side's offer should name the tolerance the whole
  root lane passes at, the smaller of `|D|/K` and `(R − |D|)/K`, rather
  than its own.
- No executed row covers this offer. `boolean/offer_rows.rs` runs at the
  process K, and the case is reachable only at K ≤ 2, so a row needs a
  K-pinned harness.

## Slate triage (2026-10-26, CLEAVE)

- Band P3: the tolerance offer at K < 2 does not get through in one step: library usability: the refusal names the wrong cause, recourse or payload, P3 by `work/README.md` Priority.
- Priced `M`, the effort the body describes.
- Moved to CLEAVETAIL (`work/cleavetail/`) by `git mv`, id and body unchanged: the track opened for CLEAVE's P3 and P4 tail: refusal stories, guards that cannot go red, latent soundness and hygiene. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
