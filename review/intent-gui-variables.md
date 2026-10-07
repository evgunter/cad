# Review — PR #4247, INTENT: the GUI's variables (frozen head 1a2ed64d)

Base `7c506747`. I did not read the PR's comments. I did not judge the nine choices. I
checked whether the code builds them as the body states them.

## Verdict: APPROVE-WITH-FIXES

There are two MAJOR defects. In both, the code does something the body says it does not:
a drag earns an offer, and an open naming field can rename an existing named variable.
Each fix is a few lines, and neither one would change any of the nine choices. If Ev
answers a choice differently, each choice can still be reworked by itself as a
follow-up: the offer is one `Derived` field plus a function in `props`, and the name is
one draft plus one function in `props`.

## Findings

**MAJOR-1 — An offer stays open after a drag, and then lists matches for the dragged value.**
Bears on C1 and C2, and on body choices 2 ("It never appears for a value nobody typed")
and 5 ("Does a drag earn an offer? … no").
`session.rs` `DocSession::offered` (~:1077) checks only that
`doc.slot(node, slot) == Some(offer.var)`. A drag moves the typed variable in place (Q6),
so the slot still reads `offer.var` and the offer stays open. `equal_variables` then
lists the variables equal to the dragged value.
Demonstrated by execution, on `two_extrudes`-style setup:
1. Type 12 mm at `a`. `offered == [w]`.
2. `BeginGesture` / `PreviewGesture(0.010)` / `CommitGesture` on `a`.
3. `offered(a) == [b's variable]`. The panel now offers to join `a` to `b`, at a value
   nobody typed.

`SetVariable` on the slot's own minted variable does the same (second probe red).
`gui_variables::an_offer_stands_only_while_the_slot_reads_what_was_typed` moves `w`,
not the slot's own variable, so it cannot catch this.
Confidence: **sure**.

**MAJOR-2 — An open naming field renames whatever the slot reads when it is committed, including a named variable.**
Bears on C3 and on body choice 8 ("A named variable still has no rename control in the panel").
`pane/properties.rs` `name_field_ui` (~:964–1003) keys the draft by `(node, slot)`
(`drafts.rs` `name_draft`). At commit it reads `var` from the slot's current reader,
not from the variable the **name…** button was opened for.
Demonstrated by execution, with an app row added locally to `properties_pane_tests`:
1. Start from `typed_beside("beam", 0.004)`.
2. Click `name…`. The field opens on `distance`.
3. Accept the offer of `beam` (`SetSlotVariable`). The offer button is drawn on the same
   row at the same time.
4. Click `Name`.
5. The result is `doc.var_name(beam) == Some("distance")`: the existing named variable
   `beam` is silently renamed.

A retype or an undo while the field is open would also send the name to a different
variable than the one it was opened for.
Confidence: **sure**.

**MINOR-1 — A redo brings back an offer that the undo had closed.**
Bears on C2 as the dispatch states it ("an undo, a redo … clears it").
Demonstrated by execution: type, `Undo` (offer empty), `Redo`, then `offered == [w]` again.
Nothing resets `derived.offer` on undo or redo. Only the reads-check in `offered` hides
it. The body's own wording ("until the slot stops reading the variable that typing
minted (a later edit, or an undo)") arguably allows this. The `SlotOffer` doc comment
(`session.rs` ~:511) says it stands "until … the slot no longer reads that variable",
which is also consistent with it. So the dispatch's claim is false and the body is
silent on redo. Ev may want to rule on it. Confidence: **sure** (fact); **unsure**
whether it is intended.

**MINOR-2 — One mutant survives: removing the `is_typed_value` filter in `offer_after`.**
Bears on C7.
Demonstrated by execution: delete `.filter(|&var| doc.is_typed_value(var))` in
`session.rs` `offer_after` (~:1912), and all 14 targeted rows stay green.
`typed_text_is_offered_and_a_formula_is_not` types `w` only after the 12 mm variable has
been retired (VR7), so nothing else equals `w`. Its setup rules out the case where the
filter matters.
My probe types 12 mm at `b`, then `w` at `a`. With the mutant, `offered(a)` lists `b`'s
variable, so a slot reading a formula gets an offer. The shipped code is right; only the
row is blind. Confidence: **sure**.

