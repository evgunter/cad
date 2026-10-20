# Designer A, round 1: guarding a constructed arc's consistency at f64

## For Ev

**Recommendation (likely).** Base the choice of who decides an arc's consistency on the scalar's witness kind, and do the
deciding at the path door.
- **At an exact scalar** (Interval, and Sym over Interval, which is the whole symbolic tier) nothing changes. The
  construction's proof holds there, and nothing is decided again.
- **At a point scalar** (f64, and the f64-backed telemetry probe) every arc the path door lowers is read back
  before the loop is minted. The read-back uses validation's own table checks (start on carrier, landing,
  sweep range), plus one check per mode for an authored point the stored form drops: a `Via` arc's
  through-point `q`, which is the only such point I found.
- **Outcome.** A miss is refused with a typed error that names the step. It has the same three outcomes a table
  arc has: definitely off, in band (escalate), or below the scene's resolution.

The point-scalar sentence and the "could only confirm Zero" sentence in the profile README are rewritten to match.
D1's principle, "checked or holds by construction, never decided twice", stays as it is. It becomes literally true at
every scalar, which it is not today.

**Premise correction (sure).** The question asks whether to check something twice. But at f64 these facts are not
checked even once.
- A construction's "proof" is a theorem about the reals. It says nothing about the f64 values its arithmetic
  rounds to, and nothing bounds that rounding.
- The README's justification for not re-deciding is "re-deciding could only confirm Zero or escalate on dependency
  width, never catch an inconsistency". That is true of an enclosure, which always contains the real. It is false of
  a point value, and the three defects are exactly the inconsistencies it says cannot be caught.
- f64 does run the registrations today, but they are no check. `register_equal` at f64 compares each coordinate
  with a slack of ε times the coordinate's own size, and the registrant drops a refusal ("Disputed") without
  acting on it.
  - At R = 1.36e4 that slack is about 1.4e-5 m, so the tangent arc's 3.4e-8 m miss passed as "Witnessed".
  - `Center` and fillet arcs ("Decided") register nothing at all.
- **So the gap is not a missing second check. It is that "holds by construction" was granted at point scalars,
  where no construction proves anything about the stored values.**

**Ratified text, and where it came from (sure).**
- On PR #3527 Ev's words were: "everything is checked or valid by construction is still true", which asks for the
  general principle. My recommendation keeps that principle.
- "A point scalar takes on the construction's proof" and "re-deciding … never catch an inconsistency" are
  agent-written text, approved in passing. They were argued only from the Interval escalations that #3527 was about.
- That PR's own body already set the obligation: "every construction has to either prove its arc's facts or
  decide the inexact ones at T". The recommendation carries out that obligation at the scalars where the proof does
  not reach.

**What it checks and why the door.**
- A constructed loop is minted at the path door, by the builder and by every replay. Only the door still knows what
  the user wrote: `q`, the authored centre, the step. It is also the home the fillet door already uses for "never
  mint what the verify layer refuses", which reads back through `seg::build_seg` and `joint_tangency`.
- So the type `ConstructedLoop` keeps meaning "consistent", and `ConstructedProfile::validate` keeps skipping the
  three checks at every scalar.
- Junction tangency is already decided at validate, and the authored radius and centre are stored verbatim. That
  leaves two authored facts that nothing reads: the landing (for every mode) and `Via`'s `q`.
- Which scalars run the read-back is decided at compile time, from `Real::WITNESS`. It is never read off a number,
  so it does not touch D9 (determinism).

**The options as final states.**
1. **The status quo: fix each construction as it is found.**
   - It makes nothing new true, and it leaves possible a silent stored arc that misses its vertex.
   - Downstream code (B-rep vertex-on-curve, the sweep) relies on that agreement within ε. A table with the same
     miss is refused, so the same stored arc gets two different verdicts depending on where it came from.
   - The crate docs' promise, "verified-never-trusted", stays false at f64.
   - Not recommended.
2. **Check the landing at storage, at every scalar** (the shape the row sketches).
   - It catches the three known misses, but it decides twice at Interval and Sym, which is exactly the
     dependency-width escalation that #3527 removed (the hairline `cert4r2` and the guided replays). It also gives
     the symbolic tier new margins to carry for every arc.
   - It does not see `Via`'s `q` miss (up to 382 m).
   - Not recommended.
3. **Decide at validate, by witness kind.** `Consistency` would come from the loop's provenance plus `T::WITNESS`:
   at a point scalar a constructed loop gets the table checks.
   - It is simpler: one place, no new error type, and it costs the symbolic tier nothing.
   - But it cannot see `q`, which never reaches the loop. Its refusal names a loop and segment rather than the
     authoring step. And it splits the meaning of "constructed" by scalar inside validation.
   - Defensible; second choice.
