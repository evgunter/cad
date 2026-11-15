# Design fork: how a construction asks whether a value is an identity (BOXFINISH)

Protocol: 747202177c (`docs/DESIGN-FORK-PROTOCOL.md`). Item: `work/boxfinish/the-axis-mint-asks-for-proof-only-where-the-value-reads-zero.md` (P1, design), with DECIDE's general form `work/decide/a-construction-can-ask-whether-a-value-is-an-identity-only-by-taking-a-sign-reading.md`.

## Draws (2026-11-15, 15:46 UTC, at dispatch)

| Byte | Value | Meaning |
|---|---|---|
| arm | 79 | below 86: THREE designers (one Fable, two Opus) |
| labels | 129 | 129 mod 3 = 0: Fable is A; the two Opus designers are B and C |

Mapping: **A = Fable 5.2, B = Opus 5.5 (first Opus dispatched), C = Opus 5.5 (second Opus dispatched)**. Versions as configured at dispatch; confirmed from each session's served model when the reports are in.

Dispatched concurrently as three cloud sessions with the same statement of the problem and the same pointer to `docs/prompts/designer.md`.

## Sessions (dispatched 15:48 UTC, one turn, concurrently)

| Label | Model (configured) | Session |
|---|---|---|
| A | Fable 5.2 (`claude-fable-5-2`) | session_013QoYWVGw17S1owoZDQWTKY |
| B | Opus 5.5 (`claude-opus-5-5`) | session_013QoYWXrRgtjC4aFXveqsMT |
| C | Opus 5.5 (`claude-opus-5-5`) | session_013QoYWXsPyvjRQb3Qf4JNAm |

The statement of the problem (identical for all three) is the orchestrator's brief of 2026-11-15, 15:48 UTC; it names no candidate solution. Two exposures to a candidate, both named to the designers as hypotheses: the two item files carry one implementer's sketch (a non-deciding query on `geom_core::Decide`), and the brief lists what the answer "may change" (the verbs, validation, the scalar traits, the symbolic tier, the doors, ratified text), which names possible homes.

Token counts for a three-designer fork: read from each session's `result` events when the reports are in (first report, and total).

## First reports (all three in by 15:59 UTC; recorded before any reconciliation)

Report times (the session's first result event): C 15:56:04, A 15:56:37, B 15:58:49 UTC. None saw another's report. Served models as the result events report them: A `claude-fable-5-2`, B `claude-opus-5-5`, C `claude-opus-5-5`.

| Label | Answer | One line of argument | Confidence | Rejected the framing? | Proposed changing ratified text? |
|---|---|---|---|---|---|
| A | One structural door on the scalar trait `Real` ("is this value zero at every point this scalar speaks for"): `f64` true at 0, `Interval` true at `[0, 0]`, `Sym` by the tier's identity test on the node, never the value channel. It replaces `is_exact_zero`. The mint branches on it; the other component's sign is one kept decision. | The kernel's two ways to ask about zero both read the number; the axis-or-quotient branch is a choice of spelling, not a decision, so it needs no verdict, only never to read a point value. | likely | Yes, three corrections: what is missing is a structural question, not a better proof reading; the branch is no topology decision; "one branch at every valuation" is too strong (a branch may rest on facts certified over the lane's whole box, as at `max`). | Yes: E12 gains a clause; DESIGN `[scalars]` names the door; the profile README's direction clause says a direction not made by a verb is minted once from its points. |
| B | A tier query `identically_zero(x)`, value-free (the decision read at `min`/`max`/`select` and the registered-identity door OFF), answering *no* at `f64`, `Interval`, `Dual`. The `line_to` verb mints and records its hop's director by one rule; validation mints only for tables. `is_exact_zero` narrowed to stored literals; the bulge's line-or-arc choice becomes a margined decision. | A decision is value-first by design, so any mint built on one inherits "the value decides whether the expression is asked"; the point lanes must stay branch-free. | likely | Yes: the defect is asking about the expression through a reading of the value; `max` is not this question's problem; two places spell one hop's direction. | Yes: E12 gains the query; `[verdicts.consumers]`' lane choice gains "in a symbolic lane a closed form that drops a dependence is taken only on identically zero". |
| C | `Decide::identity(x) -> Option<Identity>`, a proof token only the tier creates; `None` at `f64`, `Interval`, `Dual`; every rule that reads a value switched off. The one mint (`axis_or`) branches on it; the sign is a kept decision. `is_exact_zero` split into reading a record (kept) and comparing computed values (moved to the query or to a decision). | Every failure the review found is a property of `Sym::sign_within` leaking through; no spelling built on sign readings can be valuation-independent. | likely | Yes: the defect is the question, not the mint; the demand is per lane, not across lanes; making the value exact instead of branching does not cover the case. | Yes: E12 ("nothing in the kernel changes … no funnel site is edited") amended to admit a structural query that may only choose between spellings. |