**MINOR-3 — The offer is not bit-equal, even though the doc comment says it is.**
Bears on C2 and choice 3 ("bit-equal canonical value").
`props.rs` `equal_variables` (~:1123) compares with `SlotValue`'s `PartialEq`, which
uses f64 `==`. Its doc says "Equal is equal at the bits". Demonstrated by execution: a
length `z = -0.0` is offered to a slot typed `0`. Rarely seen in practice, but the
promise is stated twice. The same probe shows that typing `0` at an extrude in the
framed-square fixture offers **8** variables, the frame's anonymous zeros among them.
This is the measured size of choice 3's "anonymous variables are offered too"; it is
reported for Ev, not judged. Confidence: **sure**.

**MINOR-4 — `Refusal::NoSuchParam` is still cited in kernel prose.**
Bears on C4.
`crates/editor-core/src/edit.rs:1272` and `:3184` name `Refusal::NoSuchParam`, which this
PR renamed to `NoSuchVariable`. The PR's sweep fenced kernel *names*, but these are
citations of a *viewer* identifier, so they are now stale. Found by inspection (rg).
Confidence: **sure**.

**NOTE-1 — `SetSlotVariable` does not check the offer.**
Bears on C2 ("a stale offer can never be accepted onto a moved slot").
The op is the general slot-write gesture. Demonstrated by execution: after a retype moved
the offer from `[w]` to `[b]`, `SetSlotVariable{var: w}` still lands. The only guard is
that the pane redraws its buttons from `offered()` every frame. That is fine for the
pane, but the claim holds only at the pane, not at the op. Confidence: **sure**.

**NOTE-2 — A refused name commit loses the typed text.**
Bears on C3. In `name_field_ui`, `name_draft.take()` runs before the door answers. If
`RenameVar` refuses on a name clash (`VarNameTaken`, typed through `Refusal::Edit`), the
field closes and the person's text is gone. An invalid name never reaches the door: the
button is disabled and the hover shows the `VarName` fault. Found by inspection only.
Confidence: **likely**.

**NOTE-3 — "param" survives outside the item's list.**
Bears on C4. Everything on the item's list is renamed. Still left:
- `Refusal::DrivenByExpression { params }` (`session/refuse.rs:237`, and `affordance(params…)`);
- the egui id salts `"add_param"` (`pane/properties.rs:306`, `:309`) and `"param_unit"` (`:737`);
- the test names `an_undeclared_parameter_is_said_once_in_the_pane` (`app.rs`),
  `an_undeclared_parameters_verdict_is_drawn_loud` and
  `a_parameters_range_reading_is_said_over_its_button_inside_the_pane`
  (`pane/properties.rs`), plus a `fn param(..)` test helper.

None of these is shown to users. Confidence: **sure** (they exist); whether they count is Ev's call.

## Claims exercised

- **C1: holds**, apart from the offer half (MAJOR-1).
  - Typing writes one `SetParam`; the old variable is retired; one `Undo` restores it, and
    retyping the standing value commits nothing (my probes).
  - Drag in place is guarded: swapping the gesture's `slot_edit` for `slot_typed_edit`
    turns two `gesture_table` rows red. The probe's samples use `slot_edit` (inspection).
- **C2: partly holds.** Kind/own exclusion, accept (one step) and decline (no step) hold.
  Load clears the offer via `clear_for_new_document` → `Derived::none()` (inspection).
  It fails on a drag (MAJOR-1), on redo (MINOR-1) and on bit-equality (MINOR-3), and the
  op does not check the offer (NOTE-1).
- **C3: proposal and commit hold** (the rows plus mutant M8). Fails on MAJOR-2; see NOTE-2.
- **C4: holds.** `viewer-module-kinds.sh`, `viewer-vocab-declared-once.sh` and
  `work.py lint` pass. The backticked identifiers added to the 13 edited `work/` files
  resolve (apart from the glob `variable_*`), and open items cite no old name. Leftovers:
  MINOR-4, NOTE-3.
- **C5: holds.** Mutant M6 (the probe answers `NoSuchVariable` for a defined variable) is
  killed; `named_variable` reads `doc.var` (inspection).
- **C6: sound.** The rewritten `panel_edits` row pins `SetParam`, a new variable and the
  old one gone (M1 kills it). `DeclineOffer` with nothing on offer is a no-op, and
  declining at `b` leaves `a`'s offer open. The table entries match their siblings:
  `SetSlotVariable` is fenced like `SetSlot`, `DeclineOffer` permitted like `Select`.
