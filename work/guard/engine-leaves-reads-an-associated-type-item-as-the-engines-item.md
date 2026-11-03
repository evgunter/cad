---
id: engine-leaves-reads-an-associated-type-item-as-the-engines-item
kind: issue
title: engine-leaves counts Item in impl Iterator<Item = …> as boolean::separating::Item, so every iterator signature in topo raises a pin
status: open
opened: 2026-11-03
priority: P3
cost: E
refs: [6460]
---


Found by the delta review of ONECOPY's PR 6460 (`engine-leaves.sh --lines`). The new hits are entity.rs L318 and L328; live.rs L462, L479, L494, L507, L632 and L1029; and chart_region.rs L2488. All of live.rs's baseline of 4 on main is the same false positive (L430, L447, L527 and L910 are `Item =`). The counter reads an associated-type binding's name as the engine item `boolean::separating::Item`. Either recognise `Item =` inside a generic argument list as not a reference to the engine's item, or rename `separating::Item`. Until then every new iterator signature re-pins a line: PR 6460 raised `chart_region` 4→5, `entity` 0→2 and `live` 4→6. Filed by the ONECOPY orchestrator.
