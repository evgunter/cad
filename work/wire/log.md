# WIRE log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/wire/plan.md`.

## Opened (2026-09-11)

Opened in the tracker cut of 2026-09-11 (Ev's direction, in-chat;
`docs/WORK-TRACKS-2026-09.md` addendum 3). This is the successor
`docs/DOC-LEDGER.md` sweep 9 anticipated when EVAL closed on 2026-09-08:
its residue table put three rows in `work/issues/` naming two more
already there and called the five "the successor's opening slate". All
five are here.

Thirteen rows moved in by `git mv`, each with a `## Re-homed` record:
nine from `work/issues/`, four from `work/code-quality/` (`S40`, `S195`,
`D364`, and `profile-has-no-scalar-lift-door`).

No branch exists yet; the first dispatch is the E block, and
`contact-class-has-two-content-tag-functions` looks like a
verify-and-close against the tree as it is now.

## Orchestrator opened, E block dispatched (2026-09-11)

Posture set by Ev in-chat at the opening: **light style reviews on every
unit, full reviews on the units with a real risk of being wrong, and no
A/B protocol.** `plan.md` §Review posture records which units are which
and why, and the band 3700–3799 stays claimed for bookkeeping with no
ordinal drawn from it — which is what `docs/MODEL-AB-LOG.md` already says
about the eleven programs of this cut.

**Closed without a unit: `contact-class-has-two-content-tag-functions`.**
The cut's "appears already discharged" reading held at `8851abb`.
`contact_class_tag` is not in the tree; all three `eval/mod.rs` key sites
feed `ContactClass::content_tag`; and the third site the finding named —
the interface-crossing feed — is
`persist/kernel_wire/contact_class.rs`, a **string** vocabulary anchored
on `ContactClass::ALL` with a refusal arm for a class a newer kernel
adds. One enum, two exhaustive projections over `ALL`, different change
rates: not the twin the finding reported. Citations in the item's
`## Closed`.

