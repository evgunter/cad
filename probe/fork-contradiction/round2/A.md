## For Ev

**Decision recommended (likely).** When the numbers already give a definite answer, stop asking the symbolic tier to check that answer. Check the answers the tier itself returns, cheaply, and move the expensive cross-check into the test suite. Both design reports now recommend this final state; nothing in it is left for you to choose. What needs your sign-off is the trade below and one sentence of ERROR-DESIGN E12.

Terms:
- **The symbolic tier** (`Sym<T>`) carries an expression beside each number (the *numeric channel*). It can prove by algebra that a margin is identically zero.
- **The exact witness** is `Sym<Interval>`, where the number is a certified bracket (*enclosure*). It is the only scalar a drive replays at. At `Sym<f64>` the number is one rounded value.
- **A definite sign** is an enclosure clear of the tolerance band.
- **The receipt** (`SymCounts`) is a run's decision counts.

The final state:
1. **A definite sign is the answer, and the tier is asked nothing.** Today, after every definite sign, the tier also expands the margin's expression. It does this once to check that the algebra would not have called the margin zero, and once more where a registration (a constructor's stated identity) exists. Both walks leave the decision path. A definite decision then costs the numeric channel and nothing more. (sure)
2. **Where the tier answers `Zero` at the exact witness, the enclosure must contain 0.** This is a plain assertion in every build: one comparison, and a firing means the algebra or the enclosure is wrong. It covers every way the tier reaches a zero, including the retry attempts and registrations, which nothing checks today.
   - **The one exception:** the edge-on-surface door (`free_identity`), where the number is a bound on a distance and the expression is the identity "the edge lies on the surface". A correct `Zero` can sit there beside a bound like `[1e-12, 3e-12]`. (likely on the soundness argument, which holds for every rung: plain form, rule rungs, quotients, the decision read, retries and registrations. It is argued from the code, not run.)
3. **The old cross-check moves to the test suite.** It asks whether the algebra would claim zero on a margin the bracket proves non-zero. It becomes an instrument compiled only in test builds and switched on by the test that wants it. It covers all three of today's events and the retries.
   - It runs on every PR in small rows that plant a contradiction and see it caught.
   - It runs over the document corpus on any PR that changes the tier, and nightly.
4. **Four columns leave the receipt:** `theorems_disputed`, `checks_expanded`, `checks_frozen` and `registrations_contradicted`. Every reader of them today is a test of the check itself or of the kernel's own registrants. None reports on a user's document. (sure)

**Why the suite and not the product (sure).**
- Everything the cross-check can find is a kernel defect: a wrong algebra rule, a wrong enclosure, or a registrant stating a false identity. At `Sym<f64>`, what it finds is rounding at a scalar no drive ships.
- A user can act on none of these. So a run-time switch would be product surface for a test, which `[simple.checks]` rules out.
- The check sits on the wrong decisions. It runs only where the tier's answer is thrown away, and on M10-3's slab it is 95% of the tier's rule-walk work.

**What the assertion in 2 is, stated exactly.** It is not a test of the algebra.
- Any enclosure clear of the band has already returned as definite. So the assertion can fire only on an enclosure that excludes 0 while lying inside or across the band, e.g. `[2e-9, 5e-9]` against a 1e-9 zero band. Today such a case is stamped proven zero unchecked.
- What it guarantees: no zero the kernel returns contradicts its own certified enclosure.
- A false algebraic zero on an enclosure that straddles 0 is seen by no check in any run, before or after this change. Only the test instrument looks for those.

**What is traded.** A run can no longer say "this run's algebra was tested against its own definite margins". A document outside the test corpus no longer gets that test in the field. No record found shows the exact-witness check ever firing; both recorded firings were `Sym<f64>` false alarms. PR bodies and CI logs were not searched, so: unsure.

**Ratified text.** One sentence of E12's registry bullet changes. It was written by an agent fix pass (e80941f1bf) under your "lgtm" on #1712, not in your words.
- Today: "where a registration WOULD have, the run counts the contradiction (`SymCounts::registrations_contradicted`) instead of folding it away."
- Becomes: "where a registration would have, nothing is folded; a registered zero that decides is checked against the exact enclosure, and the test instrument checks registrations against definite signs."

