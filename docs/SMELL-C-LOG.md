# SMELL-SCAN Track C — orchestrator log

The live state of Track C: the lane roster, the rulings made inside it,
and what has landed. §D of `docs/SMELL-SCAN-2026-08.md` remains the
schedule; this file is the *execution* record, and every finished unit
still gets its `FIXED by #NNN` lead at its own finding.

**This programme runs entirely outside the model A/B experiment.** No
Fable/Opus pairing, no ordinal, no row in `docs/MODEL-AB-LOG.md` —
nothing here touches that log. (Evan, 2026-08-20: the Fable limit is
hit, so Track C is deliberately to the side of it.)

**Branch prefix:** `smellc/` for units; the orchestrator sits on
`claude/track-c-orchestration-7b06uq`.

---

## Review policy for this track

Not the full orchestrator protocol. Per Evan, 2026-08-20:

- **Style review on every unit** — `docs/prompts/reviewer-style-lane.md`,
  dispatched by path, with the per-lane emphasis the dispatch owes
  (`docs/REVIEW-STYLE-DISPATCH.md`). On top of the standing brief, a
  Track C style review answers two questions the brief does not:
  1. Was the original stylistic problem — *as the finding states it* —
     **completely** fixed, or fixed at the reported instance only?
  2. Was it fixed **in the best way**, or merely in a way that closes
     the finding's sentence?
- **Adversarial review only where the change carries meaningful risk.**
  That is a minority of the track and is marked per row below. A unit
  that rewrites prose, adds acceptance rows, or moves code without
  changing a decision does not get one.

## Recording convention

Implementer lanes **do not edit** `docs/SMELL-SCAN-2026-08.md` — three
concurrent lanes editing one file is the CONFLICTING failure mode. Each
lane puts what it would have written into its **PR body**; the
orchestrator lifts it from there and records the landing at the
finding, in batches, replacing the original problem statement (version
control keeps it) exactly as previous findings have been recorded. The
row also leaves §D's Track C table when it lands.

---

## Rulings made in this track

