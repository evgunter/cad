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

## CLAUDE.md's ratified-text rule changed today, and PR 2409 is retroactively in scope (2026-09-12)

`CLAUDE.md`'s git-workflow section gained two exception categories since
this program opened. The one that bites: a PR that **changes an
already-ratified decision** — `docs/DESIGN.md`, or a
`crates/<crate>/README.md` design page its companion table lists —
**waits for Ev's sign-off**, and explicitly

> even when a code change forces it and even when the amendment is
> mechanical — a gate that reds until a roster row goes is a reason the
> change is *needed*, not a reason it is *approved*.

**PR 2409 amended two pages in that table and was self-merged**, under
the rule as it stood (which covered only design-question ratification
and `memories/`):

- `crates/profile/README.md` — Ratified V1–V8. V4's clause: *"the `map`
  materialization door"* → *"the `map_scalar` materialization door"*.
- `docs/PATHS-DESIGN.md` — Ratified #124. The CLOSED paragraph, and not
  only the name: *"`sweep`'s loft is its production caller"* became
  *"it has no production caller"*, which is a **substantive claim
  change**, not a rename.

And the new clause's own example — a gate that reds until a roster row
goes — describes `raw_door_census`'s pinned public surface, which PR
2409 also edited (`let pinned = ["map_scalar", …]`). Reported to Ev in
chat with both diffs, for his call on whether the text stands as merged.

**Standing change for this program**: every unit brief from here names
the rule, and a lane that finds its change would amend ratified text
**stops and reports rather than amending**. `crates/editor-core/README.md`
is in the table three times, so the next unit is already exposed to it.

## Dispatched: `frame-f64-placement-is-re-evaluated-per-profile` (2026-09-12)

Lane `wire-m2`, branch `wire/frame-f64-placement-once`. **FULL review**
per `plan.md`'s posture table.

The brief carries three things beyond the item:

- **The ratified-text rule above, with the specific exposure named.**
  `crates/editor-core/README.md`'s #1151 entry is *"PROFILE-LIFT-DESIGN
  PP1–PP6: guided replay — structure f64-once as the witness, geometry
  at the lane scalar with every consumed decision re-verified at `T`"* —
  which is very close to carrying a frame's f64 placement on a
  `NodeResult`. The lane must read PP1–PP6 and say whether this change
  is an instance of that clause, an extension, or in tension with it,
  and STOP rather than amend if the latter.
- **The correctness arm stated as evidence owed, not argument**: is the
  carried placement bit-identical to what `profile_plane_f64` derives
  today, and does any **content key** change. A key is a persisted
  commitment, so "tests pass" is not an answer to the second.
- **A citation warning with a specific hit.** Three items in a row have
  been wrong about the tree. This one claims `slots.rs`'s header says
  "once per node per environment"; that phrasing is not in the file. The
  lane checks rather than inherits.

### Resolved: the rule was wrong, not PR 2409 (2026-09-12)

Ev, in chat, on the clause the entry above reports: *"it should not have
an absolute rule like that. it should say that sign off is required if
it is a design choice, which is the normal rule."* **PR 2432** merged
(`9b395984`) with that fix — the category stays, scoped to the design
choice, and a clause re-worded because an approved code change moved
something it describes lands with that change.

So **PR 2409 was in order**: both amendments tracked a rename and a
caller that went away. The entry above stands as the record of the
question; it is answered, and nothing is owed.

The absolute version also contradicted the test directly above it in
`CLAUDE.md` — *"text that binds future work rather than describing this
change"* — which already said the right thing.

**What stays a judgement call, by Ev's ruling**: whether applying a
ratified convention against an earlier ratified naming (2409's
`map` → `map_scalar`, `scalar_lift.rs`'s convention against BOOL-9's)
is a design choice. Ev: *"that kind of thing is necessarily a judgement
call"*, and the call this program made was right. It is not reducible to
a rule and none is written.

## `frame-f64-placement` landed green; PP6's citation lands with it (2026-09-12)

**PR 2435**, CI run `34684190899` green on the full matrix (twelve
`test (…)`, five `k-lint (gate, …)`). The carry is
`NodeValue::placement_f64`, minted post-op by `frame_placement_f64` from
`eval_node`'s **existing** nominal slot list through the same
`frame_from_slots` door the frame's own op uses — it evaluates no
expression. `profile_plane_f64` becomes a pure read; `FrameKind` and
`frame_kind` are deleted and the DM1c fork moves to the mint.

**The correctness arm came back negative on keys and positive on
values, which is the right shape**: `content_key`'s arguments are
untouched, the placement is value-side and was never hashed, and the
profile's dependence on the frame still rides in `upstream_keys`. What
moved is the **verdict log** — two `datum_unit_norm` decisions per frame
are now made once instead of per profile, so `N−1` are removed and none
added.

**Two baselines re-blessed, and the re-baselining lesson is the durable
part.** `kstats_bracket_rows.rs` (PRE_PASS 75→73, FRAME_LOG 2→4) and
`golden/m10_6_certifying_keys.txt`. The golden reds **only on the
interval lane**, so the lane's default-feature local run was green over
a baseline that had genuinely moved and **CI caught it**. That is a
hazard worth carrying: *a local green is not evidence about a lane you
did not build*, and it is the second time this program has seen a claim
survive a local run and die on the matrix.

**The ratified-clause question, decided under #2432's corrected rule.**
The lane found `crates/editor-core/README.md` **PP6** cites a mechanism
that moved — *"(`profile_plane_f64`, read from the document's own
slots)"* — and **declined to amend it**, because its brief predated the
rule change and said to stop and report. Right call on the brief it had.