**`S40` split, three of its four bullets re-homed.** It was one file
carrying four unrelated residues on three programs' territory, and WIRE
owns one. Filed straight onto the owners' slates per `work/README.md`
("when the owning program is clear, file the item straight onto that
program's slate"): the NaN-as-`throw` idiom and `Rim`'s two direction
fields to **PROPS** (`crates/geom-brep/src/props/*` is its glob), the one
`HashSet` to **SHELL** (`crates/topo/src/transform.rs` is its file — and
the row is re-read as a conformance to D9 rather than a call about it,
since the crate doc already names `SecondaryMap` as the preferred form).
`S40` keeps its id and bullet 1.

**And bullet 1's premise is stale, which is the finding of the sweep.**
`WitnessSlot {}` and `NodeErrorKind::WitnessBifurcation` are deliberate
forward declarations — both say so at the site — reserved for *"the M6
solver"*. `docs/DESIGN.md`'s roadmap records M6 **complete**, as the SSI
generic-`T` lift plus loft/sweep assembly plus composition surgery, with
no witness solver in it, and says the sketch solver *"re-opens as its own
design pass when constraint-driven sketches have a consumer."* So the
reservation is real and its citation is wrong: reviewer-style-lane Q4,
a premise invalidated by the thing it cites. `S40` is now `deferred`
against that roadmap sentence rather than `open`; the two-line re-citation
rides the next PR touching `eval/mod.rs`. Deleting the stubs is NOT
decided here.

**`D364` is a subset of `S195`.** `S195` names `ProgramTarget`/`WireTarget`
as one of its "three smaller pairs" and that is `D364`'s vocabulary at the
same `res_spec`/`res_target` hop. They are one sequence: `D364` first as
the prototype census, `S195` generalising it. Recorded in `plan.md`
§Order.

**`D385`'s pointer corrected** on `profile-embed-lift-has-two-homes`:
it is `work/tint/D385.md` now, moved at S-TINT's opening. The fence
consequence is recorded on the item — WIRE mints the door in
`crates/profile/src/`, S-TINT's `D385` converts the two test copies
afterwards, and `D385`'s list today names one of those two. Announce to
S-BOOL and S-TINT at dispatch.

**Dispatched, three lanes, one PR each** (E block, light style review
each):

| lane | branch | row |
| --- | --- | --- |
| wire-e1 | `wire/placement-affine-map` | `placement-lifts-its-affine-by-hand-beside-affine3-map` |
| wire-e2 | `wire/family-consts` | `wire-expected-phrases-spell-family-words-as-literals` |
| wire-e3 | `wire/names-refusal-carries-cause` | `names-flush-and-select-discard-a-refusal-with-map-err-underscore` |

wire-e3's brief carries a class the item did not: `names/discriminate.rs:18`
does the same `map_err(|_| …)` over `Band::linear`, one enum over, so the
sweep is over the shape and not the two named lines.

Held for Ev, and why each is a fork rather than a sequencing call:
`axis-flavoured-declarations-have-no-channel` (the item's own
`[ev]` shape — a placement-level declaration against a frame-level
identity that survives placement), and `product-gather-…`'s stated rule
for a tie spanning one root's output bodies, where the recommendation is
to carry the tie.

## E1 landed green, review upgraded to full (2026-09-11)

`placement-lifts-its-affine-by-hand-beside-affine3-map` → **PR 2375**,
CI run 34622723038 **green on the full code tier** (twelve `test (…)`
jobs, five `k-lint (gate, …)` rows, the python suite and all four render
lanes; the five skips are change-filter opt-ins, nothing narrowed).

**The item's suggested spelling does not compile**, which the lane found
and the cut did not: `self.affine::<f64>().map(T::from_f64)` as the body
of `affine` is unbounded recursion at `T = f64`, so the f64 value has to
come from a non-generic door. The unit mints two private ones
(`linear_f64`, `affine_f64`) and every public reader is one `map` off
them. The class estimate **E** holds for the two functions the item
named; the item's "one construction" did not.

**Review posture raised from light to FULL for this unit**, under the
rule Ev set at the opening (light except where a unit has a real risk of
being wrong). The unit went past its item: `compose`'s general arm is now
`Frame::from_affine(self.affine_f64() * inner.affine_f64())`, retiring a
hand re-derivation of `Affine3`'s own `Mul`. That arm feeds ASM-4 D-3's
split/inline bit-level volume identity, where being wrong is silent.

The orchestrator's own check, handed to the reviewer as a claim to
attack rather than as a premise: `Affine3::mul`
(`crates/geom-core/src/linalg/affine.rs:169`) is
`(a.linear * b.linear, a.linear * b.translation + a.translation)` —
term for term and in the same order as the code it replaces — and
`Frame::from_affine`'s `is_identity_bits()` snap is a strict no-op
because `bit_eq` compares `to_bits`, so a product that reads as identity
already IS `IDENTITY` bitwise and `±0.0` cannot conflate. Bit-exact, as
far as reading goes; the reviewer is asked to find the input that breaks
it.

**Residue placed** (implementer-discipline §6 — the lane reports, the
orchestrator files): `work/props/geom-core-linalg-has-no-array-doors.md`.
`crates/geom-core/src/linalg/` has no `[f64;3] ↔ Vec3<f64>` and no
`[[f64;3];3] ↔ Mat3<f64>` conversion in either direction, which is the
reason `placement.rs`'s two remaining lowerings and `mate.rs:141-143`
cannot collapse. Filed on PROPS: `crates/geom-core/src/*` is its glob and
its `keep_out` already names a linalg lane. The `mate.rs` consumer is
NAMED there rather than filed as its own row — `mate.rs` is claimed by
both DOCM and MSOLVE, and a finding on contested ground is not one to
hand over by diff.

## E2 landed green, and it re-counted the item (2026-09-11)

`wire-expected-phrases-spell-family-words-as-literals` → **PR 2376**,
CI run 34622801050 **green on the full code tier** (twelve `test (…)`,
five `k-lint (gate, …)`, the python suite; the five skips are
change-filter opt-ins, correct for a diff confined to `eval/`).

**The item's counts were wrong in both directions and the lane fixed the
row**, which is the correction `plan.md`'s class paragraph invites.
Seven bare family words in `expected:`, not nine — and the item's own
parenthetical enumerated eight for a claimed nine, so it was internally
inconsistent before the tree moved; the missing "profile" is
`"profile node"` at `:4191`, a composed phrase on the other list. Eight
composed SITES (six distinct strings), not seven — three `"datum frame"`
sites were counted as one. Fifteen literal sites against the item's
sixteen. `plan.md`'s slate row is corrected.

**No composer, argued three ways** and I accept the argument: the
composed phrases are not family words (`"datum frame"` names a variant
*within* a family, and the `found:` beside it answers `kind_name()` =
`"datum"` on purpose, so a `family::DATUM_FRAME` const would spell
`"datum"` a second time one level up); `concat!` takes literals not
consts and the tree carries no compile-time string concatenation; and
the sync-guard test the lane drafted for `"body or instances"` was
**deleted on the discipline's own rule** — both sides are fixed at
compile time, so it is documentation, and `eval6_placers_over_instances`
already reads that string off a live refusal, which is the receipt that
can go red.

Refusal text is **byte-identical at all seven sites**, so nothing
re-baselined. The lane ran the suite anyway rather than resting on a
build: `cargo test -p editor-core --test all`, 1201 passed, including
the five suites that assert over these strings.

**Two residues filed on this program's slate**, both found by the sweep:

- `interrogate-writes-the-family-vocabulary-a-third-time` —
  `names/interrogate.rs:438-446` spells six family words itself, a third
  copy of `kind_name`'s match in a file that never sees `eval::family`.
  The lane calls it the largest remaining instance of the class and it
  is: EVAL-11 gave the vocabulary a home, PR 2376 wired `wire.rs`, and
  this is the reader neither pass could see. **The file is in no open
  program's `paths`** — checked against every `program.md` — so WIRE
  takes it on the vocabulary rather than the file, and the taker draws
  the fence in its PR, TOPO's stated convention for unowned `src` files.
- `wire-refusals-answer-found-with-a-negation-of-expected` —
  `eval/wire.rs:978` and `:4191` answer `found:` with the negation of
  `expected:` ("carries kind not a datum frame"), where
  `node_value_kind` would name the family the input actually carries.
  Inside WIRE's fence, its own unit because it moves user-visible text.

**Not filed, deliberately.** `verbs/split.rs:42` quotes the
`expected: "profile"` spelling PR 2376 replaces — one stale word, in an
unowned file, caused by this PR, so it rides 2376's own fix pass rather
than minting a row. And the composed phrases at `mate/member.rs:716` and
`verbs/split.rs:157` take PR 2376's prose disposition; they are recorded
on the `found:`/`expected:` item so the sweep is not re-run.

Review dispatched **light**, per the opening posture: byte-identical
text, seven one-word substitutions, and the argument this unit actually
turns on is a documented refusal to build machinery.

## E2's review returned, and the doc rewrite is going back (2026-09-11)

Light style review of PR 2376 delivered: **style findings only, none
gating**, and it confirms the unit's re-count independently — seven bare
words, eight composed sites, no site in the wrong column. The seven
substitutions are accepted as they stand.

**The twenty-line doc comment the unit added is false, and that is the
fix pass.** Two of the review's claims the orchestrator verified rather
than took:

- `git grep -n "family::" origin/main -- crates/` returns exactly THREE
  consumer sites (`kind_name`, `node_value_kind`, `wire.rs:622-623`). The
  old *"three readers"* sentence was accurate, so the rewrite's stated
  reason — that it was already stale — is false.
- `expected: verb.tool_expected` (`wire.rs:2643`) is a real non-literal
  reader, so the new doc's *"`expected` has no reader"* is false on the
  day it lands. The unit's own blind-spot list had already admitted its
  pattern could not see that site, which makes the assertion an
  unforced one.

Four more in the same twenty lines: *"Two readers answer `found`"* is
contradicted at seven sites in the file it governs; the licensing rule
covers neither `"body or instances"` (**wider** than a family, not
narrower) nor the sentence at `:1603`; and the replacement is another
prose enumeration that omits three sites in its own file and two outside
it — the same failure mode as the sentence it replaced, one revision
later.

**Adjudication: restore the pre-PR doc, keep the substitutions.**
`docs/prompts/implementer-discipline.md` §4 decides it — comments state
the invariant, not the argument, and an argument about the shape of a
change *"belongs in the PR description"*, where this one already is. A
doc comment cannot carry an enumeration of call sites without going
stale; that was the old sentence's defect and the rewrite reproduced it.
The licensing rule is worth keeping and is going onto an item instead,
where the wider-than-a-family and sentence cases can be stated correctly.

**One adjudication against the lane's reading of the discipline.** The
unit deleted a drafted sync-guard citing §2's *"a predicate over things
fixed at compile time"*. The review found the same shape live at
`verbs/split.rs:218` (`assert_eq!(corr.tool_expected, "datum plane")`),
defended there *because the label is document-reachable*. The lane's
reading was **slightly too broad**: a pin on a committed, user-reachable
spelling is not a compile-time predicate in §2's sense, even when both
sides are consts. The unit's OUTCOME was right anyway — the behavioural
row subsumes the guard — but by §2's *other* clause, *"one its
neighbours already subsume"*. The PR body says so now; `split.rs:218`
is not touched.

**Four more items filed from the review**, none of which the unit or its
sweep could have reached:

- `work/lib/pncad-py-value-refusals-spell-family-words-as-literals.md` —
  `pncad-py/src/py/value.rs` builds five refusals with `kind_name()` on
  one side and a bare family word as a literal on the other, in the same
  sentence. The defect exactly, on the **public Python surface**, hitting
  two of PR 2376's disclosed blind spots at once (`format!` template;
  outside `editor-core/src/`). LIB's glob. It also records a contract
  nobody had written down: `Value.kind` exposes `kind_name()` to Python
  *"so the Python tag set cannot drift"*, which makes `family`'s ten
  strings a public API and not only a refusal vocabulary.
