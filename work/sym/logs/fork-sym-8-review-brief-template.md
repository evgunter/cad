# SYM-8 review brief — @@LANE@@ (the manifest sign: abs and copysign atoms whose sign the form already shows)

You are reviewer **@@LANE@@** of the v6 dual for unit SYM-8 of program
SYM (the E12 symbolic identity tier, `geom_core::sym`), PR #2616 on
`evgunter/cad`, **frozen head `b47d4ab621b931c49e7cd9c7847cb849b35c7557`**.
Your review is one of two run on this head by two reviewers who must
not see each other's work.

**Read first, in full:** `docs/prompts/reviewer-style-lane.md`
(binding; name the questions exercised; `sure`/`likely`/`unsure` on
every finding); `docs/SYM-8-SPEC.md` (the binding spec — Phase 1's
three tables and two stop conditions, Phase 2's predicate and rule F,
the acceptance "no split or ceiling moves down anywhere"); the PR body
(`gh pr view 2616 --json body --jq .body`; the diff with
`gh pr diff 2616`; the run with `gh run view 34924029751`); the diff
whole — `crates/geom-core/src/sym/manifest.rs` (the predicate and the
rule, its soundness argument and the signed-zero edge), `sym/trig.rs`
(`manifestly_nonneg` and what moved to `manifest::nonneg`), `sym.rs`
(the dial, the fold in `combine`'s `Abs` and `Copysign` arms, the order
A0 → F → C at the node, the header's rule table and `# Cost`),
`sym/signed.rs` (rule C — read what it declines, because this rule must
not take a sign READ), the theorem and negative rows
(`crates/geom-core/tests/sym_rule_f_rows.rs`), the re-baselined pins
(`m10_9_pins_interval.rs`'s pad `registered` 104 → 128,
`m10_7_r1_sym_probes`, `m10_derived_frame_tilted_interval.rs`,
`m10_10_evidence_interval.rs`'s `no_f` rung); `crates/geom-core/src/linalg/vec.rs`'s
`orthonormal_basis` on THIS head (Duff's spelling — where the atoms are
minted; NOT touched by this PR); the two items
(`derived-frame-placement-freezes-on-the-symbolic-lane`,
`coefficient-ring-width-is-not-monotone-in-reach`) and the new row
`the-tilt-u-newell-residual-is-the-next-wall`;
`memories/review-and-dependency-policy.md`.

**Context you need and the PR does not say:** since this head was
frozen, PROPS's sign-hull unit (#2468, holding) replaces
`orthonormal_basis` with a construction that has no `copysign` and
whose `abs` atoms are `|n.z|`, `|n.x|`, `|n.y|` in a decision plus a
conditioning floor; rule F's `abs(1/S) → 1/S` is one of the folds that
construction will need (SYM-10's spec, `docs/SYM-10-SPEC.md` on
`main`). Read rule F's motivation prose with that in mind: is it
written so it stays true when Duff's spelling goes?

## Your lane (v6 item 5 isolation — READ side)

- Worktree `/home/evan/cad-lanes/sym-8-@@LANE@@` at the frozen head,
  branch `sym/8-review-@@LANE@@` for probe rows. Every command:
  `cd /home/evan/cad-lanes/sym-8-@@LANE@@ && CARGO_TARGET_DIR=/home/evan/cad-lanes/sym-8-@@LANE@@-target CARGO_INCREMENTAL=0 nice -n 10 cargo …`
  on ONE line. **Wait until `/home/evan/cad-lanes/sym-8-@@LANE@@-target/SEEDED`
  exists before any cargo command** (a foreground `ls` poll in an
  until-loop; the seed is a full test build and this box is slow
  today). Private scratch `/home/evan/cad-lanes/sym-8-@@LANE@@-tmp/`.
  Never build a second worktree into your target directory.
- **This is a SHARED machine**: 8 cores, ~5 GB of RAM free, four
  orchestrators' lanes, load average ~30. `nice -n 10` every cargo;
  one crate at a time; ONE heavy row at a time, detached and polled;
  never the pad's shape report (`split_at_the_nominal` with the report
  OOMs — the PR says so); prefer release for the ceiling rows and read
  the PR's numbers as the baseline when a re-take is unaffordable —
  say which re-takes you skipped and why.
- **Until your report is delivered you must not fetch, check out, or
  read the other reviewer's branch (`sym/8-review-*` other than yours),
  worktree, target, scratch or CI artifacts, nor any other lane under
  `/home/evan/cad-lanes` or `/home/evan/.mngr/worktrees` (other
  programs' orchestrators run there).** The PR, the branch
  `sym/8-manifest-sign` and your own tree are what you read. Any glimpse
  is DISCLOSED — `pgrep -f` with your own path only, never `pgrep -af`,
  never `ps` or `git worktree list` unfiltered.
- Push your probe branch early and often (`git push -u origin sym/8-review-@@LANE@@`).
  Foreground rule: no background waiters; anything over 600 s runs
  `setsid`-detached with a recorded PID and is polled in the foreground;
  never end a turn with a detached job running. No `Co-Authored-By`
  trailer in your commits.
- Hosted run **34924029751** on the frozen head: read its job list
  yourself (`gh run view 34924029751 --json jobs --jq '.jobs[].name'`;
  twelve `test (…)`, five `k-lint (gate, …)`) before conditioning
  anything on it.

## Reviews here include end-to-end exercise

Build a document of your own that the unit did not measure — a
`FaceFrame` on a body whose plane normal has a component Duff's basis
must sign (a tilt about u AND v; a frame whose `n.z` is negative; a
frame whose `n.z` enclosure touches zero at a wide box) — and drive it
through the public doors at `Sym<Interval>` with the dial on and off,
under both lifts; report what happened, in scope and in ergonomics.
And build the adversary: a form the predicate calls positive whose
value channel can yield `−0.0` or a real zero. Local runs are
unique-signal only; keep them affordable on this box.

## The claims to falsify (the dispatcher's hypotheses, not findings)

1. **Soundness of rule F.** `copysign(1, X) → 1`, `copysign(Y, X) →
   abs(Y)`, `abs(X) → X` for manifestly POSITIVE `X`, argued as
   identities of reals under clause 1. Hunt the box: an `X` the
   predicate calls positive that is zero at a point (a sum of squares
   is NOT positive — does the predicate's term-wise branch admit one?;
   is `D`'s non-negativity enough, or can `N/D` with `D = 0` at a point
   clause 1 admits reach the fold?; is a `Sqrt` atom over a manifestly
   positive argument really non-zero?); the signed zero
   (`copysign(1, −0.0) = −1`; which real quantities in the value channel
   can be `−0.0` for a real zero?); poison. Write the row that reds if
   any of these folds a non-identity.
2. **Nothing lost, by execution — and the pad's four.** The acceptance
   is "no split or ceiling moves down ANYWHERE"; the lane found four
   decisions on the rounded pad move `symbolic_zero` → `registered`
   (858 → 854, 104 → 128, `numeric` 991 → 971) and judged it a
   DISCLOSURE, not the spec's stop condition, re-baselining the
   per-document `registered` pin 104 → 128. **This judgement is your
   first target**: is a theorem weakened to an axiom a decision "moved
   down"? Reproduce the four (release, the pad at the scale it
   certifies whole at, dial on and off — affordable as ONE row) and
   read what they are; say whether the spec's clause is met, and
   whether the pin's doc says what happened. Re-take the boss and one
   D-tab split (the ring item's patch-A case) and two ceilings on your
   box; the rest ride the gate and the PR's tables.
3. **The wall is what the chain says.** The tilt-U residual with the
   dial off carried `copysign` 12/12 and `abs` 12/12 in the frozen
   kid's terms; on, the square is built and the refusal moves to
   `newell_plane_residual`. Render both (`explain_depth 6`, ONE run
   each) and confirm; confirm the "arms apart" claim (copysign-only is
   bit-identical to off — the reach is the `abs` arm's alone).
4. **The predicate is what it says**: manifest POSITIVITY, not
   non-negativity; the shapes it accepts and refuses per the spec's
   Phase 1.2; `manifest::nonneg` is `manifestly_nonneg`'s per-polynomial
   half MOVED, not copied (one home).
5. **The ordering** A0 → F → C at the node, and against A/B/E
   "structural" (they run after `combine`): the lane says the walk
   ledger is UNMOVED by rule F on the slab and the plate, so the
   ordering is pinned by a scalar row (F before C: a theorem never
   counted `sign_gated`) and the structural argument, not the ledger.
   Is that honest, and is the scalar row a real pin (plant F after C
   and name what reds)?
6. **The cost**: leaf instrument (release) plate 0.49 → 0.50, annulus
   0.44 → 0.40, bracket 2.49 → 2.37, link 3.29 → 3.32, pad 19.7 → 18.8 —
   "free to the noise, cheaper on most". Re-take two documents if the
   box allows (release, one leaf each); read the ceiling-instrument
   table beside it.
7. **Rule C is untouched**; this rule takes no sign READ (nothing new
   in `sign_gated`); the re-baselined `m10_7_r1` row
   (`copysign(x, 1) − |x|` now decides) asserts it is not gated and
   returns with `without_rule_f`.
8. **`linalg/vec.rs` is untouched.**

## Style lane emphasis

Q1: `manifest.rs` beside `quotient.rs`, `trig.rs`, `signed.rs` — the
rule-module shape, and whether the predicate duplicates
`signed::poly_sqrt` or re-spells `exp_of`; Q3: can the theorem row and
the negatives fail on anything but the dial and the predicate?; Q5: the
header's rule table and reach sentences against the code
(document-specific reach was SYM-5's correction — is it kept? does the
`# Cost` table carry rule F's numbers?); Q6: the seven deviations
listed — improvement or debt; the SILENT ones; Q8: `manifest.rs` whole
and `combine`'s `Abs`/`Copysign` arms.

## The dispatcher's own exposure

Everything above is the dispatcher's belief; check it before building
on it, and report any correction as a finding. The spec's predicate
list and fold set were the dispatcher's from one render; the lane
narrowed the positive branch (term-wise only, never the perfect square)
and declined the boss's `abs((5/8)·sqrt(L²))` on purpose — that is the
unit's result. The pad's four is the lane's judgement call and it said
so; you decide.

## Report (≤150 lines, to the orchestrator; do not post on the PR)

Verdict (`MERGEABLE` / `MERGEABLE-AFTER-FIXES` / `NOT MERGEABLE`) with
MAJOR/MINOR/NOTE findings on the claims, each with `file:line`, a
confidence, and whether DEMONSTRATED BY EXECUTION or by inspection; a
`## Style` section; the CODE QUALITY REPORT with the fixed rubric —
counts of MAJOR / MINOR / NOTE; spec deviations as reported vs SILENT;
three ratings 1–5 with one line of evidence each (idiom/structure,
test quality, doc/comment honesty); the questions exercised; the gate
verification (run id, head SHA, job census); what you ran locally with
its numbers beside the PR's, and what you skipped for the box; the e2e
exercise and what it showed; any glimpse (disclose). Push your probe
branch and name the commit.
