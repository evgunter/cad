# SHELL-5 — shell of a hollow body: thicken every boundary

**Status: BINDING at dispatch (SHELL orchestrator, 2026-09-08).**
Closes `work/shell/shell-of-hollow-body-thicken-every-boundary.md`
(issue record 1056) to Ev's ruling, quoted there verbatim: *"the
eventual resolution must be 'thicken every boundary' — offsetting
only the outer shell is explicitly rejected."* Unit `SHELL-5`, branch
`shell/5-hollow-operand`. Deleted at merge per `docs/DOC-LEDGER.md`;
the item file is the record that survives. Read
`docs/prompts/implementer-discipline.md` in full first. Survey run
against main `891db152` on 2026-09-08; every citation below is by
symbol on that head, and a line number is a hint, not a claim.

## 0. The semantics, stated once

*Shelling* a solid keeps the material within wall thickness `t` of
its boundary: `shell(S, t) = S − offset_inward(S, t)`
(`crates/topo/src/shell.rs`, module docs). For a solid `S` with outer
shell `O` and void shells `V₁ … Vₖ` (a *void* is a cavity — a
disconnected interior boundary shell, born only through
`topo::boolean::voids::insert_void`), the inward offset moves EVERY
face of EVERY shell into the material by `t`: `O` erodes inward to
`O′`, each `Vᵢ` dilates outward to `Vᵢ′`. What remains is one thin
solid per boundary shell:

- **A** — bounded by `O` (outer) and `O′` (a new cavity), and
- **Bᵢ** — bounded by `Vᵢ′` (its outer shell, material on the inside)
  and `Vᵢ` (the operand's cavity, now the cavity of `Bᵢ`), for each
  `i`.

So the result body holds `k + 1` solids and `2(k + 1)` shells, and
the operand's own shells all survive with their keys. Today the door
refuses the whole case (`ShellError::OperandAlreadyHollow`); after
this unit that variant does not exist.

**One rule, no special case.** `inward(face, t)` already reads the
face's `sense` to move "into the material": on a void face that IS
the dilation, because a void's faces are wound with their material
side outward. The lane must not write a void-specific sign; the
acceptance rows in §3 are what make that claim checkable.

## 1. What changes in `shell_open`

The construction keeps its order; two steps widen and one is added.

1. **Gates.** `Thickness`, `NotOneSolid` (a multi-solid operand stays
   refused — that is a different question, not this ruling's; say so
   in the variant's docs), `ChartSenseMixed` (per chart, already
   shell-blind), `wall_clearance` (already walks every planar face of
   every shell; §3 proves it), the designation gates (§2). The
   `OperandAlreadyHollow` gate and variant are DELETED, with the two
   planted reds that pin it
   (`crates/sweep/tests/offd2_r1_probes.rs`,
   `crates/sweep/tests/shellfix1_r1_probes.rs`, both
   `matches!(e, ShellError::OperandAlreadyHollow { shells: 2 })`)
   rewritten as rows of the new semantics — each keeps its file and
   its reason on the page, and asserts the §0 shape instead.
2. **The moved clone.** Unchanged in mechanism: one clone, every chart
   moved by `inward` through the door the body picks (`all_planar` →
   `offset_planes_together`; `is_axial` → `offset_charts_together`;
   otherwise `replace_faces_offset` per chart). The branch choice
   reads the whole body, shells included; a hollow body of revolution
   is still axial. If a door refuses on a void chart for a reason
   that is about the door and not the geometry, that is a STOP (§5).
3. **The evidence and the insertion.** Every shell of the moved clone
   is strictly inside the operand's material — that is exactly what
   the per-face reach decides plus `wall_clearance` establish, for
   `O′` and for every `Vᵢ′` alike — so the evidence stays
   `Carried { Positive }` per clone shell and the whole clone goes
   through `insert_void` as today. The door reverts the clone (so
   `O′` faces inward as a cavity must, and `Vᵢ′` faces OUTWARD, as the
   outer shell of `Bᵢ` must) and grafts every shell under the
   operand's solid. That grafted state — every shell in one solid —
   is not the ruled shape and is never observable: step 4 follows
   before anything else reads the body.
4. **NEW — the thin solids.** For each operand void `Vᵢ` (source shell
   key, read off the operand before the clone) and its transplanted
   twin `Vᵢ′` (`VoidInserted::shell`), move BOTH shells out of the
   operand's solid into a new solid `Bᵢ`. This is a re-partition of
   ownership, not an Euler operator — the same kind of door as
   `movefac` (`crates/topo/src/movefac.rs`: mints shells, records
   `Provenance::Movefac`, asserts an `ArenaDelta`) one level up:
   `Body::move_shells_to_new_solid(shells: &[ShellKey]) ->
   Result<SolidKey, EulerOpError>` (name is the lane's; the shape is
   binding), which refuses typed when a shell does not resolve, is not
   in one solid, or would leave its solid empty, mints the solid with
   a provenance variant in `Movefac`'s shape naming the solid it was
   split from, rewrites the moved shells' `solid` back-pointers, and
   asserts `ArenaDelta { solids: 1, .. }`. The pairing `Vᵢ ↔ Vᵢ′` is
   STRUCTURAL (the graft map) — no `classify_shells`, no containment
   probe, no flux read decides which shell goes where. The op lives
   beside `movefac` (`crates/topo/src/movefac.rs` is unowned ground;
   the PR draws the fence by naming it); `provenance.rs` gains the one
   variant.
5. **The rim surgery** (opened arm) runs AFTER step 4, so a designated
   face and its counterpart are in one solid — `kfmrh`'s cross-shell
   form is the same-solid fusion, and it now fuses `Vᵢ′` into `Vᵢ`
   exactly as it fuses the cavity into the outer wall today.
6. **One validation**, as today.

## 2. The opened arm on a hollow operand

A designation may name a face on ANY shell. On a void face the
composition is the sealed construction's own steps read one solid
over: the counterpart is `inserted.face(f)` (the graft map), the lift
puts `Vᵢ′`'s chart back onto the designated face's plane, both charts
canonicalize, and `kfmrh` fuses `Bᵢ`'s two shells through the mouth,
so `Bᵢ` becomes a cup whose rim faces the space between `O′` and
`Vᵢ′`. `check_designation`'s per-shell remainder rule already reads
every shell. Nothing here is a new mechanism; if a step turns out to
need one, §5 applies.

## 3. Acceptance — every row a test, every number closed-form

Fixtures are the existing suite's (`crates/sweep/tests/verbs_shell.rs`:
`boxy`, `vessel`, the full-period `tube_along_arc_hollow` torus) so no
new fixture family is minted. `V(w, d, h) = w·d·h`.

