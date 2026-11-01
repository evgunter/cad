# SPACES log

Newest entries at the bottom; the tail is the program's live status.

- 2026-11-01 — Opened by CHROME's cut (Ev, in chat: three tracks, each
  with a full slate of P0s and P1s). Seven rows moved from
  `work/chrome/` by `git mv`, ids and bodies kept:
  `a-new-body-lands-placed-in-the-world` (P0 H),
  `an-instances-pattern-draws-nothing` (P0 M),
  `the-viewer-does-not-show-a-constructions-own-space` (P1 H),
  `view-pane-cannot-hide-a-shown-space` (P1 M), and three rows CHROME
  banded at the cut: `a-family-placed-into-an-instances-space-hides-that-space`,
  `the-pattern-tool-stands-member-zero-where-its-body-stands` and
  `the-products-recourse-names-a-door-only-a-tree-click-reaches`, all
  P1 M. P0 and P1 load 31. The first two P0s and the construction's
  own space are one mechanism (a body nothing places has to be drawn
  in its own space), so a successor should read them together before
  cutting units. (CHROME orchestrator)
- 2026-11-01 — Picked up (status active). The whole slate hinges on one
  mechanism: where a new body lands (Ev: unplaced, in its own shown
  space), what the picture draws, and how a body reaches the product.
  Ev's ruling settles the core; what is still open is whether a first
  body joins the product (the recourse row names it Ev's), where
  instances, duplicates, projections and pattern families land, and how
  a second copy keeps off the first. Those go to designers first, on
  the whole slate rather than row by row: a fork row, three-designer
  arm drawn (51, 196; record on `analysis/design-fork/spaces-landing`).
  Units are cut from the designers' answer. (SPACES orchestrator)
- 2026-11-01 — Split the `= ?` value-row refusal out of
  `an-instances-pattern-draws-nothing` as
  `a-patterns-value-rows-read-the-member-index-outside-a-member` (P1 M):
  it is how the Properties pane reads a member-indexed slot, and shares
  nothing with the landing, so it runs while the designers weigh.
  Dispatched (`spaces/pattern-value-rows`). Tier: single review, style
  plus a correctness arm on the sweep (a refusal hidden rather than a
  meaning shown is the failure to falsify). (SPACES orchestrator)
- 2026-11-01 — Designers reconciled in four rounds (fork row 228). Agreed:
  the picture draws bodies, not placements (a body is drawn unless an
  operation reads it as a body; member reads per member); nothing joins
  the product, first body included; one Add-to-the-product door; no
  gesture writes a world `Place`; per-gesture landing; free-move and
  hide keyed by shown space, the world hideable but not free-moved.
  Split: a second inserted copy's default pose (derived offset vs a
  stored insert frame). Put to Ev as `[ev]` PR 6209, which carries the
  README / G3 / spec text; units are cut once Ev answers. DOORS'
  `a-takeover-re-points-along-the-seen-path-and-a-creation-joins-the-product`
  is coupled: (b) is superseded by this, (a) is load-bearing here and
  SPACES will claim it. (SPACES orchestrator)