Agreement, judged on the final state: all three add a question that is not a decision, answered from the expression and never from the value, memoised, the same in eager and deferring sessions, licensed only to choose between two spellings of one real; all three have the mint branch on it with the other component's sign a kept decision, drop the aside and the discarded readings, keep `literal_sign` a decision, and reject PR 6990's spelling, taking the axis rule out, and tightening the value channel as final states. They split on:
1. **What the plain scalars answer and whether it is one door.** A: `f64` and `Interval` answer from the structure they have, and the door IS `is_exact_zero`. B and C: the plain scalars answer no; `is_exact_zero` keeps only its reads of stored literals. B argues against A's shape by name (its alternative D).
2. **Box-certified reads.** A keeps them (a branch may rest on a fact certified over the lane's whole box; the `max` case stores the truth about each box). B and C switch every value-reading rule off, so the `max` case is the quotient at every valuation; C adds that the `min`/`max` read is an unrecorded decision.
3. **Where the mint lives.** B: in the `line_to` verb, recorded, junctions reading the same director; validation only for tables. A and C: one mint function (A notes the pen's heading as a second spelling, off the question).
4. **The readers of "proven".** A: stay as they are. B: the unproven-coincidence lint's readers should ask the value-free test (unsure). C: keep, with `deferred::discharge` running the value-free walk and any `.proven()` read that steers construction moved to the query.

Tokens at the first report (`modelUsage`, cumulative): A 3,827,883 total (168,333 without cache reads), $4.39; B 4,543,143 (177,546), $2.88; C 2,832,135 (132,534), $2.00.


### A, first report, verbatim

## For Ev

**Recommendation.** Give the kernel one door that asks a question of a value's *expression* instead of its *number*: "is this value zero at every point this scalar speaks for?" It is not a decision: it reads no sign, has no band, counts nothing as decided and leaves nothing to discharge. The profile's direction mint becomes its one caller here, and `is_exact_zero` becomes this door. Confidence: **likely**.

What "every point the scalar speaks for" means, per scalar:
- `f64` speaks for one point: true when the number is 0.
- `Interval` speaks for its enclosure: true only for `[0, 0]`.
- `Sym<T>` (a value plus its expression over the document's variables) speaks for every valuation of the document: true when the symbolic tier's identity test proves the expression zero. It never looks at the value channel, and it is the same call in an eager session and in a deferring one (a session that postpones proofs until a reader asks), because no decision is involved.

**Premise check.** Three corrections to the question as asked.
1. The thing missing is not "a better way for the mint to learn that a Zero was proven". It is that the kernel has only two ways to ask about zero, and both read the number: a sign decision (`decide`, which consults the tier only where the number does not already read a definite sign, by a ratified numeric-first order), and `is_exact_zero` (whose doc says "structure read as structure" but which at `Sym` reads the value channel). PR 6990 bent the first into a structural question by reading at a zero-width band and throwing the answer away; every defect its review found follows from that. Confidence: **sure** (read in `Sym::decide_sign`, `deferred::defer`, `axis_or`).
2. The axis-or-quotient branch is not a topology decision. Both arms are the same real direction wherever the axis arm applies; the axis only has no width. So the branch needs no verdict and no record. What it needs is never to read a point value. Confidence: **sure**.
3. "One branch per document at every valuation" is slightly too strong. The right invariant: the branch is a function of the expression and of facts certified over the lane's whole box, never of a point value inside it. Example: `(max(a, b), 0) → (b, 1)`. On a box where `a < b` everywhere the side really is `(0, 1)` identically; on one where `a > b` it is not. Storing the axis on the first and the quotient on the second is the truth about each box, and is E12's stated claim (a zero "wherever the construction takes the decisions it took"). At `Sym<f64>`, where there is no box, the tier does not read `max` and the answer is the expression's alone. Confidence: **likely**.

**The final state.**
- *The door.* A method on the scalar trait (`Real`), beside `is_poison` and `register_equal`, not on `Decide`. `Sym` answers by the identity test asked directly on the node.
- *Its contract.* The answer may only choose between two spellings of the same real value. It never refuses, never picks topology. A wrong "no" costs width; a "yes" is a theorem. Held three ways: the callers are a short hand-kept roster (as `is_exact_zero`'s are today); a session dial makes the door answer "no" everywhere, and every corpus document must then build the same body with the same real values; and the receipt counts the door's questions in a column of their own, so the symbolic/numeric split E12 promised stays complete.
- *The mint.* One function makes "the unit direction of a vector between two points" for the line director and the arc radial alike (they are the same question: a vector over its length). Where a component vanishes identically, the direction is the axis, signed by one ordinary, kept sign decision on the other component. Otherwise the quotient. That sign decision cannot escalate: the segment's length was already decided positive, and with one component identically zero the length is the other component's size. No reading is taken to be discarded; `sym::aside`, the zero-width-band reading and the discharge call leave the mint.
- *`literal_sign` differs, and should change differently.* It asks a bulge's sign, which is a fact about a value, and the arc's side and the identities its construction registers rest on it. That is a decision, taken once where the bulge is lowered and kept; it is currently spelled as a "choice between sound candidates no reader rests on", which it is not. Confidence: **likely**.
- *`is_exact_zero`.* It is this door. At `Sym` it stops reading the value channel. Its roughly twenty callers then sort into two kinds: a choice of spelling (skip adding a zero start; an axis hub), which is unchanged in meaning and now valuation-independent; and a branch that changes what is built (a zero bulge is a line; a zero chord stands in `+x`; a value sitting exactly on a chart's end). The second kind is a value coincidence deciding structure with no record, the same class the five corpus rows caught, and each becomes a real decision or a tagged form. I have not audited the callers one by one. Confidence that the two queries are one: **likely**; on what each caller becomes: **unsure**.
- *The readers of "was this Zero proven"* (`MarginDiag::proven`, `deferred::discharge` in `topo` and `editor-core`) stay as they are. They ask how a decision that was taken and is used was settled, at a working band where rounding sits inside the band. The mint was the one construction that took a decision in order to read its provenance; after this none does. Confidence: **likely**.

**Worked example.** The hop `(x, 0) → ((x + w) − w, 1)` at `Sym<f64>`. Today: at `(1, 1)` the chord's x computes `0.0`, the tier is asked, the axis is stored; at `(0.1, 0.3)` it computes `2.8e-17`, a definite sign, the tier is never asked, the quotient is stored. Under the door: the expression `(x + w) − w − x` is the zero polynomial at both, so both store `(0, 1)` with one sign decision on the y component. Two different corner variables that agree at the nominal: the expression `a − b` is not zero, so both valuations store the quotient. The slab side `(W, 0) → (W, 1)` over a box, eager or deferring: `W − W` is zero as an expression, the axis is stored, and nothing escalates because nothing was decided.

**Ratified text I would change** (each a design choice, so yours):
- ERROR-DESIGN E12 gains a clause: a construction may ask the expression whether a value vanishes identically; the answer is not a decision and may only choose between spellings of one real value.
- DESIGN `[scalars]`, the `Real` surface: names this door as the one structural query, and says `Sym` answers it from the expression, never the value.
- `crates/profile/README.md`, the direction clause (your "makes sense" on PR 6936): add that a segment whose verb made no direction has it minted once from its points, the axis where a component vanishes identically. As written the clause says a table's directions are "decided once at validate" and that nobody divides a vertex difference by its length, and the mint does exactly that division; the text should say so.

**Alternatives, as final states.**
- *Take the axis rule out; always the quotient.* Sound and simplest. A slab drawn through four corners then has a wall that turns over the box, which is the thing this program exists to remove. Reversible. Not recommended.
- *Fix it inside the decision door: ask the tier before the number, in every session.* Makes the PR 6990 spelling valuation-independent, but still takes decisions to discard them, pays the identity test on every decision (measured at 57 ms against 1.4 ms per leaf when it was tried), and at `Sym<f64>` would overrule a definite numeric sign, against the ratified numeric-first order. Not recommended.
- *Make the scalar exact: `Sym` folds identities into its value as it computes.* The general "any value" answer, but it pays a normal form per operation, which E12 rules out, and it still cannot turn `h / |h|` into `±1` without a sign. Not recommended.

**Is it the same question for any value?** For "is zero" and "equals this constant", yes: one door. For "has the tightest enclosure the document allows", no: that is interval dependency in general, and stays the subdivision driver's job.

## For the orchestrator

**Where this agrees with the lane's sketch and where it does not.** The sketch (`proves_zero` on `Decide`, default false) is close. Differences that matter: it belongs on `Real` (it is not a decision); `f64` and `Interval` answer with the structure they have rather than `false` (plain `Interval` with a `[0, 0]` component otherwise stores `h/√(h²)`, wide by the box, the item's third bullet); it replaces `is_exact_zero` rather than standing beside it; and it carries a contract with a differential dial, which is the answer to "what stops it being used where a decision belongs".

**Claims I could not check.**
- That the identity test can be called off the decision path with no side effect on counts or pending lists. `discharge_retried` / `discharge_in` look pure apart from memos and `counts.retried`; not run.
- Which rungs the door should ask. `Path::Decision` includes the registered-identity door, and a registration is witnessed against values (`SymRegistration::Witnessed / Contradicted / Disputed`), so a "proof" through it may depend on the valuation. PR 6990's review brief raised this and I did not see it answered. Safe default: the registry-free rungs (`Path::Contradiction`'s set, plus the retry ladder), widened only once registrations are shown valuation-independent.
- The tier's answer depends on the session's budget, rules and retry ladder. Two sessions with different budgets (the coincidence door's replay against the driver's) can answer differently for one document. Under the contract that costs width only, but a test comparing stored directions across lanes must expect it.
- I ran nothing. Every statement about behaviour is from reading the tree at `36776965` and the item files.
- The `is_exact_zero` callers (`topo/boolean/spline_known`, `topo/replace_face`, `geom-brep/description`, `pcurve_cache`, `geom/curves`, `geom/cover`, `profile` bulge and `chord_director`): sorted by reading two or three, not all.
- `geom-brep/certify.rs` and `dihedral.rs` also read `margin.proven()`; the brief lists only `topo` and `editor-core`. Not read.

**Things in the brief I would correct.**
- "In every lane the same thing is stored" cannot hold bit for bit between `f64` and `Sym<f64>` for an identity that rounds: `f64` stores `(2.8e-17, 1)`, `Sym<f64>` stores `(0, 1)`. Both are the direction to rounding ([simple.conditioned]). If some row compares those two lanes' value channels bit for bit, it needs to know.
- The `max` case is not a hole in the tier (premise 3 above). What is true is that no instrument records which arm the read took; if the corpus ever runs at an exact witness it will need that.

**Off the question, worth filing.**
- *The axis is a special case.* A direction is constant whenever the two components are a constant multiple of one expression. A 45° side between formula corners `(x, y) → (x + c, y + c)` with `c` boxed stores `c/√(c² + c²)`, about `[0.58, 0.86]` for `c ∈ [0.9, 1.1]`, where the direction is `(0.7071, 0.7071)` at every `c`. The door reaches it only if the tier can also say "these two forms are in constant ratio α : β". BOXFINISH or DECIDE, design; I would not build it until a document needs it.
- *Two mints of one direction.* The pen's heading after a hop (`path::chord_director`, `d.normalize()`) and validation's `mint_director` both make the chord's unit; `ChordFrame::unit` is a third spelling. With the door they should be one function, or a tangent arc after an axis hop inherits the wide heading while the segment stores the exact one.
- *`literal_sign` uses `k_stats::choose`* under a ledger row that says no reader rests on the pick, while audit F43 says the registered radial identities rest on it. One of the two sentences is wrong.
- *A deferring session at `Sym<Interval>` returns an identity's escalation unchanged* (`deferred::defer` passes anything but a Zero through). Latent, and after this change no longer the mint's problem, but the module's claim that "the decisions are the same either way" is false at an exact witness over a box. Either the session lists escalations too, or it is typed to inexact witnesses. DECIDE.
- *`sym::aside` stays:* it has another caller (`geom-brep/pcurve_cache/projected.rs`), so the mint leaving it retires nothing.

### B, first report, verbatim

## For Ev

**Recommendation (likely).** Stop asking "is this an identity?" by taking a sign reading. Give the symbolic tier a query that is not a decision: **`identically_zero(x)`**, "is `x`'s expression the zero function of the document's variables". It is answered from the expression alone and never reads a number. At `f64`, `Interval` and `Dual` it always answers *no*, because those scalars hold no expression. Then a direction is minted by one rule in one place: **where one component of the vector is identically zero, store the axis (0, ±1) or (±1, 0), with the sign taken from a recorded decision on the other component. Otherwise store the quotient (vector over its length), and take no reading at all.** The bulge's line-or-arc choice is the opposite kind of question and becomes a margined decision. `is_exact_zero` is narrowed to reading values the kernel stored as literals.

Terms:
- **Value channel**: the number (or interval) a symbolic scalar `Sym<T>` carries beside its expression.
- **Form**: the tier's normal form of that expression in the document's variables.
- **Decision / reading**: a sign taken against a band. It is counted, recorded for re-valuation and, in a deferring session, queued.
- **Eager / deferring session**: the tier proves a decision's Zero when the decision is taken (eager), or later when someone asks (deferring).
- **Point lanes**: `f64` and plain `Interval`. **Symbolic lanes**: `Sym<f64>` and `Sym<Interval>`.

**Premise check.**
- **The defect is the question's channel, not how `axis_or` reads proof.** It asks a question about the expression through a reading of the value. A decision is value-first by design: the tier is consulted only where the value channel does not already read a definite sign. That is E12's numeric-first order, and it is sound for decisions. So any mint built on a decision inherits "the value decides whether the expression is asked". PR 6990's probes show that order, seen from a caller that wanted the other question. (sure)
- **The mint reads at `Band::least()`.** That makes it as sensitive to the value as it can be: a rounding residue of 2.8e-17 reads definite. A wider band would hide that case, but not the deferring-session escalation and not the principle. (sure)
- **`max` is not this question's problem.** The tier's *decision read* folds `max(a, b)` to whichever branch a certified comparison on this box shows. That is right for a decision, which is about this box, and the decision's own sign is recorded and flips at another valuation. A query about the whole function must not use the read, so the max case disappears from the mint. (likely)
- **Two places spell one hop's direction today.** `path::chord_director` normalises the hop for the junction decisions at `line_to`, and validation mints the stored director again. That is [one-way]'s "two readings of one fact". Over a box the junction reads the wide quotient while the stored director is the axis. (likely)

**The final state.**
1. **The query.** `identically_zero` is answered by the tier's own algebra: the normal form plus the retry ladder, which changes only the ring width and the rule set.
   - Three rule groups are off, because each reads a value or a witness that depends on the valuation: the decision read at `min`, `max` and `select`, and the registered-identity door, whose entries are witnessed numerically at registration.
   - It is memoised per expression node, and it is the same in eager and deferring sessions.
   - It counts in no decision column, makes no K sample, leaves no deferred entry and writes no re-valuation record.
   - Its answer is a function of the expression graph alone, and the graph is a function of the document and of the decisions already recorded. So re-valuing a document cannot change the answer unless a recorded decision changes first. (likely)
2. **Its one licence.** It may only choose between two spellings of one real number: a closed form that equals the general form wherever the identity holds, which is everywhere. It may never choose cells, refuse or glue. That keeps [intent.geometry] ("a body is the same whatever…") true across lanes, because the point lanes, which answer *no*, build the same real values. It is enforced by a hand-kept list of callers plus a CI gate, as `is_exact_zero`'s list is today. (likely)
3. **The direction mint, one place.**
   - The `line_to` verb mints its hop's director by the rule above and records it, and its junctions read that same director. Validation mints only for table-authored profiles. An arc's radial at each end uses the same rule on `p − centre`.
   - The sign of the non-zero component is a real decision: which way the line points. It is taken at the run band, recorded and kept.
   - If that sign escalates, the escalation propagates and the driver subdivides; there is no fallback to the quotient. With one component identically zero, the chord's length is the other component's absolute value, so an undecided sign means the segment may be degenerate on the box.
   - At the point lanes the mint is the quotient, with no branch. At `f64` that is already the axis exactly wherever a component is exactly 0, apart from the sign of zero.
   - `aside`/`discard` go, and so do the readings that are thrown away. (likely)
4. **`literal_sign`** (a bulge's sign, which signs the lowering's radials) answers a different question: a sign, not an identity. It stays a recorded decision. A tie leaves the radials to validation, whose straightness check refuses such a shallow arc anyway. (likely)
5. **The bulge's line-or-arc lowering** (`profile::bulge_leg`, `lower_to`) decides a *structural* act by `is_exact_zero(bulge)` on the value channel.
   - At `Sym<f64>`, a bulge `b₁ − b₂` whose two variables agree at the nominal lowers to a line, and re-valued it lowers to an arc. That is the same class as the five red corpus rows, and it is latent.
   - It is also a second reading of validation's `segment_straightness`. It should become that one margined decision, with a recorded Zero meaning a line.
   - It must not use the identity query, because a structural act has to follow the values in every lane. (likely)
6. **`is_exact_zero`** keeps one meaning: "this stored value is the literal zero the kernel wrote". Examples are an iso line's held slope (which fork 271's held-line representation retires), a ray's start, and a spline chart's stated end. Each of its 24 callers is audited into one of three kinds:
   - a read of a stored literal, which stays;
   - a choice of spelling from a computed value, which moves to `identically_zero`;
   - a structural act, which becomes a decision. (likely)
7. **`MarginDiag::proven` / `deferred::discharge`** keep answering "why was *this decision's* Zero taken", which the shape report and refusal texts need. The `unproven-coincidence` lint's readers (`topo::coincidence::Discharge::of`, `editor-core`'s replay) ask whether a coincidence is *structural*, which [intent.structural] defines as "zero as a polynomial in the document's variables". So they should ask the test that reads no value, not the decision's stamp, which can include the read that holds only on this box. (unsure: it moves findings only, never a body, and I have not measured which findings would move)

**Worked example** (one rule, five documents, every lane):
- **Slab side `(W,0)→(W,1)`.** x is `W − W`, identically zero. Symbolic lanes store (0, +1) at every valuation, eager or deferring, point or box. Point lanes store the quotient, which is (0, 1) at `f64`.
- **Hop `(x,0)→((x+w)−w, 1)`.** x is identically zero, so it is the axis at (1,1) and at (0.1,0.3) alike. `f64` stores (2.8e-17, 1), the same real number as (0, 1).
- **Rectangle side between corner variables `a`, `b`, with a = b at the nominal.** `b − a` is not identically zero, so every valuation stores the quotient. That keeps the five corpus rows green.
- **`(max(a,b),0)→(b,1)`.** Not identically zero without the read, so the quotient at every valuation.
- **Vertical side `(W,a)→(W,b)`.** The axis (0, sign(b − a)), with the sign a recorded decision. It flips only where the side would reverse, and that flip is recorded.

**Ratified text this changes.**
- **E12**, "Nothing in the kernel changes: `Sym<T>` is one more scalar…". It gains: a construction may ask the tier whether a value is identically zero, to choose a closed spelling of one real. The code's claim that a `Sym<T>` run's value channel is bit-identical to a `T` run (in `sym.rs`, not ratified) is already false under PR 6990. It becomes: the same except where a construction chose a closed spelling by structure. E12 was agent-written text approved with "lgtm" on #1712; its opening sentence is Ev's own. (sure on provenance)
- **[verdicts.consumers]**, *lane choice*, "Zero takes the closed form". In a symbolic lane, a closed form that drops a dependence on the variables (a literal axis) is taken only on *identically zero*. A Zero read from the value is a coincidence that holds at this valuation only (E12). Without this sentence the two clauses contradict each other at a symbolic lane. (likely)

**Alternatives weighed.**
- **A. Keep PR 6990 (proof read off a decision).** Reading the value first is that door's purpose, so it cannot be made independent of the valuation. It also takes decisions only to discard them. Reject. (sure)
- **B. Take the axis rule out.** Sound, with no new query. The cost is that the slab's side is wide over a box, and the sweep doors refuse the wall that BOXFINISH exists to certify. Reject unless the query's cost proves prohibitive. Easy to reverse in either direction.
- **C. Tighten the value channel from the form instead.** A `settle` operation would give a value whose form is a constant that constant's enclosure. It has no branch, and it would also make other stored values exact. But it cannot produce a unit y-component, because `(b − a)/|b − a|` needs a sign. It rewrites the value-channel contract everywhere and costs a normal form per stored value. Worth keeping as a later generalisation if a second value needs it; not this fork. (likely)
- **D. Make `is_exact_zero` itself answer by form at `Sym`** (one door). Tempting, but at `f64` the value is all there is. The mint would then branch on a coincidence there, taking a sign decision at one valuation and not at another. Keeping the point lanes branch-free needs the query to answer *no* where there is no structure. (likely)
- **E. Tag the stored direction** (a variant "axis X/Y, sign" beside "unit vector"). That is [unrepresentable]'s tagged encoding, but downstream doors would then branch on the tag. The literal 0 and ±1 values already compute exactly, and a tag could not come from `f64` structure ([scalars]' "f64 structure + T payload"). Reject. (likely)

