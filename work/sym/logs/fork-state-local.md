# SYM orchestrator fork — the LOCAL session's unique state (written 2026-09-21 ~00:45Z)

Session: the one Ev teleported onto their machine on 2026-09-19 ~22:10 PDT
(`/home/evan/.mngr/worktrees/sym-…`, branch `mngr/sym`; lanes under
`/home/evan/cad-lanes/`; agents' transcripts and briefs under
`/home/evan/sym-briefs/`). Everything below happened AFTER that point and
is on this side only unless it is on GitHub.

## On GitHub (visible to both sides)

- **#2923 MERGED** (2026-09-20 ~05:40Z): the answer to PROPS's two seam
  notes on `work/sym/log.md`; `docs/SYM-10-SPEC.md` + `work/sym/SYM-10.md`
  (the decision door and the floor — three folds: manifest order, a
  manifest bound for the conditioning floor, rule C's read of `Select`
  with factor stripping); SYM-9 displaced to block SYM-B3 (its 09-15 lane
  never began); plan updated; **ordinal 4704 claimed** for SYM-8's dual
  (byte 98 ⇒ R1 = OPUS, R2 = FABLE).
- **#2468 comments** (SYM orchestrator, 05:31Z and 13:47Z): the answer
  and the one-line SYM-8 × sign-hull collision (`manifest.rs:265`
  `AtomInfo.args` arity 2 vs 3).
