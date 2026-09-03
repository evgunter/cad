---
id: smell-scan-2026-08-findings-register
kind: issue
title: SMELL-SCAN 2026-08 — findings register and fix schedule (tracking)
status: closed
opened: 2026-08-18
closed: 2026-09-03
github: 614
refs: [613, 646, 651, 663, 667]
---

## From GitHub issue 614

Opened 2026-08-18; 1 comment.

Tracking issue for the structural audit in **#613** — `docs/SMELL-SCAN-2026-08.md`.

This is the register the scan's own §C3 says it needs: *"a deferral recorded only in prose is not deferred, it is forgotten on a schedule."* The report has 46 findings with stable IDs; this issue tracks the ones that need an owner.

**Read #613's PR body first** for the method and, importantly, for the list of findings the steelman passes **retracted or narrowed**. Several shrank materially under scrutiny.

---

## Wave 0 — decisions (Ev). Most other work is gated on these.

Cheap in wall-clock, large in what they unblock. No agent should make these calls.

- [ ] **D1 — S44: what is `Bounds`?** "Carries a bracket" (a semantic property, definable for `Dual` as lo=hi=value) or "may enter certified code" (an access-control marker)? *Gates S3 entirely; colours S1 and S2.* The founding ruling for the whole lane-trait pattern exists only as an agent's one-line paraphrase of an in-session chat (`M5-LOG.md:3451`), in a log whose same page records that channel as unreliable — and Ev does not recognise the conclusion drawn from it.
- [x] **DECIDED — RATIFIED 2026-08-19 (#628, `DESIGN.md:1118`); the conversion it licenses is NOT done, so W2c is unblocked and unstarted.** **D2 — S43: the bug-vs-invalid-state taxonomy.** The kernel uses five idioms for "this state can only be a bug", two of them mutual negations; D9 sanctions three of the five. *Gates S19, which it generates (~239 of ~260 sites); resolves S12/S14's residue at the same stroke.*
- [x] **DECIDED 2026-08-19 — RETIRE.** Experiment run in #624: the surgery succeeds on the whole-body input and builds the same solid. Sequencing: #640 is editing `build.rs:692` *inside* the retiring door, and W2d follows the retirement rather than accompanying it. **D3 — S7: run the one-line experiment.** Swap the arms at `fillet/build.rs:205`, `cargo test -p sweep --test all`. *Gates S6 and all fillet work in S36/S38.* The surgery door has never once been run on a whole-body input; if it fails on a cube, S7 collapses.
- [x] **DECIDED 2026-08-19 — DELETE, execution deferred to last priority** (but ahead of W3b, since trimming comments on condemned code is the waste ordering rule 1 prevents). Scope is three rows: `Mat2`/`Affine2`, `PairSolve`, the two inlined fillet helpers. #622 already took `ProfileError`'s variants; `hull.rs` is struck — the real question there is retiring the `sup_norm_bound*` API, which sits on the banked #390/#453 lane. Each deletion owes a provenance note beside its originating thread, citing a recoverable SHA: **#611** for `PairSolve`, **#319/#554** for the fillet helpers, the deleting PR body plus this issue for `Mat2`/`Affine2` (whose thread is closed — itself the evidence for its sort). **D4 — S11's four undecided rows**: `Mat2`/`Affine2`, `PairSolve`, `hull.rs`'s non-rational unused half, the two inlined fillet helpers. Delete-or-keep each.

## Wave 1 — correctness. Five disjoint lanes; can start now, in parallel.

- [x] ✅ **#620** — **W1a — S16**: `boolean/boxes.rs`'s planar arm uses a bare vertex hull, but a cylinder's planar cap has a circular rim that bulges past its endpoints, so the box is not a superset and the BVH can prune a pair silently. **Highest single-item value in the report.** Fix already named in `PERF-SCAN-2026-08.md` Tier A finding 1; `separation.rs` already holds the corrected planar rule.
- [x] ✅ **#617** — **W1b — S23**: the SSI exhaustiveness sweep switches duty on `tubes.is_empty()`, so an all-seeds-fail run returns `Ok` *plus an exhaustiveness receipt*. The acceptance row also needs replacing — its premise excludes the failing mode.
- [ ] **W1c — S41**: `Bounds for Interval` forwards `lo()`/`hi()` without consulting the decoration; a `Trv`-but-nonempty enclosure may be dropping a domain violation **today**. Also the gating question for S1.
- [x] ✅ **#618** — **W1d — S4 drift (a)**: `Rebind` never reaches `Node::Mate`'s two `StableName`s, so a mate head is either falsely refused as `RebindNoReferences` or silently left dangling. Contradicts `ASSEMBLY-DESIGN.md:566`.
- [x] ✅ **#619** — **W1e — S42**: loft's `sense = true` is pinned only on a prism — no concave arcs, no holes, i.e. the shape that did not break extrude either.

## Wave 1b — hygiene. Parallel with Wave 1; nothing blocks.

- [x] ✅ **#626** — **H1** — ci-local mirror: no `EvalScalar` step, no interval-square `powi(2)` step; hosted has both. (The two allowlist drifts are fixed in #613.)
- [x] ✅ **#635** — **H2 — S39**: nine stale claims, each classified **benign rot** vs **lost invariant** *before* its sentence is touched. `enters.rs:14` is the candidate for the second reading.
- [x] ✅ **#627** — **H3 — S40**: residue; start with the two that are not cosmetic (`emit_topo.rs:1266`'s wrong-name fallback, `seqgen.rs:853`'s discarded counter).
- [ ] **H4 — S37**: shipped-artifact naming — STL header, the runtime-visible PR number, ~124 spec codes in public rustdoc and the Python stub.
- [x] ✅ **#632** — **H5 — S4 drift (b)**: `names/select.rs:319`'s fail-quiet wildcard. One function.
- [x] ✅ **#625** — **H6** — Euler postcondition 7-tuple → named struct. Mechanical, debug-only.

## Wave 2 — structural. Sequenced.

- [ ] **W2a — S3** lane traits *(needs D1)*. A working `geom-core` collapse is already compiled: 16 impls → 2.
- [ ] **W2b — S1/S2** `RingInterval`, and whether `Interval` goes always-on *(needs D1, W1c)*.
- [ ] **W2c — S19** the three big error catch-alls *(needs D2)*.
- [ ] **W2d — S6** sweep helper unification, ~230 token-identical lines *(needs D3 — same crate, will collide)*.
- [ ] **W2e — S5** `splitting/` vs `boolean/`. Start with the forked sector predicates: dimensionally identical line-for-line, splitting one K population 29:1.
- [ ] **W2f — S4** vocabulary mirrors, cheapest first (`BooleanOp` → import + `serde(with)`).

## Added since this issue was written

- [x] ✅ **#633** — **H7**: the chart lane's empty-tube acceptance row. #617 fixed both SSI lanes but its red row covered ℝ³ only. Needed a wall whose true surface misses the cutting plane inside its own control-net hull slack (`hull_slack_wall`, a 25:1 hull-vs-truth gap).
- [x] ✅ **#636** — **H10 / S51**: loft's `v` direction was never varied — every S42 row lofted two sections at `v_degree = 1`, so no chart could twist. Verified, no defect, on a convexity-flipping pair, a three-section curved-`v` stack, and `sweep_body`'s elbow.
- [ ] **H8** — positional-census residue in `topo` (the class #625 fixed, still live at three sites). *Claimed.*
- [ ] **H9 — S50** — derive the fillet corner patch's `sense` at the mint site. *Claimed (#640). Scope to `surgery.rs` or wait: `build.rs:692` is inside the door D3 retires.*
- [ ] **W2g — S49** — the census's planar × planar skip is justified by a claim about solids. *Claimed (#637).*
- [ ] **W1c — S41** — the `Enclosure` seam may be laundering `Trv` decorations today. *Claimed. Still the only blank verdict in the report, and it gates W2b jointly with D1.*
- [x] ✅ **#646** — **W2f, units row only** — the display-unit code is now a POSITION in `quantity::UNITS`; both hand-written tables in `expr.rs` are gone and #291's size measurement became a compile-time assertion. The rest of W2f (`BooleanOp`, `ProgramStep`/`WireStep`, `SegTag`, "no usable value") stays open.
- [ ] **#650** — **S4 residue, found beside the units row, filed not fixed.** `Expr::literal_with_unit` checks the CALLER's `UnitDef.quantity` and then stores the table row found by symbol, never re-checking; a publicly constructible mismatched pair builds an `Expr` that serializes into a document its own load door refuses. D2 addendum **row 1** (reachable by input, invalid ⇒ typed error). Two lines either way, but it is a behaviour change and it carries a design question: should `UnitDef`'s fields be `pub` at all?
- [ ] **H11 — #651 (rule + one guard landed, #663), continuation #667** — **measurements have no mechanical guard.** #663 landed the rule (`REVIEW-STYLE-BRIEF.md` §Q6: a mechanical guard, a scheduled re-measuring register, or a written reason **at the claim site** that it can have neither) and one guard (`mesh/tests/profile_overrides.rs`, the `opt-level = 2` block), over 14 sites plus one six-document class — **roughly a tenth**. The sweep and its residue table are a [comment on #651](https://github.com/evgunter/cad/issues/651#issuecomment-5344413746); the remaining ~90% is scheduled as **#667**, whose first unit is fixing the search pattern per §C15. Original framing: `size_of` appears nowhere in `crates/*/src` except #646's new pin; the only other two `const _: () = assert!` pin feature-flag state, not measurements; 136 lines in `src` say "measured" and approximately none is guarded. Raised as a CLASS by #646's style review and discharged to this register rather than absorbed into that PR. The question to ask each row is *"what goes red if this stops being true?"*, and "unguardable, and here is why" is a complete answer. Start at `Cargo.toml:165-175` (the `spade`/`mesh` `opt-level = 2` block, "measured 2026-07-21", whose removal is undetectable and whose symptom is an unattributed slow suite).

## Reopened after their unit closed

- [ ] **S39 / H2 residue — two rows added by #647's style review, after #635 closed H2.** Neither is fixed or owned. (a) **`DESIGN.md:1362`'s false `topo::boolean` module path** — `splitting`, `census` and now `sector_shape` are top-level siblings. Recorded, deliberately **not edited**: `DESIGN.md` is the ratified contract, so it is Ev's call. (b) **`docs/predicate-dimension-audit.md`'s stale line anchors** — five single-line anchors off by >60 lines, two verified rot (`validate.rs:1795`→`:2005`, `pcurve_cache.rs:1664`→`:3219`), in a document whose header says a row and its disposition must never disagree. Sweep declined: each needs a per-row read of intent.
- [ ] **S39 / `enters.rs` typing fork** — #635 made the prose honest (the sense correction is the caller's; no type enforces it) and deliberately left the typing question: a `geom-brep`-side `OutwardNormal<T>` newtype, versus taking `(&Body, FaceKey)` and inverting the `geom-brep`/`topo` layering. Real exposure is **three** call sites, not the 36 first reported.

## Not covered by §D at all

**S20–S22 and S24–S34 have no row in the schedule.** They are accepted, several are argued at length, and none has a lane, an owner, or a wave; only S35's roll-up gets a Wave-3 row. Two carry questions that are Ev's rather than an agent's (S22's ε ambience, and the mesh ε-vs-δ-vs-neither snap bar), and S28's `curved.rs` grid-after-constraints ordering is arguably a correctness item that never got a Wave-1 row. Scheduling these is outstanding.

## Wave 3 — last, deliberately.

- [ ] **W3a — S36**: comb-and-rename **per suite**, never a rename pass. A PR-numbered name currently carries signal. Note the 2026-08-13 retirement licence has produced zero deletions in five days against ~10 new review-named suites — it needs an owner and a slot, not just permission.
- [ ] **W3b — S38**: comment trimming. Must follow every deletion above.
- [ ] **W3c** — remaining S35 roll-up rows.

---

## Dependency edges

```
D1 (Bounds?) ────────────► W2a (S3 lane traits)
             └───────────► W2b (S1/S2 scalars) ◄─── W1c (S41 decoration seam)
D2 (bug-vs-invalid) ─────► W2c (S19 error catch-alls)
D3 (fillet experiment) ──► W2d (S6 sweep) ──┐
D4 (dead rows) ─────────────────────────────┼──► W3b (S38 comments)
all deletions ──────────────────────────────┘
                                            └────► W3a (S36 suites, per suite)
```

## Process changes already landed in #613

Not tracked here — they are done: the hosted-CI guard, the output-stability memory, the review-protocol amendments and the style-lane brief (Protocol v5).

**Closing condition**: every unchecked box above is either closed or has moved into a milestone plan.

## Comments

**2026-08-20** — comment:

## D4 row 1 of 3 executed — provenance note for `Mat2` / `Affine2`

This discharges the "**plus this issue**" half of D4's provenance obligation. The deleting PR is **#721**.

### The note

`Mat2` and `Affine2` were written by **M0's linalg thread**, the same PRs that built `Vec2`/`Vec3`/`Point2`/`Point3`/`Mat3`/`Affine3` as one complete vocabulary (`docs/archive/M0-LOG.md:120-121`). **That thread is closed and archived.** Every other row in S11's `GENUINELY DEAD?` sort could be annotated on a live issue; this one had no live thread to annotate, and that absence *is* the argument for its sort — it was the only row with no source at all for a future consumer, against M0's own norm *"add only on consumer demand"*.

What is dead is specifically the **2-D linear-map half**. `Vec2`/`Point2` are heavily used and stay; the tangent-space *maps* over them had no producer.

**The code is recoverable from `9f559f6a4179a77a87d569bc0b8f363fa1cf2c1a`** — `main` at #721's base, whose tree still contains both definitions in full with their docs and proptests:

    git show 9f559f6a:crates/geom-core/src/linalg/mat.rs
    git show 9f559f6a:crates/geom-core/src/linalg/affine.rs

### Census, re-verified before deleting

Re-run on the tree of 2026-08-20 (`rg -i 'mat2|affine2'`, whole repo, all file types). Unchanged from the 2026-08-18 finding: outside `linalg/` the only mentions were `lib.rs:38`'s re-export and `geom-core/tests/review_m0_pr5.rs`. No new consumer had appeared, and no crate above `geom-core` in the DAG mentioned either type in `src` or `tests` — confirmed by a clean `cargo check --workspace --all-targets` after the deletion.

### One thing to know for whoever reads this next

The deletion **minted a fresh instance of the class it closed**: `Vec2::unit_x` and `Vec2::unit_y` (`geom-core/src/linalg/vec.rs:44`, `:49`) had exactly one `src` consumer in the workspace — `Mat2::identity` — so their only remaining caller anywhere is one line of their own module test. They are tabled at S11 **without a verdict**; delete-or-keep has not been asked, and #721 did not delete them.

The class to watch is anything in `Vec2`/`Point2`'s surface whose last live caller was 2-D-map code. The general lesson: a deletion's census pattern is scoped to the deleted names, so it structurally cannot see what the deletion orphans.

### Status of the other two rows

Unchanged by #721, and their notes are **not** written here — they belong to the units that delete them:

- **`PairSolve`** — note goes to **#611**. Now unblocked: #702 merged 2026-08-20 as `f382c4aa`, which was the only gate.
- **The two fillet helpers** — note goes to **#319**/**#554**. Still behind **D2** and **#705**.

D4's checkbox above still reads "execution deferred to last priority"; that remains accurate for those two.


---
_Generated by [Claude Code](https://claude.ai/code)_

## Home

Code quality: this is the SMELL-SCAN register the program was built around, and its rows are now the tracked items in `work/code-quality/` (`plan.md`'s numbering, the Track K–X row files, `process-observations.md`, and `logs/migration-census-2026-09-03.md`, which maps every `## S<n>` heading to the file that carries it). Closed on migration — the tracker is the register now.
