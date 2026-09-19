# EDIT-RADIUS — the replay record says which segment each radius drew (spec)

**Unit:** `fused-arc-fillet-steps-have-no-per-segment-radius-address`
(kernel unit, v6 dual, block EDIT-B2 slot 1 — implementer OPUS per the
block's draw; pre-draw fields: difficulty **M**, task-class
**STRUCTURAL** — a new record on `profile`'s `ReplayStructure`, DM8's
map widened from "one radius, one segment" to that record, the key's
feed widened with it). **Ruled by the EDIT orchestrator** (2026-09-19)
on the row's own analysis and DM8; nothing here changes what DM8
decides (the map reads records the evaluation produced and never
re-derives them — this unit adds the record it lacked). Deleted at
merge and recorded in `docs/DOC-LEDGER.md`, as the EDIT-DECL spec was.

Branch `edit/radius-emission-record`. Read first: the row (`## The
finding`, `## What a taker does`); DM8 in
`crates/editor-core/REFERENCES.md` (and its "Why not `crates/profile`
alone" bullet); `crates/profile/src/structure.rs` (`ReplayStructure`,
`StepSpan`, `FilletDecision`, `Guide::{Recording, Guided}`,
`into_record`, `consume`, `StructureRefusal`); `crates/profile/src/path.rs`
(`Core`: `fillet_arcs`, `program`, `step_starts`, `record`,
`step_spans`, `record_fillet_arc` and its three callers, `set_leaving`
and every caller that passes an ARC bulge — `path.rs` ~3029/3180/3298/
4062/4317 and `path/family.rs` ~335/354 — and `verbs::Pending`, the
pending fillet); `crates/profile/src/path/arc_fillet.rs` (BLEND's, by
announced seam: the fused fillet's emission); `crates/editor-core/src/program.rs`
(`radius_arg`, `StepArg::is_radius`, `step_slots`, `step_expr`,
`LoopProgram::step_radii`, `ProfileProgram::profile_edges_of`,
`ProfileProgram::segment_radii`); `eval/mod.rs`'s `content_key` feed
and `eval/wire.rs`'s `attach_swept`; the suites
`crates/editor-core/tests/edit_step_segments.rs` (§5 — the four
radius rows named below), `seat7_sweep_lowering.rs` §2b,
`edit_recorded_notation.rs` (the fused-verb chain it authors);
`docs/PATHS-DESIGN.md` (a ratified page — grep it for the replay
record before touching it; today it names none of these types);
`docs/prompts/implementer-discipline.md`.

## The ruling, as premises

1. **The replay record gains a per-radius emission record.**
   `ReplayStructure` (`crates/profile/src/structure.rs`) gains
   `radii: Vec<RadiusEmission>` with `RadiusEmission { step: usize,
   role: RadiusRole, segment: usize }` and `RadiusRole { Fillet,
   Carrier, Carrier2 }`: the authored step whose radius argument drew
   the segment, which of that step's radius roles it was, and the
   PRE-CANONICAL segment index (the same numbering `StepSpan` uses —
   segment `k` leaves vertex `k`). Profile-side vocabulary only; the
   `StepArg` names are `editor-core`'s and the map translates
   (`Fillet → StepArg::Radius`, `Carrier → CarrierRadius`, `Carrier2 →
   CarrierRadius2`). It is a decision like `steps` beside it — WHICH
   arm emitted the arc — recorded as the pass emits and reported by
   `into_record` as what THIS pass emitted (the guided arm reports its
   own emissions exactly as it reports its own spans; the caller's
   comparison is the guard). Not persisted; `ReplayStructure` derives
   no serde today and gains none.
2. **Every arc emission records its radius's address at the moment
   it sets the bulge.** A carrier arc (`arc_to` with a `Radius`,
   `Sweep` or `ArcLen` spec; the incoming and arrival specs of the
   fused verbs) records `(program.len() − 1, Carrier or Carrier2,
   verts.len() − 1)` where its bulge is set — the current step is the
   emitter and the segment leaving the chain's last vertex is the arc.
   A fillet arc records `(binder step, Fillet, leaving)` at
   `record_fillet_arc`: the STEP is the one whose `radius` argument it
   is — the `fillet(r)`/`arc_fillet(spec, r)` binder for a pending
   fillet consumed by a later arrival, the arrival step itself for
   `fillet_arc(r, spec)` and `arc_fillet_arc(spec, r, spec2)`. The
   pending fillet therefore carries the step index it was bound at
   (`verbs::Pending` or its chain-side meta gains the field; the
   binder's row reads `program.len() − 1` when it binds). A `Bulge`,
   `Via` or `Center` spec carries no radius and records nothing.
   `fillet_arcs: Vec<(usize, T)>` stays as it is (the tangency
   re-read); do not fold the two — one is a decision record, the
   other a close-time check — but say so once where they sit.
3. **DM8's map reads the new record; "one radius, one segment" goes.**
   `ProfileProgram::segment_radii` answers, for each `RadiusEmission`
   of the loop's record, `(edge of segment, expr at (step, arg of
   role))` — the edge through the SAME permutation `profile_edges_of`
   computes (factor a per-segment `profile_edge_of(structure, naming,
   loop_, segment)` out of it so the two cannot disagree about the
   permutation; `profile_edges_of` becomes a map over its span). The
   `(false, &[one])` rule and `radius_arg`'s single-answer role in the
   attach are retired; `radius_arg` survives only if a caller other
   than the attach needs it (say which, or delete it).
4. **The key's feed widens with the attach, and the inclusion
   holds.** `LoopProgram::step_radii` yields EVERY `(step, arg)` with
   `arg.is_radius()` — a fused step's two or three radii all enter
   `eval::content_key` — so "attached ⊆ keyed" stays true by
   construction and the direction the memo's stale-token guard rests
   on is unchanged. `a_step_with_several_radii_answers_no_radius`
   inverts into "…answers each of them" and
   `every_attached_radius_was_keyed_first` keeps its strict side (a
   binder whose arc no arrival ever emitted, or a `fillet` whose
   arrival the fit gate flattened, is a keyed-never-attached radius —
   author one). Corpus keys move for every document with a fused step
   or a `fillet` binder: measure by the chain-radius unit's corpus dump
   at both heads and list every moved key in the PR body (it is the
   conservative side; a stale-token row cannot red on it).
5. **DM8's clause text is re-worded, not re-decided.** "reading two
   records the evaluation already produces: the replay's per-step
   segment span" becomes the replay's records — the per-step span and
   the per-radius emission — a description the code moved, landing
   with the change (CLAUDE.md's rule; state in the PR body where you
   looked for a ratification of the sentence). The bullet "Why not
   `crates/profile` alone" stays true and is cited.
6. **The seam is announced, not owned.** `crates/profile` is PATHS's
   (`work/paths/program.md`: the persist schema is contended ground —
   this record is not persisted, say so; `docs/PATHS-DESIGN.md` is
   ratified — touch it only if a sentence is now false, and say which)
   and `path/arc_fillet.rs` is BLEND's by announced seam. The PR body
   announces both, with the exact lines each seam touches.

## Rows (each red on `origin/main` first, then green; each named with the invariant it pins)

- **The arrival-step row the filed row names**: a closed chain with
  `FilletArc { radius, spec: Via { .. } }` (or `Center`) — the fillet
  arc is credited to the ARRIVAL step and carries `radius`; the
  `Via` arc carries nothing. The row said authoring it refused
  `NoCornerForFillet` inside the chain-radius unit's budget: author
  it first, and if the fillet solver cannot meet, say exactly which
  geometry refused and author the nearest chain that does — this row
  is the one shape of the finding nothing pins today.
- `fillet(r)` binder then a `line`/`toward` arrival: the arc's wall
  carries `r` (`a_fillets_radius_is_a_program_answer_and_no_edges`
  inverts into `…_reaches_its_arcs_wall`); the radius is at the
  BINDER's `(step, Radius)` address, not the arrival's.
- `arc_fillet(Radius spec, r)` (fused binder): the incoming arc's wall
  carries the spec's carrier radius at `(step, CarrierRadius)` and the
  fillet arc's wall carries `r` at `(step, Radius)` — two segments,
  two addresses, one step (`a_one_radius_fused_step_attaches_to_no_edge`
  inverts; its `Bulge` twin stays: a bulge spec records nothing).
- `arc_fillet_arc(spec, r, spec2)` over two `Sweep` specs: three
  radii, three segments, `CarrierRadius`/`Radius`/`CarrierRadius2`
  each on its own wall; a mutant swapping `Carrier` and `Carrier2`
  reds it.
- `fillet_arc(r, Radius spec)`: the arrival's fillet arc and the
  spec's arc, both credited to the arrival step, two roles.
- A guided replay (`Guide::Guided` over a recorded structure)
  reports a `radii` record EQUAL to the recording pass's (the
  emission is structure: reproduced, not re-derived), and a record
  whose `radii` names a step outside `steps` or a segment outside its
  span refuses typed at the map (`StepSegmentsError::RecordShape` or
  a sibling arm — say which).
- `every_attached_radius_was_keyed_first`: the inclusion over the
  whole corpus with the widened feed, strict side authored (premise 4).
- The corpus key dump: every moved key listed and each attributed to
  a fused step or binder (the PR body's table, as the chain-radius
  unit did).
- `seat7_sweep_lowering` §2b and the existing per-edge rows green
  unchanged (a single-radius chain's answer does not move).

## Mutants (each named with the rows it reds)

- The fillet arc recorded at the ARRIVAL step for a pending fillet
  (the binder row).
- The emission's segment index off by one (`verts.len()` instead of
  `verts.len() − 1`; the `arc_fillet` row's two segments swap walls).
- `Carrier2` recorded as `Carrier` on `arc_fillet_arc` (the three-radii
  row).
- `step_radii` keeping `radius_arg`'s single-answer rule (the inclusion
  row: an attached radius never keyed).
- The guided arm reporting the handed-in record instead of its own
  emissions (the guided row, with a record deliberately wrong).
- `segment_radii` still filtering `(false, &[one])` (every fused row).

## Territory

`crates/editor-core/src/{program.rs, eval/mod.rs, eval/wire.rs}` and
`REFERENCES.md` DM8's sentence (EDIT; `eval/*` is WIRE's by
announcement as the chain-radius unit crossed it);
`crates/profile/src/{structure.rs, path.rs, path/family.rs}` (PATHS, by
announced seam — the record and its emission sites) and
`path/arc_fillet.rs` (BLEND, by announced seam); `crates/editor-core/tests/*`
(TCOST/TINT); `crates/pncad-py` only if a binding reads
`ReplayStructure` (grep; LIB by announcement). Kernel unit: the v6
dual on a frozen head, then the union fix pass.

## Verification

Green hosted CI on the pushed head (twelve `test (…)`, five `k-lint`,
the python suite — step conclusions); `cargo test -p profile --test
all`, `cargo test -p editor-core --test all -- edit_step_segments
seat7 edit_recorded_notation`, `--lib`; `cargo clippy -p profile -p
editor-core --all-targets -- -D warnings`; `cargo check -p viewer
--all-targets`; the corpus key dump at both heads; `cargo doc -p
editor-core --no-deps` baseline 96 warnings, none new; `python3
scripts/work.py lint`. The PR body is the record: premises verified
or corrected, the two seams announced with lines, the moved keys, the
sweep (`radius_arg`, `one radius, one segment`, `single-radius`,
`fillet_arcs`) and what it could not match.
