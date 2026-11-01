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