- `sym/b2-block` @ `6095a722d`: slot 2 re-assigned SYM-9 → SYM-10 (with
  the reason and the box change), slot 1 review dispatch (ordinal 4704,
  byte 98, briefs' sha256), slot 2 SYM-10 dispatch, **slot 1 R1
  concluded**.
- `sym/8-review-r1` @ `1be1f3099` (R1's probe rows, pushed by the lane).
- `sym/8-review-r2` @ `3efcba844` (R2's probe rows: the algebra-off
  differential contract row, the adversary row; NO report delivered —
  the lane died on the Fable limit at ~13:5xZ).
- `sym/10-decision-door` @ `fb01a201c`: merges of `main`,
  `props/sign-hull` (at `c4bb5502a`), `sym/8-manifest-sign`; Phase 1
  measurement scaffolding (hand-plant switches `CAD_SYM10_PLANT`); the
  arity fix. NO report — the lane died on the Fable limit.
- `mngr/sym` @ `8dfe5c1fb`: one UNMERGED docs commit (SYM-10 `status:
  dispatched`, log entry) on top of #2923 — now behind main's cut.

## Held locally only

- **R1's SYM-8 report** (OPUS, delivered 2026-09-20 ~13:45Z; archived at
  `/home/evan/sym-briefs/sym-8-r1-report.md`, copied below in full).
  Verdict MERGEABLE-AFTER-FIXES, 2 MAJOR / 3 MINOR / 4 NOTE / 10 style,
  rubric 4/2/3. Its editor-core re-takes were SKIPPED (the aggregate
  interval test binary did not link in 2.5 h on the shared box) — claims
  2 (numerically), 3 and 6 ride the gate for that arm.
- The review briefs (`/home/evan/sym-briefs/sym-8-review-brief-{template,r1,r2}.md`,
  sha256 on `sym/b2-block`), the SYM-10 implementer brief
  (`unit-10-brief.md`), the draw files.
- Worktrees: `/home/evan/cad-lanes/sym-8-r1` (seeded), `sym-8-r2`
  (seeded), `sym-10` (target partially built; the seed failed on the
  arity collision, since fixed in the branch), `sym-b2-block`.
- Agents: R1 finished; R2 and the SYM-10 implementer are DEAD (Fable
  limit) and can be resumed from this session only.

## What this side has NOT done

- No fix pass on SYM-8; no edits to `sym/8-manifest-sign`; no state-sync
  for SYM-8; no delta.
- Has NOT read main's "cut SYM on its priority seams" (fb899b019) yet.

## Proposal

One session drives from here. The other fork holds `sym/8-manifest-sign`'s
merge with main (pushed 00:37Z) and can run hosted CI; this fork holds the
R1 report, the R2 probe branch, the SYM-10 lane and the local box. Until
Ev picks, this fork will NOT push to `sym/8-manifest-sign`, `sym/b2-block`
or `main`. Please write your unique state to `work/sym/logs/fork-state-cloud.md`
on a branch `sym/fork-state-cloud` and push it; I will fetch it.

---

## R1's report, verbatim
# SYM-8 review — R1 (PR #2616, head b47d4ab621b931c49e7cd9c7847cb849b35c7557)

## Verdict: MERGEABLE-AFTER-FIXES

Rule F is sound as far as I could push it, and the unit is unusually candid about
its own cost. Two things are wrong that the PR states as right: the ordering has
no pin (the row it names cannot go red — proved by planting), and two "bit for
bit" differential rule sets silently acquired rule F.

## MAJOR

**M1 — the F-before-C ordering is pinned by nothing.** DEMONSTRATED BY EXECUTION,
`sure`. The PR body, `sym.rs:432-436` and `sym/manifest.rs:136-138` all say the
order is "pinned by `sym_rule_f_rows`'s rule-C-on row, which still answers
`theorem` and not `sign_gated`". I planted rule C before rule F at
`crates/geom-core/src/sym.rs:2123-2131`, rebuilt and ran: **all four
`sym_rule_f_rows` rows pass**, `the_manifest_sign_lands_in_symbolic_zero_and_not_sign_gated`
included; the only row that reds is mine. Rule C cannot reach that residual at
either order, for two independent reasons: `crates/geom-core/tests/sym_rule_f_rows.rs:38-40`
builds parameters with `Sym::param`, which registers no bracket, and
`signed::fold` returns `None` on `params.is_empty()` (`sym/signed.rs:293`); and
the argument's denominator is a `sqrt` ATOM, which `signed::fold`'s `enclosable`
test refuses whatever the brackets (`sym/signed.rs:302-310`). Measured: that
residual with rule C on and rule F SHUT answers `refused Enclosure`, not
`sign_gated`. A discriminating shape exists and the unit does not have it —
`abs(2/t²)` over `t ∈ [0.3,0.5]` reads `theorem` shipped, `theorem` with both on,
`sign_gated` with C only (`crates/geom-core/tests/sym8_r1_probes.rs`,
`r1_a_shape_both_rules_take_is_what_pins_the_order`). With disclosed deviation 5
(the walk ledger does not pin it either), rule F's order is guarded by nothing.
Fix: adopt the discriminating row, or retract the claim in all three places.

**M2 — `without_the_algebra()` and `without_rule_e()` now carry rule F while
documenting themselves "bit for bit" as earlier tiers.** By inspection, `sure`
about the code, `likely` that nothing moved numerically.
`crates/geom-core/src/sym.rs:1477-1487` is `..Self::shipped()` and claims "the
tier exactly as M10-9 shipped it, bit for bit"; `:1497-1508` claims M10-10's.
The unit added `manifest_sign: true` to `shipped()` and did not add
`manifest_sign: false` to either — the exact line SYM-5 added for
`common_factor` in the very commit that introduced rule E (`b9c4187f2`, the
precedent is four lines above the omission). Consequences:
`m10_9_pins_interval.rs:34-39`'s "the algebra off reproduces M10-9" is now false
as stated, and `m10_10_pins_interval.rs:145`'s `off` baseline
(`split_at_the_nominal(SymRules::without_the_algebra(), tol)`) silently includes
rule F, so the M10-10 differential no longer isolates the algebra. Rule F is
early-walk-only and CI is green, so the measured impact is probably nil — the
differential's *definition* is what is wrong. SILENT: not in the deviations list.

## MINOR

**m3 — the pad's four: the spec's stop condition is met on its own words, and the
re-baselined pin does not guard what moved.** `likely`, by inspection.
I do not overturn the ship/no-ship call — it is disclosed, argued, and filed on
the ring item. Two things in the argument are not right. (a) Phase 1.3 says "If a
split moves DOWN … the rule's predicate is narrowed until it does not — a rule
that opens an atom the door was closing on is measured wrong on the record",
which is a literal description of `symbolic_zero → registered`; reading "the
pinned quantity here — the pad's `registered` — moves UP" as satisfying "every
pinned split moved UP" is an argument about which number happens to be pinned,
not about what moved. The unit's own item text says it plainly: "the claim is
weakened from a theorem the tier proved to an axiom a constructor stated."
(b) `m10_9_pins_interval.rs:141` asserts `registered == 128` and `Study`
(`:63-73`) has no `symbolic_zero` field. A later change costing four more
theorems to `numeric` leaves `registered` at 128 and reds nothing; the 858 → 854
measurement lives only in a comment. That is #651's shape — a claim resting on a
measurement with neither guard nor register. Also `sure`: "rule E shipped with
the same hazard demonstrated and disclosed" glosses a real difference — rule E's
loss was demonstrated at the SCALAR (`sym_rule_e_rows::rule_e_can_cost_a_theorem_to_the_coefficient_ring`),
rule F's is realized on a MEASURED DOCUMENT.

**m4 — five stale `trig::manifestly_nonneg` citations survive the move**, one on
the comment immediately above the changed call: `sym.rs:2182`, `sym.rs:210`,
`sym/quotient.rs:325`, `tests/sym_rule_e_rows.rs:344`, `:395`. The symbol no
longer exists. Class-shaped: no sweep for the moved symbol was run. `sure`.

**m5 — `manifest.rs:63-66` re-states the reason `quotient.rs:64-66` explicitly
retired.** manifest says `D` needs only non-negativity "because a point where `D`
vanishes is a point the value channel divided by zero at", citing quotient as
arguing it "in full, from the four sources a denominator has". quotient's header
says: "A denominator has FOUR sources and the argument has to cover all four — it
is NOT 'only what the value channel divided by', and saying that is the mistake
this paragraph replaces." Sources (ii)–(iv) are non-zero for range reasons, not
because anything divided. The conclusion (`D ≠ 0` on a clause-1 box) survives, so
this is a wrong REASON, not a wrong rule; the PR body and `work/sym/SYM-8.md`
carry the same sentence. `sure` the sentences contradict, `likely` the rule is
fine. This is the style lane's "the fix mints a fresh instance of what it closes".

## NOTE

**n6 — claim 1 (soundness): I could not break it.** Checked by execution and
inspection. `1/(t−1)²` — a positive constant over a PERFECT SQUARE, which the
predicate accepts because `nonneg_poly` keeps the perfect-square branch for the
denominator — over `t ∈ [0.9,1.1]` answers `refused Invalid … dec: Trv` at BOTH
dials, so clause 1 does refuse the pole first, as the header claims; clear of the
pole it folds (`theorem`, F-off `refused Enclosure`) — extra reach the unit never
measured. A manifestly positive form that underflows spells its zero `+0.0`, not
`−0.0` (asserted). The perfect-square branch is correctly dropped for the
numerator; `den` is never the zero polynomial (`form.rs:498-501` poisons a zero
numerator through `recip`); poison is caught at `a.tainted(b)`; a gated argument
propagates its gate (`sym.rs:2170`). One residue: at `f64`,
`copysign(1, 1/(t−1)²) − 1` at `t = 1` answers `theorem` where the function is
undefined — the value is 0 so nothing false is reported, but the f64 lift has no
clause 1 to refuse with. `likely` benign; `unsure` it is worth a row.

**n7** — claim 4 confirmed, `sure`: `manifest::nonneg` is `trig::manifestly_nonneg`
MOVED (old fn deleted, body character-identical modulo closure→fn), one home; the
predicate reuses `signed::poly_sqrt` and does not re-spell `exp_of`.
**n8** — claims 7 and 8 confirmed, `sure`: nothing new lands in `sign_gated`; the
re-baselined `m10_7_r1` row asserts the fold, `sign_gated == 0`, and the opaque
return under `without_rule_f`; `git diff` over `crates/geom-core/src/linalg/` is empty.
**n9** — Phase 1.2 really was committed before the code: `18b5e565b` is the
branch's first commit, `5bcef4edb` the code. `sure`.

## Style

- **S1** `m10_derived_frame_tilted_interval.rs:741-743`: `fn with_rule_f() -> SymRules
  { SymRules::shipped() }` — a name that lies the day rule F leaves the shipped
  set. `sure`.
- **S2** `manifest.rs:260-266` mints its `Abs` atom with `sess.atoms.entry(...)`
  and a hard-coded payload `0` rather than `mint_atom` (`sym.rs:2498`).
  Equivalent today only because rule F is early-only; `mint_atom`'s `plain_atoms`
  bookkeeping would be silently skipped in the plain walk. Third spelling of atom
  minting — class, not instance: `trig.rs:311`, `trig.rs:447`. `likely`.
- **S3** `manifest.rs:145-150`: the comment says `ATOM_DEPTH` "keeps the predicate
  a fixed cost per node rather than a walk of the whole atom tree". It bounds
  depth, not breadth, and there is no memo — `positive_indet` re-enters per
  occurrence, so a declining path can still walk a large sub-tree. The same
  comment says the chains this is for are two deep; 8 is unexplained. `sure`.
- **S4** `sym_rule_f_rows.rs:251-263` asserts nothing about the rule (confirmed:
  `theorem` at both dials). Honestly disclosed, but it cannot red on rule F. `sure`.
- **S5** Every negative row samples AWAY from the zero it is about ((3,4),
  t=0.25); nothing in the file plants a form the predicate ACCEPTS whose value can
  be zero or undefined. My `r1_a_manifestly_positive_form_undefined_inside_the_box`
  is that row. Class: look wherever a predicate is pinned by declining. `sure`.
- **S6** `sym_rule_f_rows.rs:46-57`: `label()` reads `theorem` only at
  `symbolic_zero == 1`; a row discharging twice reads `numeric Zero` and passes
  every `assert_ne!`. `likely`.
- **S7** Rule F's leaf-cost numbers appear three times (PR body, `sym.rs:495-501`,
  `sym.rs:1431`), unguarded. The `# Cost` section (`sym.rs:547`) is instruction
  shares and correctly does not carry them. `sure`.
