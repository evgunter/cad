# PARTFRAMES — log

## 2026-11-21 — Opened, split off from ONEHOME

Ev chose to run fork 288's stage 2 as its own program (PR 7282: "splitting it off sounds good!"). ONEHOME moved the stage-2 row and the five rows that wait on it or are answered by it here by `git mv`, ids and bodies unchanged: `a-body-records-its-parts-in-their-frames`, `the-boolean-reads-each-operand-in-its-own-coordinates`, `vertex-on-edge-is-answered-from-the-claim-not-from-coordinates`, `the-graft-holds-a-placed-copy-twice`, `the-at-rest-localization-moves-each-copy-once-per-pair` and `a-boolean-refusing-on-a-boxed-members-widened-data-does-not-name-the-member`. Status `ready`; no orchestrator. (ONEHOME orchestrator)
- 2026-11-22 — **Seam (HELDRECORDS's first unit, PR 7124, `embed/records`):** a `⊔`'s aggregate is `topo::Assembled`, built by `graft_finished` / `graft_leaf`, which wrap `boolean::graft_placed` and carry the arrival's held records; `Framed::graft_into` (`crates/editor-core/src/eval/framed.rs`) takes `&mut Assembled` and gains `GraftIn::Carry`. (embed/records implementer)
