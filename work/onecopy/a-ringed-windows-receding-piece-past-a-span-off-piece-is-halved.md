---
id: a-ringed-windows-receding-piece-past-a-span-off-piece-is-halved
kind: issue
title: A ringed window's receding piece past a piece read off the span is halved, where its near end could certify it
status: open
opened: 2026-11-10
---


Found by PR 6740's review (S5), and disclosed in its body as not that unit.

`Window::chained` (`crates/geom-brep/src/curve_pair/pose/window.rs`) reads a ringed arc outward. Past the first piece read off `b`'s span (`Read::OffSpan`), a `Recedes` piece counts only after a piece read off `b`'s carrier (`from_off`). Any other `Recedes` piece must itself read off the span, halved by `Window::piece_off_span` until each part does.

The certificate PR 6740 gave an arc with no ring holds here too, with `z` the band's zero. A piece whose carrier distance grows outward is least at its end toward the ring, so it is off the carrier past `z` throughout where `Window::off` reads that end off. Such a piece could restart the chain (`from_off = true`) without halving.

No pose has been found red here: the halving decides these pieces, at its cost. Doing it moves which windows certify, so measure first: the reads that move, and the time, as PR 6740 measured.

## Done when

`Window::chained` lets a `Recedes` piece whose near end `Window::off` reads off restart the chain, the reads that move are measured, and a row pins one such piece.