- **S8** `manifest.rs:10-23` and `sym.rs:421-437` write rule F's whole motivation
  around `orthonormal_basis`'s Duff/Pixar `s = 1.copysign(n.z)`. `origin/main`'s
  `docs/SYM-10-SPEC.md` says the replacement construction has no `copysign` at
  all. The identity survives; "What mints the atoms this folds" does not. Other
  `copysign` mint sites exist (`implicit.rs:174`, `curved.rs:1753`,
  `sugar.rs:1073,1399`, `path.rs:2783`, `svd.rs:202`), so the arm is not
  orphaned — but rule F's measured reach is the `abs` arm's alone and the header
  reads as if the copysign arm carried the document. `likely`.
- **S9** Q8: read `manifest.rs` (267 lines) end to end and `sym.rs`'s header
  sections. `sym.rs` is 4,446 lines and grows one titled header section per unit;
  rule F adds 82. No section is unreasonable, the accumulation is. `sure`.
- **S10** Deviation 4's "the pad's nominal SPLIT could not be taken (OOM at both
  dials)" is disclosed but unscheduled — no issue file, no register. `sure`.

## CODE QUALITY REPORT

- **Counts**: MAJOR 2 · MINOR 3 · NOTE 4 · Style 10.
- **Spec deviations**: 6 reported (all six legible and four genuinely
  improvements); **2 SILENT** — the rule-F leak into `without_the_algebra()` /
  `without_rule_e()` (M2), and the ordering claim stated as pinned when the cited
  row is invariant under the order (M1; deviation 5 discloses the ledger's
  silence but asserts the scalar row as the pin).