- 2026-11-01 — `a-patterns-value-rows-read-the-member-index-outside-a-member`:
  PR 6208 opened (per-member slot readings in the Properties pane; new
  kernel doors `Doc::expr_reach` / `Doc::member_envs` over the
  evaluator's own member tuples). CI red only on three
  `refusal_concision_chains` rows, reproduced on main's own tree at
  `fbfebd92`: PR 6200's blend recourse puts blend refusals over the
  75-word budget. Filed on CHROME's slate
  (`blend-recourse-words-overrun-the-refusal-concision-budget`, riding
  6208) and annotated on 6208; CHROME owes the fix. Review dispatched
  (single, style + correctness arm). The lane also filed
  `doctail/a-profile-reshaping-replays-a-member-indexed-step-outside-a-member`
  and `chrome/selecting-an-index-variable-shows-no-row`. (SPACES orchestrator)
- 2026-11-01 — Fork row renumbered 228 → 230 at the merge of main into PR 6209 (CHROME and TANGRIM took 228 and 229 first). (SPACES orchestrator)
- 2026-11-01 — PR 6208 review (single, style + correctness): MERGEABLE-WITH-FIXES,
  1 MAJOR (a refused member past the sixth is hidden behind the spelled
  cap and a first…last span), 1 MINOR (tests cover one shape only), 8
  style. Claims 1, 3, 4, 5, 6 held under executed probes. All taken in one
  fix pass by the implementer. Class noted: the unit's fix minted two
  copies of what it centralised (member-env construction beside
  `eval_family`; an expression's reach beside `index_within`) — the
  reviewer-style-lane trap, caught by review; and a reading carried as a
  fault type, with every `Err` reader treating it as "did not evaluate".
  (SPACES orchestrator)
- 2026-11-01 — 6208 fix pass in (`ef593164`): every review item taken, the
  `defined_rows` deferral fixed in place rather than filed; CI red only on
  main's three concision rows. The pass rewrote the evaluator's member
  enumeration (`eval_family` now through `eval::members::members`), which
  is kernel ground beyond the unit, so a verifier runs before merge, with
  "the evaluator computes nothing differently" as its first claim. The
  implementer's local viewer run filled the disk; its target was handed to
  the verifier by rename (lane finished). (SPACES orchestrator)
- 2026-11-01 — 6208 verifier: VERIFIED-WITH-NOTES. Evaluator output
  byte-identical to the merge base (both lanes; nested, variable counts,
  count 0 and −1, a refusing member); the MAJOR holds under two
  mutations. Notes taken in a second short pass: the refusals note is
  unbounded; the per-member note is built twice in the pane (a copy the
  fix pass minted); three kernel refusals re-worded in the viewer; an
  `ADMITS` copy; `members()` materialises every member's env; no test
  reads the drawn note; a defined row drops its fault. (SPACES orchestrator)
- 2026-11-01 — `a-patterns-value-rows-read-the-member-index-outside-a-member`
  closed (PR 6208, green on `248d6f2f`). Second pass took every verifier
  note; it made `members()` lazy, which reorders one refusal (an earlier
  member's refusal now reports ahead of an inner count fault at a later
  member) — accepted on the orchestrator's read, both are typed refusals
  and in-order is the better answer. Main's concision red was CHROME's
  and is fixed on main (6222, 6227); the duplicate row this PR had filed
  is dropped before merge. (SPACES orchestrator)
- 2026-11-01 — PR 6208 merged (`387df94f`). Every other P0/P1 row now
  waits on Ev's answers in `[ev]` PR 6209: each carries `design: true`
  and refs 6209 until then. Lanes reclaimed. (SPACES orchestrator)
- 2026-11-01 — Ev answered PR 6209: (1) the picture rule confirmed, with
  every body toggleable in the View pane (the rule sets only the
  default); (2) nothing joins the product unasked ("might be changed
  later for convenience"); (3) after the orchestrator's clarification
  showed the narrow derived offset leaves two different parts
  overlapping, Ev rejected a derived layout outright ("unnecessarily
  complex to store/derive where on the screen something is displayed in
  multiple ways"): the "Add part…" gesture stores the new space's
  display frame. README, G3 and fork row 230 updated in place.
  Slate cut into four units: `the-picture-draws-bodies` (P0 H, dual,
  dispatched first), then `a-new-body-lands-unplaced` (P0 H, single
  full), `the-add-to-the-product-door` (P1 M, single full) and
  `the-view-pane-toggles-spaces-and-bodies` (P1 M, style), each parked
  on the first. Claimed DOORS'
  `a-takeover-re-points-along-the-seen-path-and-a-creation-joins-the-product`
  (its (b) superseded, its (a) carried by the landing unit). Seam notes
  left on PLACING's and DOORS' logs. (SPACES orchestrator)
- 2026-11-01 — Seam note from DOORS:
  - DOORS' `a-takeover-re-points-along-the-seen-path-and-a-creation-joins-the-product` is now parked on PR 6209. On Ev's answer, (b) closes as superseded or comes back, and (a) moves here by `git mv` as 6209's body says.
  - DOORS' split, fillet-operand and mirror-two-pieces rows are parked on 6209 as well, to be re-read against the drawn-bodies rule.
  - **Coupled, live now:** DOORS is dispatching the P0 `a-mate-on-a-taken-over-copys-face-goes-stale`. It adds an editor-core `refactor::take_over` door: re-point the copy and write a `Rebind` for each carried name, in one undo step. `feature_over`, `add_mirror`, `sketch_on_face` and `add_transform` in `crates/viewer/src/session.rs` will call it.
  - SPACES' "gestures stop placing" unit rewrites the same gestures. Its re-points should go through that door, or its log should say why not. The branch is `doors/take-over-rebinds`.

  (DOORS orchestrator)
- 2026-11-01 — PR 6209 merged (`531773d4`). `the-picture-draws-bodies`:
  PR 6265 green at `2fee5472` (the rule over operand / one-input /
  placement-source seats, per-member through constant indices only;
  operator results drawn and pickable; outside-the-product tint; hide by
  body and by space; free-move by space, world refused). Dual review,
  CONCURRENT arm (H), both on frozen head `2fee5472`, identical briefs
  modulo lane name: R1 a local Opus agent (inherits the finished
  implementer's warm target by rename), R2 an Opus Claude Code Remote
  session (`session_01EdpJs4bGYnGXNCZkTBTht9`) — the box's ~15 GB free
  disk could not hold two cold reviewer builds beside DOORS' three lanes
  (Ev 2026-10-02: lanes may run in their own cloud sessions). Neither
  reviewer's method differs; recorded for the fair-pair test. The PR
  discloses: `⊔` read as operands drawn as themselves; five fixtures
  hide or delete a coinciding extra body; the corpus pick tally
  re-pinned. (SPACES orchestrator)
- 2026-11-01 — PR 6265 dual review delivered (concurrent arm, frozen
  `2fee5472`). R1 (local, ~40 min): NOT-MERGEABLE-AS-IS, 1 MAJOR / 5
  MINOR / 2 NIT / 10 style. R2 (cloud session, ~26 min):
  APPROVE-WITH-FIXES, 1 MAJOR / 5 MINOR / 2 NIT / 11 style. Neither
  disclosed a glimpse. **Correspondence pre-note** (the blinded coding
  confirms): BILATERAL — the non-constant member read hides members
  nothing reads (R1 MINOR / R2 MAJOR: severity divergence); a `⊔` piece
  keyed without `within`; the `∪` + product-`⊔` overlap; a `Pick` read
  takes the whole copy; style: two readings of what the product reads,
  `drawn_bodies` recomputed per helper, `view`'s ignored `eval`.
  UNILATERAL R1→ — MAJOR (executed): a split or pattern of a fresh box
  makes the box unpickable, masked by `frame_policy.rs`'s split hide
  (R2 read the other five fixture edits, not that one); MINOR the
  copy→body doors drop `source.at`; the product's single body undrawn
  when a `∪` reads it. UNILATERAL R2→ — MINOR (executed base vs head)
  "families excluded, as before" false, gesture seats changed; a copy
  into its body's own space now counted; no row for split-half
  granularity. **Tally candidate:** R1's MAJOR (unilateral, code,
  executed). Adjudication: it is real and an interim of U1 without U2
  (split/pattern leave their target's world `Place`, ruled 2026-10-08,
  `chrome/a-split-or-pattern-gesture-leaves-its-target-placed`), so the
  fix is sequencing — `a-new-body-lands-unplaced` is built on 6265's
  branch and the two merge back to back, with the row and the fixture
  un-hide in that unit's done-when. Every other finding goes to one fix
  pass off the union (the implementer, resumed). (SPACES orchestrator)
- 2026-11-01 — PR 6265 fix pass in, green at `7bbc92fd`: every union
  finding dispositioned (members by index range through
  `Doc::member_envs`; drawings keyed by what draws them; the product's
  body always drawn; one copy→body door with gesture behaviour back to
  base, remaining gesture-visible changes disclosed with rows; the
  split-half row; style all taken). Filed
  `a-pick-read-takes-the-whole-copy` and
  `the-feature-trees-hide-toggles-an-undrawn-instance-silently`.
  No second review on the concurrent arm; `a-new-body-lands-unplaced`
  dispatched stacked on 6265's branch (it inherits the U1 lane's warm
  target by rename); its review covers the pair's interaction, and the
  two merge back to back. Tier for it: single, FULL. (SPACES orchestrator)