4. **Read back at the door, by witness kind (recommended).** This is option 3's coverage plus `q` and the step's
   name, and it puts the guard where the claim is made.
   - Cost: the door calls into the verify layer's checks, which it already does for fillets.
   - It is easy to reverse: deleting the read-back gives back today's behaviour.

**What the refusal is.** It reuses the table's classification, carried on a `PathError` that names the step and
the check:
- definitely off, past scene resolution: refuse. This is a kernel defect, but evaluation code is total, so it is a
  typed refusal and not an abort;
- in band: escalate;
- below scene resolution: the `ArcBelowSceneResolution` shape.

It is not an assertion, because at a point scalar a legitimately ill-conditioned input and a bad formula cannot be
told apart, and the stored arc is unusable either way.

**Worked example (likely).** The pre-#5403 tangent arc, at ε = 1e-9 and R = 1.36e4. The read-back's landing margin
is 3.4e-8 m, definitely past ε. The scene resolves it: 16 ulps at 1.36e4 is about 5e-11 m, well under ε. So the arc
is refused, naming the step, where today it validates silently. The pre-#5441 `Center` arc at R = 1.37e6 probably
lands in "below scene resolution" at ε = 1e-9 (16 ulps there is about 5e-9 m). That is still a loud, typed refusal
rather than a silent one.

**Confidence.**
- The recommendation: likely.
- The symbolic tier's receipts (`m10_9_pins_interval`, `m10_10`) do not move: likely. They run at an exact witness;
  I did not run them.
- `q` is the only authored point the stored form drops: likely. I read the modes' lowerings but did not audit every
  verb.

## For the orchestrator

- **Brief compliance.**
  - I did not read the forbidden branch or PR #5449. While checking where the point-scalar sentence came from, a
    `git log --all -S` listed commit `bf97558f1b`, which is on `paths/stored-arc-landing-check`. I saw only its
    subject and stat, and stopped there. That branch apparently edits the same README sentence and
    `docs/predicate-dimension-audit.md` (ledger row "F23").
  - Nothing was built or run. The worktree is removed.
- **Where the provenance comes from.**
  - Commit `11941b7774` moved the point-scalar sentence from DESIGN into the README.
  - Ev's own words on #3527 are two comments that carry no "Generated by" footer: (1) "the DESIGN.md wording change
    feels far too specific … 'everything is checked or valid by construction' is still true", and (2) "looks good!".
  - The other `evgunter`-authored comments on that PR are agent-generated.
- **Stale doc (a defect off the question).** `crates/profile/src/path.rs`'s crate docs, under § Lowering, say "The
  #101 verify layer (`Profile::validate`) runs UNCHANGED on the lowered output: every constructed joint is verified
  at build". That is false since `Consistency::ByConstruction` existed: arcs are skipped. The doc of
  `Consistency::ByConstruction` in `seg.rs` also says "verified there … by the exact witness", which is untrue at
  f64. Both should be fixed by whichever option lands. Under option 1 they should be rewritten to say that nothing
  checks at f64.
- **Points the implementer should know.**
  - `seg::resolves` takes `LANDING_ULPS` / `ON_CARRIER_ULPS`, which count rounding in the `from_chord` lowering
    (lowered centre 4, sweep 4) as well as in the check itself.
  - For the door's read-back, the resolution gate should count only the check's own reading rounding. A
    construction's larger rounding is the very miss being caught, and must not widen the gate.
  - Whether to re-count is a detail for the implementation, not a fork.
- **A waste to remove (minor).** `BuiltArc::register` at f64 computes ten `register_equal`s whose answers it can only
  drop ("Disputed" is never acted on). Under the recommendation, that work at point scalars could be skipped once
  the read-back exists.
- **Effect on the telemetry lane.** The probe lane (`k_stats`'s `Probe`, which is inexact) will record the new
  read-back decisions as samples. That is correct, since they are real decisions, but any K-experiment counts
  pinned per predicate will move.
- **The evaluator's passes.** At f64 the evaluator's pass 1 is `ConstructedProfile::validate_recording`, and the
  guided Interval pass consumes its structure. A door refusal at f64 stops before either pass, which is the intended
  fail-loud ordering. I did not check whether any corpus row's f64 replay currently carries a silent miss that would
  begin refusing. A generator sweep over the arc modes at f64 should come before landing (the hairpin row's
  generator is the model).
- **Assumption.** I took "the symbolic tier = Sym over Interval", based on `m10_9_pins_interval` and `Sym`'s
  `WITNESS = T::WITNESS`. If any shipped path runs Sym over f64, it would get the read-back too, which is correct
  but would move its receipts.
- **Not on main.** The `Via` row (`a-via-arcs-carrier-is-never-checked-against-its-authored-through-point`) is not
  there, so I took the 382 m fact as given.