Adjudicated: **it lands with this PR.** PP6's *decision* is unchanged —
an authored frame's plane still stays f64 under every lift, and the fork
by frame kind is preserved exactly — and what is stale is a symbol that
moved. That is exactly #2432's *"not a second decision, and lands with
the change that caused it."* No roster row reds either; the PP1–PP6
companion row names `prepare_profile`, `lane_profile` and `section_of`,
not `profile_plane_f64`.

The lane's reading that this change is an **instance of PP1** rather
than an extension or a tension is accepted: PP1's "structure f64-once as
the witness" is about the profile *program*, and the frame's placement
is an *input to* that pass — the one input still re-derived from the
document rather than handed in.

**The item was stale and the lane checked rather than inherited.** It
claimed `slots.rs`'s header "cannot say 'once per node per environment'
while this holds"; the header says no such thing and already documented
the duplicate accurately. What the change buys is that it can now say
the stronger thing, and does.

**Residue filed**:
`section-of-re-derives-the-whole-f64-precompute-the-profile-node-already-made`
— the same shape one level up and far more expensive (an entire program
resolve-replay-validate per section, against nine slots per profile),
and it reaches **PP1/PP2's structure record**, so it is its own unit and
should not assume this one's answer. Two neighbours the same sweep found
are dispositioned on that row so the next sweep does not re-derive them.

Full review dispatched.

## Disk, second time today

The box hit 100% again (189 MB free) while the lane worked; the lane
reclaimed its own target and reported the rest rather than touching
them, which is correct — a lane cannot judge whether a sibling is live.
Orchestrator swept the finished lift-pair and review lanes:
**12 GB used, 26 GB free.** `pgrep cargo` empty before deleting.

The standing rule from `memories/agent-lane-operations.md` is that this
is done **when a review returns**, and both times today it was done when
a lane ran out of space instead. Worth doing at each seam rather than at
each crisis.

## PR 2435's review, and a process gap it exposed in the orchestrator (2026-09-12)

**APPROVE-WITH-FIXES, 0 MAJOR / 2 MINOR / 3 NOTE**, ten style findings,
all eight style questions exercised. **All three load-bearing claims
survived**, and the reviewer went past argument on each: it established
that `op_env.lane.nominal` is *literally the same `&ParamEnv<f64>`* for
a frame and every profile drawn on it — not merely an equal one — and it
mutated `eval_node` to serve a prior's placement, finding that **row 2
is the only row in the whole target that catches it**. The memo guard
the unit wrote is the only thing standing between this change and a
silent stale placement.

**The two MINORs are one seam, and the fix pass treats them as one.**
The carry is `Option<SketchPlane<f64>>` where `None` means *derived* —
an overloaded encoding that fails in both directions:

- **S2**: the recipe-kind fork sits behind a catch-all `else`, so a
  future `Datum` variant producing `DatumValue::Frame` would pass the
  payload door, carry `None`, and **silently place profiles at the
  lane** — where the deleted `frame_kind` refused loudly. A fail-loud
  regression.
- **MINOR 2**: because `None` is spoken for, a frame whose **nominal**
  axes refuse has nowhere to go but failing the node, so `AxisInPlane`,
  the mate solve, measures and the viewer all become
  `Poisoned { through: frame }` though none wanted the nominal
  placement. `likely`, reachable in the E6 subdivision, no fixture
  built.

Directed: determine reachability, and replace the `Option` with an
explicit exhaustively-matched carry **regardless**, since S2 stands
alone. If MINOR 2 is reachable, that type is also where a nominal
refusal goes so it is raised at the profile that needed it.

