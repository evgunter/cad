# SHELL-8 — shell is per solid, on every solid

**Status: BINDING at dispatch (SHELL orchestrator, 2026-09-08).** Unit
`SHELL-8`, branch `shell/8-multi-solid`, block SHELL-B2 slot 2. Closes
`work/shell/shell-open-on-a-multi-solid-body.md`. Deleted at merge per
`docs/DOC-LEDGER.md`; the item file is the record that survives. Read
`docs/prompts/implementer-discipline.md` in full first. Survey against
main at SHELL-7's merge (2026-09-08); citations by symbol.

## 0. The semantics, stated once

`shell` / `shell_open` refuse a body with more than one solid
(`ShellError::NotOneSolid`). Shelling is a per-solid verb — `S − offset_inward(S, t)`
is defined solid by solid — so on a body of `N` solids it applies to
EVERY solid: each solid `Sᵢ` becomes its own thin solids exactly as
SHELL-5 built them for one (one per boundary shell of `Sᵢ`), and the
result body holds all of them. Nothing crosses between solids: a
solid's cavity is inside its own material, its clearance is its own,
its door choice is its own. A designation names faces on any solid,
and opens the thin solid that face's wall became. `NotOneSolid` is
retired; a body with NO solid is what still refuses (say which
variant, and why it is not `NotOneSolid` renamed).

Rejected, logged here: a designation of WHICH solid to shell. The verb
has no vocabulary for naming a solid, a user who wants one solid
shelled has a body with one solid, and "every boundary thickens" (Ev,
issue record 1056) extends to every solid without a new decision.

## 1. The construction, per solid

Today's `shell_open` is one solid's construction with three
whole-body reads in it; the unit makes each read per solid and keeps
the rest.

1. **Gates.** `Thickness` once. `ChartSenseMixed` per chart (a chart
   belongs to one solid; unchanged). **`wall_clearance` per solid**: a
   planar pair of DIFFERENT solids never gates (two parts facing each
   other across space are not a wall) — the inventory carries the
   face's solid and the pair loop skips cross-solid pairs; pin it with
   a row where two boxes sit `< 2t` apart and both shell. The roles
   read (`classify_shells`) per solid, on hollow solids only, as
   SHELL-5 does for one. `check_designation` per shell (unchanged).
2. **The door, per solid.** `offset_door` reads the whole body
   (`all_planar` over every face, `is_axial` over every face). A body
   with a box beside a vessel is neither all-planar nor axial and
   would take the per-chart door for both — refusing the vessel's
   corners it shells alone. Decide the door PER SOLID over that
   solid's faces (`is_axial` gains a face-set form, or a per-solid
   wrapper; the three-way ladder is unchanged) and record it; the lift
   reads the designated face's solid's door.
3. **The moves, per solid.** The simultaneous doors (`offset_planes_together`,
   `offset_charts_together`) require every face of the BODY in
   `moves`. The reason is corner coherence — a corner's every chart
   must move in one solve — and a corner belongs to one solid, so the
   coherent unit is the solid: relax both preconditions to "every face
   of every SOLID the moves touch appears exactly once, and no solid is
   touched partially" (a move set that names some faces of a solid and
   not others refuses as today — `TogetherFaceMissing` or the variant
   that exists; measure). Both doors are SHELL's files. The per-chart
   door is per chart already. So the moved clone is ONE clone of the
   whole body with each solid's charts moved through that solid's
   door in turn — the doors touch only the named solid's corners and
   edges (verify: `is_axial`'s frame and the corner walk must read the
   named solid, not the body — measure what each door iterates and
   restrict it; a door that still walks every vertex of the body after
   the relaxation is the STOP in §3).
