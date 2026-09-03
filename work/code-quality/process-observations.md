<!-- Moved verbatim from `docs/SMELL-SCAN-2026-08.md` §C at the tracker migration (2026-09-03); the findings it cites are items in this directory or closed records in git. -->

# §C. Process observations

Ev's standing request (2026-08-18): *"i wonder how they happened; it'd be
cool to get a 1-3 sentence postmortem on each covering the rationale and
whether (per the associated pr description, A/B log, and/or orchestrator's log)
the reviewer flagged it as an issue. i think this could be really useful for
improving process."*

Each finding carries a **Postmortem** line where one has been done. This
section collects only the observations that generalise. **It is being filled
in as the postmortem passes land — treat it as incomplete.**

## C1. The review protocol is claims-driven, not surface-driven

This is the mechanism behind most of the façade findings, and the evidence cuts
against the obvious reading. Reviewers are handed *"explicit claims to
falsify"* (`memories/orchestration-model.md:152`) and they falsify them
**behaviourally and well** — LIB-U1's reviewer compiled `use topo as _;` to
kill a false documentation claim. The effort is *not* aimed at prose over
behaviour.

It is aimed at **asserted** things over **unasserted** ones. A code-free
module, a 449-line accumulated header, and a duplicate type name across a
façade assert nothing, so nothing points a reviewer at them.

## C2. Disclosure functions as immunity, and the scoreboard rewards it

The A/B rubric's headline column is *"silent devs"*. PR #364 scored **"0 silent
(5 deviations reported)"** — and one of those five reported deviations was the
constant `DocumentId` that makes two Python-authored documents un-coexistable
in a workspace. Writing a hole into the PR body converts it into a *positive*
metric. There is no counter-metric asking whether a disclosed deviation was
**acceptable**, only whether it was disclosed.

## C3. Deferrals must land in a register that executes

The repo has exactly one self-enforcing register — `docs/guide/north-star-audit.md`,
whose test fails as doors land — and several prose ones. Every deferral behind
a finding here went into prose: a spec sentence, a constructor comment, or a
residual register that **had closed the day before the PR merged**. Prose
registers have no way to notice. Compare S15's row sort: **zero of its nine
rows had a tracked issue** when this was written, even though the repo
demonstrably knows how to do better (issue #214 for a census,
`attach.rs`'s KNOWN HAZARD block for a named-and-pinned gap, since retired
with the gap it named). D5 closed
that gap the way this section asks — the one row it could not fix left as
**#708**, pointed at from a KNOWN HAZARD block, rather than as prose in a
merged PR body.

## C4. Nothing in the process reads a whole file, a whole namespace, or a whole crate

Specs are per-unit, diffs are per-unit, reviews are per-unit, log rows are
per-unit. The ~1170-lines-of-narrative-over-~180-lines-of-code ratio, the three
`DimensionError`s, and the `lib.rs`/`workspace.rs` contradiction are all
properties that exist **only above the unit** — which is precisely the altitude
at which this repo had no reviewer before this scan.

## C5. Documentation is a growth sink under review pressure

#232's completeness MINOR could only be discharged by making the essay
*longer* (nine more types plus a paragraph on why each was missed). Note the
timing against Ev's own later standing brief line — *"comments state the
INVARIANT, not the history: no retired-type archaeology, no unit tags"*
(2026-08-08) — which is exactly what `closure.rs:135` and `select.rs`'s
unit-tagged sections violate, having been written days before it and never
re-swept.

## C6. Some of these were ratified before they were written

Three of the seven findings in this batch trace to a **design document or spec
clause**, not to a lapse: S23's data-switched dual role
(`CURVED-DESIGN.md:796`, *"one structure, two duties"*), S26's
area-as-denominator (PR #192 deviation 1), and S30's counterfactual third pass
(TESS-SPAN D-4). **The design conversation is where these entered, and it is
the only place they could have been caught.** No amount of implementation
review would have found them, because the implementation was faithful.

## C7. The deviation ledger works as an amnesty, not an alarm

Independently confirmed by two postmortem passes over different scopes. PR
bodies disclose the shape with near-perfect honesty — *"deviation report
pending"*, *"no gate consumes it"*, *"`SAFE_ASPECT = 5` … above the derived
line"*, *"`MAX_GRID_RETRIES` 4 -> 6"* — and **disclosure closes the item**.
Nothing escalates when the same deviation appears in a third PR. Cf. **C2**:
disclosure also scores as a *positive* on the A/B rubric's "silent devs"
column.

## C8. The acceptance rows for degenerate modes are written so they cannot fail

Both correctness-shaped findings in this batch share one failure, and it is
mechanical enough to be a rule.

- S23's floor row is named
  `..._refuses_typed_even_though_branches_were_found` — **the premise excludes
  the failing mode.**
- S26's area row asserts `area_pad > 0.0` plus containment — **both monotone
  in the wrong direction**; the row gets *easier* as the enclosure degrades.
  Its neighbour twelve lines up asserts a real tightness bound, so the file
  contains both the right and the wrong shape.

Neither is a weak assertion by accident; each was written to pin the *feature*,
and the degenerate mode was never a row. The candidate rule: **every "never
silent" or "certified enclosure" claim needs a row that goes red when the
guarantee degrades, not merely when it is violated at a chosen fixture.**

## C9. Reviewers are exceptionally strong at soundness and blind to structure

This is the sharpest characterisation the postmortems produced, and it is
evidenced in both directions. The same reviews ran 8000-matrix SVD
differentials, re-derived a meters conversion by hand, found a certificate
excluding true 2π by ~1111 widths, and wrote the strictest floor-refusal probe
in the repo — and produced **zero** findings on: a mode switch on
`is_empty()`, a two-ε signature, a file holding four engines, or three
parallel CDT pipelines.

Structural findings appear only as **side-effects of bug hunts** (#472's
"unguarded at four sites", #313's shared area rule). Nothing in the protocol
asks a structural question directly.

## C10. Cross-lane invariants do not propagate; only imports do

`planar::triangulate_chart`'s header warning, PR #116's pre-scan, and PR
#157's `SelfTouchingTrimLoop` are **three encounters with one hazard**, each
closed inside its own lane — while the lane that predates all three
(`curved.rs`) still carries the ordering the warning describes. A fix that
establishes an invariant needs an explicit sweep of sibling implementations as
part of its **acceptance**, not just its own regression row.

**The sweep was run (2026-08-19).** For `curved.rs` the *ordering* came back
CLEAR — the ordering is inert there because `curved` builds no crossing
bookkeeping for a split to corrupt — but the clearance is not where it ended.
What the sweep actually found was an *unstated premise* doing the work
(`curved`'s UV domain is its own bounding rectangle), which nothing checked;
#648 turned it into a typed refusal
(`TessellateError::UnsupportedCurvedDomain`). Read that as the general shape:
a sibling that survives the sweep survives *for a reason*, and **the reason
is either enforced or it is the next defect** — writing it down is the
minimum, not the deliverable. S28 carries the detail.

**And the reason has to be right.** #648's first pass wrote the premise
down as an *exact* property and cited the mechanism that supposedly
guaranteed it ("assigned once per edge"). Adversarial review executed
the counterexample in an afternoon: the mechanism guarantees the
property only when a side is one edge, and every in-tree fixture and
the whole wild corpus happened to satisfy that, so a green suite proved
nothing. The refusal became a **false refusal** on valid parts (#653).
**#648 then corrected the premise only where it bit** — in
`curved.rs`'s own docs — leaving `walk.rs`'s module header (the home of
the mechanism) still asserting it flatly, three lines from a paragraph
#664 was rewriting, so for a milestone two files in one crate said
opposite things about one sentence. #664's fix pass qualified it in
`walk.rs`, `mesh/lib.rs` and `Chart::rim_v`. *A correction lands where
the defect bit; the claim lives wherever it was written down, and the
two sets are not the same.*
Two things generalise. A premise stated as *exact* is a claim about
float representation, not about geometry, and needs a fixture that is
adversarial to representation — an oblique placement and a subdivided
edge, not another shape. And a sweep that records pass/fail where it
could record **margins** discards the evidence that would have shown
the claim was fragile; #648's payload now carries the margin for the
same reason.

**The sweep the fix itself owed, run in the same PR (2026-08-19,
#653's option 2).** The invariant established there is *"a curved
face's iso side gets ONE constant coordinate, however many edges carry
it"* — and §C10's rule is that such an invariant is swept across
sibling implementations as part of acceptance. Two siblings existed and
both were taken in the same change rather than left for a follow-up:
**meridian columns** (u) were the reported half, and **rim rows** (v)
are the same shape one axis over, so `iso_side_starts` classifies both
and the `Rim` arm shares its row exactly as the `Meridian` arm shares
its column. The rim half has no live reproducer: a `split_edge` keeps
one carrier circle, so both sub-edges compute the same `rim_v` bitwise,
and every DIRTY row in the 1524-sweep came from a *meridian* split. It
is reachable through import all the same, where two co-`v` arcs are two
independently stated `CIRCLE`s. Fixing only the half with a live
reproducer is exactly the failure mode this section is about.

**The rim half is therefore SHIPPED WITHOUT RED-WHEN-REVERTED
EVIDENCE, and that is scheduled, not shrugged at.** Its `Rim` arm is
*executed* on every rim split in the #653 row — the code runs, it just
cannot come out differently, because the two sub-edges of one carrier
read the same centre and radius. The reproducer that would change that
is a STEP file stating two co-`v` `CIRCLE`s independently, and the
machinery to mint one is already in this PR:
`crates/step-import/tests/fixtures/split-iso/generate.py` writes
hand-authored AP214 with per-edge carriers, and its `arc_edge` helper
already emits one `CIRCLE` per edge. The work is a fifth fixture —
split the D-prism's bottom arc `e1` in two, each half with its own
`CIRCLE` entity, obliquely placed — plus a row beside
`a_split_iso_side_meshes_watertight_under_an_oblique_placement`. It is
named in `curved.rs`'s §C15 blind-spot list at the #653 row so a reader
of the code meets it too, not only a reader of this document.

**And the global-floor class came straight back, in the PR that cited
it.** #653's headline row opened with `assert!(checked > 100)` against
an actual 254, so a 60% collapse would have passed — the same shape as
`walked >= 14`, one section up, written by an author who had just read
that paragraph. Now per-fixture: every edge of every fixture must
produce a placed body, and the helper returns its skips instead of
`continue`-ing on them. The lesson is not "remember the rule"; it is
that a *derived* floor (fixture's own edge count) cannot go stale the
way a transcribed number does, and a transcribed number is what both
failures had in common.

The third sibling was checked and **is not one**: `trimmed`'s
pcurve-driven lane never *derives* a constant coordinate from a
midpoint evaluation — it reads the stored pcurve — so it has no
derived-per-edge premise to break. (An `IsoLine` pcurve does carry a
constant, but as stated data: two sub-edges disagreeing there is a
statement about the file, not about the walk.) `planar` has no iso
coordinates at all.

**A second class the same review named, swept in #648.** *"Sweep a list of
bodies, assert a global count"* has the same hole as a global floor:
`curved.rs`'s row asserted `walked >= 14` against 20 actual walks, so the
boolean-cut die pip — the fixture the row exists for — could have dropped out
through either of `curved_walks`'s two silent `continue`s and left the row
green. Fixed to per-fixture participation. The two siblings the reviewer
named, `crates/mesh/tests/review_m2_pr6_walk_shapes.rs` and
`crates/mesh/tests/revolves.rs`, were checked and are **clear**: both assert
inside the loop (per-θ `check_mesh_acceptance` / `signed_volume > 0`), so a
fixture that stopped contributing cannot hide behind its siblings. Neither
carries an accumulated counter.

## C11. Self-disclosed copies are invisible to everyone, and greppable

Every duplication in S18 is **honestly declared in prose at the copy site**:
*"the profile crate's ratified bulge closed forms, **verbatim**"*; *"**verbatim**
`crate::recognize`'s"*; *"the face bound's quotient-rule assembly **one
dimension down**"*; *"the `boolean::boxes::face_box` construction
**re-derived** in the evaluation lane"*. The codebase is candid about all of
them and **nothing in CI, review, or the log ever reads that prose**.

A grep for `verbatim|re-derived|ported from|mirror of` across `crates/*/src`
would surface most of S18 in seconds. This is the cheapest concrete mechanism
any postmortem in this document produced.

**And it is a floor, not a census.** The standing prose sweep finds only
**disclosed** copies; undisclosed copies are the majority, and only their
*data* can find them — the same constant, the same magic number, the same
literal ladder, written twice with no sentence admitting it. Run the constants
grep beside the prose sweep, and read a clean prose sweep as evidence about
the prose.

**Three rules the sweep's own execution established**, relocated here from the
`sweep`-crate instance that carried them:

- **Any claim of the form *"X now has one home"* is owed a marker-vocabulary
  grep of the tree, in the scope it claims** — over the **working tree**, not
  over `git log`. A unification that asserted one home was falsified by a
  marker already sitting in a file inside its own declared scope, and both of
  that unit's sweeps missed it: one grepped the *not-sharing* vocabulary, the
  other grepped the *marker* vocabulary but over history. It is one command.
- **Every hit has to be read; the grep is a candidate list, never a count.**
  Measured on one crate: 13 hits, 9 or 10 of them false positives —
  *"verbatim"* overwhelmingly modifies a **value** carried through unchanged
  (a carrier, a parameter interval, a caller's centre and radii), and
  *"re-derived"* twice cited a book.
- **A duplication declared in words the tree's own greps do not carry is a
  duplication nothing will find.** The one honest marker the vocabulary missed
  said *"the same transform `swept_segments` applies"*; a first rewrite to *"a
  HAND-APPLICATION of"* was still invisible. That is this observation's own
  mechanism, and it is the argument for the vocabulary being written down
  somewhere a marker's author will read.

## C12. Specs name the *method* to copy but never the *home* to reuse

*"Port `point_in_loop`'s METHOD to 2-D"*; *"the face Hessian … **AND** the
curve-side gate"*; *"built on the MERGED M8-2 template — deliberately NOT
lifting #309's unmerged machinery"*; *"reads `containment.rs` for METHOD only
(**no refactor**)"*. The convention describes the source as a **reference to
imitate**, not a dependency to call.

Two structural amplifiers: **concurrent file-disjoint lanes** are mandated by
the orchestrator for good reasons and leave behind no unify-after-merge
obligation; and the **K-ledger rule** (new predicate names = new rows) makes
the copy the path of least resistance, since parameterizing would disturb the
census.

## C13. Half-fixes read as whole fixes when the finding is narrower than the drift

Three independent instances:

- #152's reviewer forced **one home for `tangent_certificate_lane`** — and the
  same fix pass shipped **two divergent sample schedules**, with a third
  hardcoding the literal `9` two days later.
- The perf scan corrected `face_box`'s stale **NURBS** premise while the
  identical stale premise for **planar-with-conic-rim** sat fifteen lines from
  the text it quoted.
- M4 PR 5's reviewer correctly identified **information loss** (*"catch-all
  launders tier-2 diagnostics → preserve real reasons"*); the remedy was
  `format!`, and `DESIGN.md` D4 ¶2 then **canonised that outcome as the in-repo
  precedent**.

The generalisation: a review finding gets discharged **at the granularity it
was phrased at**. An information requirement is met by stringification; a
one-home requirement is met by unifying the name and not the schedule it
drives.

## C14. Pins guard the invariant as it was reachable *then*

Three of S22's four rows were introduced by a **later** change to code whose
contract an **earlier** reviewer had checked and pinned — and in each case the
pin still passes:

- `survives_eps_row_bitwise_independence` pins *"ε is read once, for pole
  identification"*; #481 added a second structural ε read; the test passes
  because only a **foreign STEP file** produces a nonzero residue. (#664
  removed that second read and corrected the comment — which by then had
  to name a *third* consumer, #648's domain guard, rather than restore
  the original wording. A stale claim does not become true again by
  undoing the change that falsified it.)
- `parallel_schedule_preserves_verdict_logs` pins **thread** confinement;
  ASM-2A broke **re-entrancy**.
- Four process-isolated binaries pin the ε global's init discipline; none
  observes that a *document*'s ε still commits into the same lock.

Nothing re-derives a pin when a new caller arrives. And a stale comment on a
still-passing test reads as **evidence the invariant holds**.

**The weaker case — no pin at all — is #651**, raised by the style review of
#646. The rule it produced is in `docs/prompts/reviewer-style-lane.md` §Q6 (a
measured claim owes a mechanical guard, a scheduled register that
re-measures it, or a written reason at the claim site that it can have
neither); the classification sweep is a comment on #651, and is not
repeated here. #667 continued it over a corrected population (a
provenance-vocabulary pattern restricted to comment text, deduplicated to
the comment block and filtered by a numeral: **197** blocks, 37 of them
claim-bearing, against #663's 146 `measured` lines) and its rows are the second comment on the
same issue. Its finding for THIS clause is C14's own shape one turn
further on: `ci.yml` runs more registers than #663 found, and two of them
gate — but each re-takes a **subset of the columns** of the document it
produced, so "`docs/TESS-BUDGET.md` is re-measured per merge" is itself a
guard described wider than it is read. The register roster and what each
one actually re-takes live in the sweep comment, not here.

---

## C15. A sweep's result is worth nothing without a statement of what its pattern cannot match

**Observed three times in one day**, across three independent wave-1b fix
lanes, each of which reported its sweep as verified and each of which was
blind in exactly the shape it was hunting:

- **#632** scanned for arms beginning `RoleSeg::` at the wildcard's
  indentation, so **every arm wrapped in `Some(…)`, `Ok(…)` or a tuple was
  invisible** — which is the shape of what it missed. **Its conclusion did not
  survive** (established by #731, 2026-08-20): the same pattern was blind in a
  second way it did not name — a **binding** catch-all (`other => other`) is
  not `_`, and a match written through `use RoleSeg as R` never spells
  `RoleSeg::` in the window the scan required — and `eval/anchor.rs`'s
  `remap_seg` was both at once, a live fail-quiet of exactly the shape #632 was
  hunting, under a body that reported *"no fail-quiet wildcard in any `RoleSeg`
  or `Qualifier` match in the workspace."* The instrument that found it asks
  **rustc** rather than the text — `--force-warn
  clippy::wildcard_enum_match_arm`, reading each diagnostic's missing-variant
  list — which no alias, wrapper or indentation can fool. Its own blind spot,
  stated because that is this clause: an enum nested inside `Option`/`Result`
  is attributed to the outer type, so the lint cannot see the very arms #632's
  correction was about, and it says nothing about `if let` / `matches!`.
- **#635** used a line-scoped `rg`, so a claim that **wrapped across a line
  break** could not match. Two survivors of the premise it was sweeping sat
  in the file it had just edited, one of them 25 lines above the list it
  fixed.
- **#639** scanned **prefixed** codes (`LIB-*`, `ASM-*`, `Mn`, `PR n`,
  `#nnn`) and so could not see **bare** clause letters (`F5`, `G1`, `C4`,
  `S13`). It therefore shipped S37's own named example — `LIB-DOORS F5` — in
  a live Python `__doc__`, in one of the three crates its body reported at
  zero.

This is C11/C13's mechanism one level down. Those say a class gets fixed at
the reported instance; this says that even a lane *trying* to sweep the class
will under-report by exactly the margin its pattern cannot express, and will
then state the shortfall as a verified negative.

**The same rule binds a DELETION's census, whose pattern is scoped to the
deleted names** — so it structurally cannot see what the deletion *orphans*,
only what still refers to what went. A sweep for `mat2|affine2` is silent
about the helper whose last caller was `Mat2::identity`.

**And it binds a REPRODUCTION, whose fixture set is its pattern.** A
probe that replays a recorded stream is evidence only about the rows its
fixtures actually generate: an all-planar corpus carrying zero arc-rung rows
cannot witness a change to the arc rungs, however many rows it reproduces
byte-identically. *"The probe reproduced"* is therefore not the same claim as
*"the change was neutral"*, and a lane owes the second one a corpus that
reaches the arms it touched.

*The original entry read "in all three cases the conclusion happened to
survive; in all three the method did not." **Corrected by #731**, and the
correction turns on which conclusion is meant, so the reading is stated
rather than assumed.* Take **the conclusion** to be **the negative result
the lane reported at the time** — *"nothing else matches", "the class is
closed"*. On that reading **none of the three survived**, and the bullets
above are the evidence for all three: #635's two survivors sat in the file
it had just edited, #639 shipped an instance in a crate its body reported
at zero, and #632's *"no fail-quiet wildcard in any `RoleSeg` or
`Qualifier` match in the workspace"* was falsified by #731's
`eval::anchor::remap_seg`. The original sentence was true of a different
quantity — whether the class turned out to be closed once each lane's own
correction had landed — and that is a claim about the corrected state, not
about the sweep. It is the sweep this clause is about.

What differed is only how the shortfall surfaced: #635's and #639's inside
their own programme, #632's not until a lane a day later ran an instrument
of a different shape. Where a sweep's blind spot is unstated, *"the
conclusion happened to survive anyway"* is itself an unverified claim.

**Proposed in #666, awaiting Ev's sign-off** — it amends the review
instrument, which is Protocol v5's territory. The rule text lives in
`docs/prompts/reviewer-style-lane.md`; it is not restated here, because two copies of
one rule is the shape this report exists to hunt.

## C16. A prose-hygiene pass can manufacture the defect it exists to remove

`props/quad.rs:42`'s liveness claim — the one row of eleven that #635
classified as a **lost invariant** rather than benign rot — was itself written
by a **previous stale-claims sweep** on 2026-08-05 (`git log -S` puts it in
`e2222617`, whose message names its own "§7 stale-claims sweep tranche"). It
replaced **two honest sentences** with one naming the wrong engine, and missed
a third, inside the same function, that contradicted it for the next two weeks.

The generalisation is not "sweeps are bad". It is that a pass which rewrites
prose to state the present will, wherever its author guesses at liveness
instead of checking it, **launder a guess into an assertion** — and the
resulting sentence is indistinguishable from a verified one to every later
reader. That is the argument for S39's classify-before-you-touch discipline
being permanent rather than a one-off framing of one finding: the question
"benign rot or lost invariant?" forces the check that the 2026-08-05 pass
skipped.

**A record that replaces its subject in place has the same hazard, and it is
not one PR's slip.** Version control becomes the only surviving copy of what
was replaced, so a replacement that *characterises* the original ("the list
was too narrow", "the citation was wrong", "the finding missed X") asserts
something no future reader can check without `git show` — and reviewers do not
reach for `git show` by default. The failure mode is specific and it is not
carelessness: a correction is written while looking at the NEW tree, and the
sentence about the OLD text gets composed from memory of it. The cheap
discipline is to **quote the original inline whenever the replacement makes a
claim about it**: a quotation survives the replacement, a characterisation
does not.

Method note, cheap and reusable: this repository's checkouts are **shallow**,
so `git blame` misattributed that sentence by ten days. `git log -S` is the
instrument for dating a claim. The same shallowness makes a **cited SHA look
like a bad citation**: this document's own scan base `4258584` is not an object
in a fresh agent container and resolves only after `git fetch --unshallow`
(D23). A pointer that does not resolve is therefore not evidence of a wrong
pointer until you have unshallowed — check before reporting one.

## C17. "Green when run alone" is not a verification when lanes share a target directory

#639 reported `cargo test` green for three crates and shipped **ten broken
string assertions**. Two causes, and only the second is the author's.

The orchestrator had put six concurrent lanes on one `CARGO_TARGET_DIR`, which
**clobbers across git worktrees**: at least two lanes were served results from
another lane's binary. Confirmed by counting — the same crate reported 156
tests on the shared directory and 155 on a dedicated one, from identical
sources. So the lane's re-check, run to rule out contention, was green for the
wrong reason.

The author's half is that two failures **of the same shape** — a string
assertion on text just rewritten — were read as load flakes rather than as the
first two members of a class. This is C13 in the verification lane rather than
the fix lane.

Two rules follow, both now in force: **one target directory per lane**, and a
run is trusted only when a `Compiling <crate>` line was observed. And the
deeper one: a lane that rewrites text asserted anywhere must run the affected
crates' **tests**, not their builds — `cargo build` cannot see a broken
`assert!(msg.contains(…))`, and every one of the ten was invisible to it.

## From the second scan (C18–C25)

Continuing this document's process numbering. **Renumbered by the merge:** these were written as C15–C22 on the premise that §C ran to C14. It ran to **C17** — and the first document's own forward pointer repeated the same wrong number, so the collision was invisible from inside either file. They are **C18–C25** here. The
first scan's §C was written from PR descriptions, A/B logs and
orchestrator logs. This one is written from something better: **a
controlled second look at code that the first round's findings had
already been applied to.** That is the closest this project has come to
measuring its own fix quality.

## C18. Two of my dispatch briefs were wrong, and both agents checked anyway

I wrote thirteen briefs. Two contained a false premise:

1. I told the `topo` agent that CI never runs the corrupt-input contract
   in release. It does — `ci.yml:769-816`. The agent opened its report
   with *"One correction to the dispatch premise"* and then reported on
   what the job **covers** rather than on whether it runs.
2. I told the `geom-brep` agent that S26 (area enclosure metering) was
   fixed, and asked which direction the metered faces moved. It is not
   fixed. The agent: *"Whatever briefed this as fixed was reading #472's
   deferral, not the tree."* My brief was internally inconsistent — it
   said #472 deferred it and then asked which direction moved — and the
   agent resolved the inconsistency against the tree rather than against
   me.

**This is the finding, not the errors.** A dispatcher's brief is the
highest-authority text a scanning agent sees, and the failure mode it
invites is confirmation: an agent told "X was fixed, check for
regressions" can produce a plausible regression report about a fix that
never landed. Both agents refused the frame. What made that possible is
plainly in the stance — *"a textual justification is not a defence"* and
*"your taste is evidence"* generalise to the brief itself, and neither
agent needed to be told that a dispatcher can be wrong.

**Worth making explicit in the reviewer's own document anyway.**
`docs/REVIEW-STYLE-DISPATCH.md` §3 already tells the dispatcher that
*"reviewers correcting the dispatcher is a working lane, not a
malfunction — say so in the brief"*, and
`docs/prompts/reviewer-style-lane.md` §1 does not yet say it. The
missing sentence is that the dispatch is a **hypothesis**, and that
contradicting it is a first-class result. It cost nothing here; it will
not always. (I have not edited either document — that is a ratified
process artefact and the change is Ev's call.)

## C19. The dominant defect shape is now "the fix pass had the file open"

This is the single strongest signal in the scan, and it is a *new* shape
— the first scan could not have seen it because there were no fixes yet.
Count: S59 (the ruling swept to two gates, not the third, in the same
directory), S60 (`volume_pad` fixed, `area_pad` twelve lines away not),
S63 (same), S68 (`split_edge`'s discards ten lines below the same diff's
`unreachable!` conversions), S74 (markers deleted at the copy sites), S80,
S84, the `Bounds` headline (`Enclosure` and `CertifiedEnclosure`
corrected by the same sweep, the trait D1 demoted out of the role left
saying it), S101 (the sweep deleted the fact rather than re-aiming
the pointer), S102, S110(b), S114(f), S116(m). (`S110` has since
closed, with #1329; the citation is to what it recorded, which is how
every closed number in this list reads.)

The mechanism is consistent enough to state as a rule: **a fix pass
scoped by the finding's citation list sweeps the citations and stops.**
The sibling instance is one screen away, in a file the author had open,
and the scope sentence in the fix's own prose is what makes it
invisible — several of these fixes *state* their scope
(`euler{,_ring,_kill}.rs` plus `link_half_edges`; "the per-variant
ladders stay where they are"; "the reported instances"), and the
statement reads as completeness.

**The rule already exists, on both sides, and it did not fire.**
`docs/REVIEW-STYLE-DISPATCH.md` §2 names *"the fix reproducing the
defect it closes"*; `docs/prompts/reviewer-style-lane.md` §3's
class-not-instance rule says that sweeping only the reported instance
*"is a **half-fix** and should be labelled one"*; and
`docs/prompts/implementer-discipline.md` §5 puts the obligation on the
fix pass directly: *"If your unit fixes an instance of a class, say what
pattern you swept with and **what that pattern could not match**."*
Thirteen instances landed anyway. The interesting question is not what
rule is missing but why the one we have does not bite.

**Two mechanisms, both visible in the artefacts.**

- **§5's trigger is the author's own classification.** *"If your unit
  fixes an instance of a class"* — and the recurring failure is a fix
  that was never classified as a class fix. `volume_pad` was fixed as a
  row, not as an instance of the monotone-enclosure class (S60). The
  both-operand-orders ruling was applied to the gate that reported it
  (S59, S63). The condition is exactly the judgement that fails.
- **§5's deliverable is a pattern, not a hit list.** An author who greps,
  sees three hits, fixes one, and writes *"swept `euler*.rs`; the pattern
  could not match delegating callers"* has complied in full. And a scope
  sentence reads as completeness even when the claim above it does not
  share its scope: `euler.rs:24` asserts the universal — *"at every
  write"* — while its evidence is *"these modules"*, which is how
  `split_edge` ended up three discards deep in the same diff (S68).

**So the amendment is small and specific: make the trigger unconditional
and make the artefact the hits.** Grep for the *shape* — not the symbol
(Q4's distinction) — before writing the scope sentence, and put the hit
list and its disposition in the PR description, one line per hit: fixed,
or not-this-unit and why. A pattern with no hits recorded is a claim; a
hit list is a receipt. S60 is the cleanest demonstration:
`rg area_pad crates/*/tests` returns two tightness-relevant sites,
neither bounds it, and the fix pass was editing the file that contains
both.

## C20. C11's mechanism is real and has now been observed running backwards

C11 (first scan): every duplication in this codebase is self-declared in
prose (`verbatim`, `re-derived`, `ported from`, `mirror of`), and
nothing ever reads that prose. It was proposed as the cheapest
actionable mechanism available.

S74 is the counter-case that proves its value: the `revolve`/`extrude`
twins carried exactly those markers, and a consolidation commit
**deleted both markers while leaving both copies**, replacing them with
a sentence asserting the two are not twins — a sentence that is
factually wrong about `reverse: bool`. The greppable evidence was the
only evidence, and a well-intentioned cleanup removed it.

If the marker vocabulary is ever mechanised, the guard has to include
"marker removed without the code converging", not just "marker
present".

## C21. Q3 ("can this test fail?") is carrying the scan

Of ~110 findings, the largest single class is assertions that cannot go
red: S60, S75, S76, S78, S84, S91, and the ten sites in S110 (closed,
#1329 — eight had gone before it), plus
S66's acceptance suite, S72's pad probes and S73's `ratio`. Several were
found by *executing* a mutation — the `interval-transcendentals` agent
set `PAD_ULPS = 64` and `PAD_ULPS = 0` and reduced the rounding helpers
to round-to-nearest; the `scripts/gates` agent planted fixtures against
every gate. **Every claim so produced held.**

One sub-shape dominates:

- **Monotone in the wrong direction.** `area_pad > 0.0` plus
  containment; `assert_contains` on a widening enclosure;
  `worst_ratio ≤ 1` as `bound` grows; `holds(&box, sample)` on a box
  that only widens; `!contains(&anchor_idx)` on a list that may empty.
  The pattern is: *the assertion is satisfied more easily by exactly the
  degradation it exists to catch.* `reviewer-style-lane.md` Q3 already
  names this one; these are measurements of it, not a gap in the brief.

**A second shape — a skip reading as a pass — is deliberately left
un-rolled-up.** The instances stand on their own (S84 and the
`else { continue }` / `if let Ok(...)` / tolerant-arm /
`println!("SKIPPED")` sites cited with them). A class-level rule was
drafted and dropped: it was written around giving skips *floors*, which
concedes the skip, and the prior question — whether a test should be
skipping at all — is the one to answer first. Recorded here so the
next scan re-opens the question rather than re-proposing the floors
(Ev, 2026-08-20).

**Cheapest mechanisation available:** for every enclosure-style
acceptance row, require a *ceiling* alongside the containment. The
volume rows already do it; the pattern is three lines and it is the
difference between S60 and a row that works.

## C22. Executing the mutation beats reading the code, and it was rare

Three of thirteen agents ran experiments rather than only reading. Those
three produced the scan's most certain findings — every "green with the
guard removed" claim is a fact, not a judgement, and none needed a
steelman pass. The other ten produced findings that are mostly still
*questions*.

This is a cheap upgrade to the brief: **when a finding is "this guard
does not guard", try to break it.** A scratch copy of the crate and a
one-line mutation is minutes, and it converts a `likely` into a `sure`.

## C23. The A1 rule (non-improvement deviations owe a scheduled followup) has not taken yet

S115 is six disclosures written *after* the rule, none with an issue
number or a named plan unit, several stating "unscheduled" as though it
were the schedule (`tools`' `agree` column says it in two crates
independently; `doc-gate.sh` says *"a row is owed … and it is
unscheduled"*). S90 is the sharpest version: the D1 ruling's three
*smaller* residues all got issue numbers (#687, #700, #701) and the one
seam it actually left unguarded got prose.

The disclosures are honest and well written, which is exactly the C2
diagnosis. `docs/REVIEW-STYLE-DISPATCH.md` §4 already warns the
dispatcher not to let the `## Style` section *"become the place where
known problems go to be recorded and forgotten"*, and Q6 exists to close
it — so this is not an unnamed problem. It is a named problem with no
mechanism.

What the rule lacks is a place that *executes*; C3 said this. The
register has to be mechanical: a grep for the disclosure vocabulary that
fails without an adjacent issue number would be a gate in the style of
the fourteen that already exist — and S63 is the warning about how
carefully that regex would need to be written, since every one of the
six existing grep gates has a hole of exactly that kind.

## C24. The style brief worked, and here is what it cost

First use at scale: thirteen agents, ~110 findings, of which I judge
roughly a dozen to be over-reaches and ten hand-verified to hold. The
question-numbered self-reports at the end of each agent's output (*"Q1
— findings 2, 4, 6, 7; Q4 not exercised, no diff to invalidate
against"*) were unexpectedly useful as a coverage receipt, and I would
keep them.

Two observations for the next revision (recommendations only — I did not
edit `docs/prompts/reviewer-style-lane.md`):

- **Q8 (read a whole file end to end) produced the findings nothing else
  would have.** S116(e) (the euler header is now two screens of another
  module's contract), S116(g) (60% comments), and the demos agent's
  honest note that `lily.rs` was sampled rather than read. C4 said
  nothing in the process reads a whole file; Q8 is the fix and it is
  working.
- **The stance's "report more rather than fewer" produced a long tail
  that needs a coordinator.** Roughly a third of the raw findings became
  roll-up bullets here rather than standing rows. That is the right
  outcome, and saying so in §3 ("what your findings must look like")
  would stop agents calibrating toward fewer, better-defended findings —
  the defended ones are not the valuable ones.

## C25. Documentation growth is still the default response to a finding

C5 measured this in the first scan; it has not turned. Measured this
round: `real.rs`'s `Bounds` block 156 → 234 lines, and 399 by the time a
lane took it;
`crates/mesh/src/curved.rs` 243 → 712 production lines, 60% comments,
with ~180 doc lines over ~55 lines of guard code (S116g);
`SAFE_ASPECT`'s doc ~20 → ~50 lines while the constant did not move
(S116h); `crates/topo/src/euler.rs`'s header +55 lines (S116e);
`scripts/gates/bounds-allowlist.sh` — 130 lines of header defending a
20-line function, restating a ledger it declares it is not restating
(S116m). **Re-measured: 131 lines when #791 opened, and #791 takes it to
204** — it cut five lines of comment archaeology and compressed its own
additions twice, and the remainder is three newly disclosed blind spots
(GAPs 3, 4, 5), the reason the definition skip is exact text, and the
correction of a mitigation that was published false. The finding is *not*
discharged by that accounting and the number is recorded rather than
restored, and **placed as D106** — split the ratification ledger out of the
script — because an argument recorded is not a row that executes.

**And the growth is itself the finding's answer, which is worth more than
the row.** A gate whose gaps are honest is longer than one whose gaps are
silent: every line #791 added past its own fix is a blind spot named, a
false claim retracted, or a repair the next reader is told *not* to make.
The conclusion is that **this directory wants the ratification ledger
split out of the script** — the per-seam justifications are a document
that happens to live in a comment block, and they are what makes a 20-line
function carry a 204-line header. That is a real observation about the
shape of `scripts/gates/`, not an argument for un-disclosing anything, and
it is the disposition S116(m) should close on. **The full progression —
131 → 157 → 195 → 204, and what each step bought — is in D106's record**, where
its taker will read it.

In several of these the prose is the *only* change: S116(g) answers
"three parallel pipelines with no shared core" with a long argument that
this lane does not need one; S107 closes a naming confusion by argument
rather than by change; S116(h) converts one undecided constant into a
more honest account of the same undecided constant.

None of this is dishonest — the opposite; it is unusually candid. But
the brief's own rule (*"unusual justification length is mild evidence
for a smell"*) now has a large, measured corpus behind it, and the
question it raises is a policy one for Ev rather than a finding:
**when a finding's honest answer is "we are not going to change this",
what is the maximum acceptable length of that answer, and where does it
live?** A 234-line trait doc and a 130-line gate header are both past
the point where the rule is findable, which is the failure mode that
matters.

## C26. Never-versus-sometimes: the grade a CI defect actually deserves

**Ruled by Ev, 2026-08-22, while Track J was grading four of these at
once:** *"it's very bad if an error can mean that a check **never** runs,
but really not all that bad at all if an error means it only **sometimes**
runs."*

The reasoning is the second half: a check that runs less often still runs,
so the next code PR catches what the skipped one missed, at no particular
cost. A check that can be silently switched off entirely is caught by
nothing, ever.

**It re-graded four live findings the moment it was stated**, in both
directions, which is why it is recorded rather than left in a thread:

- `REQUIRE_RUFF` and `REQUIRE_FREECAD` — one env var away from a job that
  prints `SKIP`, exits 0 and verifies nothing, on every PR, forever.
  **Never. Severe.** Both are closed — `REQUIRE_RUFF` by #905, and
  `REQUIRE_FREECAD` by #911, which took it the same day it was graded and
  confirmed the hole by running the pre-patch script (`GITHUB_ACTIONS=true`,
  no binary, **exit 0**) rather than trusting the report.
- A check whose **enabling condition the repository itself supplies** —
  `crates/pncad-py/tests/test_ty.py` gated on
  `@unittest.skipUnless(ty_binary(), …)` while `unittest discover` exits 0 on
  skips, so deleting one install step leaves the job green having checked no
  stubs. **Never. Severe.** It is also the member a sweep misses: the lane
  that closed `REQUIRE_FREECAD` swept `REQUIRE_*`, `command -v` and
  `SKIP.*exit 0` and **all three were blind to it**, because what disables
  this one is a deleted *install step* rather than a variable. A sweep for
  the class has to key on **what supplies the enabling condition**, not on
  the shape of a flag.
- A parity or roster gate scoped to one file, so a second file is
  unchecked (`OUTLIER_GATES` before #903; claim 9's single-workflow scope).
  **Never. Severe.**
- A gate landing mode `0644`, invisible to every `[ -x ]` roster
  derivation. **Never** — which is why `D59` was worth closing.
- `_is_docs` misclassifying a change set, so a gate skips *that* PR.
  **Sometimes. Cheap.** Worth a self-test case and nothing more.

**The orchestrator graded the last of those a MAJOR and was wrong**, and
the error has a shape worth naming: a defect that is *easy to demonstrate*
reads as severe. A one-line edit that makes a self-test go green while a
page falls out of the gate is a vivid reproduction — and it is still only
"sometimes", because the doctests it skips run on every code PR anyway.
Vividness is not severity.

**How to apply it.** When deciding how hard to guard something, ask which
kind it is. The first deserves a mechanism that cannot be defeated by an
edit — key the fatal condition on something the environment sets, not on
something the repo declares. The second deserves a case in a self-test.
Do not spend the first budget on the second problem.

## C27. Three things Track J's lanes found outside their rows

Recorded here rather than as rows because each is either a class needing
one decision or a fence question, and none belongs to the unit that found
it. Every one was found by a lane sweeping the *shape* of its own fix
rather than the symbol.

**Two files are in no track's fence.** `crates/topo/src/separation.rs`
appears in neither Track P's list nor Track Q's; `scripts/check_step.sh`
appeared in neither Track J's nor Track K's and is now on retired-`J`
ground, which is unowned rather than covered. Both were found by a lane
asking where a row it wanted to file would go, and in both cases the lane
declined to widen its own fence to swallow the file — which is the correct
move and the reason the gap is visible at all. **The partition's headline
rule is file territory, so a file in no territory is the one shape it
cannot express.** A third instance would make this a re-partition question
rather than two footnotes.

**`pub` fields on non-`pub` types: 54 types, 239 fields, 33 files in
`crates/*/src`.** Largest: `topo/src/fixtures.rs` (62), `profile/src/sugar.rs`
(23), `mesh/src/nurbs_cert.rs` (15). Raised while closing `D24`, and
deliberately **not** swept there — because the lane executed the case and
found the shape does **not** reproduce `D24`: `dead_code` reports an unread
field regardless of keyword, including a bare `pub` field of a `pub(crate)`
struct (rustc 1.97.0). So no corpse hides behind a `pub` field; the fault is
keyword honesty, not a reachability hole. That is what makes 54 a style
question rather than a residue, and nine of them (`pub(super)` structs in
`topo/src/boolean` and `topo/src/splitting`) are not obviously wrong as
written. `crates/*/tests` is at **0** after `D24`'s fix — that population is
closed. Note also that enum variants cannot carry visibility in Rust
(E0449), so there is no variant half of this class.

**The ladder-marker class: six dangling provenance markers.** Four `(L5)`
in `topo/src`, two `(L7)` in `geom-core` — `M0-PLAN.md` and `M0-LOG.md` are
both deleted, so the numbers resolve to nothing. Two more were resolved in
passing by `D99`'s fix. **The right fix is one convention decision, not six
edits**: either L-numbers get a surviving home, or the markers go. Left
unfixed and disclosed per the style brief's Q6 rather than half-swept.
