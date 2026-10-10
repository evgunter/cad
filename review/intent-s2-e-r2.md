# INTENT stage 2 PR E (#4523) — review r2

Reviewer r2, one of a concurrent Opus pair (`docs/DUAL-REVIEW-PROTOCOL.md`, CONCURRENT arm, H tier).
Frozen head: `17f07b62b669f897f13b02a7869145774985f1f2`; merge base `5d7934ae45a0a780c8fc5b829f72b7b4ddf7d568`.

Lane isolation: I did not fetch or read any other `review/intent-s2-e-*` branch, any other reviewer's scratchpad, or the PR's comments and reviews. I read the PR description only. Nothing glimpsed.

## Verdict

**APPROVE-WITH-FIXES.**

One MAJOR, demonstrated by execution: inline no longer re-hosts a selection whose body is downstream of the instance. Split followed by inline then leaves a fillet naming the deleted instance. Before E, the body-less `Rebind` repaired those names because they were payload names. The fix is local to `refactor::inline`. Every other claim held under execution: two probes, two mutant rounds, the full suite, and a re-run at the merge base.

Counts: MAJOR 1 · MINOR 3 · NIT 2 · style 9.

## Findings

### MAJOR-1 — inline strands a selection whose body is downstream of the instance (claim 5)

`crates/editor-core/src/refactor.rs`, `inline`. Two places miss the case:

- The `readers` scan (around :4110–4127) collects only nodes whose read's `read_operation` is the instance itself.
- The re-anchor loop (`let unbodied = …`, :4488) skips every `NameCarrier::Select` with the comment "The selections of the instance's body are authored afresh on the inlined body below".

Only selections whose body is the instance are authored afresh. A selection on a body downstream of the instance, such as a boolean or transform that reads it, can hold the instance's wrapped `InPart` names. That selection is neither re-authored nor rebound. The instance is then deleted, so the selection names a dead node. The inline reports `StrandedSelection`, and the fillet refuses at evaluation.

Before E, the same inline ran `Rebind { from, to }` for every wrapped name (merge base `refactor.rs:4337–4352`). That rewrote `Node::payload_names`, which included a fillet's selection, so the fillet was re-hosted. This is a regression. It breaks the claim "inline re-hosts on the heir; nothing is silently dropped". It also breaks the A4 split/inline inverse for any document in which a remainder node downstream of the cut selects cut entities: a boolean of the part with a local body, then filleted on the part's edge.

**Failure scenario, DEMONSTRATED BY EXECUTION.** The document:

1. Extrude E is placed in the world.
2. Prism Q is a separate local body.
3. U is `Boolean(Union, E, Q)`.
4. F is `fillet(U, 0.05, [E's lateral edge])`, and F is placed.

The probe then splits with the cut `{frame, profile, E, E's placement}`. After the split, F's selection reads U and names `I/InPart(E edge)`, where I is the instance; the split side is correct. It then runs `inline(remainder, I)`. After the inline, F's selection still names the edge minted by node ordinal 69, which is I, now deleted. The inline's `maintenance` holds `StrandedSelection { readers: [Fillet], name: I/InPart(…) }`. The probe's assertion "every name the fillet's selection holds is minted by a live node" fails. The probe source is in the appendix (`probe_inline_rehosts_a_downstream_selection`).

The split half handles this case. Its rebind loop collects every remainder selection body naming `from` (`refactor.rs:3744–3775`). Inline's loop is the asymmetric one.

### MINOR-1 — a re-point strands an unread named selection silently; a delete of the same name reports it (claim 4)

`crates/editor-core/src/edit.rs`, `stranded_by_repoint` (:5815). The selection pass collects only selections read by `id` or by a node downstream of it. A named selection whose last reader was deleted survives, because `selections_going` drops only unnamed ones. When that unread selection's body is re-pointed so its names fall out of reach, nothing is reported. A `DeleteNode` of the minting node does report the same selection, because `stranded_references` walks every `NameCarrier::Select`. The two strand paths disagree on one document.

**DEMONSTRATED BY EXECUTION.** The document:

1. P, Q and R are prisms.
2. U is `Boolean(Union, P, Q)`.
3. F is `fillet(U, [P edge])`, and F's selection is named `kept`.

The probe counts `StrandedSelection` rows under three edits:

| Edit | `StrandedSelection` rows |
| --- | --- |
| Re-point `U.a → R` with F present | 1 |
| Delete F, then the same re-point | **0** |
| Delete F, then delete P | 1 |

The probe source is in the appendix (`probe_unread_named_selection_strand_by_repoint_and_delete`).

### MINOR-2 — a whole-body measure reference drops its name; a foreign body name is accepted and measures `at` (by execution)

