# SMELL-UV — execution log for Tracks U and V

**Constituted 2026-09-01, by this session's orchestrator, per Evan's
in-chat instruction to take the unclaimed tracks.** Track U
(`crates/step-import/`, `crates/step-export/`, `crates/stl/`,
`crates/pncad-py/`, `crates/pncad/`) and Track V (`crates/editor-core/`,
`crates/profile/`) of the `docs/SMELL-SCAN-2026-08.md` §D schedule,
claimed whole. This file is the execution record — rulings, lane state,
review outcomes, incidents. **The schedule is §D and stays so**: a row's
live status is that file's table, and a landed row leaves it.

**Branch prefix:** `smelluv/` for units; the orchestrator rides this
session's own branch. **Outside the model A/B experiment**, following
the F/G/I/T/KPW precedent for smell tracks: no pairing, no ordinal, no
row in `docs/MODEL-AB-LOG.md`. As the T-log says: precedent-following,
not forced; Evan can reverse it for later lanes.

## The ground, and how it was checked

Taken 2026-09-01 against `origin/main` at `5f82295`. Verified rather
than assumed:

- **No claimant exists.** No `SMELL-U`/`SMELL-V` log; the claim
  registers (`docs/WORK-STREAMS-2026-08.md`, the T and KPW logs, the
  S-* plans) name K, P, W, T, J, M, N, Q and R only.
- **Every open PR's branch was diffed against current main** and
  filtered for the two fences. Exactly one contests: **#1423 (MATE-3)**
  edits `editor-core/{assembly.rs, eval/mod.rs, persist/wire.rs,
  program.rs}` and `profile/{path.rs, path/program.rs}` plus tests.
  Keep-out ruled at UV-R4. (An earlier read against a stale main showed
  every mate/verbs branch inside the fence; re-fetched, that overlap
  evaporated — diff against main as of *now*, never as of clone time.)

**Adjacent live programs, and the seams stated once:**

- **LIB** (reactivated 2026-08-29; active in `pncad`/`pncad-py`
  bindings and refusal-display curation). Seams: LIB drafts the
  #741/#742 plans (its log, 08-29: they "wait on LIB drafting plans,
  not on Evan"); U keeps rows C13/C14 and does not draft. Any UV lane
  in `pncad-py`/`pncad` merges main before opening and expects
  census/pyi/audit contention.
- **M10** — `editor-core` parameters/analysis/eval, `product.rs` Dual
  arms, schema versions: its ratified slate, not takeable here.