**Reversibility:** the query is additive, and its list of callers says where it is used. Rolling back is B.

## For the orchestrator

- **Not checked:**
  - Whether any test compares `f64` against `Sym<f64>` stored values bit for bit. The rounding-identity hop would differ.
  - The cost of value-free forms in deferring sessions on large documents.
  - Whether `signed::order` (the decision read) is the only rule that reads a value. I found it gated by `SymRules::decision_read` at `min`/`max`/`select`; `manifest`'s rule F reads syntax only.
  - Whether excluding the registered-identity door loses a mint the doors need. For a chord between authored points I expect none, since a constructed arc's radials are recorded by its verb.
- **Assumed:** the sweep doors need only exact literal values, not a tag. The brief says the doors' unit reads the eager `Sym<Interval>` lane, where PR 6990 holds.
- **Off-question, worth filing:**
  1. `bulge_leg`/`lower_to` decide line or arc (a structural act) by bit equality, duplicating `segment_straightness`. Profile, P2, design. Same class as the corpus rows; no corpus document has a variable bulge.
  2. `path::chord_director` and the validation mint are two spellings of a hop's director. `chord_director`'s zero-chord `+x` stand-in (`is_exact_zero(witness)`) stores a constant at `Sym` where a value coincidence would vary. Low risk: vertex separation refuses the zero hop later.
  3. `deferred.rs` says "deferring changes which sign an escalation reads and nothing else". But an identity that escalates is a Zero when eager and an escalation when deferring, so the two sessions branch differently on it. The review's `Sym<Interval>` probe is this. It belongs to DECIDE with the general row.
  4. The 24 `is_exact_zero` callers need the three-kind audit. The iso-line readers (`row_of`, `iso_image_row`, `patch.rs`) fall to fork 271's representation.