`crates/editor-core/src/operand.rs`, `impl From<SitedRef> for Operand` (:283). A `SitedRef` whose name is body-kind lowers to `Operand::Node(r.at)`, and the name is discarded. `SitedRef { at: P, name: <body of Q> }` therefore inserts cleanly and measures P's body.

**DEMONSTRATED BY EXECUTION:** `editor_core::measure` with `MinClearance { a: SitedRef::new(P, body(Q)), b: <face of Q> }` returned `Ok`.

Before E, the ladder resolved the body name in `at`'s table, so a body name with no lineage into `at` would have refused there. I established that by inspection and did not re-run it at the base.

This is reachable from Python. `MeasurePrimitive.min_clearance((node, "<body name>"), …)` passes any body-name text, and `MeasurePrimitive.refs` hands back the name the user gave, not the body that is measured.

### MINOR-3 — comments citing the retired measure site (claim 8, Q4)

All by inspection, in `crates/editor-core/src/eval/mod.rs`:

- `:4389–4394` still says the keys read "then the sites a measure reads at". There are no sites; `reads` is operand rows only.
- The `UpstreamRead` doc (`:6050–6053`) says "`None` for a measure's site". `port` is now always `Some`, so the `Option` is vestigial.
- The `feed_declared` doc (`:6540–6548`) justifies feeding a node id "as a measure's reference feeds both … it is fed for the measure's reason". E removed that reason: a measure's content key no longer feeds a site or an id. The declared-pair site may still be right to feed, but the premise it cites is gone.

### NIT-1 — history in test headers (claim 8)

Each of these is history, not invariant:

- `crates/editor-core/tests/m10_p_fence.rs` header: "ALL THREE NUMBERS MOVED WHEN A SELECTION BECAME A VARIABLE (INTENT stage 2 PR E) … Merged with stage 5 A's `relation`, the numbers were re-taken once more".
- `lib_g16_corpus_name_digests.rs`: "Re-pinned for INTENT stage 2 PR E".
- The same pattern in the perf2 differential, the tour document pin and `RE-BLESSED for INTENT stage 2 PR E`.

The merge-with-stage-5-A sentence in particular says nothing that holds of the tree. These files already carry such sections, so this follows a convention. I raise it because claim 8 asks.

### NIT-2 — the PR body's `Rebind` compatibility argument

The PR body says `body` is `serde(default)` "so an existing log's rebinds read as `None`". No persisted `DocEdit` log exists in the tree: a grep of the corpus, goldens and fixtures finds none. The semantic change is what matters to a caller. A body-less `Rebind` that repaired a fillet, shell, face-frame or measure name before E now refuses `RebindNoReferences`. Python's `DocEdit.rebind(a, b)` without `body=` is that caller. The pyi docstring documents the new form; the PR body should say the break plainly instead of implying continuity.

## Claims — what I did to each

1. **Single resolution site.** I grepped every `ladder::`, `resolve_in`, `entity_door::entity` and `select(` caller in `editor-core/src`. The callers are:
   - `select`, at `wire.rs:2401` and `:2413`;
   - the declared-pair landing, at `:3398`, `:4056–57` and `:4270–80`;
   - `eval/class.rs:661`, which is a fixture.

   The fillet, chamfer, shell, `FaceFrame` and measure each call `select` once, then read keys off `Selected` (`wire_blend`, `wire_shell`, `wire_datum`, `wire_measure`). None re-resolves. **Mutant:** I made `select` skip its `entity_door` kind check. 7 existing tests went red: `wire_entity_door` ×3, `m6_5_selection_refusals`, `lib_g16_blend_messages` ×2 and `lib_g17_shell_node`. **Holds.**
2. **Canonical at both doors; rebind in place.** `Select::fault` is asked at `mint_selection` and at `Walk::Selection`. `rewrite_selection_names` touches only `select.body == body` and re-canonicalizes. **Mutant:** I dropped that re-canonicalization. 4 tests went red: `edit_blend_canonical`, `lib_g17_r1/r2_probes` and `m6_composed_node`. The row-17 test checks that reader ids are unchanged. `body: None` keeps the merge-base payload and appearance branch verbatim. **Holds.**
3. **FORK-VTX.** The edit-door table test passes. My probes add:
   - A named `Faces` set (a shell's `open`) or `Edges` set (a fillet's selection), read by name at `distance`, `gap` and `min_clearance`: all six refuse `SlotVarKind` (`found: Faces` / `Edges`).
   - A `SetParam` re-point of a `min_clearance` reference to a vertex refuses `SlotVarKind { found: Vertex }`.

   By construction, `clearance_operand`'s `unreachable!` sees only `Body` (the non-selection branch) or `Face` (a `Face`-kind selection, kind-checked in `select`). **Holds.**
