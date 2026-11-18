You are a designer lane for the CAD kernel repo at /home/user/cad (a greenfield Rust B-rep kernel, API-first, functional style, fail-loud). Read `docs/prompts/designer.md` IN FULL first: it binds you, including the report format (`## For Ev` / `## For the orchestrator`, ≤150 lines). Then `CLAUDE.md`, and `memories/ev-profile.md` (who reads your `For Ev` section).

You work READ-ONLY: do not edit, commit, build or push anything in /home/user/cad, and do not write to the session's /tmp scratchpad (other lanes share it). If you need scratch files, use `~/.local/share/cad-work/export-fork/<a short random name you pick>/`. Other designers are weighing the same problem independently; you will not see their reports until all are in.

## The problem

`step_import::import_step` reads a STEP (ISO 10303-21) file into kernel bodies. The bodies it ships carry no provenance: their geometry descriptions have no `GeomSource` (the D5/N6 identity channel — the kernel's record of *where a description came from*, compared by token equality so that two descriptions from the same source are known to be one geometry without numeric comparison). An imported body is therefore indistinguishable from a hand-built one, and every consumer that reads identity structurally — the boolean's coincidence rung, axis-shaped declarations (`docs/AXIS-DECLARATION-DESIGN.md`, ratified with "absence refuses"), the chart/one-surface readings — treats imported geometry as having no identity at all.

Meanwhile the importer holds a candidate identity and drops it: its per-solid maps are keyed by the file's own entity ids (two faces that reference one `#n` surface entity genuinely share that surface in the file's own structure), and it discards them at the door. Placed assembly instances go through `transform_rigid`, which clears sources.

What needs deciding: **whether, and how, an imported body should carry identity derived from the file, and what that identity means.** The row that asks it, with the filer's own analysis (treat its suggestions as one reader's hypotheses, not as options you must choose among), is `work/export/step-import-discards-the-entity-ids-that-are-its-identity-channel.md`. The questions it says must be designed rather than assumed include: how an import-minted identity is kept from ever colliding with a recipe-minted one (a collision is silent wrong geometry — two unrelated surfaces glued as one — not a refusal); how instancing (N placed copies of one component) keeps copies distinct; where in the import pipeline identity is attached relative to placement; and what the identity layer's scope claim (identity holds per evaluation against the current document) means for a body that was never evaluated from a document. Check each of those against the tree; some may be stale or mis-framed.

Rows that wait on or touch this answer, worth reading:
- `work/origin/axis-source-lowered-bytes-carry-no-minter-namespace.md` (parked on this row)
- `work/origin/kernel-direct-origin-does-not-separate-hand-built-from-derived.md`
- `work/origin/axis-per-component-source-beside-geom-source.md`
- `work/topo/geom-source-absence-conflates-four-origins.md` (closed: how origin is now represented, PR 2576)
- `work/origin/two-provenance-free-keys-holding-one-surface-read-as-two-charts.md`

Where to look in the code: `crates/topo/src/provenance.rs` and wherever `GeomSource`, `SourceExpr`, `GeomOrigin`, `AxisSource` and `RecipeNodeId` are defined (grep), `crates/topo/src/source_walk.rs`, `crates/step-import/src/` (`lib.rs`'s `import_step`, `assemble.rs`, `adopt.rs`), the recipe layer's stamping (`stamp_minted` and its callers), `transform_rigid`. Ratified text: `docs/DESIGN.md` (its `[identity]` and `[assemblies]` sections, D5, N6), `docs/AXIS-DECLARATION-DESIGN.md`, and the README design pages DESIGN.md's companion table lists for topo and step-import.

The deliverable is your recommendation with its argument, per the designer prompt. The answer may be that the code should change somewhere other than the importer, or that part of what the row asks for should not be built.