- **C7: mostly holds.** I re-ran 8 mutations: M1 typing moves in place; M2 own variable not
  excluded; M3 no reads-check in `offered`; M4 kind filter dropped; M5 `close_offer` a
  no-op (also killed by the `keep_separate` app row); M6 probe says absent; M7
  `is_typed_value` filter dropped; M8 proposal ignores held names. Seven were killed;
  **M7 survives** (MINOR-2). I did not mutate against the `the_offer_…` app row.

**Runs** (private `CARGO_TARGET_DIR`; lavapipe installed; the GPU rows
`every_pass_builds_on_a_real_device` and `the_culled_passes_…` PASS):
- `cargo nextest run -p viewer --features app`: 1184/1185 passed, 7 skipped.
- Viewer default suite: 902/903 passed, 7 skipped.
- In both runs the single failure is `every_suite_file_is_aggregated`, which my own
  uncommitted probe file caused. I removed the probe afterwards. I did not see the
  "pre-existing failure on bare main" that the body mentions in the viewer suites.

Probes: `review/intent-gui-variables-probes.rs` and `…-app-probe.diff` (not built).

## Style

- **Q1, two keys for one lifetime.** `pane/properties.rs` `name_field_ui`, `drafts.rs`
  `name_draft`, and `session.rs` `SlotOffer`. The offer and the naming field are both
  "a transient affordance about the variable this slot reads". The offer stores the `var`
  and re-checks it. The name draft stores only `(node, slot)` and checks nothing. That
  asymmetry is how MAJOR-2 arises. Look for other `(RecipeNodeId, SlotId)`-keyed drafts:
  `expr_target` in `drafts.rs` has the same shape. **likely**
- **Q1, two ways to say an unnamed variable.** `props.rs` `variable_label` (~:1163) says
  "the distance of Extrude …" (VR2). Every other place in the viewer (variable rows,
  refusals, `slot_notes`) speaks an unnamed variable as its `SpokenVar` tag. So the offer
  buttons call a variable one thing and the rows call it another. The kernel's
  `Doc::spoken_var` is the existing home. **likely**
- **Q3, the offer-lifetime row cannot fail where it matters.**
  `gui_variables::an_offer_stands_only_while_the_slot_reads_what_was_typed` moves a
  *different* variable (`w`) and asserts that no offer appears. The failure that actually
  happens is moving the slot's *own* variable. The comment "offered nothing, since nobody
  typed that depth" sits on the wrong case. **sure**
- **Q3, the formula row's premise rules out its failure.**
  `typed_text_is_offered_and_a_formula_is_not` (see MINOR-2). **sure**
- **Q5, a promise the code does not keep.** `props.rs` `equal_variables` doc: "Equal is
  equal at the bits of the canonical value". The code uses `==` (MINOR-3). **sure**
- **Q7, a magic cap.** `props.rs` `proposed_name` has `.take(1000)`. It is not explained.
  Past 1000 the function returns `None`, and `name_button` then opens an *empty* field
  with no word on why. It is small, but it is a policy number with no home. **unsure**
- **Q7, the offer re-evaluates on every draw.** `props.rs` `equal_variables` builds
  `var_env` and evaluates every variable each time `offered()` is drawn. That only
  happens while an offer is open at that slot, so it is cheap in practice. I would still
  have computed the list once, when the offer opens. **unsure**
- **Q7, a no-op op with a table row.** `DeclineOffer` is a session op that touches no
  document. It is permitted in both gesture tables and counted as a status-line action,
  so a declined offer may wipe the status line. Clearing the draft on the pane side would
  not need a table row. I did not check the status-line effect by execution. **unsure**
- **Q4, citation rot.** `editor-core/src/edit.rs:1272`, `:3184` (MINOR-4). The prose
  rotted; the code is right. Also sweep `crates/*/src` for other viewer identifiers this
  PR renamed: I grepped only the item's list. **sure**
- **Q6, the stated bound and the redo question have no home.** The redo revival
  (MINOR-1) and "bit-equal" (MINOR-3) are each stated once, in the PR body, and are not
  guarded anywhere. **likely**
- **Q2: clean.** The rg for `by hand|kept in step|same rule as|mirror of` over the added
  lines of `crates/viewer/src` matched nothing. The pattern is blind to new phrasings.
  **sure**

Exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7. **Q8 only in part**: I read the offer, naming and
`props` sections of `props.rs` (~:890–1215) and `pane/properties.rs` (~:860–1003). I did
not read `session.rs` (the largest touched file) end to end.
