---
id: ratified-is-asserted-across-viewer-src-and-some-was-never-ratified
kind: issue
title: ratified is asserted across crates/viewer/src and some of it names no clause anywhere
status: open
opened: 2026-09-20
priority: P2
cost: D
---

**Read §The census (2026-09-24) first.** It is the population of
record, and it supersedes three things in the sections above it: the
35-line count, the claim that `platform.rs` and `theme.rs` are
unclaimed, and the FALSE reading of `Standing`'s GQ7 claim, which the
census finds PARTIAL.

Filed by VNEWS's frame.rs-cluster adjudication (2026-09-20), which
found one instance, proved it, and was told — correctly — that it had
swept for a phrase and stopped. **This row is the census. It does not
fix the population.**

## The instance that started it

`frame::frame_status`'s ranking was called *"the ratified ranking"* by
`work/vnews/rank-one-discards-the-frames-other-news` and *"rank 1's
ratified 'a refusal wins, alone'"* by
`work/vnews/one-line-one-subject-loses-a-mixed-frames-expiry`. It is
neither: the rule's only normative statement is a doc comment an agent
wrote on 2026-08-31, and an agent called it ratified on 2026-09-04.
That row's adjudication section holds the full search and is the
canonical home for the finding; this row is the class it is an
instance of.

## The sweep rule that produces the population

**Every `ratif*` occurrence in a doc comment, module header or code
comment under `crates/viewer/src`.** Case-insensitive, `.rs` files
only. Run at filing (2026-09-20) that was **35 lines in 17 files** —
re-derived by the census below, which is the population of record:

    session/refuse.rs 6   platform.rs 4   session/op.rs 3
    frame.rs 3   display.rs 3   widgets.rs 2   tree.rs 2
    session/select.rs 2   blend.rs 2   theme.rs 1   props.rs 1
    pane/properties.rs 1   lib.rs 1   g1.rs 1   camera.rs 1
    bounds.rs 1   app.rs 1

**Re-derive it; do not quote 35.** The orchestrator's own pass over
the same ground reported **32**, and the gap is the enumeration rule
rather than the tree: this rule counts LINES matching, including the
three `platform.rs` hits where the verb has a different subject
(`:16` *"`scripts/gates/no-ambient-env.sh` ratifies that…"*, `:170`
and `:258` *"CONTRACT-RATIFIED holds vacuously"*), which a rule
counting *claims that a DESIGN DECISION is ratified* excludes. Both
rules are defensible and they give different numbers, which is the
point: a count carries its rule.

The instrument's blind spot, stated because the last one was not: a
grep for `ratif*` cannot see a sentence that asserts ratification
without the word — *"the GUI plan's rulings"*, *"settled"*, *"D5
decides"*. `session/select.rs:166` is inside the population only
because it happens to spell it *"by ratification"*.

## The test that sorts a true claim from a false one

`CLAUDE.md`, in terms: *text is not ratified by sounding official or by
sitting in a file whose companion-table row says Ratified* — **find
the clause, then find the commit that wrote it.** Applied here:

- **TRUE** when the sentence names, or can be traced to, a clause in a
  document `docs/DESIGN.md`'s companion table marks *Ratified*, or an
  Ev ruling recorded in the tracker.