4. **Strand reporting.** A delete of the body under a two-edge fillet gives 1 `StrandedRead` and 2 `StrandedSelection`, one per name. That matches pre-E's `StrandedRead` plus `Strand` pairing. The re-point path is incomplete (MINOR-1). A **mutant** that disabled the re-point selection pass was caught by `intent_s2_b_reads::the_slot_door_re_points_an_operand_and_reports_what_it_strands`. **Holds except MINOR-1.**
5. **Split and inline.** The split side is right on my probe: the crossing selection was rebound on its own body to `InPart` names. Inline is not right: **MAJOR-1**.
6. **Content key.** In one document, with one *named* radius, I compared:
   - F1, with its own selection;
   - F2, reading F1's selection by name;
   - F3, with its own selection of the same edge;
   - F4, on another edge.

   F1, F2 and F3 have one key; F4's key differs. **Holds.** My first probes used the literal radius `len(0.05)`, and each fillet keyed apart. The cause is `feed_scalar_join` feeding the anonymous radius variable (`param_source::feed_var`), not the selection; see style S6.
7. **Pins.** I re-ran `m10_p_fence` (all three) and `lib_g16_corpus_name_digests` at the merge base in a separate worktree:
   - The id-masked fence prints `(2b03cbaca56c3ad0, 6637f0f0be0bf72c)` at the base and at the head: identical.
   - The f64 and interval numbers at the base are the old pins `(7c7a…, 1e6c…)` and `(f550…, 0a71…)`, and the head prints the new ones.
   - In the digest table only the seven named rows moved.

   **Holds.**
8. **Surfaces.** I grepped the whole tree (rs, py, pyi, md, excluding `work/`) for every retired symbol and tag (`measure_selection_kind`, `*_selection_resolve`, `shell_open_*`, `face_frame_resolve`, `unresolved_site`, `measures_world_copy`, `repeated_designation`, `selection_not_canonical`, `named_entity`, `resolve_selection`, `resolve_open_faces`, …). The only hits are `docs/INTENT-STAGE2-SPEC.md`, a forward spec, and an unrelated test name. `tags.rs`, `slot_word.rs` and `errors.rs` agree with the kernel enums, as the compile forces. I did **not** run the Python wheel suite, the viewer or the tour (cost). History in comments: MINOR-3 and NIT-1.

## Style

Questions exercised: Q1, Q2 (prose grep only), Q3, Q4, Q6 and Q7. Q5 was partial (the `wire.rs` module doc and `Select` doc only). Q8 was **not** exercised: I read `intent_s2_e_select.rs` whole, but not `node.rs` or `edit.rs`.

- **S1 (Q1) — sure.** `MeasureVerb::admitted()` (`measure.rs:222`) is a hand-written prose copy of the `admits()` table beside it. The two drift independently, and nothing checks them against each other. The class is a "kinds as data, kinds as words" pair; `VarKind`'s `Display` is the other place to look.
- **S2 (Q1) — likely.** The `VarKind` ↔ `EntityKind` correspondence is written three times:
  - `VarKind::selection` (`var.rs`);
  - `SlotKind::selection_kind`'s `Measured` arm (`operand.rs:259`);
  - `select`'s per-entity `read` fn match (`wire.rs`, with a `Body => unreachable!`).

  Adding a `Vertices` kind, which the FORK-VTX ruling foresees, has to touch all three.
