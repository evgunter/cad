---
id: engine-leaves-passes-a-raised-pin-and-its-message-points-the-wrong-way
kind: issue
title: engine-leaves.sh passes a raised pin (it 'reads only in review') and its failure text sends a root reader into the engine
status: closed
opened: 2026-11-06
priority: P2
cost: E
refs: [the-census-sort-of-a-boolean-refusal-has-no-legal-home-under-topo-homes]
branch: onecopy/engine-leaves-pins
closed: 2026-11-06
---


From the census-sort fork (2026-11-06; both designers).

## What

1. **A raised pin passes.** `scripts/gates/engine-leaves.counts`' header says a raised pin "reads only in review", because CI checks out one commit. So PR 6451's 506 → 510 passed. CI's PR job checks out the test merge; if the gate job has the base available (the merge's first parent), the gate can compare each pin against the base's counts file and fail a raise. First check whether the `lint` job has the history.
2. **The failure text points the wrong way.** `engine-leaves.sh` prints "code naming an item declared under an engine belongs to that engine". That sends a root reader into the engine, which is the misreading behind the census-sort row. State the README's remedy instead: the named item is a reading the engine hosts, so move it to the root or below.
3. **The header should state the migration practice.** A new site is paid in the reader's `names-*` pin, citing the unit-5 move row that retires it, never by a new `pub(crate)` in the engine.

All of this is agent text (PR 6458), not ratified.

## Closed (2026-11-06, onecopy/engine-leaves-pins)

1. **A raised pin reds.** ci.yml's `lint` job checks out one commit, so it fetches the change filter's base and passes it as `GATE_BASE`. `lib.sh` `gate_base_file` reads a file at that base, and that read fails loud when the base is unreachable. `engine_leaves.py --base` prints `RAISED` for a pin above the base's pin. The one rise admitted is a reader's `names-*` pin whose line cites a `work/` row. It is printed as `CITED` in the green log. The citation's spelling has one home (`lib.sh` `GATE_ROW_CITATION_ERE`), read by both gates. `pin-cites-an-open-row.sh` now reads `.counts` pin lines, so the citation must name an open item: `kind` not `program`, and `status` `open`, `spec`, `dispatched` or `review`. `plan.md`, `log.md`, a `program.md`, and a parked or deferred row are refused. Self-test cases cover: an uncited raise, a new line, a cited `crate-visible` raise, an unreachable base and a base with no counts file (each red); a lowered pin and a cited reader raise (each green).
2. **The text states the remedy.** It now reads: "a red here means that what you need belongs at the root or below … move it out of the engine to the crate root (to geom-brep if it takes no body); do not move the reader into the engine".
3. **The header states the migration practice.** A new site is paid in the reader's pin, citing its move row, never by a new `pub(crate)`. This PR does that once: census.rs `names-boolean` 505 → 506, citing the census-sort move row.
