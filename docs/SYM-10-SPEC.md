# SYM-10 — the decision door and the floor: the folds the sign-hull frame needs (spec)

**Program:** SYM (`work/sym/plan.md`, the ceiling lane). **Item:**
`work/sym/the-decision-door-is-opaque-to-the-tier.md` (PROPS's
finding, #2697; Ev's ruling #2728: the tier learns the fold, #2468
holds). **Track:** kernel change — the standard v6 unit (binding spec,
drawn implementer arm, cross-model dual review, union fix pass,
record-at-merge; §Review). Block SYM-B2 slot 2 (the slot SYM-9 held;
the swap is on `sym/b2-block`). **Pre-draw fields, logged before the
draw for the slot:** difficulty **H**, task-class **NUMERIC**.

- **H** — three folds in two rule families (a syntactic one beside
  rule F; a gated read beside rule C), measured against a row of
  another program's that must come back green without being re-aimed.
- **NUMERIC** — it moves what the tier decides; three red rows on the
  sign-hull merge are the acceptance, and every other split and ceiling
  must stay.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item whole (both PROPS measurements: the plain lane is unchanged; the
decision atom is not the cause); the SYM log's 2026-09-19 answer to
PROPS (the reading this spec is cut from); `sym.rs`'s header
(`# The form-level algebra`, `# Freezing`, the rule table),
`combine`'s `Min | Max | Copysign | Atan2` arm and its `Select` arm on
the sign-hull branch (`git show origin/props/sign-hull:crates/geom-core/src/sym.rs`),
`sym/signed.rs` whole (rule C — enclosability, the gated count, what it
declines), `sym/manifest.rs` (rule F, SYM-8 — read it on
`origin/sym/8-manifest-sign` if #2616 has not landed when you start;
your branch merges it when it has), `sym/quotient.rs` (rule E), the
sign-hull construction (`origin/props/sign-hull:crates/geom-core/src/linalg/vec.rs`
`orthonormal_basis`, `norm_witness`, `select`; `real.rs`'s
`select_le_zero`), the three rows named below, and PR #2468's body
(§Fix pass items 1 and 2 — the comparison and the floor, and why).

## The claim

On `main` merged with `props/sign-hull`, three of this program's rows
are red: `m10_derived_frame_tilted_interval::m10_the_tilted_derived_boss_certifies_where_its_authored_twin_does`
(SYM-5's rule-E acceptance: the derived boss refuses at ε/8 on the cap's
`newell_plane_residual`), `m10_derived_frame_interval::m10_the_derived_frames_refusal_is_not_a_freeze`
(the refusal it names moved to a side plane), and
`m10_sym_profile_interval::the_forms_the_walks_build_are_pinned_per_eps_row`
(the walk ledger — the forms are different forms). The plain numeric
lane is unchanged character for character (PROPS): this is symbolic
REACH. The new frame is `Select(d, cz, cy)` with `d = |n.z| −
max(|n.x|, |n.y|)/2`, each candidate `v / max(‖v‖, k·scale)`, `scale =
min(‖n‖, max|n_i|)`; on a tilted normal every one of `‖·‖`, `|·|`,
`max`, `min` and `Select` is an atom the rules do not open, and the
tier cancels the norms only (rules A, E, A0) and `|1/S|` (rule F).

**Three folds, each an identity of reals or a gated read:**

1. **Manifest order** (a sibling of rule F, in `sym/manifest.rs`):
   `max(A, B) → A` and `min(A, B) → B` when the form `A − B` is
   manifestly non-negative (`manifest::nonneg`); `max(0, X) → X` is the
   instance the item names.
2. **Manifest bound**: `max(A, B) → A` when `B` is manifestly bounded
   above by something manifestly ≤ `A` — the predicate `manifestly_le`
   over the shapes the floor uses (`min(c, Y) ≤ c`; a positive constant
   times a bounded form; `abs`/`sqrt` ≥ 0). The floor `max(‖v‖,
   k·min(‖n‖, M))` folds to `‖v‖` once `‖v‖` and `‖n‖` fold to `1` and
   `k < 1`. State the predicate before coding it, as SYM-8 did.
