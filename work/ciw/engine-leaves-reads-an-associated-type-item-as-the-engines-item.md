---
id: engine-leaves-reads-an-associated-type-item-as-the-engines-item
kind: issue
title: engine-leaves counts Item in impl Iterator<Item = …> as boolean::separating::Item, so every iterator signature in topo raises a pin
status: closed
opened: 2026-11-03
priority: P3
cost: E
refs: [6460]
closed: 2026-11-22
---


Found by the delta review of ONECOPY's PR 6460 (`engine-leaves.sh --lines`). The new hits are entity.rs L318 and L328; live.rs L462, L479, L494, L507, L632 and L1029; and chart_region.rs L2488. All of live.rs's baseline of 4 on main is the same false positive (L430, L447, L527 and L910 are `Item =`). The counter reads an associated-type binding's name as the engine item `boolean::separating::Item`. Either recognise `Item =` inside a generic argument list as not a reference to the engine's item, or rename `separating::Item`. Until then every new iterator signature re-pins a line: PR 6460 raised `chart_region` 4→5, `entity` 0→2 and `live` 4→6. Filed by the ONECOPY orchestrator.

Moved from GUARD to CIW on 2026-11-20, by `git mv`, when GUARD merged into CIW: GUARD held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CIW carries.

## Closed

2026-11-22, CIW triage (stale). Fixed. `scripts/gates/engine_leaves.py:49` now excludes "an associated type's binding (`Item` in `Iterator<Item = X>`)". The line arrived with PR 7017 (merge 714efc6239, 2026-11-15), found by `git log -S'Iterator<Item = X>'`.
