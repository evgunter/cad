---
id: a-joined-section-arc-runs-its-kept-carrier-without-re-seaming
kind: issue
title: An edge join runs the kept section carrier over the removed edge's span without re-seaming, so a long joined arc can sit back near its carrier's ±π/2
status: open
opened: 2026-10-14
priority: P2
cost: M
refs: [4861]
---



## What

Found in review of PR 4861
(`a-section-ellipse-is-re-posed-at-its-nearest-principal-vertex`).
A section chord's ellipse is now seamed at the principal vertex
nearest its own arc (`chord_join::chord_arc`, through
`geom::Curve3::seamed_near`), so its parameter resolves to the arc's
scale. `topo::boolean::edge_join::joined_spec` then runs the KEPT
edge's carrier on through the joined vertex to the removed edge's far
end, through `Curve3::param_near`, and never re-seams it. That carrier
was seamed for the kept arc alone. A long joined arc, such as two
chords joined across a minor vertex of an elongated section ellipse,
can therefore end with its far end back near `±π/2` of that seam. There
the parameter resolves only to `ulp(π/2)` times the semi-axis across
it. That is the miss the re-pose removes at the chord.

Not measured: PR 4861's census probe never joins section chords.

## Owed

Measure whether a joined section arc's ends land within the carrier's
resolution at ε = 1e-12, on the coin-flip and through-axis families of
`crates/sweep/tests/section_ellipse_seam_poses.rs` under a Boolean that
joins their chords. If a far end does not, re-seam the joined carrier
at the vertex nearest the joined arc's middle, as `chord_arc` does.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P2: a joined arc's far end may land past its carrier's resolution: interval and error propagation, P2 by `work/README.md` Priority.
- Moved to HONEJOIN (`work/honejoin/`) by `git mv`, id and body unchanged: whose charter is the curved and chord join's residue. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