`[simple.checks]` and `[failure]` need no change. The new assertion is `[failure]`'s "a load-bearing check panics in release" case. Today's check broke row 5's contract (its absence changed the receipt), which is why its columns cannot stay behind debug assertions.

**Reversible:** yes. The instrument is the same predicate behind a feature.

## For the orchestrator

**Q1, the `free_identity` exception: yes, amended.**
- The principle to state is that the assertion covers a zero whose number was computed from its own expression. Inclusion isotonicity then gives "contains 0": every leaf (`Param`, `Opaque`, `Hull`, `Lit`) stands for some point of its own interval.
- `free_identity` is the one door that sets a node beside a number it did not compute (`Sym { value: self.value, node: identity(&free).node }`, sym.rs).
- Recognising it by "the form reaches `INDET_FREE`" is right: derived, no flag. `SymOp::Free` is minted only in that impl. Its one caller is gated by `scripts/gates/free-identity-allowlist.sh`.
- Other doors of the kind: none in sym.rs. Every other constructor (`opaque`, `param(_over)`, `literal`, `nullary`/`unary`/`binary`/`ternary`, τ) takes its value from the same op it mints.
- I couldn't find `READS_BY_SUPPORT` under `crates/geom-core` or `crates/geom-brep` on main `ae076e0507` (the grep found no match). If it says that `Sym` derives a certificate instead of reading a stored one, it keeps the number computed from the expression, and so agrees with the rule. A read-by-support value at `Sym` would be a second door of the `free_identity` kind.

**Q2: yes, B is right.** My "every shipped zero is cross-checked" overstated it. The text above claims only consistency within the window.

**Q3: yes on both.**
- Planted-contradiction rows on every PR, the corpus lane on PRs touching `crates/geom-core/src/sym*`, and nightly.
- Feature-compiled AND installed at run time, through a thread-local recorder as `revalue::RECORDER` does. Main's new `read-pins-testing` (geom-brep) shows why: it is enabled through dev-dependencies in every crate's tests, which suits a check that costs one comparison per read. The same wiring here would put the walks back into every test row.

**Q4: closed by reading.**
- **Hull:** a `Hull` node's value is `enclosure_hull(a, b)` and its form is a fresh unknown keyed by the operands (`SymOp::Hull` in `combine`). A form that is zero for every value of that unknown evaluates, over its interval, to an enclosure containing 0. The same holds for `Opaque`.
- **Decision read:** it is asked only when `sess.witness == Some(Exact)` (`combine`'s `read`). `signed` encloses the decision over the parameter brackets in the outward-rounded `Interval`, and takes an arm only when that is one-signed.
  - On the value side, `Interval::select_le_zero` hulls both arms when its condition straddles 0, and `min`/`max` are envelopes. So the value contains the read arm's value in every case.
  - If the read's certified sign and the value's certified sign disagree, that is a bug, and the assertion is right to fire.
- **`frozen`:** left to the measurement lane, as you say. Don't claim it in the PR until measured.

**Q5: main changes nothing in the recommendation.**
- `decide_sign` at main `ae076e0507` has the same definite arm: `door_zero`, then `debug_assert!(!contradicts)` / the dispute count. The decide path ends where it did.
- DOCTAIL's `Role` (`k_stats::checking`) records the side reader's rays as `Role::Check`, so the slow decisions are checks by role. That is not a discriminator for this question: the wrong-population argument holds for choices and checks alike, so the check should not be kept for one role.
- `read-pins-testing` is a precedent for a test-only re-derivation feature, but not a home: it is geom-brep's, about certificate reads, and on in every test. The instrument's home is geom-core beside `sym-revalue-testing`.

**For the unit:**
- The assertion's message names the rung (`Rung`) and the enclosure. The soundness argument above goes at the site.
- Flip `predicate::tests::a_proven_zero_is_offered_no_tolerance` to a straddling fixture.
- Fix the profile-attribution defect: `door_zero` runs before `set_origin(Origin::Assertion)`. It disappears with the move.
- Count numeric-of-total for the revolved circle's slow predicates (`curve_pair_span_end_chord`) before promising its 1.0 s row.
