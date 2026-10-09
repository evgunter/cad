---
id: the-travel-slack-could-be-read-away-near-the-site
kind: issue
title: The join's travel margin pays a 2ε cot ψ slack that a tangent projection near the site would not
status: open
opened: 2026-10-09
priority: P3
cost: M
refs: [a-steep-ellipse-travel-margin-ties-band-apart-sites-and-falls-back-to-the-chord]
---


Found by PR 4396's fix pass, which built the slack.

## What

`crates/topo/src/boolean/join.rs` `travel` decides `turned_past` (the
axis-plane side, read as arc) against `off_conic_slack`, `2ε cot ψ`.
That is how far two sites, each certified only within ε of the conic,
can move a reading taken across the plane through the axis. On an
ellipse's flank it is `ε (k² − 1)/k`: 9.9 bands at `k = 10`, 60 at
`k = 60`. So partners closer than about that plus the band now escalate
(`bool_join_arc_clear`), where they are apart in space by much more
than the band.

The amplification belongs to the axis-plane reading, not to the sites.
Near the site, the projection onto the conic's tangent there,
`t̂·(p − site)`, is also the arc to first order, and a site's offset
across the conic moves it by second order only. The axis-plane side is
still needed far off, where the tangent projection stops being
monotone.

## Done when

`travel` reads the tangent projection where it is monotone (the two
agree in sign, and the conic has turned well under a quarter-turn),
and the axis plane elsewhere. The slack is then charged only where the
axis plane is read. `travel_rows`' 24 bands at `k = 60` then order
instead of escalating.