4. **The evidence and the insertion, per solid.** Every clone shell is
   strictly inside ITS solid's material, `Carried { Positive }` per
   clone shell as today. The void door inserts into ONE destination
   solid; the clone now has `N` solids, each of which must land in its
   own operand solid. `insert_void` is S-BOOL's file
   (`crates/topo/src/boolean/voids.rs`), and its graft already has the
   N-ary form (`combine::graft_solids_with`: one destination per
   source solid, positionally in the source's solid order). Add an
   ADDITIVE sibling `insert_voids(dst, dst_solids: &[SolidKey],
   cavity, evidence, tol)` with the same evidence discipline, which
   `insert_void` becomes the `N = 1` case of (one implementation, two
   doors), and use it with the operand's solids in the clone's solid
   order (the clone is a clone, so the orders agree — assert it, do
   not assume it). This is an announced seam to S-BOOL: minimal,
   additive, contract unchanged for the existing door; name it in the
   PR body and the orchestrator announces it on S-BOOL's board.
5. **The partition, per solid.** SHELL-5's step: for each operand
   solid, each of its voids and that void's twin move to a new solid
   (`Body::move_shells_to_new_solid`). `thickened` rows for every
   operand shell of every solid, shell-arena order.
6. **The rim surgery.** Unchanged in mechanism: a designated face's
   counterpart is the graft map's; both live in one solid after step
   5; `RimShell` by the sealed arm's roles. The lift's door is the
   designated face's solid's (step 2).
7. **One validation.**

## 2. Acceptance — closed forms, existing fixtures

1. **Hollow, hollow, open** — the row this item was filed from
   (`shell5_r1_probes::r1_e2e_hollow_twice_then_open_the_inner_wall`,
   step 3a asserts `NotOneSolid { 2 }` today): `shell_open(shell(shell(boxy(2,3,4),
   0.25), 0.05), 0.01, [the inner wall's ceiling])` builds: the
   twice-hollowed body's two thin solids each shell again (four thin
   solids: `[V(2,3,4)−V(1.98,2.98,3.98)] + [V(1.9,2.9,3.9)−V(1.88,…)]`
   … write every term out) with the designated ceiling's thin solid
   opened, volume to `1e-12`; per-solid roles; `thickened` rows.
2. **Two disjoint boxes in one body** (build through a boolean union
   of disjoint operands, or two `insert`-free bodies grafted — measure
   which door builds a two-solid body; `subtract`'s disjoint-union
   fallback exists): `shell` at `t` gives 2 thin solids, 4 shells,
   volume the sum of two walls. At a separation `< 2t` between the
   boxes it STILL builds (row 1.1's cross-solid non-gate).
3. **A box beside a vessel** (the door-choice row): one solid takes
   `PlanesTogether`, the other `ChartsTogether`; both closed forms.
   Then a box beside a full torus (SHELL-7's door): the same.
4. **A hollow solid beside a solid one** (`shell(box)` grafted beside a
   plain box): 3 thin solids.
5. **A designation on each solid of a two-solid body** in one call:
   two rims, two cups, closed forms; and a designation on one solid
   only leaves the other sealed.
6. **The single-solid domain is byte-identical**: SHELL-5's and
   SHELL-7's dump instruments at the merge base and head, diff empty;
   the door choice on every single-solid fixture unchanged (assert the
   recorded door per solid equals today's whole-body answer on the
   corpus).
7. **The doors' relaxed precondition**: a move set naming some faces of
   a solid refuses typed as today; a move set naming every face of one
   solid of a two-solid body builds and leaves the other solid
   untouched (its vertices bitwise).
8. **`insert_voids`** in `topo`'s own tests: `N = 1` byte-identical to
   `insert_void`; `N = 2` lands each cavity in its own destination;
   wrong arity refuses typed.

## 3. Stops

STOP and report if a simultaneous door, after the relaxation, still
walks or moves anything outside the named solid (the corner walk,
`is_axial`'s frame, the edge re-author) — measure by a two-solid body
whose second solid's vertices must be bitwise untouched; if the
boolean has no door that builds a two-solid body from two operands
(then row 2's operand is grafted by hand through crate-internal doors
and said so); if `graft_solids_with`'s positional contract does not
hold for the clone (solid order differs); or if a single-solid
fixture moves.

## 4. Docs and owed

`shell.rs` module docs: the sealed arm's opening sentence becomes
per-solid; `NotOneSolid` retired and its replacement documented;
`Shelled::body`'s "the thin solids" sentence. `docs/KERNEL-VERBS.md`'s
shell row: one sentence. **S-BOOL**: `insert_voids` (additive; the
orchestrator announces it). **LIB-G17**: nothing structural changes in
the record; `thickened` grows rows. `offset_together.rs` and
`offset_axial.rs` are SHELL's. Lane rules as every SHELL brief: own
worktree, own `CARGO_TARGET_DIR`, narrow builds (`-p topo -p sweep`),
one heavy cargo job, no `Co-Authored-By` trailer in lane commits, push
after every coherent step, hosted CI is the gate (nothing narrowed),
report ≤ 150 lines.