1. **The ruled composition.** `shell(shell(boxy(2,3,4), 0.25), 0.05)`:
   2 solids, 4 shells, tier 3 green; per SOLID, `classify_shells`
   reads exactly one `Outer` and one `Void` (read the roles grouped by
   `Shell::solid`, not as a flat multiset — tier 3 does not check
   solid membership, so the grouping is pinned HERE); volume =
   `[V(2,3,4) − V(1.9,2.9,3.9)] + [V(1.6,2.6,3.6) − V(1.5,2.5,3.5)]`
   to `1e-12`. The face-key survival claim: every face of the operand
   resolves in the result under the same key, in the same shell.
2. **The record.** On row 1: `naming.outer` has one row per operand
   face (both shells, face-arena order) with equal columns —
   `ShellNaming`'s header says the identity "stops holding" on a
   hollow operand; it does not (operand shells are never regrafted),
   so the sentence is corrected to the true one. `naming.inner` rows
   resolve through the graft to the moved twins for both shells.
   `naming.thickened: Vec<(SolidKey, ShellKey)>` — NEW field, result
   solid ← the operand shell whose wall it is, shell-arena order —
   lists two rows, the operand's solid key for `O` and the new solid
   for `V`. The field is additive (LIB-G17 reads the record while this
   unit is in flight; it constructs nothing).
3. **The clearance gate is shell-blind.** `shell(shell(boxy(2,3,4),
   0.25), 0.15)` refuses `WallClearance` naming one face of `O` and
   one of `V` (0.25 < 0.30). A two-void operand (two disjoint boxes
   inserted through `insert_void`, or two subtractions if the boolean
   fallback builds it — measure, and say which) with material `g`
   between the voids refuses at `t > g/2` naming two VOID faces, and
   builds 3 solids at `t < g/2` with the closed-form volume.