- `composed-expected-phrases-are-hand-copied-across-sites` — the
  composed phrases PR 2376 licensed to stay prose are duplicated among
  themselves: `"datum frame"` ×3, `"datum axis"` ×3, `"datum plane"` ×2.
  The PR answered *"do these belong in `family`?"* (a fair no) and not
  the class's actual question. Filed because PR 2376 closes its item
  with this disclosed, and `work/README.md` is explicit that disclosing
  a residue is not scheduling it.
- `frame-plane-lane-and-axis-frame-are-one-door` — two functions, the
  same `DatumValue::Frame` destructure, the same refusal, and
  `axis_frame`'s own doc saying *"Same door … and same refusal."* Q2's
  sharpest shape: the comment reconciling two spellings is the only
  evidence, because the code compiles either way. Two of `"datum
  frame"`'s three copies are these two functions, so it and the row
  above are one pair of lines seen from two directions.
- `wire-rs-module-header-describes-five-sixths-of-the-file` — from the
  reviewer's Q8 read of all 4731 lines, which nothing in this process
  otherwise does. The header promises each F4 node maps to an existing
  kernel op; `:3092-3810` is a ~720-line union-declaration-routing
  subsystem that maps to none. It also carries two measured accumulation
  facts (41% comment; `wire_sweep` exists to fail) so they are not
  re-derived.

Q8 earned its keep on the first review of this program.

## E1's full review returned: APPROVE-WITH-FIXES, and one finding outgrew the PR (2026-09-11)

Full review of PR 2375 delivered — **0 MAJOR, 3 MINOR, 3 NOTE**, plus
nine style findings and all eight style-lane questions exercised. **Every
one of the five dispatched claims survived falsification**, including the
two the orchestrator had verified by reading: `compose` is bit-identical
term-by-term AND under a differential harness over 200 000 random `Frame`
pairs drawn from `f64::from_bits`; `map(T::from_f64)` at `f64` moves no
bits; the public surface did not move (both demo roots checked
separately, which `--workspace` does not cover); the sweep table was
honest. The code is right. The fix pass is test strength and prose.

**The review's instrument is the story.** An eleven-mutation harness,
and two mutations went UNCAUGHT by a unit whose tests looked complete:

- **M1** — a `linear_f64` that snaps every column entry to `+0.0`
  passes all three tests, because the fixture's `-0.0` is in
  `translation` only. The test claims to pin twelve components and pins
  three.
- **M9** — permuting the product's columns inside `Mat3::mul` leaves all
  three tests green, because the compose oracle is built from
  `Mat3::Mul`, the operator under test. An oracle sharing an
  implementation with its subject is not an oracle. The fix rebuilds it
  from explicit scalar arithmetic.

Both are in the fix pass with the mutations to re-run as the receipt.
This is the answer to *"can this test fail"* (Q3) doing exactly what it
is for, on a unit that would have merged green.

**And one finding is bigger than the PR it came from.** In `--release`
the differential harness diverges in exactly one place: the **sign bit of
a generated NaN**, where one summand is a propagated NaN and another is
x86's QNaN-indefinite from `(-0.0) * inf`. Both spellings called the
identical `Mat3::mul`, so it is LLVM commuting `fadd` across two inline
sites — **a standing fact about every `Mat3`/`Affine3` product, not about
this PR**: NaN payload and sign are not stable under code motion, so D9's
fixed evaluation order buys determinism for **non-NaN outputs only**.

Not a live bug: every door that cites D9 for a bit claim is reached with
finite inputs and this kernel refuses non-finite geometry at its gates.
It is a premise nobody wrote down, on the one axis where a reader assumes
the opposite. Filed on PROPS
(`nan-sign-is-not-stable-under-code-motion-so-d9s-fixed-order-covers-non-nan-only`)
with the qualifier's home argued to be `geom-core/src/linalg/`'s docs
rather than `docs/DESIGN.md` — and with the explicit note that if a taker
judges otherwise, amending D9's text IS a design-doc change and goes to
Ev as one. **Flagged to Ev in chat** rather than left in a tracker file.

**The geom-core array-door row grew.** The review's differently-shaped
sweep found the class is **five** non-test sites, not two — and
`crates/geom-brep/src/ssi/system.rs:92,96` **already carry the missing
door, privately and by hand** (`fn v3(&[f64;3]) -> Vec3<f64>`,
`fn p3(...) -> Point3<f64>`). A consumer has built `geom-core`'s absent
function where it could not be shared, which is stronger evidence than
any count. The item is amended, and it now says a fix landing only the
matrix pair and leaving `Vec3`/`Point3` is a **half-fix** and must be
labelled one.

**Three more filed:**

- `frame-linear-generic-door-has-no-consumers` — after this PR,
  `pub fn linear<T>` has zero call sites workspace-wide, proven
  mechanically (dropping `pub` warns "never used"). *"A public generic
  door kept alive by its own test is exactly what the item was retiring
  at the other end."* Deliberately NOT done in the fix pass: it is a
  public-API removal, the review classed it non-gating, and a fix pass
  repairs what its review found — it does not widen a PR into a surface
  decision. Three answers argued on the row.
- `placement-rs-states-its-exactness-rule-in-nine-paragraphs` — S6/S8/S9
  together: one rule in nine doc paragraphs none of which is the
  authority for any other (this PR added two of the nine), a
  `#[must_use]` inconsistency inside the file, and a 60% prose ratio
  with `rotate_then_translate` at 38 doc lines over a 21-line body.
  Nothing in it is wrong, which is the finding.