- **Relation to the two rows:**
  - This answers DECIDE's row: the query, its licence, how it relates to `is_exact_zero`, and the plain-`Interval` `[0,0]` bullet (point lanes answer *no*, by design).
  - The BOXFINISH row's "closed when" holds under it, and the two review probes become ordinary green rows at the symbolic lanes.
  - `sym11` re-baselines: two fewer readings per direction minted as a quotient, one more sign decision per axis.

### C, first report, verbatim

## For Ev

**Recommendation (likely).** Give the symbolic tier a second question that is not a decision: "is this value identically zero in the document's variables?" It is answered from the expression alone and never by reading a value. Have the one direction mint branch on that answer instead of a sign reading. Terms used throughout:
- **Symbolic tier**: E12's check that an expression's polynomial normal form, built from the parameter symbols, is zero.
- **Lane**: a scalar the kernel runs at (`f64`, `Interval`, `Sym<f64>`, `Sym<Interval>`), each in an eager or a deferring session.
- **Valuation**: the parameter point or box the document is evaluated at.

The query is:
- `Decide::identity(x) -> Option<Identity>`. `Identity` is a proof token that only the tier can create. The answer is `None` at `f64`, `Interval` and `Dual`. At `Sym` it is the tier's normal-form walk, memoised per node.
- Never a decision. It adds no decision count and leaves no deferred entry. It runs immediately inside a deferring session too, because deferring postpones the discharge of decisions, and this is not one.
- Run with every rule that reads a value of the box switched off. Today the walk takes a certified read at `min`/`max` (`SymRules::decision_read`). Without that read, the answer depends only on the expression graph (the DAG), which is the same at every valuation.
- Typed so it cannot be read as "nonzero". `None` means "not shown". There is no arm that says "differs", so the answer cannot drive a glue or a split. It may only choose between two spellings of one real function: a *lane choice* in `[verdicts.consumers]`.