4. **Axial and curved.** `shell(shell(vessel(1,2), 0.2), 0.05)`
   through the axial door, closed form
   `π[(1²·2 − 0.95²·1.9) + (0.85²·1.7 − 0.8²·1.6)]` within the props
   pad; the full-period hollow torus (`R`, outer `r`, wall `w`)
   shelled at `t`: two thin tori, `2π²R[(r² − (r−t)²) + ((r−w+t)² −
   (r−w)²)]`, within the pad. If the torus refuses for a reason in
   the offset doors (not this unit's), the row stays as the measured
   refusal with the door named and the case is filed.
5. **The opened arm, both shells.** On `shell(boxy, 0.25)`: designate
   the OUTER top → `A` becomes a cup, `B` untouched, volume = row 1's
   outer term minus the `1.9 × 2.9 × 0.05` slab, `naming.rims` one
   row, `thickened` unchanged; designate the VOID's ceiling → `B`
   becomes a cup opening upward into the gap, `A` untouched, volume
   by the same arithmetic on the inner term.
6. **No crossing machinery.** `shell_runs_no_intersection_machinery`'s
   filter over the row-1 composition (the `bool_` prefix, the named
   validator exception).
7. **The old domain is byte-identical.** On every single-shell operand
   the suites already run, the result is unchanged — the existing rows
   are the differential, and the verdict-corpus instrument at
   `verbs_shell.rs` ("byte-identical … diffed") is run on the
   acceptance corpus against the merge base and cited in the PR.
8. **The new op refuses typed** at each of its three preconditions
   (§1.4), in `topo`'s own tests.

## 4. Docs

`shell.rs` module docs: the sealed arm's step list gains the thin-solid
step; the "what it carries" paragraph says the evidence covers every
clone shell; the record section's "will break that identity" sentence
is corrected (§3.2). `ShellError::NotOneSolid`'s docs say what a
multi-solid operand would need. `docs/KERNEL-VERBS.md`'s shell row
gains one sentence (the register is another program's file — a seam
the PR names, one sentence, no restructuring). `crates/topo/README.md`
does not change unless the new op's doc belongs in its op roster —
read it and decide; say what you decided.

## 5. Stops

STOP and report, with the measurement, if: an offset door refuses a
void chart for a door reason (a per-chart refusal on the dilation
direction, an `is_axial` verdict that reads only one shell); the rim
surgery on a void face needs a mechanism §2 does not name; the
boolean fallback builds a hollow-`B` subtraction into the same
one-solid shape step 3 passes through (that is an S-BOOL finding —
report it, do not fix it); or tier 3 accepts a body whose solid
grouping is wrong (a missing check — report to TOPO, pin the grouping
in your own rows per §3.1). A golden that moves is a STOP too.

## 6. Owed to neighbours (report in the PR body; do not file there)

- **S-BOOL**: `insert_void`'s docs say "positively oriented
  single-solid closed body" — after this unit its one shell producer
  hands it a clone whose voids it then re-homes; state that in the
  door's docs in one sentence. The hollow-`B` subtraction question
  (§5) is theirs.
- **LIB**: the `thickened` field, for `Node::Shell`'s emitter.
- **SHELL-4**: the cavity clone it will check is the whole moved body,
  every non-adjacent pair — note it in the module docs' window
  paragraph rather than leaving it to SHELL-4 to rediscover.
- **TOPO**: `movefac.rs` and `provenance.rs` edits, fence drawn in
  the PR.

## 7. Lane rules

Own worktree, own `CARGO_TARGET_DIR`, one heavy cargo job at a time,
push after every coherent step, hosted CI is the gate (a code-tier
run gates every lane and eps row; nothing is narrowed — if the job
list shows fewer than twelve `test (…)` jobs, find out why before
reading it as green). No `Co-Authored-By` trailer in lane commits.
Report ≤ 150 lines: what landed, every deviation from this spec with
its reason, the §5 measurements, the §6 notes, the CI run id.
