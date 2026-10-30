---
id: intent
kind: program
title: INTENT — build DESIGN.md D10, one way to say dependency, placement and intent
status: ready
opened: 2026-10-03
group: api/eval
prefix: intent/
tag: (INTENT orchestrator)
ab_band: 10800-10899
paths: [crates/editor-core/src/expr.rs, crates/editor-core/src/doc.rs, crates/editor-core/src/parse.rs, crates/editor-core/src/eval/slots.rs, crates/editor-core/src/param_source.rs, crates/editor-core/src/persist/*, crates/pncad/src/document.rs, crates/viewer/src/props.rs, crates/viewer/src/drafts.rs]
keep_out: [D10 is ratified (Ev, PR 3990); this program builds it and does not re-open it — a question D10 does not settle goes to designers and, if it is a fork Ev cares about, to an [ev] PR, never decided by a lane. The ground is shared with DOCTAIL (doc.rs, edit.rs, node.rs, persist; RECIPE's until it closed on 2026-10-11), MSOLVE (mate.rs, mate/*), WIRE (eval/mod.rs, eval/wire.rs), CONTACT, SECT and FLUSH (topo's boolean declarations and census), PATHS (profile tangent joints) and the viewer programs; each crossing is announced on that program's log. The refactor hold every active program carries waits on d10-one-way-to-say-intent-is-unbuilt, which this program owns and closes last.]
budget: 45
---

The program that builds D10 (`docs/DESIGN.md`), ratified by Ev on PR
3990 from the ruling `work/intent/one-way-to-say-dependency-and-intent`
(closed; Ev's words verbatim at the commits it cites). Every row is P0
(Ev: the problem worsens as more is built haphazardly, and the work
blocks nearly everything else).

`budget: 45` because the first stage is one indivisible representation
change (a variable table, the literal arm, persistence, the façade and
the GUI move together or the tree does not build); later stages split
into their own programs as `plan.md` says.

## Orchestrator handoff (2026-10-19)

The INTENT orchestrator stopped here when the account's weekly usage ran low. A fresh orchestrator starts from this section plus each lane's item, not from any session's chat. Lanes report on their items' `## Handoff` sections.

**Rulings landed in the last two days** (each in `docs/DESIGN.md` and the fork log, with Ev's words there):
- `[placement]` restated from Ev's diagram (PR 5312, row 162):
  - only a placement relates two spaces;
  - every freedom is written, and the identity placement is six zeros;
  - downstream reads a space's bodies, never a placement.
- A space names its bodies by picks (PR 5315, row 165):
  - a pick's `of` is a path to the first variable;
  - a duplicate pick is two variables.
- The at-rest census is per space, between arrivals (PR 4666, row 164):
  - inside an arrival it reads nothing; the transitional check is parked in `the-in-arrival-census-check-retires`;
  - spaces outside the world are checked;
  - a placement carries its source's census decisions.
- The loft station (PR 5295, row 161).

**Stage 3: `transform-retires-into-a-placement` (D), in flight, unmerged.**
- D lands as built. Its `stand` is the interim, and it is a legal #5312 bundle.
- Its state is in its item's `## Handoff` section.
- It needs the Tier-H concurrent pair and the row last.
- D's m10 Interval blocker is fixed in the solve, with the f64 bits unchanged.
- The C-5 Sym-snap change is orchestrator-owned and accepted.

**After D merges, dispatch in parallel:**
- **D′** `a-placement-pins-by-written-values-without-a-mate` (P0 H, filed on D's branch). It carries the unmated spin chart, optional `Value.mate`, F counting spins by axis, the door refusal for an under-pinned bundle, the PR 5315 picks (retire `RoleSeg::Placed`/`InPart`/`CopyKey`/`copy_of`), and spec §1, §4 and §5. Its item lists `a-lone-slide-refuses-the-chart-it-never-reads` and two of D's width items to absorb.
- **E** `an-operation-computes-in-a-frame-of-its-reads`.
- **G** `patterns-are-index-variables`.
- The census units from PR 4666: `each-space-runs-its-census-between-arrivals` (P1 H), then `a-placement-carries-its-sources-at-rest-result` (P1 M). These are independent of D′, but D′'s carry shape must not preclude them.

**Stage 4 J (contacts at rest):** #5325 (a part's census findings cross the seam) is finishing. Its A5 sentence describes PR 4666.

**INLANE (C6):**
- S1–S5c are merged or merging (#5309, #5322, #5332).
- Next: S5d…k (readers by crate), S5z (the door lifts), S6, then the instrument unit.
- Rulings in force: the typed-refusal end state, with a ratchet on unwrap/expect of the typed accessors to 0 by S5z; and no door check for an unbounded pole (an entire pole is legal, and its decoration carries the hazard).
- State is in the C6 slices item's `## Handoff`.

**Process lessons** (orchestrator-side; not yet in `docs/prompts/`, which needs Ev):
- Merge main with `git merge --no-commit`, grep for conflict markers, and commit only after a clean grep. Never `commit -a` during a merge: bf2fa7c4ef pushed markers that way.
- Fork-log rows collide across concurrent PRs. Take the next free number at merge time and re-check before pushing.
- Reviewer pairs need separate `CARGO_TARGET_DIR`s (`work/issues/a-review-lane-runs-another-lanes-binary.md`).

**Addendum at stop (2026-10-19 10:30Z).**
- **Merged:** J #5325 (a part's census findings cross the seam), INLANE S5c #5332 and its handoff #5340, and #5335 (this handoff).
- **Stage 3 D parked, unmerged.** Its branch is `intent/s3-d-transform-retires-into-a-placement` at 54306ddbe4. Its item's `## Handoff` lists the remaining reds with next steps. The PR is not opened and the Tier-H pair has not run. Before the PR, revert the uncommitted probe `[[test]]` entries in `crates/editor-core/Cargo.toml`.
- **Ruling for the next D lane** (orchestrator-owned C-5, code-level since 99774f9395): the rest snap applies only to a bundle that reads no variable. A placement whose bundle reads a toleranced or free variable always runs its map. Without this the tour's 1-link chain drive certifies 0 leaves: f64 snaps at the nominal, the other leaves do not, and the decision lists differ.
- **Nightly run 65 (main 07a57312bf) is red in 7 jobs:** the full suite, viewer app, rustdoc, three k-lint rows and the gui montage. Triage was in flight at the stop. Its findings follow on this PR, and the next orchestrator owns any red that is INTENT's or INLANE's.
