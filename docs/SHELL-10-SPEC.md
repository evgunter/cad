# SHELL-10 — the simultaneous doors walk only their scope

**Status: BINDING at dispatch (SHELL orchestrator, 2026-09-08).** Unit
`SHELL-10`, branch `shell/10-scoped-walks`, block SHELL-B3 slot 1.
Closes `work/shell/shell-doors-still-walk-the-whole-body.md`. Deleted
at merge per `docs/DOC-LEDGER.md`; the item file is the record that
survives. Read `docs/prompts/implementer-discipline.md` in full first.
Survey against main at SHELL-9's merge (2026-09-08); citations by
symbol.

## 0. The semantics, stated once

Since SHELL-8 a simultaneous offset door (`offset_planes_together`,
`offset_charts_together`, `crates/topo/src/offset_together.rs` and
`offset_axial.rs`) solves and writes over a `Scope` — the solids the
move set touches — and its doc says "both doors read and write nothing
outside it". Three reads are still whole-body, disclosed at SHELL-8
and measured at SHELL-9: (1) `Scope::of_solids` walks every shell,
face, loop and half-edge of the body to build the entity → solid
partition and returns `None` (raised `ReplaceFaceError::Corrupt`) on
a malformed entity of ANY solid; (2) each door ends with
`crate::pcurves::mint_pcurves(&mut work, tol)`, which clears every
row of the clone and re-derives them face by face; (3) then
`crate::validate::validate_closed(&work)` over the whole clone. The
first refuses a call about one solid for another solid's corruption;
the second re-derives rows the door did not touch (bit-identical on
every committed fixture — SHELL-8's and SHELL-9's differentials — so
the cost is the only visible effect today, and a face whose chart
cannot mint would refuse a call it has nothing to do with); the
third is a read.

**The decision: each door reads exactly its scope.** The partition
is built from the named solids' shells only (a solid the moves do not
name is not walked — its corruption is not this call's to find), the
pcurve pass clears and re-derives the rows of the scope's faces only,
and the closure check runs over the scope's shells only. A caller
that already holds the partition (the `shell` verb, which builds it
once and re-scopes per solid) keeps sharing it. Rejected, logged
here: leaving (3) whole-body "because it is a read" — the doc's
sentence is either true or it is not, and a read that refuses is a
write to the caller.

## 1. The construction

1. **The scope.** `Scope::of_solids` walks the shells of the named
   solids only (`Body::get_solid(k).shells`), so its three maps hold
   the scope's entities and `solid_of` answers `None` for an
   out-of-scope face — which is what `holds_*` already returns
   `false` for. Every caller that asked the maps about an out-of-scope
   entity must be found and read (the corner walk's `seen`, the seam
   branch, `axial_frame`'s extent, `scope_of_moves`'s coverage check
   that a move set names every face of every solid it touches — that
   check needs the touched solids' faces, which the scope now holds,
   and nothing else). `Scope::whole` stays and is `of_solids` over
   every solid. `re_scope` keeps its contract only if the maps cover
   the new solids — measure: `shell` builds the partition over every
   solid once (`Scope::whole`) and re-scopes down, which stays valid;
   a re-scope UP to a solid the maps do not hold is a programming
   error — refuse it typed or make `re_scope` rebuild, and say which.
2. **The pcurve pass.** An additive `mint_pcurves_of(body, faces,
   tol)` beside `mint_pcurves` in `crates/topo/src/pcurves.rs`
   (TOPO's file — a pure refactor, `mint_pcurves` delegating over
   every face; the orchestrator announces it): clears the rows of the
   named faces' half-edges only and re-mints those faces. The doors
   call it over the scope's faces. The pass's idempotence and
   determinism statements hold per face and are restated for the
   subset.
3. **The closure check.** An additive `validate_closed_of(body,
   shells)` beside `validate_closed` in `crates/topo/src/validate.rs`
   (TOPO's file, same announcement), or — if tier 1's passes cannot
   be restricted to a shell subset without a second implementation —
   the door checks closure of the scope's shells through the existing
   per-shell machinery it can reach and SAYS in the PR which passes it
   runs and which it no longer does. Do not write a second validator.
4. **The producers' contract holds.** Each door is a `Maintains`
   producer on its own clone; the posture table's sentence for the
   doors must stay true for the scope (the rows outside the scope
   were never touched, so they are as fresh as they were — state it).
   `shell`'s closing mint (SHELL-9) still runs whole-body once; it is
   the verb's, not the doors'.

## 2. Acceptance

1. **Byte-identity, twice.** SHELL-9's cache-row instrument
   (`crates/sweep/tests/shell9_rows.rs`, plus the `[rows]` hooks in
   the dump corpora) at the true merge base and the head, in a
   detached worktree on a PRIVATE target with `Compiling topo`
   confirmed both sides: diff EMPTY. And the SHELL-8 body-dump
   instruments (`shell5_r1_dump`, `shell7_dump`, `shell8_dump`): diff
   empty.
2. **An out-of-scope solid's corruption is not this call's.** A
   two-solid body built through crate-internal doors in a `topo` unit
   test, one solid made structurally malformed (a loop whose cycle
   does not close, or a half-edge whose mate is stale — whatever
   `Scope::of_solids` refused on at the base), the other sound: a
   scoped move set naming the sound solid BUILDS at the head and
   refused `Corrupt` at the base (pin both facts: the base's refusal
   as the row's doc, the head's build as its assertion).
3. **The pass is scope-sized.** A row that counts the rows minted (or
   the faces walked — instrument the subset pass by its return value
   or a counter the test can read) on a two-solid body with one solid
   in scope: exactly that solid's rows, none of the other's; and the
   other solid's rows are bit-identical before and after (the
   SHELL-8 deep comparison, reused).
4. **Cost.** `shell_open` on SHELL-8's box-beside-vessel and on the
   hollow-hollow-open body, release, three runs, median, base and
   head; and the direct door on a two-solid body naming one solid.
   State the numbers; the STOP is a cost that rises.
5. **Every committed row green**, both eps lanes, interval included
   where the suites use it.
6. **The mutant.** Restore any one of the three walks to whole-body:
   row 2 (for the scope) or row 3 (for the pass and the check) goes
   red — name which row catches which walk.

## 3. Stops

STOP and report if: a row moves on any fixture in either differential;
the corner walk, the seam branch or `axial_frame` reads an
out-of-scope entity through the maps (then the scope was load-bearing
as a whole-body read and the unit's premise is wrong — say where);
tier 1 cannot be restricted without a second implementation and the
per-shell machinery does not exist (then §1.3 stays whole-body,
disclosed, and the doc sentence is corrected instead); or the cost
rises on any §2.4 fixture.

## 4. Docs and owed

`Scope`'s doc block (`offset_together.rs`): the "still a whole-body
structural walk" paragraph replaced by the truth. Both doors' fn-docs:
the closing passes described as scope-sized. `shell.rs`'s doc: the
closing mint remains the verb's whole-body pass (one sentence).
`pcurves.rs` and `validate.rs`: the additive entries documented; the
posture table if it names the doors' pass (a doc line — TOPO seam,
announced by the orchestrator). Lane rules as every SHELL brief: own
worktree, own `CARGO_TARGET_DIR`, narrow builds (`-p topo -p sweep`),
one heavy cargo job, foreground runs, no `Co-Authored-By` trailer in
lane commits, push after every coherent step, hosted CI is the gate
(nothing narrowed), report ≤ 150 lines.