**MINOR 1 is a lesson about how a false absolute survives a check.**
*"No decision is added anywhere"* is false for a frame no profile is
drawn on: old `2 + 2N`, new `4`, so a **cost** at N = 0 (measured, 4
against main's 2). It survived because the unit checked the one-profile
case, where `2+75 = 4+73` — **exactly the case where a wrong absolute
still balances.**

**NOTE 3**: the PR called a breaking API change additive. `NodeValue`
has no `#[non_exhaustive]`, and the proof is inside the PR itself —
`m4_pr4_resolve.rs` had to change because a new public field breaks
out-of-crate struct literals.

**S4 is the sharpest test finding this program has had.** A mutant
placing with the raw authored `v` instead of the Gram-Schmidt residual
**survives the entire `editor-core` test target**, because
`fixture::plane_of` *asserts* its fixture's `u`/`v` are unit and
perpendicular — so orthonormalization is the identity on every row, and
the unit's bit-equality row only proves nine literals were copied. The
reviewer wrote and ran the closing row; it is being adopted with
authorship kept.

That is now **five of eight merged-or-merging units whose central test
claim was corrected by an instrument rather than a reading** — mutation
twice, a built fixture, the ε matrix, and now a fixture's own asserted
precondition hiding the thing under test.

**Filed**: `the-is-this-a-frame-door-was-deleted-and-its-classification-dispersed`
— verified against `origin/main` before filing, because it is an
accusation about a deletion, and `frame_kind`'s doc there says exactly
what the reviewer quoted. A class with **seven more sites** over other
kinds; S7 is recorded on it rather than given a row of its own.

### The process gap: filed rows are invisible to lanes until the orchestrator branch merges

The review flagged the `section_of` hit as **unscheduled**. It is not —
`section-of-re-derives-the-whole-f64-precompute-the-profile-node-already-made`
was filed before the review was dispatched. The reviewer could not see
it because it lives on `wire/orchestrator`, which has not reached `main`
since PR 2386.

That is the orchestrator's gap, not the reviewer's, and it has a real
cost: **a lane reporting a finding cannot tell whether the orchestrator
already filed it**, which is the exact confusion
`docs/prompts/implementer-discipline.md` §6 says the orchestrator exists
to prevent — *"you cannot tell whether the item already exists… the
orchestrator could."* That only holds if the orchestrator's filings are
visible. **Merge the orchestrator branch at every seam from here**, not
when it happens to be convenient.

## PR 2435's fix pass turned a `likely` into a measurement (2026-09-12)

CI run `34688987262` green, 39 jobs. Every review finding held up and
the lane pushed back on exactly one thing, correctly.

**MINOR 2 is reachable, and the reviewer's `likely` understated it.**
The review reasoned it reachable "in the E6 subdivision" and did not
build the fixture. The lane built it and found **no interval is needed
at all**: `BoxAxis::Varying` need not contain zero, and the env binds
`nominal + offset`, so nominal and lane are two different points **at
`f64`** under a degenerate `ParamBox`. Fixture `u = (p,0,0)` with `p`
nominal 0 under `p ∈ nominal + [1,1]`; row 7 reddens when the mint's
refusal arm is made to fail the node. A finding rated `likely` on a
reading came back `sure` on a fixture, and cheaper than predicted.

**The carry shape is better than what was asked for.**
`Option<FramePlacement>` — `Authored(SketchPlane<f64>)`,
`Unreadable { role, error }`, `Derived` — with **`None` meaning "not a
frame" and only that**, turned into the loud `WrongOperand` the deleted
`frame_kind` door used to raise. The mint forks on an **exhaustive match
over `Datum`**, so a new variant is a compile error rather than a silent
lane placement. `Unreadable` is where a nominal refusal lives, raised
**at the profile that needed it**: decision logged where made, refusal
raised where needed.

**The lane caught itself minting a fresh instance of the class,
mid-pass.** Its first cut of the re-shape spelled the frame read a
second time inside the mint (`need_point3` + `frame_axes` instead of
`frame_from_slots`); `FrameRead` exists so one door serves both
dispositions. Found by its own adversarial diff re-read, which is the
standing lesson working rather than being quoted — the first time on
this program that the trap was caught by the lane that would have sprung
it.

**NOTE 3 answered by measurement, and the answer is "no".** The lane
applied `#[non_exhaustive]` and built: `m4_pr4_resolve.rs` fails
`E0639`. An integration test is out-of-crate, so the attribute does not
make field additions non-breaking there — **it makes the struct
permanently unconstructible there**, and paying for that needs a public
constructor whose only caller is a test. Accepted. The right change is a
door for building a stub `Evaluation`, with its own argument, not a
rider here.

**Two things neither review named, and CI found both.** A new public
type had to answer to two façade censuses: `pncad`'s
`every_document_layer_root_export_is_carried_or_listed` and `pncad-py`'s
`test_every_curated_name_is_bound_or_listed`. Both caught it before a
human could. A new fence crossing was disclosed as a result
(`crates/pncad-py/tests/test_binding_census.py`). Worth recording that
the censuses are load-bearing for exactly the thing a review is worst
at: noticing that a new public name exists at all.

**The lane's one pushback, accepted.** The orchestrator's framing of S2
("a future `Datum` variant producing `DatumValue::Frame`") understates
it: the same hole was open for a future **non-datum** node producing a
frame payload, and it is the `None`-is-loud rule that closes both — the
exhaustive match over `Datum` closes only one.

**Delta review dispatched**, deliberately narrow: a new public type, a
re-shaped carry and moved refusal routing is more than a fix pass, and
the earlier full pass covered everything that did not change. The brief
says plainly that "nothing further" is a complete answer.

## The delta round: MERGEABLE, and it answered the one question a fix pass cannot answer about itself (2026-09-12)

**0 MAJOR, 2 MINOR, 6 NOTE.** Every load-bearing claim held under
*independent test* rather than re-reading: a scratch `Datum` variant
really does fail `E0004` at the mint; `v.placement` has exactly one
reader outside tests, so the mate solve, `axis_frame`, `stackup`,
`topo::query`, the viewer and `pncad-py` all still read the untouched
payload; `refusal()` is a pure match with no decide on the carry road;
`different-shape` is honest because `ParamBox` is
`#[cfg(feature = "interval")]` and the wheel is default-feature; and the
`#[non_exhaustive]` measurement reproduced exactly.

**The narrow round earned itself on item 6.** The lane had caught itself
minting a fresh instance mid-pass and disclosed it — which was the right
thing and is not what this round found. The delta found **a second one
the lane did not catch**: `wire.rs:1070` hand-writes
`DirectionRefusal::node_error()`'s body, spelling `DATUM_UNIT_NORM` a
second time 218 lines from the door — **and `node_error` was minted by
that same diff to be the one spelling of that map.**

The cause is structural and the style lane named it:
`FramePlacement::Unreadable { role, error }` flattens a
`DirectionRefusal` into two loose fields, so the carry road **cannot
call the door it was given**. Seventh instance of the standing trap on
this program, and the first found only because a third pass ran. Fix
directed: hold the `DirectionRefusal`, which closes the MINOR and the
style finding together.

**MINOR 2 is the test finding, and it is the same shape as the last
unit's.** A mutant hard-coding `role: "datum frame y axis"` survives
**20/20 rows** — so a frame whose `u` is degenerate would tell the user
its `y` axis is, undetected. `role` is the entire user-facing content of
that refusal. The `error` half is pinned; the `role` half is pinned by
nothing, because row 7's `DegenerateDirection { .. }` is a wildcard.
The reviewer wrote row 8, which degenerates the Gram-Schmidt *residual*
with `u` fine so the two rows disagree about `role` — adopted with
authorship kept.

**Six of nine units now have had their central test claim corrected by
an instrument**, and the instruments have been: mutation (×3), a built
fixture, the ε matrix, and a fixture's own asserted precondition. Not
one came from reading a diff.

**Two false sentences this diff wrote**, both going back: `NoPlane`'s and
`Unreadable`'s *"span no plane"* is false for three of the carried
error's four arms — `UnderflowedLength` is documented as *"a vector that
has a perfectly good direction"* and `Escalated` is reachable at `f64`;
and the defence *"An `Err` would have forced one of them"* is untrue,
since `match` and `map_err(node_error)?` are both available and are what
the two call sites already do. The unusual length of that defence was
the tell, per the brief's stance bullet.

**NOTE 4 is a real gap in the earlier measurement.** The
`#[non_exhaustive]` question was asked of `NodeValue`, where **field**
additions are the growth surface and the answer is correctly no — and
never asked of the **new public enum**, where **arm** additions are, and
where it costs nothing (applied, `cargo check` clean, precedent
`topo::ContactClass`). Right answer to the wrong subject is its own
failure mode.

**NOTE 6 catches a claim that cannot go red where it sits.**
`FRAME_LOG`'s new sentence carries the N=0 disclosure, and the only
assertion is over a one-profile document — where a lookahead mint, the
design the comment rejects by name, produces the same constant. Q6:
a guard, a register, or a written "unguardable and why".

**Filed**: `guided-lift-refuses-a-nominal-degeneracy-it-never-reads`
(NOTE 3 — measured: under Guided the profile never reads the nominal
yet a nominal degeneracy still refuses it, while a *derived* frame in
that position builds; refusing may be right and nothing states which is
intended). The door class is updated with the delta's sharper finding:
the three `WrongOperand` sites have **stopped being one door** — two
test the payload, one tests `placement` — which is worse than when that
row opened, because a copy testing a *different thing* cannot be unified
by a rename. NOTE 5's convention-not-type point is recorded there too.

## 2026-09-12 — `frame-f64-placement-is-re-evaluated-per-profile` merged (PR 2435, `ceed1fc`)

Verified at the check-run level before merging: twelve `test (…)` jobs
({default, interval} × {default, 1e-6, 1e-12} × 2 shards), five
`k-lint (gate, …)` unifications, `gate ok` — zero failures, zero
in-flight. The skipped names are the usual optional lanes (interval
oracle, corrupt input, step import (freecad), cache priming), which is
what a code-tier run at this filter looks like; no green name sits over
a skipped step in the gated set.

**Two residues recorded rather than grown into the diff.**

**NOTE 1 is filed** as
`frame-direction-refusal-lands-on-the-profile-without-naming-the-frame`.
The fix pass was right to scope it out: the arm is one line in
`wire.rs`, but the variant behind it costs a `NodeErrorKind` arm, its
`Display`, two exhaustive matches in `crates/pncad-py/src/tags.rs` and
a roster string in that crate's census — another crate's error
vocabulary, arriving after the delta verdict. The row carries the
`DerivedFrameSection { profile, frame }` precedent and the open
question of whether to add a new variant or an id field to all four
direction variants.

**The public-API side effect is adjudicated, not filed.** The lane
flagged that fixing MINOR 1 made `DirectionRefusal` public
(`wire.rs:845`, re-exported at `editor-core/src/lib.rs:118` and
`pncad/src/document.rs:195`) as a side effect of a duplication fix, and
asked for it to be seen as a surface decision rather than a code move.
It is one, and it is the right one: `FramePlacement` is public by
necessity — it rides on `NodeResult`, which `pncad` re-exports — and
`Unreadable` has to carry something. Inlining `{ role, error }` into
the arm would expose the same two fields with no type to hang the
invariant on, and `node_error`'s "**the one spelling**" doc is written
on the type. Same surface area, one more name, and the name is where
the invariant lives. No ratified clause governs `editor-core`'s export
list and no census gates it, so there was nothing for Ev here.

**The instrument tally closed at six of nine**, and every one of the
six was an instrument rather than a reading: mutation (×3), a built
fixture, the ε matrix, and a fixture's own asserted precondition
hiding the thing under test. That is the guidance for the rest of the
program — ask what instrument answers the claim, not whether the claim
is argued well.

## 2026-09-12 — PR 2442 full review (product gather, tie across halves)

**The review closed a gap the implementer had declared rather than
papered over**, and the answer is operationally useful: the lane's 403
on the hosted log blobs was **lane-local**, not a property of the
proxy. `mcp__github__get_job_logs` reaches a run's job logs directly,
and both new rows are named by name in run `34694794356` on `d924863`
(`two_roots_aliasing_a_strict_name_still_refuse` at line 951 of `test
(eps = default, 1/2)`; `a_split_separating_a_tie_gathers…` at line 947
of shard 2/2 and line 847 of the interval default shard), with shard
totals passing. The implementer's `every_suite_file_is_aggregated`
chain was sound, but it is no longer the evidence of record. **Reach
for `get_job_logs` before concluding a lane cannot see its own rows
run.**

**All four of the implementer's mutants reproduced exactly** when
re-taken privately (M4's truncated "≥ 16" resolves to 29), and four
more were added. The one that matters, M1 — the bug re-minted inside
the new door, narrowing by surviving count instead of the source tie
bit — reddens **exactly one row of 1219**, the one the PR added. That
is the shape a guard should have.

**Two claims came back stronger than posed.** Tied candidate ORDER is
moot, not merely unchanged: `insert_tied_ref` sorts and dedups before
storing (`table.rs:287-289`), so `Entry::Tied` is canonical in
`EntityRef` order whatever the accumulation order was — measured by
reversing the accumulated order (0 red). And the name key is
untouched: `NameTable::iter` is literally
`iter_refs().map(|(n, e)| (&**n, e))` (`table.rs:377-379`), so the
population and order are the same iterator; minting a fresh `NameRef`
as the old door did reddens nothing (0 red of 1219).

**The unreachability argument had a missing link, and the review
supplied and measured it.** The doc said two sources agreeing on a name
"share that node's strict pass-through rows too" without saying why one
must exist. The reason: two roots can only share a name if both reach
it verbatim, and only `Transform` and split-intact pass-through leave a
name unwrapped (`emit_topo.rs:1380-1384`, `:385`) — every other emitter
wraps — and **a plane never subdivides a vertex**, so every vertex of
the common ancestor passes verbatim into both tables with its strict
name. Probed: `transform(subtract)` + `split(subtract, y = 1.25)` gives
`both_unique=59, mixed=2` and the gather refuses on a strict collision
first; an oblique plane gives `both_unique=53 {Edge: 26, Face: 3,
Vertex: 24}, mixed=1`. The `Unique`/`Tied` disagreement the arm needs
**does exist** — it is simply always beaten to the refusal.

**One blocker, and it is the recurring kind**: the field's doc-comment
was updated to disclose `node`'s new dual meaning and the sentence that
RENDERS it was not (`product.rs:237-243`), so the flush path prints
"root 6's face name (minted by node 6)" — a non-root called a root and
named twice. A disclosure written where no reader of the output will
see it.

**The rule as implemented is wider than the rule as approved** (S9):
nothing restricts the merge to one root's output bodies, so two roots
carrying the same tied name would merge. Adjudicated as the SAME rule
at its right generality rather than a new one — `TieRows`'s own doc
already says equally-admissible candidates stay equally admissible
downstream, and the split case is a special case of that — and the
difference is unreachable for exactly the reason above. Not Ev's to
rule, but the PR must say it rather than leave the code implementing a
rule wider than its prose. Raised with Ev in chat as a generalisation
he may want to narrow.

**The class-not-instance rule bit the filed row.** The PR filed
`name-placed-union-spells-the-narrowing-rule-itself` — accurate, and
one instance. The 1-vs-many decision is written six more times in
`emit_topo.rs` in a form the PR's `insert_tied` sweep structurally
could not match (`if X.len() == 1 { put } else { tie.push }` at `:601`,
`:726-733`, `:952`, `:1009`, `:1236`, `:1400`), which is precisely the
blind spot the PR disclosed. `rg 'if .*\.len\(\) == 1'` finds them in
seconds — **the sweep pattern is the finding**, and the row is being
widened into the class with it recorded.

**Filed**: `product-gate-says-verbatim-then-states-the-difference` —
the review's S11 sharpened. `product.rs:664-668` calls the per-source
gate "the import loop's rule, verbatim" and the next sentence explains
how it differs; the import loop counts INSTANCES
(`step-import/src/lib.rs:700`), the product counts SOLIDS. A
self-declared copy is a claim no test can read.

**Held rather than changed**: the seam (`names::defer` now knows the
gather's body model) — the door is NAMED the gather's carry, so knowing
its shape is coherent, and the alternative puts the filter back in
`product.rs` where the third copy lived; and `TieRows`/`CarriedRows`
coexisting with divergent ownership, which is meaningful (`flush` runs
at every stage boundary, `finish` once). Both get a sentence saying so,
so the next reader does not re-open them.

## 2026-09-12 — 2442 merged (`9a208a3`), and a fifth face of the silent-coverage class

The fix pass took all six items. The blocker's fix is the shape worth
copying: `ProductError`'s `Display` is now two arms guarded on
`*node != name.node`, and the guard is sound in BOTH directions — on the
flush path `node` IS `name.node`, so the per-root sentence is
unreachable there, and the tie-merge sentence claims only what is true
wherever it is reached. The new row
`the_naming_refusal_claims_rootedness_only_on_the_per_root_path`
reddens on the exact defect: with the guard removed it fails printing
the reviewer's own quoted sentence. A refusal-text row that reproduces
the reviewer's quote is the strongest form of that guard.

`product.rs:38` was adjudicated by the lane as standing in meaning and
sharpened in wording — the claim sits in a provenance paragraph and
means the gather writes no table of the EVALUATION's, which was true
before and after since `carry_names` always built a separate aggregate.
Right call; the ambiguity was real and the fix was to remove the second
reading, not the sentence.

**The render-drift neutrals were not this PR's, and chasing them found
a new hole.** `gate ok` is green over three `render drift (…)` neutral
CHECK RUNS which it cannot cover by construction (`ci.yml:5199-5204`:
it reads the jobs API, so check runs are *"outside the one name
entirely"*). Their cause: main's run for the teapot merge `f2a4adf`
(13:08:21) was **cancelled** 51 s in by the next merge, `593ed19`
(13:08:57) — `ci.yml:133-135` has `cancel-in-progress: true` on a group
keyed only on workflow+ref, which on `main` every push shares. The
teapot's re-baseline never ran; #2441 was docs-tier so its render lanes
skipped; the drift surfaced on the NEXT PR's checks as if it were that
PR's. It self-healed only because 2442 happened to be code-tier.

**The window is merge spacing, and this orchestrator opened it** — two
merges 36 seconds apart. Filed as
`work/issues/a-merge-cancels-the-previous-merges-main-run-and-its-main-only-work.md`
with the timing table and three candidate fixes. The general form is
the part to carry: **`cancel-in-progress` is a claim that the older
run's work is worthless, and that claim is false for any branch whose
runs write back.**

Operational consequence adopted now, ahead of any fix: **space merges
to main past the previous merge's run, or check that the previous
merge's main run was not cancelled.** A `cancelled` main run is not a
neutral event.

## 2026-09-12 — PR 2445 full review (D364, the target tag and census)

**The round's keeper, from the lane rather than the reviewer**:
`cargo clippy --workspace --all-targets` **compiled**
`every_profile_layer_root_export_is_carried_or_listed` and said
nothing; the facade guard only spoke when hosted CI ran it. A build is
not a test. That is the same silent-coverage family as the rest — a
step green having EXECUTED nothing — arriving through the local
pre-push check every lane runs.

**The shape argument was confirmed by instrument, not accepted.** The
lane argued a macro over a hand-written trio because a hand-written
`ALL` is forced by nothing, so a census over a short `ALL` reports
green over the hole it exists to find. The reviewer hand-shortened
`TargetKind::ALL` to 2 of 3 inside the macro body: **all five switch
rows green**; with the matching pin dropped too, **6 of 6 green**. The
macro is not consistency with `arc_modes!` — it is the only thing
holding the census up.

**The lane's own caveat was retracted in its favour.** It reported M1's
red as compile-time and flagged that this says nothing about which
other rows stay green. The reviewer discharged the compile error in
place (`TargetKind::CurvePose => ProgramTarget::Start`, well-typed and
what a lazy implementer writes) and got a **runtime** red — *"the
document target for CurvePose resolved to Start"* — plus a second in
the lib test on the content-key tag. Compile-time is the first of three
failure modes. It is a census, not a type check, and the other four
rows are correctly blind.

**The old census's hole was reproduced independently**: main's tree +
`CurvePose`, every break discharged the lazy way (8 arms in `profile`,
3 in `editor-core`'s lib, 4 in tests) — **6 of 6 green**, with a kernel
target form no document program can express, silently lifted to
`ProgramTarget::Start`.

**A framing both the lane and this orchestrator had wrong.** The corpus
clause's `Step`-match widening is a **loosening**, not a strengthening:
the assertion is `missing.is_empty()` over `ALL`, so adding sources to
`seen` can only shrink `missing`. Narrowing it back leaves the census
green — no form became "seen" because of it. The widening buys
ACCURACY; the exhaustiveness is the tightening. Worth carrying: "the
match got bigger" and "the test got stronger" are independent, and the
direction is decided by which side of the assertion the change lands
on.

**The sweep contained its own third instance.** The lane's pattern
returned 55 hits; **nine were `crates/viewer`** and the disposition
named none of them. `viewer::sketch::PathTarget` is a third short
spelling of the target vocabulary, lowered by an exhaustive match over
its own enum, so the GUI cannot author the declared tangent arrival
either. **This orchestrator then filed the pncad-py instance alone** —
the same half-fix, made while holding the class-not-instance rule.
Replaced by `work/lib/both-authoring-surfaces-are-short-of-the-target-vocabulary.md`,
which carries all six spellings and marks the three that are short and
forced by nothing. The lesson is not "sweep harder": it is that a
disposition listing five of 55 hits is where the dropped instance
hides, and the count of hits against the count of dispositions is the
check.

**Filed**: the class above;
`res-target-slot-roles-are-unguarded-and-duplicate-spec-slots` (the
reviewer dropped the `Target2*` twins and **all 1324 editor-core tests
passed** — the roles are unguarded and `spec_slots` is a second
untested spelling of the same assignment); and
`the-third-tag-vocabulary-macro-owes-a-unification-trigger` (a named
trigger with no schedule, where the third instance is arguably already
in the file).

## 2026-09-12 — D364 merged (`b3eaef1`), and the cancelled-run hole fired again

The body-only fix pass took both corrections; head unchanged at
`d58fec78`, run `34698904483` still the only run on it and still
green — twelve `test (…)`, five `k-lint (gate, …)`, `gate ok`, nothing
in flight. **A body edit triggers no run**, which is why this round
cost nothing and why the lane's target was reclaimed before it started.

**The cancelled-main-run hole fired a second time, 66 minutes after the
first, and the second instance breaks the mitigation this log adopted
for it.** 2442's own main run (`34698293875`) was cancelled 89 s in by
**VIEW's** #2444 merge — a different program. The first instance was
self-inflicted and suggested "space your own merges"; that cannot work
when the cancelling merge comes from outside, and on this repo the
merge stream is the union of every program's seams. Two instances in 66
minutes by two authors is the rate, not a race.

It also corrects an attribution made here an hour ago: `c13aa67`'s
re-baseline was recorded as landing off 2442's main run. **2442's run
was cancelled**; `c13aa67` came from 2444's, which happened to be
code-tier and swept up the drift. The "self-healing" was luck twice
over — the next merge being code-tier, and that merge's own run
surviving. Recorded on the row, with a fourth fix option that survives
a cancellation from outside: make the write-back owed by STATE rather
than by event, which is what 2444's run in fact did by accident.

The general form stands and is worth keeping in front of the next
orchestrator: **a mitigation that depends on one agent's own pacing is
no mitigation on a tree where every program merges its own work.**

## 2026-09-12 — the axis channel cut, and S195 dispatched as a measurement

**S195 was not dispatched as written, because three of its four claims
look stale.** Filed 2026-08-20, it says the arc-mode vocabulary has no
census, no `ArcData::ALL` to anchor one, and — the sharp part — that a
seventh mode's slot role would address nothing silently because
*"nothing forces the corpus to grow when a mode is added."* Against
today's tree: `ArcMode::ALL` exists (`arc_modes!`), a mode census exists
(`switch_program_vocabulary.rs:404`, `:456`), and **the corpus is
generated from `ArcMode::ALL`** (`:175`, `:188-199`, fused blocks
included) — precisely the thing the row says does not happen. What still
looks live is `spec_arg_access!`'s `_ => None` (`program.rs:593`) and
two unanchored discrete pairs.

So the lane's job is to **run the row's own thought experiment**: add a
seventh mode and watch which guard fires.
`every_enumerated_slot_addresses_a_distinct_expression` has two
independent ones — a `slots.len() == expressions` count and a
`panic!("is enumerated but addresses nothing")` — and which one trips
depends on whether `spec_slots` is compiler-forced. Nobody has run it;
the row is an argument about what a census would catch, and this program
has learned that arguments of that shape are settled by instruments.
**A PR that closes a row with proof and files the residue is the
expected outcome and a complete unit** — the brief says so, so the lane
does not manufacture a diff to justify itself.

**The axis channel is cut, and the headline is that WIRE cannot start
it.** Steps 1–3 are TOPO's and EXCH's; WIRE's own parts (the
`crates/verbs/` clause, the vocabulary, the P3 rows) are downstream of
all three. Row moved `open` → `parked`, `blocked_on` the TOPO row.

The spine that made the cut fall out: **this channel is `ParamSource`'s
three-part shape at `GeomSource`'s granularity**, and
`crates/verbs/README.md`'s own file table (`:35-37`) already says which
crate owns each part — P1 the lowered token, P2 attach/propagate/
consume, P3 absence refuses. Given that table the ordering is forced,
including the one ordering constraint that is not obvious: **the
document-level declaration node lands LAST**, because a declaration is
persisted document content and a shape change after shipping is a file
migration rather than a refactor.

Two things written into the cut so a taker cannot lose them:

- **Round 3's correction, which reverses Round 2's pricing.** The axis
  channel sits on `GeomSource`'s side of §3 P1's line, not
  `ParamSource`'s, because P1's exclusion is scoped to *motion-invariant*
  fields and an axis is not one. P1 stands untouched. An earlier reading
  (this orchestrator's) had it backwards.
- **The staleness table's row-three under-claim is deliberate.** It
  refuses when two different chains compose to the same relative motion
  though coaxiality survives. A taker who "fixes" that by comparing
  composed motions numerically has replaced a token comparison with a
  measurement and broken ruling 1.

**Cross-references written onto steps 1 and 2** (`work/topo/…-four-origins`,
`work/exch/step-import-discards…`) naming them as steps of a ratified
sequence, with the reverse dependency recorded on step 1: whoever
designs the origin representation should look at what `import_step` can
supply, since step 2 has to write one of the four origins with real
content. A ratified design whose first mover is another program is
exactly what gets lost when the ratifying program exits; the
cross-references are the cheap insurance.

## 2026-09-12 — S195 measured: the row was three-quarters discharged, and the fourth claim was wrong

The lane ran the row's own thought experiment and the answer is clean.
**21 sites compiler-forced by a seventh arc mode, one not** —
`spec_arg_access!`, still `_ => None` — all found as build failures
rather than by grep, which is the right instrument for "what does the
compiler make you touch".

Which guard fires is a **laziness gradient, not a coverage gap**:

- the row's exact scenario (every forced site discharged, access arms
  omitted) trips the **`panic!`**, naming role and step — `Profile
  { loop_: 0, step: 17, arg: HeightVal } is enumerated but addresses
  nothing` — and it was the ONLY red in a 1224-test binary;
- one notch lazier (a `spec_slots` arm enumerating nothing) trips the
  **count** clause instead, which said only *"the program has 133
  expressions and enumerates 118 slots"* — naming neither mode nor
  step. That is the gap, and the diff is 58 test lines closing it on
  the failure path.

Verdict on the row: three claims **stale** (`ArcMode::ALL` exists, four
censuses exist, the corpus generates from `ALL`), and the fourth —
"addresses nothing **silently**" — **wrong**, caught by name.

**The sharper finding, which the row and this orchestrator's brief both
missed**: `mode_witness` is a **compile error, not an assertion**. The
mode census's teeth are in rustc, and the only assertion-level catch is
the bijection clause. Both the row and the brief talk about the census
as a runtime thing; it is mostly a type check with one runtime clause
behind it. Under review as the unit's real result.

**The brief was corrected on its own premise**, which is worth
recording against the orchestrator rather than the lane: it asked which
guard fires "depending on whether `spec_slots` is forced". `spec_slots`
IS forced (exhaustive `match (spec, second)`), so that was never the
variable — the variable is how an implementer fills the arm rustc
demands.

**`ArcSide`/`ArcSweep` need nothing, measured rather than assumed.**
Both directions are compiler-forced (`from_side`/`into_side`,
`from_sweep`/`into_sweep`), and both enums are binary **by geometry** —
a half-plane bit and a travel sense — so there is no third variant to
add. Nothing filed; the row now says so.

**A fifth mode-keyed roster nobody in this row's lineage mentions**:
`crates/pncad-py/src/surface_census.rs`. Noted for the class row.

**Filed**: `wire-roundtrip-census-localises-nothing` — the wire
round-trip clause fires correctly on a laundered vocabulary member and
then prints two whole-corpus `Debug` dumps. Same shape as the count
clause, different failure, correctly scoped out rather than swept in.

**The classification cost, recorded as process.** The **H** class was
set at the 2026-09-11 cut by reading the row's prose, when three of its
four claims were already discharged — one of them by PR 2445 the day
before dispatch. A five-minute read of `switch_program_vocabulary.rs`
against the row would have re-dispatched it as a measurement a day
earlier. **Second row today overtaken by adjacent work** (D364's census
already existed too), which makes it a pattern rather than an accident:
**read the row against the tree before dispatching it, not against its
own prose.** Review posture lowered from full to light in `plan.md`
with the reason recorded — the census it was going to build already
exists, so nothing here can report green over a hole.

## 2026-09-12 — three wrong datings, one root cause: the orchestrator's clone is shallow

S195's classification finding was corrected three times, each time by
someone other than its author, and the last correction found the cause
of all three.

1. The lane's row cited PR 2445 as discharging one of three stale
   claims, indicting the 2026-09-11 cut.
2. The review caught that 2445 merged **2026-09-12**, the day AFTER the
   cut, so it cannot be evidence against it — and this orchestrator had
   repeated that error to Ev and in this log.
3. The lane, re-checking commits rather than restating, found 2445
   discharged **none** of the three: two commits on **2026-09-01**
   (`70aaee60d`, `592685539`) discharged all three, ten days before the
   cut. 2445 closed the `ProgramTarget`/`WireTarget` pair, a different
   part of the row.

Verified here against full history: both SHAs resolve with those exact
dates and subjects, and D364's target census landed `7de944e91`,
**2026-09-02**, nine days before the cut — not 2026-09-10 as this log
previously recorded.

**The cause of every one of the orchestrator's dating errors: this
session's checkout is a SHALLOW clone whose history begins
2026-09-09.** `git log -S … --reverse` in it bottoms out at the
truncation and names the root commit — `7902971`, a
`render(uv): re-baseline` with no parent and 3520 files — as where a
change "first appears". That produced the false 2026-09-10 date, and it
is why the lane's two real SHAs came back `Not a valid object name`
when checked here, which read as the lane having invented them.

**A lane's dating is more trustworthy than the orchestrator's**, and
that is the reverse of the usual direction. `new-lane.sh` does a plain
`git clone` and gets full history; the session checkout does not.

Fixed for this session with `git fetch --unshallow origin main`.
`git rev-parse --is-shallow-repository` answers the question in one
call. **Any "X landed on date D" claim made from a session checkout
without one of those two is unsound**, and this program made three.

The corrected reading makes the case against the cut **stronger**: all
three claims stale ten days before, not two. Recorded on
`work/meta/the-2026-09-11-cuts-class-estimates-are-untested-and-both-tested-ones-were-wrong.md`
with the root cause, because the next orchestrator to check a class
against the tree will be reading from the same shallow clone.

## 2026-09-12 — S195 closed (`1b1146f`), and the cancellation hazard measured properly

The fix pass took both corrections and found a third I had not: my own
correction of the lane's dating was itself wrong. See the entry above;
the root cause was this session's shallow clone.

New title, which is what the board now shows: *"What the arc-mode
vocabulary's four guards catch, and the one site a seventh mode reaches
unforced."* The old one asserted "it has no census at all", which the
body calls stale by a factor of four — a row going to review under a
headline it refutes is the one line most people read.

**The lane owned the report error squarely**, and its diagnosis is
worth keeping: it collapsed the mode census to its compile-time limb
"because that is the limb my mutant exercised — a seventh MODE trips
existence, so laundering never came up in what I ran, and I generalised
from one mutant to the whole clause." **One mutant answers one
question**; a claim about a clause needs a mutant aimed at each limb.

### The cancellation hazard, measured rather than inferred

Holding 2447 to avoid cancelling PERF's in-flight main run produced the
dataset the two earlier instances only suggested. In thirteen minutes,
**five merges by four programs; three of the five runs cancelled**, one
after twenty-six seconds. The run I deferred to **succeeded** — it was
short enough not to need protecting — while three others died during
the wait, to other programs' merges.

So: there is no clear window to wait for, the forbearance protected
nothing, and voluntary spacing is a **unilateral tax** rather than a
mitigation — it only works if every program does it, and no program can
see another's merge coming. That kills the interim behaviour the first
two fix options were ranked cheap *because of*. The fourth option —
make the write-back owed by STATE, so the next main run re-baselines
whatever differs whoever caused it — is now the recommendation, because
it is the only one correct under a cancellation from any cause, and at
a 60% cancellation rate "any cause" is the common case.

Recorded on the row by the orchestrator whose own merge was the one
held back, so the incentive runs against the conclusion. **This is the
strongest form of evidence this program produced today**: not a
finding read off a diff, but a mitigation tried, measured, and reported
failed by the party it cost.