| # | Question | Ruling | By |
|---|---|---|---|
| **C-R1** | **H16 — the STL header is not caller-settable.** New public API, which #639 deliberately left as a residue for Evan rather than closing. | **Take it**, as a design PR that **waits for sign-off**. `StlOptions` mirrors `StepOptions` (solid name + 80-byte header; defaults leaking nothing). Preserve `ascii.rs`'s *"constant in this build"* wording — phrased so a caller-settable header does not falsify it. The only pins are `export.rs`'s `NAME:` row and `review_m2_pr7.rs:172`'s `HEADER:` row; both move with the change. | Evan, 2026-08-20 |
| **C-R2** | **S29 — nothing states what the mesh sizing *policy* is.** Writing one down is a design act, not a cleanup. | **Split the lane.** The mechanical vocabulary unification self-merges; the **policy statement** goes out as its own design-conversation PR and waits for sign-off. The question has been routed around twice already. | Evan, 2026-08-20 |
| **C-R3** | **S31 — the `geom-curves`/`geom-surfaces` boundary.** The duplicated constants and helpers need a home, and where that home is *is* the crate-boundary call. | **Merge the two crates.** Named `geom`, above `geom-core` and `bvh` (the name is the orchestrator's call). Authorises editing `DESIGN.md`'s crate-table row and its pointer at the crate docs as authoritative text — and nothing else ratified. | Evan, 2026-08-20 (name: orchestrator) |
| **C-R4** | **S29 is not blocked on a design conversation.** §D routes it to `docs/TESS-SPLIT-SPEC.md` / PR #568. | **Correction, and it stands.** Checked by #684's reviewer: both #568 and TESS-SPLIT-SPEC are scoped entirely to the NURBS per-cell schedule in `nurbs_cert`. **No open conversation covers `curved::grid_steps`**, so S29's analytic-chart half was never waiting on a venue — it does not have one. §D's C3 row is wrong on this point and is corrected when C-j lands. | the parallel orchestrator, 2026-08-20 |

---

## Lane roster

Gates are the *live* ones as of 2026-08-20; §D's own edge list is
superseded for Track C by this table.

| lane | finding | scope | gate | review |
|---|---|---|---|---|
| **C-a** | **S24** — the assembly gate's success path is documented unreachable | `editor-core/src/{assembly,mate}.rs` | none | style |
| **C-b** | **S30** — ~1,050 lines of instrument in the mesh hot loop | `mesh/src/{budget,probe_stats,trimmed,chords}.rs`, the feature matrix, the CI row, `memories/telemetry-gating.md` | none (disjoint from #684's `curved.rs`) | **adversarial** + style |
| **C-c** | **S31** — the crate split that buys nothing | `geom-curves/`, `geom-surfaces/`, new `geom/`, 11 dependents, `step-export/src/writer.rs` | none | **adversarial** + style |
| **C-d** | **H12** — the SSI sweeps' other never-silence doors have no acceptance row | `geom-brep/tests/` | #692 | style |
| **C-e** | **H13** — `sweep_body`'s helix rows have no orientation coverage | `sweep/tests/{m8_14_long_turn_sweep,m7_skin_integral}.rs`, `step-export/tests/common/mod.rs` | none | **adversarial** + style — #636's level-plane oracle trips its own precondition here (`cos ≈ 0.011`), so this needs a *new* oracle, and the oracle carries the soundness |
| **C-f** | **H11** — #632's two residues | `editor-core/src/{resolve/,select.rs,refactor.rs}` | none | style |
| **C-g** | **S32** — `Surface`'s one-partial-per-call API and the shadow SSI enum | `geom-surfaces/` (→ `geom/`), `geom-brep/src/ssi/system.rs` | **C-c**, #692 | **adversarial** + style |
| **C-h** | **H14** — the census's `bridged` skip | `topo/src/census.rs`, `splitting/rules.rs:268` | #690 | **adversarial** + style — a live soundness hole of S49's exact shape |
| **C-i** | **H15** — #635's unclassified siblings | `mesh/src/planar.rs:63`, `topo/src/validate.rs:426`, `topo/src/splitting/mod.rs:194` | #690 | style |
| **C-j** | **S29** — the sizing vocabulary across five modules | `mesh/src/{nurbs_cert,curved,chords,trimmed,budget}.rs` | #684 | **adversarial** + style on the mechanical half; the policy half is a design PR (**C-R2**) |
| **C-k** | **S28's duplication half** — three tessellation lanes, three pipelines | `mesh/` | #684 | **adversarial** + style |
| **C-l** | **C7 + S33** — the lane-trait collapse, `RingInterval`, the scalar ladders | `geom-core/`, and W2b's 535 refs across 15 files | **#682** | **adversarial** + style; expect to split into 2–3 lanes |
| **C-m** | **S27** — `props/quad.rs`'s four quadrature engines | `geom-brep/src/props/quad.rs` | A2 / S56 / **#649** | **adversarial** + style |
| **C-n** | **H17** — the rustdoc spec-code remainder, ~1115 lines / 130 files | per crate: `topo` 300, `editor-core` 267, `geom-brep` 192, `geom-core` 107, `sweep` 64, rest < 70 | **deliberately last** — it touches 130 files and would conflict with every open lane | style, per crate batch |
| **C-o** | **H16** — the STL header is not caller-settable | `stl/` | none | style; design PR, waits for sign-off (**C-R1**) |

**Not taken by Track C:** C6's rows (blocked on other programmes — OnArc
+ RESPELL-TABLE, the workspace's first proc-macro crate, a persisted
format), and **S26**, which #472 deferred *in writing* as needing its own
proposal with re-measured floors — a proposal, not a patch.

**H17's measurement is not to be re-run.** #639 walked each crate's
`pub mod` tree from `lib.rs` and measured ~1189 against an estimate of
~124; its reviewer independently parsed 1188. Start where the density
is. The pattern must cover **bare** clause letters (`F5`, `G1`, `U7`,
`R3`, `C4`, `S13`) as well as prefixed codes, and must follow
`\`-continued multi-line literals.

---

## Landings

*(none yet)*