- `work/issues/inert-allow-attributes-on-test-modules-…` — the instance
  (this PR's inert `#[allow]`) is in the fix pass; the class is **124 of
  257 `#[cfg(test)]` modules** carrying the attribute with nobody knowing
  which are load-bearing. In `work/issues/` because it spans every
  crate's `src/` and GUARD's `paths` are `scripts/gates/*` only — the
  genuinely-undecided-owner case the directory is for, not a waiting
  room. The count's own blind spots are stated on the row.

**One reviewer finding accepted against this orchestrator.** MINOR 3:
the lane disclosed its residue with no schedule and said so ("this
residue has no tracker file"). The reviewer is right that this is the
"recorded as a pickup" shape the style brief refuses, and right that the
obligation was the ORCHESTRATOR's, not the lane's — and that it should
read as an unmet obligation rather than be absorbed. It was met, but
after the PR body was written; the fix pass now cites the item file so a
reader of the PR can follow it.

## E3 landed green, and it refuted a doc in another crate to decide its own question (2026-09-11)

`names-flush-and-select-discard-a-refusal-with-map-err-underscore` →
**PR 2378**, CI run 34624968259 **green on the full matrix**. The first
run (34623155730) was red on `clippy (--all-features)` and the python
suite for one missed feature-gated site; see the fence note below,
because that red is the interesting part.

**Decision: CARRY, at all four sites**, and the argument is better than
the one the brief handed down.

1. D9 chose a typed error **over a panic**. A `Result` whose `Err` is
   destroyed one frame up is a bool in enum clothing — strictly worse
   than the panic D9 declined, since a panic at least prints the
   formatted `BandError`. Citing D9 to defend the unit variant is
   self-defeating.
2. **The cause is not unique, and the doc claiming it is, is wrong.**

**The measurement, re-derived by this orchestrator before it was filed
anywhere.** `Band::linear`'s `# Errors`
(`crates/geom-core/src/predicate.rs:355-366`) says `BandError` arises
*"only when K·ε … overflows to infinity"*. `Tolerance::validate`
(`tolerance.rs:479-487`) admits **any** finite ε > 0 and **any** finite
K > 1. At ε = 5e-324 (min subnormal) and K = 1 + 2⁻⁵² the increment
`K·ε − ε` falls below half of 2⁻¹⁰⁷⁴ and rounds away, so `K·ε == eps`
**exactly**, `Band::new` gets `zero == escalate`, and the band is
rejected as `BandError::Empty`. Nothing overflowed. Checked:
`K*eps == eps` is `True`; at K = 10 it is `False`, which is why no
ordinary tolerance meets this.

For any **normal** ε it genuinely cannot happen (K ≥ 1 + 2⁻⁵² forces one
ulp of headroom), which is presumably the reading the sentence was
written from. It is not true of the set the validator admits, which is
what the word *"only"* claims — and **the two reachable ends want
opposite repairs** (lower ε, against raise ε-or-K), so a refusal naming
neither sends half its readers the wrong way. That is the whole case for
carrying, and it could not have been made without the measurement.
Filed on PROPS as
`band-linear-errors-doc-is-false-empty-is-reachable-at-subnormal-eps`.

**Class estimate E was wrong and the row is corrected to H.** The lane
proposed "F", which is not in this plan's vocabulary; read as H, and the
reason is the lane's own and is right: *"the decision the item delegates
to the owner could not be made without a measurement that refutes a doc
in another crate — that measurement, not the edit, was the unit."* Four
sites not two, two crates, three refusal surfaces, ten files.

**Review raised light → FULL**, second time today under Ev's rule. Two
public enum variants gain a field; the argument rests on subnormal-float
reachability; user-visible Python refusal text moves. `plan.md`'s
posture table records both raisings with their reasons.

**Fence crossings, and the one the lane could not see.** The unit edits
three territories that are not WIRE's, and the reviewer is asked to
check each against the PR body rather than take the lane's word:

- `crates/pncad-py/src/{tags.rs,tests.rs,py/select.rs}` — **LIB's**.
  **Forced**: a public enum gained a field, so its renderer one crate
  over had to change. Legitimate, and still owed an announcement.
- `crates/editor-core/tests/display_contract.rs` — S-TCOST's/S-TINT's
  `crates/*/tests/*` glob, taken under the standing convention (a unit
  adds rows there as ordinary tests and says so in its PR).
- `names/geompred.rs`, `emit.rs`, `discriminate.rs`, `names/README.md` —
  **in no open program's `paths`.** WIRE's list names only `flush.rs`,
  `select.rs`, `table.rs`.

That last line is the standing question this program should settle:
**WIRE's `paths` should probably just be `crates/editor-core/src/names/*`**
rather than three files, since every unit that touches `names/` is now
drawing the fence one file at a time. It is a `program.md` edit with no
design content, so it does not wait on Ev; it waits on the next unit
that would otherwise draw the fence again, and is recorded on
`emit-topo-destroys-the-edge-key-in-a-split-lineage-cycle`.

**The CI red is a finding about sweeps, not about this lane.** The
missed site was `pncad-py/src/py/select.rs:819`, feature-gated and one
crate outside the grep scope the brief gave. Two generalisations the
lane recorded and that this program should keep: **the shape does not
respect the crate fence a sweep is scoped to**, and **a refusal's
RENDERING sites are part of its class** — the payload had reached the
type while that arm still threw it away.

**Three residues placed**, each with its measured reason:

- `work/shell/clearance-reports-a-no-bodies-payload-as-a-bad-body-index.md`
  — `clearance.rs:1979` reports `InterrogateError::NoBodies` (the node
  denotes no bodies) as `NoSuchBody { index }`. **Stronger** than the two
  instances this unit fixed: those raised an honest kind and lost detail;
  this raises a kind that points at the wrong argument. SHELL's file.
- `work/fix/remap-name-misses-lose-the-id-they-caught-at-six-refactor-sites.md`
  — six sites discard `remap_name`'s `Err(RecipeNodeId)`. Not redundant
  with the name the raised error carries: a `StableName` embeds names in
  its `path`, so the failing node can be a path segment two levels down
  and the outer name does not identify it. FIX's file.
- `emit-topo-destroys-the-edge-key-in-a-split-lineage-cycle` — kept on
  this program's slate (unowned file inside `names/`). The weaker
  instance: kind honest, locator destroyed, on the one failure where the
  edge is the only thing worth saying.

**One stated gap accepted as honest.** The lane could not build an
end-to-end row through `select_where`, because `editor-core`'s
integration suite is one binary with one committed `Tolerance` and a
pathological ε would poison every other suite. It said so rather than
writing an assertion that cannot go red — but the reviewer is asked to
verify the constraint is real, because "no door exists" is exactly the
claim a lane is least placed to check about its own unit.

## E3's full review: 5 MAJOR, and the fresh-instance bullet earned its keep (2026-09-11)

Full review of PR 2378 delivered — **APPROVE-WITH-FIXES, 5 MAJOR / 6
MINOR / 5 NOTE**, all eight style questions exercised plus the stance
bullet that landed on `main` mid-review. The **decision is right** and
the shipped code is correct and green; every MAJOR is a false sentence
or a test that does not guard what it claims.

**The bullet was sent to the lane mid-review and immediately produced
the sharpest finding.** `docs/prompts/reviewer-style-lane.md` gained
*"when the diff is itself a fix for a structural finding, check whether
the fix mints a fresh instance of the defect it closes"* — and PR 2378
mints two:

- **M4, on the public Python contract.** `pncad-py/src/tags.rs:211` and
  `:1248` map `Band { .. } => "band"` **flat**, while `py/select.rs:705`
  says in as many words *"The fields are the contract; the message is
  prose."* The unit moved the cause into `message` and left `reason` as
  information-free as before, so **a Python caller branching on `reason`
  still cannot tell an overflow from a collapse — which is the unit's
  entire thesis.** `band_error_tag` already returns the three words, the
  PR body cites it as precedent, and `TAG_INVENTORY`'s own doctrine
  twenty lines from the entry the PR added forbids exactly this
  flattening. The fix pass delegates and widens the inventory; that is
  LIB's ground and the crossing is already being made, so it is
  announced rather than avoided.
- **M1, at the test layer.** The unit's premise is about `Band::linear`;
  its test exercises `Band::new(ε, K·ε)`, a hand re-derivation of the
  `linear → from_zero_threshold → from_thresholds → new` chain. The
  reviewer mutated `from_thresholds` to make `Empty` unreachable from
  `Band::linear` **entirely** and the test stayed **green**. Line 322
  does it again, hand-copying `Tolerance::validate`'s predicate instead
  of calling it. A unit whose thesis is *"a proxy for the cause is not
  the cause"* asserted over a proxy where the real thing was available.

**Two of the orchestrator's own filed sentences were wrong and are
corrected.** The review attacked the numbers instead of accepting them:

- **The collapse region is not a knife-edge.** At ε = 5e-324, **every
  K < 1.5** collapses the band — 1.1, 1.25, 1.4 all give `K·ε == ε`;
  1.5 is the first that does not. True condition: ε subnormal with
  ε < 2⁻¹⁰²³ **and** K < 1 + 1/(2n) for ε = n·2⁻¹⁰⁷⁴. The "K within an
  ulp of 1" framing — the lane's, this log's and the PROPS item's — made
  a region sound like a point.
- **`Band::angular_at` carries the same false `# Errors` sentence and it
  is WORSE**: ε = 1e-9 (the ordinary session-box order) with
  `lever_arm = 1e300` gives a subnormal `zero`, reachable with a
  perfectly normal tolerance. What is extreme is the **lever arm**,
  which is a caller argument per predicate, not a run configuration —
  and `angular_at`'s own doc tells callers to pass a large one. The
  PROPS item is amended and now says a fix correcting only `linear` is a
  **half-fix**.

**M3 is the methodology finding and it has been filed as a class.** The
unit's sweep missed a live instance **in a file it swept**:
`clearance.rs:1076` discards a `BandError` into the unit variant
`ToleranceHasNoBand` — bit-for-bit the pre-PR defect — 900 lines above
the site the sweep did triage. It was invisible because **the discard is
a `let-else`, not a `map_err`**. The instrument was a spelling; the
class is *a typed cause is destroyed*, which is also spelled `let-else`,
`expect`, a narrowing `impl From`, `.ok()`, and a unit variant at the
rendering site — the last being the shape the CI red found, and the one
the lane had itself written down as a lesson before missing it again.

Filed as `work/issues/every-band-construction-is-the-class-not-every-map-err.md`,
with the transferable rule: **sweep for the construction of the thing
whose cause can be lost — every `Band::linear`/`angular_at`/`new` call —
not for the syntax of one way of losing it.** That row also asks whether
amending `docs/prompts/implementer-discipline.md` §5 is an `[ev]`
question, since it is read by every lane by path, which is close to the
`memories/` rule.

The `clearance.rs` instance went onto SHELL's existing row, which now
carries both of that file's destroyed causes as one unit's work.

**Style S1, which the orchestrator declined to mandate.** The unit
spells the new field `source`. The tree's dominant shape is a
**newtype** (eighteen sites, including `eval/wire.rs:702` inside WIRE's
own fence), and where a named field is used the tree spells it
**`error`** — including `mate/solve.rs:673`'s `MateFault::Band { error }`,
which is **MSOLVE-3's fix, the precedent this item cites.** Against
that, `source` is locally consistent with `PairInBand` and `Escalated`
in the two enums edited. Within-file against tree-wide, on a PR about
spelling discipline: the lane picks and argues it in the body rather
than being told, because either is cheap now and neither is later.

**Fence crossings, and the largest source change was undisclosed.**
`names/geompred.rs` (+28/−5, where `SelectRefusal` and its `Display`
live) is named **nowhere** in the PR body; `tests/display_contract.rs`
(+80) is never announced as crossing S-TCOST's/S-TINT's glob; LIB is
never named as `pncad-py`'s owner. All three in the fix pass. This is
the third unit in a row to touch an unowned `names/` file, which settles
the standing question: **WIRE's `paths` should be
`crates/editor-core/src/names/*`**, and that edit rides the next unit.

## The E block is merged (2026-09-11)

All three E units landed on `main`, each green on the full code tier and
each verified at the CHECK-RUN level before merge rather than on a
lane's summary — 33 success / 5 skipped, twelve `test (…)` jobs and five
`k-lint (gate, …)` rows on every one, so nothing was narrowed.

| unit | PR | merge |
| --- | --- | --- |
| `wire-expected-phrases-spell-family-words-as-literals` | 2376 | `2d47435f` |
| `placement-lifts-its-affine-by-hand-beside-affine3-map` | 2375 | `cec951ed` |
| `names-flush-and-select-discard-a-refusal-with-map-err-underscore` | 2378 | `477a2cb7` |

**Three items closed, twenty filed.** Every residue has its own file, on
the slate of whichever program owns the ground — PROPS (4), SHELL (2),
FIX (1), LIB (1), `work/issues/` (2), and the rest here. Nothing was left
as a sentence in a merged PR body.

### What the block actually bought, beyond three diffs

**Two of the three items were wrong about their own subject**, and each
lane corrected the row rather than implementing the row:

- `placement-lifts-…`'s suggested one-liner does not compile.
- `wire-expected-phrases-…`'s counts were wrong in both directions and
  internally inconsistent before the tree moved.
- `names-flush-and-select-…` was class E and is H, because answering its
  delegated question required refuting a doc in a third crate.

**The reviews found two units reminting the defect they closed**, which
is the stance bullet that landed on `main` the same day — twice
independently before it existed, and once (PR 2378's flat Python tag)
after it was relayed mid-review. It is now three for three across this
block. Worth saying plainly: **on this program's evidence the bullet is
not a tendency, it is the default**, and the only thing that caught any
of the three was a reader who had not written the fix.

**Two reviews found a test that could not go red**, both by mutation
rather than by reading: PR 2375's compose oracle was built from the
operator under test, and PR 2378's band test asserted over a hand
re-derivation of the chain whose door it claimed to guard. Neither was
visible in the diff. Q3 is the highest-yield question this block
exercised and the only instrument that answered it was a mutation
harness.

**Three of this orchestrator's own filed sentences were wrong** and were
corrected from lane and reviewer measurements: the band-collapse region
is not a knife-edge (every K < 1.5 at the smallest ε); the default
K = 10 puts it out of reach at **every** ε, so it needs two knobs turned
and not one — this file's earlier claim that K = 10 "sits outside it only
because ε does" was backwards; and `Band::angular_at` carries the same
false sentence and is worse.

### The standing fence question, now settled by repetition

Three units in a row touched `names/` files in **no open program's
`paths`** (`geompred.rs`, `emit.rs`, `discriminate.rs`, `emit_topo.rs`,
`names/README.md`), each drawing the fence in its own PR. That is the
convention working, and it is also three PRs paying for a `program.md`
edit with no design content in it. **WIRE's `paths` take
`crates/editor-core/src/names/*`** at the next unit.

### Next

The lift pair (`profile-has-no-scalar-lift-door` then
`profile-embed-lift-has-two-homes-anchor-and-loft`) as one sequence, with
the door announced to S-BOOL and the ordering announced to S-TINT, whose
`D385` converts the two test copies afterwards. `frame-f64-placement-…`
and `product-gather-…` follow, the second still waiting on Ev's read of
the carry-the-tie recommendation.

## The `[ev]` question is out, and the lift pair is dispatched (2026-09-12)

**`[ev]` PR 2404** — `docs/AXIS-DECLARATION-DESIGN.md`, `needs_ev: true`
on `axis-flavoured-declarations-have-no-channel`, a `docs/DESIGN.md`
companion-table row marked OPEN QUESTION, and a PR subscription so Ev's
comments wake this session. Docs and tracker only; marked not-for-merge
until answered.

**The fork this program opened with dissolved under Ev's pushback**, and
the doc records the resolution rather than the fork. Ev confirmed the
reading of `cs_pair_frame`'s sentence — "never inferred" means a
declaration cannot be OBTAINED by measuring — and then rejected the
premise:

> *"I don't like relying on the numerical check to tell if it's been
> rotated."*

Right, and live rather than hypothetical: `Node::Declare`'s pairs name
entities by `StableName` and re-resolve at every evaluation, so a
rotated operand re-asserts the declaration and only a band check would
catch it. The recommendation is therefore **(a) and (b) composed** —
declared intent, invalidated structurally by comparing the two carriers'
placement chains, on the fact that coaxiality is invariant under a rigid
motion applied to BOTH carriers and destroyed by one applied to one.
`SourceExpr::Placed`'s cons-list already carries what that comparison
needs, and its own doc states the rule (*"Equal chains ⇒ equal maps
applied to equal descriptions ⇒ equal bits (D9)"*).

**Two of this orchestrator's claims were wrong and the doc carries both
retractions.**

- A variant hanging identity on the **axis datum** is inventing a
  channel, not wiring one up: `DatumValue` is by its own doc *"geometry
  VALUES, not kernel entities and not recipe references"*. Comparing
  CARRIERS avoids that, which is most of why it is the recommendation.
- **It is not a kernel-anchors problem.** Ev asked whether recipe-shaped
  identity is required; it is not. `topo/src/source.rs`'s module doc:
  the fields are *"the lowered pure-data forms (`u64` node ids,
  structural expression addresses)… this crate only ever compares them
  for identity and flips orientation"*, and nothing maps a
  `GeomSource.node` back to a `RecipeNodeId`. The link this program drew
  to `two-verb-seats-do-not-compose` is **withdrawn**. What a new minter
  owes instead is the retirement theorem plus **namespace
  disjointness** — load-bearing, because `RecipeNodeId(pub u64)` is a
  full `u64` with no free high half and a collision would have the
  boolean's coincidence rung glue two unrelated surfaces.

**Ev's adoption-step idea is recorded** and is cheaper than it looks:
`import_step` already keys its maps by the file's entity ids and
discards them at the door, a STEP entity id is real identity, and M8
instancing is a pattern in all but name so `Placed`'s `instance` field
serves it unchanged. Not a prerequisite. It would shrink question 3 to
hand-built bodies alone.

**Dispatched: the lift pair, as ONE unit** — `wire-m1`, branch
`wire/profile-lift-door`, carrying both
`profile-has-no-scalar-lift-door` and
`profile-embed-lift-has-two-homes-anchor-and-loft`. Staffing them apart
mints the door twice, which is `plan.md`'s ordering rule 5.

Three fence crossings, all of which the PR body must disclose and which
the review will check: `crates/profile/src/*` and
`crates/sweep/src/loft.rs` are **S-BOOL's** (the door is minted there by
announced seam, as EVAL-1's and FILLET's were), and
`crates/editor-core/tests/*` is **S-TCOST's/S-TINT's** glob. And the
ordering constraint the lane may not break: it mints the door and
retires the PRODUCTION copy only — the two test copies are S-TINT's
`D385`, afterwards, against a door that by then exists. The PR body is
how S-TINT learns the door landed, since `D385`'s list today names one
of those two copies.

The brief carries this program's three standing lessons by name, because
this unit is the one most likely to hit the first: a fix for a
structural finding tends to mint a fresh instance of the defect it
closes, and this lane is retiring hand-spelled lifts.

**Posture: FULL review**, per `plan.md`'s table — it changes what a loft
builds if the door is wrong.

## The axis question is RATIFIED and merged; the lift pair is in full review (2026-09-12)

**PR 2404 merged** (`4d40f670`). Ev ruled in two rounds: *"axis shaped
sounds good / refuse on absence also sounds good"*, with the shape
itself — declared intent, structural invalidation — ratified the round
before. `docs/DESIGN.md`'s table carries it as **Ratified; unbuilt**.

**A correction in the ratified option's favour, found by reading P1's
actual text before acting on it.** Round 2 priced axis-shaped as costing
a reopening of `crates/verbs/README.md` §3 P1. Wrong: P1's exclusion is
scoped to **motion-invariant** fields (*"a stored scalar field is
motion-invariant, so no kernel op composes or interprets one"*), and an
axis is not one. An axis is placement data, so it falls on
`GeomSource`'s side of the line P1 draws — where `Placed` composition
already lives in the kernel. VS-Q4 says the same from the other end: it
rejects a SourceExpr-style address *"with nothing to compose a
motion-invariant field"*, and an axis gives it something to compose.
P1 stands untouched; what is new is granularity, not scope.

**The row's standing obligation is discharged by evidence**:
`crates/verbs/README.md` §3 P2's SPHSPH sentence, which this row
required to be corrected when answered, is **gone** — `SPHSPH`,
`CoaxialEvidence` and `parallel` all return zero hits. It left in other
work since 2026-09-04.

**Three rows opened for the unbuilt design**, because ratified-and-unbuilt
is work and not a row staying open: `axis-shaped-identity-channel`
(this program — and cut as a SEQUENCE, not dispatched as one lane: it
spans WIRE, TOPO's `source.rs`, S-BOOL's `join.rs` and editor-core),
`work/topo/geom-source-absence-conflates-four-origins.md`, and
`work/exch/step-import-discards-the-entity-ids-that-are-its-identity-channel.md`.

## The lift pair landed green, and the trap fired a fourth time

**PR 2409**, CI run 34666874179 green on the full code tier (twelve
`test (…)`, five `k-lint (gate, …)`; the four skips are cache primers
and two `interval-transcendentals` rows the change filter did not buy).
Both items are one PR, as `plan.md`'s ordering rule required.

**The items were wrong about the tree in four ways** and the lane
corrected them: half the door already existed (`ProfileLoop::map`,
BOOL-9, ratified in `crates/profile/README.md` V4); `Section` is
`pub type Section = Vec<ProfileLoop<f64>>` in **sweep**, an f64-pinned
alias that cannot carry a door, so the top rung is `Profile`;
`end_profile` is at `:223`-`:232`, not `:225`-`:245`.

**And the fourth: `ValidatedLoop::lift` hand-spelled the vertex rung** —
so BOOL-9's fix for the loop rung **minted a fresh copy one level
down**, which is the stance bullet's shape found live in the code this
unit was sent to fix. Fourth instance on this program, and the first
found by an implementer rather than a reviewer.

**The substantive change is not the door.** `end_profile`'s re-`validate`
at `T` is retired — validation happens at `f64` and lifts through
`ValidatedProfile::lift_onto`, dropping `T: Decide` → `T: Real` — on the
argument that `skin::validate_sections` has already validated each
section at `f64`, so the end profiles were the one part re-deciding.
Disclosed behaviour move: a section whose `f64` validation decides but
whose `T` re-validation would escalate now lofts instead of refusing.
Class estimate: **M was right for the pair and wrong as two Ms** — the
door is a morning, the weight is this decision.

**Review dispatched FULL**, with seven claims. The three the orchestrator
weighted heaviest:

- **The k-lint sentence is the weakest link.** The PR says the
  end-profile predicates now fire at `f64` rather than the assembly's
  `T`, *"so they leave a `Probe` lane's sample stream — k-lint came back
  green, so nothing moved in practice."* Green over a CHANGED
  distribution is a different claim from green over an unchanged one,
  and if k-lint cannot see this family at all then its green is not
  evidence about it. The reviewer reads the RUN's steps, not the
  workflow source.
- **`Profile::map_scalar` has no production consumer**, which is exactly
  the shape this program filed three PRs ago as
  `frame-linear-generic-door-has-no-consumers` — *"a public generic door
  kept alive by its own test is exactly what the item was retiring at
  the other end."* Minted fresh by a PR whose job was retiring
  hand-spelling, or genuinely different because a scheduled consumer
  exists? Argued either way, not skipped.
- **The stated test gap**: no fixture separates the two paths by outcome
  without knife-edge tuning. Stated rather than hidden, which is right —
  and it is Q3 about the unit's own central claim, so the reviewer tries
  to build one anyway.

## Disk: reclaimed, and the orchestrator was late

The lane reported `/` at **100%** mid-run and skipped its local
`demos/tour`/`demos/wild` clippy because of it (grep-verified instead;
CI's rows cover them, and the reviewer confirms from the run record).
`agent-lane-operations` is explicit that reclaiming a finished lane is
the **orchestrator's** job and is done *when a review returns* — six
lanes had reported and none had been swept. 23 GB in eight `*-target`
directories, of which the review lanes are the documented biggest
consumers.

Reclaimed the three merged E lanes' targets and all three review lanes
(targets and clones): **1.5 GB free → 24 GB**. `wire-m1`'s target is
kept, its PR being in review with a fix pass likely. Nothing was running
— checked `pgrep cargo` and every target's mtime before deleting, per
the memory's rule that a running build's target is never reclaimable.

## PR 2409's full review: 2 MAJOR, and the reviewer built the fixture the lane said could not exist (2026-09-12)

**APPROVE-WITH-FIXES, 2 MAJOR / 6 MINOR / 4 NOTE**, plus nine style
findings and all eight style questions exercised. The lift is **exact**
and claim 2 was verified term-for-term rather than accepted:
`ValidatedSegment::lift` calls the same `pub(crate) seg::arc_carrier`
that `build_seg` does, on scalars carried verbatim, with nothing
rounding between.

**MAJOR-1 — there is a SECOND behaviour move and the unit's new doc
states its opposite.** `loft_body`/`sweep_body` call `loft_geometry` and
then `assemble` with the same arguments, and `validate_sections` has
already validated **every** section including first and last. So
`end_profile`'s `Profile::validate` is a **provable no-op** and
`LoftError::Profile` is **unreachable from both public doors** — with
dead arms left in `eval/wire.rs:4319` and `pncad-py/tags.rs:1217`. The
new doc's *"the `f64` validation is the gate, and it refuses here"* is
false; the gate is `loft_geometry`'s.

**And style finding S2 names the fix**: `LoftGeometry` **already** keeps
`sections` under the comment *"no re-derivation, no drift"* and
**throws away** `validate_sections`' `Vec<ValidatedProfile<f64>>` —
exactly what `end_profile` recomputes. Adjudicated: carry the validated
profiles on `LoftGeometry` and have `end_profile` READ one. That deletes
the redundancy instead of documenting it, retires the dead variant
honestly, and makes the PR's own *"structural rather than assumed"*
sentence true — which, as the reviewer notes, it currently is not.
Fallback if threading proves structural: retire the variant and correct
the doc; shipping a knowingly-unreachable arm under a doc that says it
fires is not on the table.

**MAJOR-2 — the stated impossibility was not one.** The lane wrote that
no fixture separates the two paths by outcome without knife-edge tuning,
and stated it rather than hiding it, which was right. The reviewer built
one with round parameters: two sections, vertices `(∓h,0)` at bulge `b`,
`h=5, b=1e3` — `Ok` at `f64`, `Escalated` at `Interval` on main, `Ok` on
the PR head. **Broad, not a knife edge**: six `(h,b)` pairs across four
orders of magnitude separate at profile level. Adopted as a permanent
row with the reviewer's authorship. The one disclosed behaviour move had
nothing pinning it.

That is the second unit in a row where **the answer to "can this test
fail" came from building the thing, not from reading** — and here from
building the thing the author had argued could not be built.

**One correction owed to the lane, from the orchestrator.** The review
dispatch quoted the PR body as saying *"k-lint came back green, so
nothing moved in practice."* That sentence is in the lane's **report**,
not in the PR body, whose actual text is properly hedged and leans on no
green. NOTE-1, and the dispatch's error.

**MINOR-5 is the finding that outlives the PR.** The reviewer
instrumented the coverage change instead of arguing it: one minimal
two-section loft at `Probe`, **main 455 samples / 33 predicate names,
PR head 397 / 24**. Nine whole profile-validation families leave the
loft path. And the gate **cannot see it** — k-lint flags margins per
row, so a shrunken population yields weakly fewer flags, and
`predicate_roster.rs` reads kernel source rather than the sweep. That is
a **third face of the silent-coverage class**
`memories/agent-lane-operations.md` collects: a green gate over a
population that got smaller. Filed with three dispositions and no pick,
because the cause is S-BOOL's and the instrument is INSTR's.

**Five rows filed:**

- `loft-path-loses-nine-predicate-families-from-the-probe-stream` (here)
  — the schedule Q6 owes for a disclosed narrowing.
- `frame-linear-generic-door-has-no-consumers` **promoted to a class**
  with `profile`'s two `map_scalar` rungs as its second instance, proven
  the same mechanical way (drop `pub`, read the dead-code warnings).
  Adjudicated **not the same defect**: `Frame::linear<T>` LOST its
  consumers, while `profile`'s rungs were minted to
  `scalar_lift.rs`'s written convention, which a library owes an
  external caller. The class's real question — does that convention mint
  doors ahead of consumers, and if so is `Frame::linear<T>` a violation
  that should therefore STAY — disposes of both rows and neither answers
  alone.
- `work/bool/arc-carrier-has-three-spellings-under-a-comment-saying-one.md`
  — `seg.rs` calls itself *"the ONE spelling"* and the crate has three,
  with different association and `abs` placement. PR 2409's bit-identity
  claim is unaffected; the sentence was already false when written.
- `work/bool/validate-rs-hosts-a-quarter-of-the-fillet-subsystem-it-never-runs.md`
  — six `FILLET_*_RECOURSE` consts whose own docs say *"No caller reads
  this sentence"*, in a file whose inventory says fillet rows never fire
  in validation. From the Q8 whole-file read.
- `work/tint/the-two-vertex-bulge-one-circle-fixture-has-eight-copies.md`
  — the standing trap's **fifth** instance and its first at the test
  layer, which matters because every instrument so far has pointed at
  `src/`. Filed with the warning NOT to assume a shared helper: three of
  the eight are reviewer-authored probe suites whose independence is
  their value.

**NOTE-3 cleared the disk-skip worry**: `demos tour fmt + clippy` and
`demos wild fmt + clippy` both succeeded at STEP level on run
34666874179, and the `Probe` sweep really did re-cut. The lane's grep
substitute was sound.

## The lift pair is merged (2026-09-12)

**PR 2409 merged** (`17e96880`) after a conflict round. CI run
`34672529546` green on the full code tier at the resolved head —
verified at check-run level before merge: 34 success / 4 skipped, twelve
`test (…)`, five `k-lint (gate, …)`.

**The conflict was one file and the lane's verification of the fix is
the part worth recording.** `crates/sweep/tests/all.rs`'s mod list,
against another lane's `census_containment_cause`. Resolved as the
union. What the lane then did, on the two cautions
`memories/agent-lane-operations.md` supplies:

- it did **not** read CI off the PR's checks list — it polled the
  workflow RUNS list for a run whose `head_sha` matched the resolved
  head, then polled `/jobs` until `total_count > 0` before waiting on a
  conclusion. That is the answer to "a CONFLICTING PR gets no run, and a
  run can queue with zero jobs behind a superseded one";
- it checked the post-condition against the **merged tree**, deriving
  the declaration set from the merged `all.rs` and diffing it against
  the directory listing — 261 declared, nothing declared-but-missing,
  nothing on-disk-but-undeclared, and both suites' rows executing. A
  dropped `mod` in that file silently stops compiling a whole suite and
  would never appear in a diff.

**What this unit actually was.** Not the door — the door was a morning,
and half of it already existed. The weight was at `end_profile`, and the
review found it was **re-deciding data already decided**:
`validate_sections` validates every section at the geometry door, so
`Profile::validate` there was a provable no-op and `LoftError::Profile`
was unreachable from both public doors, carrying dead arms in two
crates. `LoftGeometry` already kept its section curves *"no
re-derivation, no drift"* and threw away the verdicts; it keeps them
now, `end_profile` reads one, and `assemble` lost the parameter that
existed only to feed it.

**The gate out-reasoned both the lane and the reviewer**, which is the
result to remember from this unit. The lane said no fixture could
separate the two paths without knife-edge tuning. The reviewer **built
one** from round parameters and proved it. Then CI reddened three ε rows
on it: a **pinned** `(h,b)` separates the arithmetics only at the
default ε — at 1e-12 the `f64` side escalates too, at 1e-6 the
`Interval` enclosure fits inside the wider band — and scaling with ε
fixes one and not the other, because the enclosure width and the `f64`
residual come from the same rounding and only ε moves between them. The
row now searches the ladder at the run's own ε and **panics when nothing
separates**, so it cannot pass vacuously. Neither party's reading
produced that shape; the three-ε matrix did.

**Four of this program's seven merged units have now had their central
test claim corrected by something other than reading** — two by
mutation, one by a built fixture, one by the ε matrix. That is the
strongest pattern this program has produced, and it is worth carrying
into how the remaining units are reviewed: *ask what instrument answers
the claim, not whether the claim is argued well.*

## Program state at this point

Merged: the three E units, the `[ev]` ratification, the lift pair.
`plan.md`'s remaining slate is `frame-f64-placement-is-re-evaluated-per-profile`,
`product-gather-…` (mechanism settled — `defer::TieRows` and
`narrow_into` already exist; Ev approved carry-the-tie), then `D364`
before `S195`, plus the rows this program has filed on itself.

Deferred and not dispatchable: `S40`, `two-verb-seats-do-not-compose`,
`axis-shaped-identity-channel` (ratified, unbuilt, and a SEQUENCE to cut
rather than a lane to dispatch).

**Still owed to Ev, and not blocking**: whether amending
`docs/prompts/implementer-discipline.md` §5 with a worked example on
choosing a sweep's instrument is his call, since that file is read by
every lane by path — recorded on
`work/issues/every-band-construction-is-the-class-not-every-map-err.md`.
