---
id: main-red-bounds-roster-misses-the-four-along-side-door
kind: issue
title: Main red: the sole-bracket-bound roster misses four::along_side (PR 5938), which branches on a bracket's ends
status: closed
opened: 2026-10-29
priority: P3
cost: E
closed: 2026-10-29
branch: splice/census-reds
pr: 6040
---


Found by nightly triage 2026-10-29, run
[40823227622](https://github.com/evgunter/cad/actions/runs/40823227622)
(main `5648e577eb`), job `full suite (every eps row, slow set included)`.
Red at every ε row.

## What

`geom-core` `bounds_census::every_sole_bracket_bound_door_is_in_the_roster`
(`crates/geom-core/tests/bounds_census.rs:883`):

> the sole-bracket-bound roster no longer matches the tree.
> arrived or moved (owe a roster line WITH its disposition):
> `("crates/geom-brep/src/ssi/four.rs", "along_side")`

`four::along_side<T: Bounds>` (`crates/geom-brep/src/ssi/four.rs:677`)
arrived with PR 5938 (`splice/tube-side-tripwire`, merge `e56509e0`). It
reads each control point's bracket `lo()`/`hi()` against a domain side
and answers a bool. Its callers (`ssi/certify.rs:1792`, `four.rs:888`)
use that bool only to name a refusal: `Shortfall::AlongSide` in place of
`Undecided` (`four.rs:568`). The certified answer itself does not branch
on it. The PR gate does not run the census, so #5938 merged green.

The same merge also moved `ssi/four.rs` in the certified endpoint census,
from (3, 2) to (4, 2). That move is logged on
`main-red-certified-endpoint-census-moved-in-six-files`.

## What closing it takes

Write `along_side`'s `Site` line in `bounds_census.rs`'s roster with
its disposition. The disposition is probably that it names a refusal,
so no answer branches on it at a `Dual`; the owner decides that at the
site. Alternatively, tighten its bound if the read does need one.

Reproduce: `cargo nextest run -p geom-core --ignore-default-filter -E
'test(=bounds_census::every_sole_bracket_bound_door_is_in_the_roster)'`.

Banded P3: the row is a guard that went red. Nothing reports a wrong
answer.

## Closed

Closed 2026-10-29 by `splice/census-reds`. Main `0011f58a` also missed
`edge_nurbs.rs`'s `impl<T: Bounds> FeetRefusal<T>`, from PR 5959. Both
doors are now in the roster as `Payload`. `along_side`'s bool only
renames a shortfall `Shortfall::AlongSide`, and an `Ok` never reads it.
`FeetRefusal::lane` reads the reach's low end into the refusal's lever.
