---
id: a-ringed-windows-receding-piece-past-a-span-off-piece-is-halved
kind: issue
title: A ringed window's receding piece past a piece read off the span is halved, where its near end could certify it
status: closed
opened: 2026-11-10
closed: 2026-11-16
branch: onecopy/ringed-near-end
---


Found by PR 6740's review (S5), and disclosed in its body as not that unit.

`Window::chained` (`crates/geom-brep/src/curve_pair/pose/window.rs`) reads a ringed arc outward. Past the first piece read off `b`'s span (`Read::OffSpan`), a `Recedes` piece counts only after a piece read off `b`'s carrier (`from_off`). Any other `Recedes` piece must itself read off the span, halved by `Window::piece_off_span` until each part does.

The certificate PR 6740 gave an arc with no ring holds here too, with `z` the band's zero. A piece whose carrier distance grows outward is least at its end toward the ring, so it is off the carrier past `z` throughout where `Window::off` reads that end off. Such a piece could restart the chain (`from_off = true`) without halving.

No pose has been found red here: the halving decides these pieces, at its cost. Doing it moves which windows certify, so measure first: the reads that move, and the time, as PR 6740 measured.

## Done when

`Window::chained` lets a `Recedes` piece whose near end `Window::off` reads off restart the chain, the reads that move are measured, and a row pins one such piece.

## What `onecopy/ringed-near-end` settled

`Window::chained` lets a `Recedes` piece past a piece read off the span restart the chain where `Window::off` reads its end toward the ring off the carrier past the band's zero. `d` is least at that end, so the piece is off the carrier throughout.

Measured before the change, with the old and new chain computed side by side in each `holds_only` (release, debug assertions on), over all of geom-brep, topo, sweep and editor-core:

| ε | windows | verdicts moved | windows halved less | `piece_off_span` calls |
|---|---|---|---|---|
| default | 134,283 | 0 | 15 | 25 → 4 |
| 1e-12 | 115,747 | 0 | 15 | 25 → 4 |
| 1e-6 | 113,262 | 0 | 15 | 25 → 4 |

The 15 windows are in four `curve_pair` rows: `plane_rows::two_meetings_across_a_whole_turns_seam_in_a_narrow_window_are_both_read` (8), `plane_rows::two_meetings_across_a_whole_turns_seam_are_both_read` (4), `order_rows::the_needle_far_from_the_origin_reads_as_at_it` (2) and `plane_rows::two_circles_at_a_shallow_tilt_meet_twice_near_one_plane_root` (1). Summed window time fell from about 850–870 ms to about 800 ms at each ε, though the old chain always ran first, so the timing is loose.

The change is a saving, not a verdict, so no row is red without it. `window::rows::a_receding_piece_past_a_span_off_piece_restarts_the_chain_by_its_near_end` pins the restart's premises on a real piece (receding outward, not off whole, its end toward the ring off) and the chain's verdict over it.