3. **The decision read** (rule C, `sym/signed.rs`): `Select(d, a, b)`
   folds to `a` where `d ≤ 0` is CERTIFIED over the parameter brackets
   and to `b` where `d > 0` is — counted `sign_gated`, never
   `symbolic_zero` (rule C's contract, ratified). Rule C's enclosability
   test is extended by **manifestly-positive factor stripping**: the
   sign of `P/Q` with `Q` manifestly positive is the sign of `P`, the
   sign of `P·R` with `R` manifestly positive is the sign of `P`; the
   stripped form must then be enclosable (parameters and π) or the fold
   declines. `abs(t) → t` on a certified bracket is rule C already.

**Ratified and not re-litigated:** E12; rule C's gating (a READ is
`sign_gated`); rules A/B/D/E/F; the freezing budget; the construction
(Ev's ruling #1944, option 1; #2728) — SYM does not ask PROPS to change
the comparison or the floor, and says in the PR what a squares
comparison would have saved if the render shows it.

## Phase 1 — before touching anything (the measurement)

Branch from `main` merged with `origin/props/sign-hull` (a merge
commit; never rebase). Then:

1. **The three rows red, reproduced**, with their refusals and
   enclosures as the item states them.
2. **The chains rendered** (`explain_depth 6`, the profile's tables) for
   the tilted row's refused residual at ε/8 under `Guided`, and for the
   derived-frame row's: which atoms stand in the residual's early form
   (`Select`, `Max`, `Min`, `Abs`, `Sqrt` — by op, count, and the form
   each is over); which of the three folds above each atom needs; and
   any FOURTH piece (an atom identity such as `sqrt(1/X)` against
   `1/sqrt(X)`; a frozen node) — one table per row in the PR body.
3. **A hand-planted fold, reverted**: for each of the three pieces,
   patch the walk to take it (unsoundly if need be — this is a
   measurement) and re-run the two rows; the table says which pieces
   together turn each row green. **If the three together do not, STOP
   after Phase 1**: report the fourth piece and what it would take; the
   fork returns to Ev as #2728 asks.

## Phase 2 — the folds the measurement picks

Behind one dial `SymRules::decision_door` (or the names the folds earn,
at most two dials), on in `shipped`; `SymRules::without_*` the
differential. Fold 1 and 2 in `sym/manifest.rs` beside rule F, argued
as identities of reals in its header; fold 3 in `sym/signed.rs` beside
the `sqrt`/`abs` reads, with the factor stripping argued where the
enclosability test is. The `Select` arm of `combine` takes the fold
after A0's constant-decision fold (PROPS's, already there) and before
the atom is minted. Ordering against rules A/B/E pinned by the walk
ledger where it moves, argued where it does not. Rows: a theorem row
per fold at the scalar door; the negative rows (`max(A, B)` where
`A − B` is not manifestly signed stays; a `Select` whose decision
straddles the bracket stays an atom and is counted a refusal, not a
read; a stripped factor that is only non-negative — not positive —
declines); the three red rows green WITHOUT edits to their assertions
(they are another program's acceptance; re-aiming them is the path Ev
rejected); the walk ledger re-baselined with its reason; every other
split and ceiling on the six measured documents unmoved (rule F's
table is the baseline once SYM-8 lands), on both instruments.

## Scope

- Files: `crates/geom-core/src/sym.rs`, `sym/manifest.rs`,
  `sym/signed.rs`, tests under `crates/geom-core/tests/m10_*` /
  `sym_rule_*` and `crates/editor-core/tests/m10_derived_frame*`,
  `m10_sym_profile_interval.rs` (the announced tests-family overlap).
- No change to `linalg/vec.rs`, `real.rs` or the construction (PROPS';
  the seam is announced as read-only).
- No new value read outside rule C's gated fold; no new tolerance.

## Acceptance

- Phase 1's tables in the PR body; the two predicates stated before the
  code.
- The three rows green on the branch with their assertions untouched;
  the ledger re-baselined with its reason; splits and ceilings on the
  six documents unmoved; the cost on the leaf instrument and the
  ceiling instrument, disclosed against the line.
- The `sign_gated` column moves where the decision is read and
  `symbolic_zero` does not gain a read (a row asserts it).
- Local full checks green (fmt; clippy `-D warnings` on `geom-core` at
  default, `interval`, `interval,sym-profile-testing`, `editor-core`
  `interval`; the gates; `work.py lint`); the PR targets
  `props/sign-hull` (hosted CI runs only on PRs to `main`), so the
  hosted gate is #2468's next run after the merge, which the PR body
  names as the acceptance's home.

## Review

The full v6 dual. Claims to falsify: (1) soundness of each fold —
`max(A, B) → A` with `A − B` manifestly non-negative but zero at a
point (both sides equal there: fine — say so); a manifest bound that is
false at a boundary; a `Select` read whose decision's certified sign
rests on a stripped factor that can be zero (a `Q` that is manifestly
NON-NEGATIVE only must decline); the gating (nothing lands in
`symbolic_zero` through a read); (2) nothing lost anywhere; (3) the
chains are what Phase 1 says; (4) the rows green without edits; (5) the
cost; plus `docs/prompts/reviewer-style-lane.md` in full. Union fix
pass on the implementer's lane; delta by R1; the row lands at merge.

## Landing

Status `review` on `work/sym/SYM-10.md` when the PR opens; the
orchestrator closes the unit and deletes this spec at merge; the item
closes when #2468 lands green.