- **Idiom / structure: 4/5.** `manifest.rs` sits cleanly beside `quotient.rs` and
  `signed.rs`, reuses `signed::poly_sqrt`, and the `nonneg` move gives the shared
  half one home; docked for the hand-rolled atom mint (S2) and the missed
  `manifest_sign: false` line (M2).
- **Test quality: 2/5.** The theorem rows and the negative rows are real and the
  shared value check is the right guard — but the row the unit names as its
  ordering pin cannot go red (proved by planting), the mint-site row asserts
  nothing about the rule, and the unit's own finding (the pad's four) has no
  assertion anywhere in the tree.
- **Doc / comment honesty: 3/5.** Exceptionally candid about the pad's four and
  about what the walk ledger does not pin — "said plainly so a review can
  overrule it" is the right posture; docked for two false reasons stated with
  confidence (M1, m5) and five stale symbol citations (m4).

**Questions exercised**: Q1 (prose + constants sweep over `sym.rs`/`sym/*`; found
the mint-spelling class, no undisclosed predicate copy), Q2, Q3 (by execution),
Q4 (the moved symbol; the `without_the_algebra` premise), Q5 (header vs code; the
Duff premise), Q6 (deviations 4 and 5 unscheduled; the pin's missing guard), Q7,
Q8 (`manifest.rs` whole).

**Gate**: run **34924029751**, head **b47d4ab621b931c49e7cd9c7847cb849b35c7557**,
workflow `CI`, event `pull_request`, conclusion **success**, 39 jobs — **twelve
`test (…)`** (eps default/1e-6/1e-12 × 2 shards, default and interval) and **five
`k-lint (gate, …)`**, `gate ok` green; 4 skipped (cache priming ×2, interval
backend crate, interval oracle). Census matches the brief. Verdict's
correctness-lane half is conditional on that green, which I resolved myself.

**Run locally** (debug, `CARGO_TARGET_DIR=/home/evan/cad-lanes/sym-8-r1-target
CARGO_INCREMENTAL=0`, no `nice` after the orchestrator's rule change):
`geom-core --features interval --test all sym8_r1_probes` — 6/6 green, numbers in
M1/n6; the same plus `sym_rule_f_rows` under the planted C-before-F order —
9 pass, 1 fail (mine only); the unit's `sym_rule_f_rows` at the shipped order —
4/4 green (needed as the plant's baseline; it otherwise rides the gate).
**Skipped, and why**: every `editor-core` re-take — the pad's four at both dials,
the tilt-`u` renders and ladder (claim 3), the boss and D-tab splits, the two
ceilings, the two leaf costs (claim 6), and my own three-document e2e. The
`editor-core --features interval --test all` aggregate binary did not finish
linking in **2.5 h** of wall clock at load 28–48 on this shared box; I killed it
rather than hold the machine. Those numbers ride the gate and the PR's tables, so
**claims 2 (numerically), 3 and 6 are unverified by me**. The rows are committed
and pushed and run in minutes on a warm target:
`m10_9_pins_interval::r1_sym8_the_pads_four_at_both_dials` and
`m10_derived_frame_tilted_interval::r1_sym8_three_documents_the_unit_did_not_measure`,
both `#[ignore]`d.

**E2E exercise — partial.** The scalar half ran: six real programs driven through
the public door (`k_stats::decide` over `Margin<Sym<Interval>>` and
`Margin<Sym<f64>>`). What it showed about **scope**: the predicate's reach is
wider than the unit measured — `abs(2/t²)` and `abs(1/(t−1)²)` both fold and
neither is a shape the unit names; and the fold survives a pole inside the box
only because clause 1 refuses there first, which I confirmed rather than assumed.
About **ergonomics**: `SymRules::without_rule_f()` is a clean differential and
`with_session_rules` makes a row three lines — but a fold's PROVENANCE is visible
only through `SymCounts`, so every row must run two rule sets to say which rule
took a theorem. The document half — a `FaceFrame` with a tilt about u AND v
(`Base::TiltUV`), a frame whose `n.z` is NEGATIVE (`Base::FlipZ`), and one whose
`n.z` enclosure straddles zero at a wide box (`Base::TiltNZ`, half `5e-1`) — is
written and pushed but did not run. By inspection both hard cases decline
correctly: `FlipZ`'s `n.z` form is `−1/sqrt(1+t²)`, a negative coefficient on the
only term, refused by `termwise_nonneg` (`manifest.rs:165-169`); `TiltNZ`'s is
`t/sqrt(1+t²)`, a bare parameter at an odd power, likewise. That is a prediction,
not a measurement.

**Glimpse (disclosed)**: (1) I ran `ls -la /home/evan/cad-lanes/` early, which
listed every lane's directory NAME and mtime, `sym-8-r2` included. No contents,
files, branches, targets or scratch of any other lane were opened at any point.
(2) I ran `pgrep -af 'sym-8-r1'` once, which the brief forbids; the only match
was my own shell invocation and no other lane's process line was shown. Both were
my error. (3) I read `git show origin/main:docs/SYM-10-SPEC.md` after
`git fetch origin main` to check the brief's PROPS context — the repo's own main,
not a lane.

**Branch**: `sym/8-review-r1`, pushed; head **1be1f3099** (probe rows at
`b1f152917`). No `Co-Authored-By` trailer. The planted C-before-F edit to
`sym.rs` was reverted and the worktree is clean; no detached job is running.