The mint becomes: if the chord's x is identically zero, store `(0, sign(y))`; the same with the components swapped; otherwise store `w/|w|`. The sign is a real decision and is kept. Once one component is identically zero, the vertex-separation decision already taken means the sign reads definite. No reading is taken only to be thrown away.

**Premise check.**
1. **The defect is the question, not the mint (sure).** "Is this identity" is a fact about structure. The kernel can only reach it through `Sym::sign_within`, which is built for decisions:
   - It reads the value first, and a definite value short-circuits the form. At an inexact witness (`f64`) the numeric answer wins even against a zero form (counted as "disputed").
   - It counts the reading.
   - It defers it inside a deferring session.

   Every one of the three failures the review found is one of those properties leaking through: the `2.8e-17` that is never asked about, the escalation in the deferring session, and the discarded readings. No spelling built on sign readings can be valuation-independent. PR 6990 is the best one possible, and it still is not.
2. **The demand is per lane, not across lanes (likely).** `f64` has no expression graph, so it cannot know that `(x+w)−w−x` is zero. It stores `(2.8e-17, 1)/L`, while `Sym<f64>` stores `(0, 1)`. That is rounding noise, which `[simple.conditioned]` treats as the same real number. Within each lane the branch is now a function of the document alone. Proof is checked by one tier walk, so it is identical in eager and deferring sessions.
3. **Making the value exact instead of branching ("settle") does not cover the case (sure).** The alternative is to replace a `Sym` value by its exact constant wherever the normal form is constant, so the mint needs no branch. A slab side `(W, 0)→(W, H)` with `H` boxed shows why that fails:
   - Settling the chord gives `(0, H)`, but `H/√(H²)` still encloses `[H_lo/H_hi, H_hi/H_lo]`.
   - Only the axis form (an identity, plus a sign decision) gives `(0, ±1)`.