- **S3 (Q1) — likely.** The block "re-author a selection on a new body with mapped names, kind from `source.var(var).map_or(VarKind::Faces, Var::kind)`" appears three times in `refactor.rs`: `carry`, the split's crossing reads, and the inline's readers. The `Faces` default cannot fire, since `selection(var)` was `Some`, but it is written each time. MAJOR-1 is what happens when one of three copies has a different scope.
- **S4 (Q7) — likely.** `mint_selection` (`edit.rs`) decides the entity from `names.first()` and defaults to `Face`. At a fixed seat it ignores the entity entirely. A `Faces` seat given edge names mints a `Faces` holding edges and refuses at evaluation (ruling 1). An `Edges` seat given out-of-order face names refuses at the door with `NotCanonical`, a message about edge sort order, when the real fault is the kind. The door has the names' kinds in hand.
- **S5 (Q5) — sure.** `Select::fault`'s doc says it is "the one predicate both doors ask". `SelectionFault::Seat` is never produced by `fault`; only `mint_selection` builds it. The load door's `Walk::Selection` therefore cannot produce one of the variants of the fault type it reports.
- **S6 (off-target, Q3) — unsure if pre-existing; observed by execution.** Two fillets in one document on the same body, with the same names and the literal radius `len(0.05)`, have different content keys. `feed_scalar_join` feeds the radius through `param_source::feed_var`, apparently by the anonymous variable's id. Two extrudes with the literal depth `len(1.0)` key equal. If this predates E, a blend with a literal radius never memo-hits its twin. I did not check the base.
- **S7 (Q2/Q7) — unsure.** `wire_measure`'s whole-body read hard-codes `EntityRef { body: 0, key: Body }` (ruling 4). That holds only while a `Body`-kind variable's payload holds exactly one body. Nothing at this site checks it, and `output_body(…, 0)` would silently pick the first of several.
- **S8 (Q6) — sure.** Ruling 2 (`SelectResolve` rather than spec row 16's `UnresolvedRead`) and ruling 5 (a measure is exempt from DM5) are disclosed deviations. Ruling 2 reads as an improvement. Ruling 5 is a carve-out in a door rule, with no work item recording it as settled or scheduled. A grep of `work/` for DM5 together with measure finds none.
- **S9 (Q7) — likely.** `Rebind { body: Option<VarId> }` folds two edits with disjoint carriers into one variant. `Some` touches only selections and `None` touches only non-selections. Each caller (split, inline) now has to enumerate carriers to decide which to issue, and inline got that wrong (MAJOR-1). Two edits would have made the address type-checked.

## What I executed

- `cargo nextest run -p editor-core --profile ci --no-fail-fast` on the frozen head: **2886/2886 passed** (173 skipped), 359 s.
- A probe module of 10 tests, added temporarily to `tests/all.rs` and not committed:
  - 2 demonstrate findings: MAJOR-1, and MINOR-1's counts;
  - 1 demonstrates a behaviour change: MINOR-2;
  - 5 confirm claims 3, 4 and 6;
  - 2 were invalidated by the radius confound in S6.
- Mutant 1, the `select` kind check skipped: 7 existing tests red.
- Mutant 2, re-canonicalization removed plus the re-point selection strand disabled: 4 + 1 existing tests red (full suite, 2897 run). Both mutants were reverted.
- Merge base `5d7934ae`, in a separate worktree: `m10_p_fence` ×3 and `lib_g16_corpus_name_digests`, 4/4 passed; the id-free fence prints the same number as the head.
- Not run: the Python wheel suite, the viewer, the tour, clippy, ε sweeps.
- Wall clock about 1h45m. Token count not available to me.

## Appendix — the MAJOR-1 probe

Written against `crates/editor-core/tests` fixtures. `prism` is the helper from `intent_s2_e_select.rs`.

```rust
#[test]
fn probe_inline_rehosts_a_downstream_selection() {
    use crate::fixture::resolver::PartStore;
    use editor_core::{DocumentId, inline, split};
    use std::collections::BTreeSet;
    use std::sync::Arc;
    let doc = ProfileDoc::empty(DocumentId::derive("r2-inline"), Tol::witness());
    let (doc, profile) = on_frame(doc, [0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![square(0.0, 0.0, 1.0)]);
    let frame = doc.ids()[doc.ids().len() - 2];
    let (doc, ext) = insert(doc, Node::Extrude { profile: profile.into(), distance: len(1.0), side: ExtrudeSide::Along });
    let (doc, placed) = fixture::place(doc, ext);
    let edge = fixture::prism_edges(&doc, ext, 4)[2].clone();
    let (doc, q) = prism(doc, 0.5);
    let (doc, u) = insert(doc, Node::Boolean { op: BooleanOp::Union, a: ext.into(), b: q.node.into(), declare: vec![] });
    let (doc, fil) = insert(doc, Node::fillet(u, len(0.05), vec![edge.clone()]));
    let (doc, _) = fixture::place(doc, fil);
    let out = split(&doc, &BTreeSet::from([frame, profile, ext, placed]), DocumentId::derive("r2-inline-part"), Tol::witness(), None).unwrap();
    let mut store = PartStore::default();
    store.insert(out.part.clone(), Tol::witness());
    let inlined = inline(&out.remainder, out.instance, &(Arc::new(store) as Arc<dyn editor_core::PartResolver>), Tol::witness()).unwrap();
    let names = fixture::selected(&inlined.doc, fixture::selection_read(&inlined.doc, fil));
    // FAILS on 17f07b62: the name's node is the deleted instance, and
    // `inlined.maintenance` holds a StrandedSelection for the fillet.
    assert!(names.iter().all(|n| inlined.doc.node(n.node).is_some()), "{names:?}");
}
```

The MINOR-1 probe uses the same fixtures:

1. Insert P, Q and R, then `U = Boolean(P, Q)` and `F = fillet(U, [P edge])`.
2. Name F's selection, then `DeleteNode F`.
3. Apply `SetParam { node: U, slot: Operand(A), value: Operand::Node(R) }` and count `StrandedSelection` rows: 0.
4. Instead apply `DeleteNode P` and count them: 1.