- **FALSE** when the only backing is `crates/viewer/README.md` (the
  implementation record the companion row says *"the program maintains
  itself"*), another doc comment, or nothing.

**The class is mixed, which is why it is a census and not a sweep.**
Three worked examples, all in one file:

| site | claim | verdict |
|---|---|---|
| `crates/viewer/src/frame.rs:565` | *"the ratified expression-driven affordance"* | **TRUE** — `crates/viewer/GUI-DESIGN.md:93-94`, G4: *"Dragging an expression-driven dimension refuses, with an affordance offering to edit the expression"* |
| `crates/viewer/src/frame.rs:1191-1192` | *"the ratified argument the checks badge carries"* | **FALSE so far** — `checks badge` appears 0 times in `crates/viewer/GUI-DESIGN.md` and 0 times in `docs/DESIGN.md` |
| `crates/viewer/src/frame.rs:2040` | *"The ratified pattern is refuse-then-offer"* | **FALSE so far** — `refuse-then-offer` appears 0 times in either, and has already spread to four sites: `crates/viewer/src/frame.rs:2040`, `crates/viewer/src/app.rs:1171`, `crates/viewer/src/pane/properties.rs:237`, `crates/viewer/tests/frame_policy.rs:2620` |

*"so far"* is deliberate: a claim is FALSE only once the search for its
clause has been run and reported, and this row has run it for three of
thirty-five.

## The tracker half of the population

The same assertion travels in item files, where it decides whether a
row believes it needs Ev. **`work/vnews/a-tree-rows-message-line-picks-its-affordance-by-hand:26`**
calls `frame::Affordance::{Read, Opens}` *"the ratified value"*; its
only documentation is `crates/viewer/README.md` and `frame.rs`'s own
doc comments, which is the implementation record this adjudication
argues is not a gate. It was opened **2026-09-20**, the same day as the
adjudication that found the class — so the class is still minting
members, which is the argument for the census going before any of the
fixes.

## What this row delivers

The population, each member's verdict with the clause or the failed
search beside it, and the correction of whichever members come back
FALSE. **Not** a rule that `ratified` may not be written: the word is
right thirty-odd times here and the census is what tells them apart.

`crates/viewer/README.md` and `crates/viewer/tests/*` members are
VDOC's by `work/vnews/program.md`'s `keep_out` and are filed there, not
fixed here. `platform.rs`, `theme.rs` and `blend.rs` are among the
eleven files `work/view/viewer-src-files-no-successor-claims` reports
as claimed by no dispatching program.

## A tracker-side member, and it is the orchestrator's — 2026-09-20

`crates/viewer/src/session/select.rs`'s `Standing` doc calls the clause
*"the affordances that need a live entity switch off"* **GQ7's recorded
constraint**. Verified by grep over all three candidate documents:

- `crates/viewer/GUI-DESIGN.md` — **0 occurrences**, and its GQ7 is
  selection mechanics (single-select, pick priority, `PickKinds`),
  which defers vanishing-entity semantics elsewhere;
- `docs/SELECT-DESIGN.md` — **0 occurrences**;
- `crates/viewer/src/session/select.rs` — **1**, the assertion itself.

So this is the same shape as `frame.rs`'s *"ratified"* ranking: a source
comment naming a ratified home that does not contain the rule. It is
listed here as a source-side member.

**What makes it worth recording separately is where it went.** The VNEWS
orchestrator read that comment, did not check `GUI-DESIGN.md`, and wrote
*"ratified design in `crates/viewer/GUI-DESIGN.md`, so the shape I
recommended would have needed an `[ev]` PR"* into a dispatch, into
`work/vnews/plan.md` §Dispatch rules, and into the merged body of
PR #2942 — **two hours after ruling that the status line's ratification
was an agent's word nobody had checked, and while this row was being
filed.** The properties-pane lane caught it.

That is the class's real cost, and it is not that a comment is wrong.
**An unchecked ratification claim does not stay in the comment**: it is
believed by the next reader, who writes it somewhere more binding, and a
plan or a merged PR body is harder to correct than a doc line. The
sorting test this row states — find the clause, then the commit — has to
run at **the document that would carry the gate**, never at the comment
that cites one.

Corrected at `work/vnews/plan.md` §Dispatch rules, which now carries the
retraction rather than a quiet rewrite.

## The census (2026-09-24, merge base `f4b178189`)

Verdicts only, no corrections: those are a later unit. Every verdict
below is a claim, and its evidence sits beside it. **TRUE** means both
halves of the sorting test turned up: a clause that decides what the
sentence says, plus the commit or log entry that records Ev agreeing
to it. **FALSE** means the only backing is the README, another doc
comment, an agent's decision, or nothing. **PARTIAL** means part of the
sentence is backed and part is not, and the entry says which. **OUT**
means the line matches the rule but asserts no ratification.

### How the evidence was taken, and one caveat on it

- **The commit author is no evidence.** Commits made on Ev's own box
  carry his git identity whether or not he wrote the text. The
  evidence taken is a dated attribution that the text wrote down at
  the time (*"agreed 2026-07-19"*, *"(Evan, 2026-08-30)"*,
  *"RULED (Evan on #217, 2026-08-06)"*), or a log entry quoting Ev.
- **Where a clause sits in `crates/viewer/GUI-DESIGN.md` does not make
  it Ev's.** PR #2462 (`e9824abf3`, 2026-09-12, not an `[ev]` PR)
  moved agent-written README paragraphs into that file. The pick-index,
  fit-seam, `app`-feature and wasm paragraphs, and *"GUI-3's §5
  ratification"*, arrived that way. Ev did approve the move itself: the
  PR body opens *"Ev approved this shape in chat (2026-09-12)"*. What
  he approved was where to cut one file into two. No paragraph was
  rewritten, and approving the cut does not ratify each paragraph that
  crossed it. So every verdict below traces its
  clause to the commit that first wrote it, never to the file it sits
  in now. Filed as `work/issues/gui-design-holds-agent-text-that-reads-as-ratified`.
- The G-clause sources: G1, G2 and the three micro-decisions (now G4)
  are `5267a9193` (2026-07-19, *"G1 (agreed 2026-07-19)"*,
  *"Ratified micro-decisions (2026-07-19)"*). GQ2 is `57d762ef1`
  (*"ratify GQ2/GQ3"*). G3 is `d972a1b36` (*"G3 (Evan, 2026-08-03)"*).
  G5 is `df8cc2787` (*"Colour … (Evan, 2026-08-30)"*). The rulings
  `docs/GUI-PLAN.md` carries (*"Rulings (Evan, 2026-08-27,
  in-conversation)"*) are `a800a4bc3` and `70977813f`. That file is
  deleted and readable at `70977813f`.
- **G4 is a clause about a DRAG.** Ev's text reads *"Dragging an
  expression-driven dimension → refuse, with an affordance
  ("driven by `width/2 − margin` — edit the expression?"). …
  explicitly cheap to replace"*. The example sentence was dropped by
  `585b3422f` (an agent, 2026-09-03). The shipped refusal
  (`crates/viewer/src/session.rs` `guard_driven`) also fires on a typed
  `set_slot`, on a profile restructure and on `probe_bounds`. So a
  sentence that cites G4 for a typed write or a probe is **PARTIAL**,
  and so is a sentence that cites it for the WORDING. The parenthesised
  example illustrates the affordance and does not make the words part
  of the decision.

### Population 1: `ratif*` in `crates/viewer/src` — the row's rule

**Rule:** `grep -rniI ratif --include=*.rs crates/viewer/src`, counting
LINES. Every hit is inside a `//`, `///` or `//!` comment: I checked
for a hit outside one and found none. The rule cannot see the word
wrapped across two lines (I grepped for a line ending in `rati-` and
found none).

| member | claim | verdict | evidence |
|---|---|---|---|
| `readout.rs` `EPS_CAP` doc (:98) | ε's default is ratified | TRUE | `docs/DESIGN.md` D4 ¶1 *"ε ≈ 1e-9 m"*, `9f0bb4d91` *"ratify D4/D6"* |
| `g1.rs` module docs (:5) | G1 ratifies preview/commit | TRUE | G1 *"Preview versus commit is structural"*, `5267a9193` *"ratified in-conversation 2026-07-19"* |
| `blend.rs` module docs (:36) | fillet freeze is ratified | TRUE | `editor_core` `Node::Fillet` *"The selection FREEZES (ruled, #217)"*; M7-LOG `acc2e0669`: *"RULED (Evan on #217, 2026-08-06): … freeze semantics made explicit and accepted"* |
| `blend.rs` `FREEZE_NOTE` (:74) | ratified #217 semantics | TRUE | the same |
| `frame.rs` `acts` (:571) | the expression-driven affordance is ratified | TRUE | G4, `5267a9193`. The row's worked example (at `:565` when filed) holds |
| `frame.rs` `Affordance::Opens` (:1271) | *"the ratified argument the checks badge carries"* | **FALSE** | `checks badge` occurs 0 times in `GUI-DESIGN.md` and `docs/DESIGN.md`; `findings window\|opens a window\|window listing` occurs 0 times in `docs/`, `memories/`, `work/`; the only backing is `crates/viewer/README.md:944-946`. Origin `bce8486e9` (2026-09-01, *"GUI fixes: … a findings window"*) and `4db112ada`. No log entry records a ruling (checked the GUI log at `a1425f92f^:work/gui/log.md`) |
| `frame.rs` `creation_offer` (:2177) | *"The ratified pattern is refuse-then-offer"* | **PARTIAL** | Refuse-with-an-offer is ratified for ONE case: G4's drag, where the offer is to edit the expression. The parse door's unknown-name refusal offering to CREATE a parameter is `02befd6a9` (agent, 2026-08-28). The GUI log calls it *"the ratified refuse-then-offer pattern"* (PR #1129 entry, *"Ev-requested"*), which is an agent's word for a generalisation. `refuse-then-offer` occurs 0 times in any design doc. The row's *"FALSE so far"* is corrected to PARTIAL |
| `pane/profile.rs` `tests::drawing_a_locked_split_circle_above_the_cap_leaves_it_alone` (:502) | D4's micron-to-kilometre coverage | TRUE | D4 ¶1, `9f0bb4d91` |
| the same test (:527) | K = 10 is ratified | TRUE | `docs/DESIGN.md` *"K = 10 is the permanent ratified default (#89 CLOSED)"*, `184f0f813` *"Evan's #169 rulings: #89 CLOSED (K=10 permanent"* |
| `session/refuse.rs` `Refusal::DrivenByExpression` (:146) | a direct numeric edit is refused, citing the ratified affordance | **PARTIAL** | G4 decides the drag. A typed `set_slot` is not a drag, and the G4 clause does not reach it |
| `session/refuse.rs` `Refusal::rank` (:382) | `BeginGesture` is refused with the ratified affordance | TRUE | G4: this one is a drag |
| `session/refuse.rs` `Refusal::rank` (:388) | the affordance is a ratified decision | TRUE | G4. The RANK built on it is the doc's own argument and is not claimed to be ratified |
| `session/refuse.rs` `Refusal::affordance` (:475) | *"The ratified affordance sentence"* | **PARTIAL** | The behaviour is G4's. The sentence is not decided anywhere (see the note on G4 above) |
| `session/refuse.rs` `Refusal::affordance` (:478) | *"a ratified micro-decision whose WORDING is part of the decision"* | **PARTIAL** | The same. The quoted clause is Ev's. *"whose WORDING is part of the decision"* is not |
| `session/refuse.rs` `Refusal`'s `Display::fmt` (:532) | the affordance arm's wording is a RATIFIED decision | **PARTIAL** | The same |
| `app.rs` `ViewerApp::perform_batch` (:1099) | dragging shows the ratified affordance | TRUE | G4: a drag |
| `tree.rs` module docs (:5) | GQ2's ratified codomain | TRUE | GQ2, `57d762ef1` |
| `tree.rs` module docs (:6) | the ratified error rule | TRUE | micro-decision 2 (G4), `5267a9193`: *"failures are typed values … never exceptions or strings"* |
| `platform.rs` module docs (:16) | `no-ambient-env.sh` *"ratifies"* that the environment reads have ONE home | **FALSE** | The viewer's allowlist entry was agent-added in PR #1125, *"flagged for Ev's retroactive glance"* (GUI log, post-close maintenance, 2026-08-28). I found no follow-up there or in the VIEW log. `scripts/gates/README.md` (Ratified, PR 2067) never mentions the viewer entry |
| `platform.rs` `prefs_path` (:170) | *"CONTRACT-RATIFIED holds vacuously"* | OUT | This is row 1 of the gate's four-row test (`no-ambient-env.sh:25`). The sentence says the row holds vacuously and claims no ratification |
| `platform.rs` `running_under_wsl` (:239) | the gate's allowlist entry ratifies one home | **FALSE** | as `:16` |
| `platform.rs` `launch_dir` (:258) | CONTRACT-RATIFIED holds vacuously | OUT | as `:170` |
| `widgets.rs` `number_text` (:292) | *"that band is the ratified render accuracy"* | **PARTIAL** | ε is ratified (D4). The band is `readout::tolerance`: a cap one decade below ε, met with a relative arm. `readout.rs` itself calls that *"legibility choices"* and *"a display choice stated once against the ratified default"* |
| `widgets.rs` `drag_ops` (:574) | G1 ratifies the shape | TRUE | G1, `5267a9193` |
| `widgets.rs` `drag_ops` (:585) | two spellings of a ratified rule | TRUE | the same |
| `lib.rs` module docs (:50) | δ-not-ε is a ratified micro-decision | TRUE | micro-decision 3 (G4), `5267a9193` |
| `session/select.rs` `Selection` (:166) | *"Single-select, by ratification (the GUI plan's rulings)"* | TRUE | GUI-PLAN Rulings: *"Selection: single-select for v1"* (`a800a4bc3`) |
| `session/select.rs` `Standing` (:268) | *"the ratified resolution-failure semantics"* | TRUE | `crates/editor-core/src/names/README.md` N5 *"Typed resolution failure"*; `docs/SELECT-DESIGN.md` §4 (Ev-ratified, `d3f4517f1`); GUI-PLAN GUI-2: *"Tools survive a selected ref vanishing (the ratified resolution-failure semantics)"* |
| `bounds.rs` module docs (:69) | the ratified terminal sliver | TRUE | `docs/ERROR-DESIGN.md`: *"Terminal sliver … → refuse, never refine"* (`28b174f6e`; ratified PR #110) |
| `camera.rs` `Camera::ray_through` (:837) | the ratified interval-square rule | TRUE | `scripts/gates/README.md` (Ratified, PR 2067, `4b3ed4478`) keeps `interval-square-allowlist.sh` as a grep. The rule's earlier home was a memory, retired by `4ffcde545` with *"the CI step is now the rule's home"* |
| `theme.rs` module docs (:13) | colour precedence is *"ratified in `crates/viewer/README.md`"* | **PARTIAL** | The precedence IS ratified: G5, `df8cc2787`, *"(Evan, 2026-08-30)"*. The home cited is wrong: the README is the implementation record. The pointer was right when written: `df8cc2787` cited `docs/GUI-DESIGN.md`, and `585b3422f` repointed it to the README when the design moved there. It went stale when PR #2462 moved G5 back out to `GUI-DESIGN.md` and left the pointer behind, a second casualty of that move |
| `display.rs` module docs (:14) | free-move is never persisted — *"G3's ratified boundary"* | **PARTIAL** | G3 said so (`d972a1b36`). DI5 (`crates/editor-core/IDENTITY.md`, ratified in chat 2026-09-04, `087779036`) reverses it: *"The viewer may record a free-moved placement persistently"*. The tree still matches G3 because DI5 is unbuilt (`work/vseam/no-persistent-setplacement-session-op`), but the sentence presents a reversed decision as the standing one |
| `display.rs` `AdmissionFault::NoSuchNode` (:167) | `Standing`'s *"ratified rule"* is that the vanished reference is rendered *"while the affordances that need a live entity switch off"* | **PARTIAL** | Survival is ratified (GUI-PLAN GUI-2 and SELECT-DESIGN §4, as at `select.rs:268`). The switch-off clause, which is the half this sentence rests on, is in no design doc. Its only statement is `Standing`'s own doc |
| `display.rs` `DisplayState::free_move` (:734) | *"the ratified change (DI5)"* | TRUE | DI5, `087779036` |
| `props.rs` module docs (:77) | setting a number into a driven slot is refused, *"the ratified micro-decision"* | **PARTIAL** | as `refuse.rs:146`: the drag half holds, the typed-write half does not |
| `pane/properties.rs` `slot_notes` (:939) | *"Its wording is the ratified one"* | **PARTIAL** | as `refuse.rs:478`, and a note shown BEFORE any refusal is not a refusal of a drag either |
| `session/op.rs` `SessionOp::BeginParamGesture` (:220) | the ratified preview-vs-commit decision | TRUE | G1 |
| `session/op.rs` `SessionOp::AddFillet` (:670) | ratified #217 semantics | TRUE | as `blend.rs:36` |
| `session/op.rs` `SessionOp::permitted_during_value_gesture` (:1070) | DI5 (ratified) rules that release emits `SetPlacement` and `moves` empties | TRUE | DI5, `087779036`, which says both, word for word |

### Population 2: authority asserted WITHOUT the word — the blind spot, exercised

The row names the blind spot: *"settled"*, *"the GUI plan's rulings"*,
*"D5 decides"*. Two sweeps aimed at it:

- **Sweep 2a, authority phrasings:**
  `grep -rnI -E "\(Ev,|\bEv('s)? (ruled|ruling|decided|authoris|asked|agreed)|\bruling\b|\bruled\b|\brulings\b|\bsettled\b|recorded constraint|design question|\bsign-?off\b|\bagreed\b|open clause" --include=*.rs crates/viewer/src`.
  I drop lines that already match `ratif`, then drop
  `grep -v -E "datums.rs|ruled (out|at|over|along|completely|on the frame|toward)|is ruled out|plane's ruling|the ruling (stops|reaches|runs|always|of part|is dense)|keeps the ruling|a ruling's|whose ruling lost|ruling legible|app.rs:96[56]"`.
  Those are the grid-line senses of "ruling" and `app.rs`'s
  `settled` variable. That leaves 48 lines. Fifteen of those use the word in its
  ordinary sense: `pickcache.rs:252,307`, `marks.rs:79,82,308,309`,
  `theme.rs:634`, `frame.rs:163,2086`, `session/op.rs:321`,
  `session/probe.rs:167`, `widgets.rs:1533`, `gpu.rs:68,1544` and
  `drafts.rs:1103`. `frame.rs:163` and `op.rs:321` do name design
  questions, but tracked ones, and neither claims a ratification.
- **Sweep 2b, clause ids used as authority:**
  `grep -rnI -E "\b(G[1-5]|GQ[1-7]|D[1-9]|DI[1-5]|DM[1-8]|A[1-9][0-9]?|N[1-7]|W[1-9]|E[1-9][0-9]?|L[1-8]|S[1-4]|V[1-8]|DS[1-9])('s)?\b" --include=*.rs crates/viewer/src | grep -E '^[^:]+:[0-9]+:\s*//' | grep -vi ratif`.
  That gives 118 lines. The `E[1-9][0-9]?` arm is deliberate: it
  matches ERROR-DESIGN's E1–E12 but not rustc error codes. A looser
  `[A-Z]+[0-9]+` form gives 133, and the extra 15 are all `E0…` rustc
  codes. I did **not** verdict the 118. I spot-checked four: *"G3's honesty
  requirement"* matches `d972a1b36` (*"an honesty requirement, not a
  solver one"*); A11 exists in `crates/editor-core/ASSEMBLY.md`;
  `pickindex.rs` `PickKinds`'s *"GQ7's open clause"* is below; and
  `matetool.rs:60` is below. Sweep 2b is what found `matetool.rs:60`,
  which sweep 2a misses because its *"recorded constraint"* breaks
  across two lines.

| member | claim | verdict | evidence |
|---|---|---|---|
| `frame.rs` module docs (:64) | the channel is decided by provenance, (Ev, 2026-09-06) | TRUE | VIEW log (`9e26fc281^:work/view/log.md`), *"Ev ruled the rule over its own example (2026-09-06)"*: *"your recommendation seems fine here"* |
| `frame.rs` `unindexed_refusal` (:1516) | Ruled (Ev, 2026-09-06) | TRUE | the same |
| `frame.rs` `prefs_badge` (:2085) | the argument Ev ruled on for the absent chooser | TRUE | VIEW log 2026-09-10: *"Ev ruled (c) on #2275 — '(c) is right!'"* |
| `app.rs` `ViewerApp::remember_prefs` body (:1230) | the same ruling | TRUE | the same |
| `platform.rs` `NO_CHOOSER_BACKEND` (:287) | *"Ev's ruling live[s] in `crates/viewer/README.md`"* | TRUE | The ruling is real (above). The README records it at `:877-878`; its primary record is the log |
| `pickcache.rs` `NotIndexed::AnotherPicture` (:546) | refusing is a ruling (Ev, 2026-09-15) | TRUE | VIEW log 2026-09-15: *"Ev ruled the product question on 2026-09-15: refuse"* |
| `pane/viewport.rs` `drawn_index` (:149) | the gate is *"a product ruling"* | TRUE | the same |
| `pane/viewport.rs` `drawn_index` (:151) | Ev ruled on 2026-09-15 | TRUE | the same |
| `pane/viewport.rs` `ViewerBehavior::viewport_ui` body (:502) | Ev's 2026-09-15 ruling | TRUE | the same |
| `pane/viewport.rs` `tests::a_click_over_a_picture_the_index_did_not_draw_is_refused_typed` (:1395) | Ev's ruling, 2026-09-15 | TRUE | the same |
| `evalseam.rs` `crashed` (:897) | a crashed worker ends the process (Ev, in-chat, 2026-09-17) | TRUE | `ee4913dd0` (VIEW log) quotes *"panic on crash is good"* |
| `combine.rs` module docs (:8) | single-select stays ruled | TRUE | GUI-PLAN Rulings |
| `blend.rs` module docs (:13) | the same | TRUE | the same |
| `revolvetool.rs` module docs (:8) | the same | TRUE | the same |
| `matetool.rs` module docs (:2) | the mate tool is the plan's ruled addition to G3 | TRUE | GUI-PLAN: *"Mate authoring is v1 scope … ruled in"* |
| `matetool.rs` module docs (:9) | two sequential picks, per the ruling that closed OQ-a | TRUE | GUI-PLAN: *"two sequential picks in tool state (GUI-4) — ruled, closing the round-2 OQ-a"* |
| `matetool.rs` `MateToolState` (:364) | the two sequential picks of the ruling | TRUE | the same |
| `app.rs` `ViewerApp::ui` body (:1916) | the two-sequential-picks ruling | TRUE | the same |
| `props.rs` module docs (:10) | v1's ruling was canonical units | TRUE | GUI-PLAN *"Units: canonical meters/radians"*, marked superseded at Ev's request by `2cd5a267d`; the doc says what changed from it |
| `matetool.rs` module docs (:60) | tools survive a vanished pick, *"GQ7's recorded constraint, the GUI-2 semantics"* | TRUE | GQ7 recorded *"tools must survive the referenced entity vanishing under them"* (`5267a9193`), but as an OPEN question: that file puts GQ7 under `## Open questions`. The survival half was ratified later, by `docs/SELECT-DESIGN.md` §4 (`d3f4517f1`, 2026-08-09) and GUI-PLAN GUI-2 (2026-08-27). The comment's *"the GUI-2 semantics"* names the ratification |
| `pickindex.rs` `PickKinds` (:164) | filter presentation is GQ7's open clause | TRUE | `docs/SELECT-DESIGN.md` §4 keeps it *"still deferred to sketcher/tree design time"* |
| `session/select.rs` `Standing` (:274) | vanished-is-a-state and switch-off is *"the whole of GQ7's recorded constraint (tools survive the referenced entity vanishing)"* | **PARTIAL** | The parenthesis is GQ7's own text (`5267a9193`), recorded there as an open question. The survival half was ratified by `docs/SELECT-DESIGN.md` §4 (`d3f4517f1`, 2026-08-09, which decided the re-homing) and GUI-PLAN GUI-2 (2026-08-27). `4eda8abec` (2026-08-28) only deleted the clause from GUI-DESIGN. The switch-off clause is in no design doc. This agrees with the orchestrator's 2026-09-20 retraction, which said the switch-off clause is not GQ7's ratified constraint. The census adds only that the survival half is ratified, so the sentence is PARTIAL rather than FALSE |
| `session/probe.rs` `evaluate_with` (:269) | *"the seam's ruled cancel-and-restart policy"* | **PARTIAL** | GUI-PLAN rules *"busy indicator + the shipped `CancelToken`"*. RESTART-on-a-new-edit is the implementation (`a9a660b83`), and `cancel-and-restart` occurs in no ruling |
| `platform.rs` `prefs_path` (:162) | *"the ruling in `scripts/gates/no-ambient-env.sh`"* | **FALSE** | as `platform.rs:16` |
| `prefs.rs` `FileStore::new` (:532) | *"the one-door ruling"* in that gate | **FALSE** | the same |
| `pickindex.rs` comment on its `editor_core::resolve` import (:81) | a direct edge, never a new re-export, is *"the ruling `pncad`'s own crate docs state"* | **FALSE** | The only statement is `crates/pncad/src/lib.rs`'s crate docs, `079632988` (agent, 2026-08-20). `docs/LIBRARY-DESIGN.md` U1 ratifies payload reachability, not this rule. `git log -S'direct edge' -- work/ docs/` returns nothing |
| `theme.rs` module docs (:31) | the same ruling | **FALSE** | the same |
| `session/refuse.rs` comment on its `UNDECLARED_PARAM_RECOURSE` import (:25) | the same ruling | **FALSE** | the same |
| `session/op.rs` `SessionOp::NewDocument` (:423) | *"The identity ruling (logged in `docs/GAUTH-LOG.md`)"* | **FALSE** | GAUTH log (`a1425f92f^:work/gauth/log.md`) lists it under *"Unilateral decisions at opening (Ev reviews retroactively)"*, and records no review. The GAUTH plan calls it the orchestrator's and says *"nothing here ratifies an open design question"* |
| `session/refuse.rs` `Refusal::EmptyName` (:221) | the identity ruling | **FALSE** | the same |
| `session.rs` `DocSession::new_document` (:1937) | the identity ruling | **FALSE** | the same |
| `forms.rs` `PatternKindChoice` (:52) | `Explicit` is absent *"by the plan's ruling"* | **FALSE** | The GAUTH plan's unit-4 spec (`58500afb1`, agent) says *"`Explicit` is not a form's job"*. Ev ruled that plan's SCOPE (Phase A/B) and not this line. GROUP-BOOLEAN's ratified A′ (`7f174f688`) includes `Explicit` |
| `session/author.rs` `PatternRuleSpec` (:220) | the same | **FALSE** | the same |
| `frame.rs` `frame_status` (:664) | the per-subject line is *"a design question for Ev"* | **FALSE** | already the subject of `work/vnews/frame-rs-says-the-per-subject-line-is-a-question-for-ev` |

### Population 3: the tracker half — `ratif*` in `work/vnews/*.md`

**Rule:** `grep -niI ratif work/vnews/*.md`, excluding this row. That
gives 47 lines. Some lines make no claim that a decision is ratified,
and are OUT:

- row ids and `refs:` (`a-disabled-control-…:7,:319`,
  `hand-maintained-counts-…:85`, `log.md:155`, `plan.md:105,209,337`);
- meta-prose about the class (`frame-rs-says-…:44-46`);
- retractions and refutations already argued in their own rows
  (`rank-one-discards-…:30,76,86,93,119,143,145-147`,
  `one-line-one-subject-…:55,105-106`, `a-disabled-control-…:112-113`,
  `plan.md:175,176,188,525,536,543`);
- a process step (`log.md:19`);
- the count at `plan.md:202`. That count is stale: today's rule gives
  the population in the table above, not thirty-five.

The claims:

| member | claim | verdict | evidence |
|---|---|---|---|
| `a-tree-rows-message-line-picks-its-affordance-by-hand:26` | `frame::Affordance::{Read, Opens}` is *"the ratified value"* | **FALSE** | backed by the README and `frame.rs:1271`, which is itself FALSE |
| `outstanding-and-progress-…:46` | a bare `bool` *"which the README ratifies"* | **FALSE** | the README is the implementation record. The same row's §No Ev gate (`:98-104`) already says so, and this sentence contradicts it |
| `outstanding-and-progress-…:103` | the README sentence is the only ratification cited; no Ev gate | TRUE | Right in substance. Note that `GUI-DESIGN.md:229-231` does mention `frame::progress`, but that paragraph is agent text (`2622d14fa`, moved by `e9824abf3`), so the row's conclusion stands |
| `is-instance-collapses-absent-and-wrong-kind:110` (closed) | `Standing`'s ratified rule: a vanished reference is a state | TRUE | GUI-PLAN GUI-2 survival, as at `select.rs:268` |
| `a-disabled-control-says-why-in-four-shapes:314` (closed) | `probe_bounds` refuses with *"the ratified affordance"* | **PARTIAL** | G4 is a drag, and a range probe is not one |
| `the-range-button-re-mints-the-ratified-affordance` (id, `:2`) | the button re-mints *the ratified affordance* | **PARTIAL** | The behaviour is ratified for a drag. The sentence is not ratified, and the button is a probe |
| `the-range-button-…:15` | the home is *"documented as ratified"* | TRUE | accurately hedged: it is documented so, at `refuse.rs:475` |
| `the-range-button-…:43` | quotes `refuse.rs:478` | **PARTIAL** | inherits that verdict |
| `the-range-button-…:52` | the row shows *"the ratified affordance"* | **PARTIAL** | as `:2` |
| `log.md:150` | *"a third spelling of the ratified affordance"* | **PARTIAL** | as `the-range-button-…` |
| `log.md:329` | *"`Standing`'s second clause is GQ7's ratified constraint"* | **FALSE** | the switch-off clause (see `select.rs:274`). The orchestrator's 2026-09-20 retraction corrected the plan, not this log entry |
| `program.md:12` (`keep_out`) | *"the GUI-3 section 5 seam and GQ7 are ratified design in crates/viewer/GUI-DESIGN.md and a revision is an ev PR"* | **PARTIAL** | Ratified: GQ7's single-select (GUI-PLAN) and GQ6's toolkit row (`dc5f15444`, 2026-08-16). Not ratified: GQ7's pick-priority text, which is agent-recorded (`f3411bbf1`, *"Nothing here widens GQ7"*), and *"the §5 seam-friction re-take is GO"*, which is an orchestrator's reading at GUI-3's merge. SELECT-DESIGN §4 (ratified) leaves the slimmed GQ7 *"still deferred"* |
| `plan.md:204` | `frame.rs`'s checks-badge line is unratified | TRUE | as `frame.rs:1271` (the line number has since moved) |
| `plan.md:205` | *"refuse-then-offer"* is unratified | **PARTIAL** | as `frame.rs:2177`: G4's drag half is ratified |
| `plan.md:207` | `frame.rs`'s expression-driven affordance is genuine | TRUE | G4 (now `frame.rs:571`) |

The dispatch named a fourth instance, *"itself ratified reasoning"*
about `slot_unit_ui`'s one-picker rule. It is no longer live: it was
struck in the row that minted it by `b62e678bc` (2026-09-21), and
`grep -rn "ratified reasoning" work/` now returns nothing.

### Population 4: VDOC's ground — `ratif*` in `crates/viewer/tests` and the README

This falls outside the row's rule, but is swept because the dispatch
named it and the class spreads into tests (`frame_policy.rs` is one of
the row's four refuse-then-offer sites).
`grep -rniI ratif crates/viewer/tests` gives 12 lines:

- TRUE: `blend_authoring.rs:489,1168` (#217), `edge_pick.rs:742` (N5 and
  GUI-2), `frame_policy.rs:60` (G4), `tree_badges.rs:124` (GQ2),
  `panel_edits.rs:7` (G4, scoped by *"where"*), `panel_edits.rs:303`
  (G1) and `panel_edits.rs:388` (G4, a drag).
- **PARTIAL**: `story_parametric.rs:11` and `:391` (G4 cited for a
  numeric write), and `panel_edits.rs:423`, an assertion message
  saying *"renders the ratified wording"*.
- **FALSE**: `frame_policy.rs:1162`, *"The checks badge is a BUTTON,
  and that is ratified"* (as `frame.rs:1271`).

`grep -niI ratif crates/viewer/README.md` gives 15 lines, and they are
**not verdicted here**. Most use *"ratified list/kind"* as the README
gate's own vocabulary, which is a different sense of the word. They
are handed to VDOC with the rule.

### The population, its rule, and what the rule cannot see

**Rule 1**, for the row's population, counts lines: every `ratif*` in
a `.rs` file under `crates/viewer/src`. Re-run at this merge base it
gives the first table. The filing counted 35 lines in 17 files. The 4
extra lines: `readout.rs` gained 1, `pane/profile.rs` gained 2, and
`widgets.rs:292` was added by `8e904637c` (2026-09-21). `frame.rs`'s
line numbers also moved. The difference is the tree, not the
rule. Rule 1 cannot see a ratification asserted without the word.
**Rules 2a and 2b** were run at that gap. Every hit of 2a has a
disposition. 2b is counted, spot-checked and **not verdicted**: 118
clause-id citations remain the unexercised part of this census. The
review sampled 20 of them and got about 15 TRUE, 2 PARTIAL, 1 FALSE
and 2 OUT. On that rate, roughly 12 to 25 of the 118 would come back
FALSE or PARTIAL. That is an estimate, not a verdict. They
are a class of their own, *"X's rule"*, and the rule for re-deriving
them is stated above. A citation that names neither a clause nor
Ev — *"by design"*, *"as decided"* — is matched by neither sweep. That
gap cannot be searched short of reading every comment. **Rule 3** is
the tracker half, confined to `work/vnews`. It does not reach the
sibling slates (`work/vseam`, `work/vgeom`, `work/vdoc`, `work/chrome`),
which also cite viewer ratifications.

**Totals, re-derived from the tables** by counting each table's
verdict column (population 1 / 2 / 3):

| verdict | P1 | P2 | P3 |
|---|---|---|---|
| TRUE | 23 | 21 | 5 |
| PARTIAL | 11 | 2 | 7 |
| FALSE | 3 | 11 | 3 |
| OUT | 2 | — | — |

### Where the out-of-fence members were filed

VNEWS claims these files (per `python3 scripts/work.py territory`):
`frame.rs`, `display.rs`, `platform.rs`, `prefs.rs`, `theme.rs`,
`tree.rs`, `session/refuse.rs`, `pane/properties.rs` and
`pane/profile.rs`. **This is a correction to the dispatch and to this
row's own last paragraph above**: `platform.rs` and `theme.rs` have
been VNEWS's since the 2026-09-21 residue claim in `program.md`'s
`keep_out`, and `blend.rs` belongs to AUTHOR, CHROME and VSEAM. So
every Rule-1 FALSE member (`frame.rs:1271`, `platform.rs:16,239`) is
inside this fence. The members outside it that need a correction are
filed one row per owning slate:

- **VSEAM**: `work/vseam/ratification-claims-in-session-docs-name-no-ruling`,
  holding the FALSE members `session/op.rs:423`, `session.rs:1937`,
  `forms.rs:52`, `session/author.rs:220` and `pickindex.rs:81`, and
  the PARTIAL members `session/select.rs:274` and
  `session/probe.rs:269`.
- **VGEOM**: `work/vgeom/two-viewer-docs-cite-a-drag-clause-for-a-typed-value`,
  holding `widgets.rs:292` and `props.rs:77` (both PARTIAL).
- **VDOC**: `work/vdoc/viewer-tests-assert-ratifications-that-are-not`,
  holding `frame_policy.rs:1162` (FALSE), `story_parametric.rs:11,391`
  and `panel_edits.rs:423` (PARTIAL), and the README's 15 unverdicted
  lines.
- **GUARD**: `work/guard/gates-readme-cites-a-deleted-memory`, a
  side-finding. `scripts/gates/README.md:111` cites
  `memories/interval-square-poison.md`, which `4ffcde545` deleted.
- **No owner**: `work/issues/gui-design-holds-agent-text-that-reads-as-ratified`.