**The three sites.**
- **Line director and arc radial (sure):** the same question. Both go through the one mint, `axis_or`, with the query in place of the readings.
- **`literal_sign` is different (likely).** It reads the sign of a bulge, a quantity the author wrote. A flip there is a real change of the arc's sense, and the re-valuation recorder should see it. It stays a recorded choice.

**Ratified text I would change.**
- **E12.** It says: "Nothing in the kernel changes … no funnel site is edited", and only `sign_within` behaves differently. `sym.rs` also promises that the value channel matches the plain `T` run bit for bit. Both need an amendment along these lines: "the tier also answers a construction's structural query, which may only choose between two spellings of one function, never decide". Provenance: E12 was agent-written text that Ev approved with "lgtm" on #1712. The sentences in `sym.rs` are in code docs and were never ratified. PR 6990 already breaks the bit-identity in effect.

**What becomes of the other "exactly zero?" and "proven?" readers.**
- **`geom_core::is_exact_zero` has two meanings (likely).** Split them:
  1. *Reading a record*: a field a construction set to a literal zero (an iso-line's slope, a stored bulge). That reads the same at every valuation and lane. Keep it.
  2. *Comparing two computed values*: `x − lo`, `w0 − w2`, `p.x − first.x`, and others in `geom-brep`, `topo` and `geom`. This is PR 6963's defect in another form: it reads equal where two variables agree at this valuation, and at `Dual` it turns a sensitivity into a constant. Those callers should use the structural query if they are lane choices, or a decision if they are not.
- **`MarginDiag::proven` / `Discharge::of` in `topo`, and `discharge_or_why` in `editor-core`'s replay (likely): keep them.** They report how a decision that was taken for its own sake was discharged. That is the unproven-coincidence lint's question, and it is asked after the decision, not instead of it. `deferred::discharge` should run the same value-free walk as the query, so that "structurally one" (`[intent.structural]`: zero as a polynomial in the document's variables) means one thing everywhere. A `.proven()` read that steers construction has the mint's exposure and should move to the query.
- **The `min`/`max` read is an unrecorded decision (likely).** Inside the tier it is literally `select(B − A, A, B)`. When it is used for a decision's Zero, it should be recorded as a decision, so that re-valuation sees the flip. That belongs to the DECIDE row.

**Final state.** One door for structural identity, one for decisions, and a type that keeps them apart. The mint takes the same branch at every valuation, in every lane and session. A constant is stored only where the tier proved it. Dropped readings and stray deferred entries are gone.

Possible later: the same query, generalised to "the chord's direction is identically constant" (a constant ratio of its components), would also make a chamfer side such as `(W, 0)→(W+c, c)` exact. That should be measured on a real document before it is built (unsure).

Reversible: yes. The query is additive, and removing it reverts the mint to always taking the quotient.

## For the orchestrator

- **Root cause of the `Sym<f64>` probe.** `axis_or` reads at `Band::least()`. At a K·ε band, `2.8e-17` would have been in band and the tier would have been asked. The query removes the least-band chord readings altogether.
- **Not checked:** whether the form walk with `decision_read` off still proves the corpus's hop identities. It should, since they are polynomial. The lane should measure it on the five `revalue_corpus` rows and the two review probes (`80ffc4c5`).
- **Found off-question: `.proven()` readers outside `topo`/`editor-core`.** `geom-brep/src/certify.rs` (two `Refused::Zero … if margin.proven()`, one `shown.proven()`), `dihedral.rs` and `pcurve_cache.rs:~15869`. If any of them steers construction rather than reporting, it has the mint's exposure. Worth an audit row under DECIDE.
- **`is_exact_zero` on a difference:** `topo/boolean/spline_known.rs:118`, `geom-brep/description.rs:466`, `pcurve_cache.rs:8655`, `8676`, `9184`, `patch.rs:86`. Also `profile/lib.rs:310`, `330` and `529` on a bulge that may be computed. Candidates for one issue: "is_exact_zero compares computed values".
- **Assumed:** the DAG is a function of the document plus the decisions taken, so a value-free walk is valuation-independent. Float literals are keyed by their bits; I did not audit for any value-keyed node minting.
- **Budget freezing and SymRetry:** both depend only on form size and ring guards, which are fixed by the DAG. Unverified beyond reading the docs.
