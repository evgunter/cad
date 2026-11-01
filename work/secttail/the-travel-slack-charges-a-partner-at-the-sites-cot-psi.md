---
id: the-travel-slack-charges-a-partner-at-the-sites-cot-psi
kind: issue
title: The join's travel slack charges the partner site's offset at the reading site's cot psi, which holds only near the site and, on a centrally symmetric conic, at the half-turn
status: open
opened: 2026-10-12
---


Found by GERM's U-T4 (`germ/plane-torus-chord-spiric`), 2026-10-12,
measuring whether the slack holds on a spiric oval, as that unit's
brief asked. Sibling of `the-travel-slack-could-be-read-away-near-the-site`
(which is about the slack being too large near the site); this one is
about it being too small away from it.

## What

`crates/topo/src/boolean/join.rs` `off_conic_slack` charges `2ε cot ψ`
at the reading site: `ε cot ψ` for that site's own offset across the
curve, which is exact to first order, and the same again for the
partner's. The partner's offset `δ n̂_p` moves `turned_past` by
`δ·|â·((s − c) × n̂_p)| / |turn(t̂_s)|`, which is `cot ψ_s` only where
`n̂_p = ±n̂_s`: at the site, and at the half-turn of a centrally
symmetric conic. A spiric oval is not centrally symmetric, and between
those points no conic is either.

## Measured

Instrumented `travel` (scratch, never committed) over every reading of
every op on the U-T4 poses (S1, M3, P4, the box between two ovals'
planes, B1 and B2 on the donut, the one-face tori and the seamed torus;
B5, B5′ on the fat torus): 15 700 readings. The true first-order slack
`cot ψ_s + far_p` exceeds the charged `2 cot ψ_s` on most readings of
B1, B2, P4, B5 and B5′, by up to `0.98ε` (charged up to `1.05ε`, true up
to `1.28ε`). On every under-charged reading `|turned_past| ≥ 0.051 m`,
over `4·10⁷` times the true slack at the default ε, so no decision
moved, and every body those poses build holds to the truth.

## Done when

The slack charges the partner's offset at the partner
(`ε·|â·((s − c) × n̂_p)| / |turn(t̂_s)|`, the partner's tangent being its
germ direction), or a row shows a pose where the difference decides
and the rule's reading is right there anyway.

## Moved to SECTTAIL (2026-10-25)

From SECT, cut on its priority seam (SECT measured 46 budget points against 31) in a chat-approved refold. Read SECT's step 3 first: SECT's plan says several of these may close by retirement then.