- **S-MATE** — the `editor-core` mate/assembly surface; #1423 above.
- **S-BOOL** — owns `topo/census.rs` (Track Q's fence): S190's kernel
  half is theirs (UV-R6).
- **KPW session (Track W)** — `D380`'s closure reaches
  `profile/src/sugar.rs`; per §D that reaching edit is V's row to
  file when W takes the row. Nothing to do until then.

**Environment (remote container, the LIB 08-29 adaptations):** hosted
CI is the verification of record; no monitor scripts or away-channel
(the tracker is read at check-ins; Evan reads this log and the session);
GitHub via MCP tools; lanes are worktrees with lane-private
`CARGO_TARGET_DIR`, heavy cargo behind `local-scripts/with-build-slot.sh`.
Lanes do not edit `docs/SMELL-SCAN-2026-08.md` — that file conflicts by
construction; its bookkeeping is the orchestrator's.

## Review policy (the F/G/I/KPW shape)

Style review on every unit — `docs/prompts/reviewer-style-lane.md`
dispatched by path, with the dispatcher notes
(`docs/REVIEW-STYLE-DISPATCH.md`) and the two standing track questions:
(1) is the row's original problem COMPLETELY gone — not narrowed, not
relocated; (2) was it closed the best available way. Plus the KPW
emphasis: does the defect the unit closed reappear in a slightly
different form (§D rule 5). Adversarial correctness review only where a
wrong answer is reachable (C-R12 criterion).

## Rulings

| # | Question | Ruling | Who, when |
|---|---|---|---|
| **UV-R1** | Claim | **Tracks U and V claimed whole by this session**, per the partition's own rule (a track can be claimed the day it is read) and Evan's instruction. Recorded in §D's two track headers in the same change as this log | orchestrator, 2026-09-01 |
| **UV-R2** | U's inherited `E-m / #711` row — its premise is *"PR #784 is open and red"* | **SPENT.** #784 merged 2026-08-27 (#711 closed with it). Its red was `D86`'s false positive (`interval-only-selection.py` reading a doc comment as a cfg gate), and D86 is fixed — the script now matches the cfg attribute and its header records the old bug. Nothing left for a lane; row deleted from §D with the citation sweep (E-m/#711/#784 appear nowhere else in the ledger) | orchestrator, 2026-09-01 |
| **UV-R3** | `D362` re-derived against the tree | **Half closed by LIB's curation lanes**: `HitTestError` has `Display` (`resolve/hit.rs:61`), so does `InterrogateError` (`names/interrogate.rs:129`); the `#1103` `ParseError` member closed per LIB's log. **The open remainder is `NodePickError` (`resolve/pick.rs`) and `ResolveIndeterminate` (`resolve/mod.rs`)** — the egui-label member. §D row updated per the partly-closed rule (closed members deleted). The viewer-side consumption stays #1111's/viewer's, out of fence | orchestrator, 2026-09-01 |
| **UV-R4** | #1423 (MATE-3) is open and edits inside both fences | **KEEP-OUT while #1423 is open**: `editor-core/{assembly.rs, program.rs, persist/wire.rs, eval/}`, `profile/{path.rs, path/program.rs}`. Rows HELD by it: **D121** (`res_spec` is `program.rs`'s; the vocabulary reaches `profile/path`), **D39** (+ tracker sibling #1282 — `PathError` is `path.rs:522`), **S190's editor-core half** (`attribute` is `assembly.rs`'s). A keep-out is lifted by observing the merge, not by citing a brief (T-R7's lesson) | orchestrator, 2026-09-01 |
| **UV-R5** | C13 (#741) / C14 (#742) — *"plan signed off before implementation"* | **Not takeable now and not waiting on Evan today.** LIB holds the plan drafting (its 08-29 register correction says so explicitly); the issues' own faces say the plans then go to Evan. U keeps the rows; when the plans exist and are signed off, implementation lands by fence (step crates = U's) | orchestrator, 2026-09-01 |
| **UV-R6** | S190 / #855 — the fix is a `topo` signature change (`ValidationError::CensusUnsupported` carrying the pair), and `census.rs` is Track Q's fence, claimed by S-BOOL | **The kernel half is S-BOOL's; filing is the handoff** (comment posted on #855 naming this). V's residue is consumption-side only: when the pair-carrying variant exists, `attribute` answers by `by_pair` and the width-1 caveats retire. HELD on that change AND on #1423 (same file, `assembly.rs`) | orchestrator, 2026-09-01 |
| **UV-R7** | Wave 1 composition | **uv-a** (V): D362 remainder + `StableName` `Display` + D81's 23 sites — one lane, one class (typed payloads reaching user prose through `Debug`); D81's own row says re-derive the count first. **uv-b** (V): G4 + D361 — both profile trait-surface rows, G4 ruled mechanical by Evan with both gates fallen (#791, #801). **uv-c** (U): D94, `step-import/src` only. Wave 2 queued: D47+D37 as one `pncad-py` lane once uv-a lands (the §D note says one lane is cheaper than either alone); C16; the UV-R4 holds when #1423 merges; D75 with LIB coordination | orchestrator, 2026-09-01 |

**Noted for later, not rowed here:** (a) the **unscanned-crates
commission** — §D's closing note says U and V now own commissioning a
scan of `step-import/`, `step-export/`, `stl/`, `pncad-py/` and
`profile/`; scheduled after wave 1 lands. (b) Tracker items on this
fence filed since the scan and unclaimed: **#1240** (pncad-py wheel
latently red at TIER=all — coordinate with LIB before adopting),
**#1282**/**#1280** (profile; #1282 rides D39's hold). (c) `D68`
(Track K's) is the visibility half of G4 and is not discharged by
uv-b — stated so nobody reads uv-b's merge as closing it.

## Lane state

| lane | rows | state |
|---|---|---|
| **uv-a** | D362 (remainder), D81, + `StableName` Display | dispatched 2026-09-01 |
| **uv-b** | G4, D361 | dispatched 2026-09-01 |
| **uv-c** | D94 | dispatched 2026-09-01 |
| (held, UV-R4) | D121, D39 (+#1282), S190's editor-core half | until #1423 merges (S190 also on its kernel half, UV-R6) |
| (held, UV-R5) | C13, C14 | until LIB's plans exist and Evan signs off |
| (queued) | D47+D37 (one lane), C16, D75 | wave 2 |
| (blocked, kept visible) | C6 | each member on something real (OnArc + RESPELL-TABLE, a first proc-macro crate, a persisted format) — unchanged |
| (not work) | D360 | a sweep rule, binding on any lane that sweeps `topo` refusal enums in this fence |

## Lane records

(appended as reviews return)
