---
id: geom-brep-keeps-six-subdivision-walks-of-its-own
kind: issue
title: geom-brep keeps six subdivision walks with their own stacks and depth caps beside topo's one bisection core
status: open
opened: 2026-10-13
priority: P3
cost: M
---

Found by FUSE's sweep for `the-localization-core-is-half-unified`
(branch `fuse/the-localization-core`), which put `topo`'s walks —
`carrier_touch`, `circle_roots`, `spiric_arc` (three) and
`census::curved` (two) — on `topo::bisect`. The same shape (a stack of
pieces, popped depth first, a depth or piece cap) stands six more times
in `geom-brep`, which `topo::bisect` cannot serve from its crate:

- `ssi/exhaust.rs` (the exhaustive cell walk, `stack` of `(cell, depth)`);
- `ssi/section.rs` (`stack` of pieces);
- `ssi/one_arc.rs`, twice (`(s, t, depth)` and `(x, depth)`);
- `ssi/four.rs` (`(x, depth)`);
- `props/quad/` (the quadrature's `(block, depth)` refinement).

Owners: `ssiarith` / `ssiedge` / `ssimarch` (the `ssi` walks) and
`flux` / `fluxtail` / `quad` (`props/quad/trimmed.rs`) — two programs, so filed
here. The question is whether the core moves down a crate (to
`geom-core`, beside `Band`) so every localization shares one budget
and one stop, or whether these walks' differing semantics (depth caps
that settle rather than refuse, quadrature refinement that is not a
localization) are a reason to keep them apart — answered at each site.

Not on this list: `topo::shell`'s `MovedWall::cut`, a breadth-first
walk whose budget degrades to reading a piece as its ball rather than
refusing — a different contract.
