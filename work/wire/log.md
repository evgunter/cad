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

## 2026-09-12 — orchestrator handover, and the opening block of the second half

New orchestrator on WIRE. Ev reaffirmed the posture in chat at the
handover, in the same words `plan.md` already carries from 2026-09-11:
**light style review on every unit, full review only on the units with a
real risk of being wrong, and no A/B protocol here.** Nothing changes.

### The first act was closing a gap the previous entry had named

`wire/orchestrator` was **sixteen commits ahead of `main` and had not
reached it since PR 2386** — including nine newly filed rows. The entry
above names that as this program's own process gap and the cost it
carries: *"a lane reporting a finding cannot tell whether the
orchestrator already filed it"*, which is exactly the confusion
`docs/prompts/implementer-discipline.md` §6 says the orchestrator exists
to prevent, and it had already made one review flag an already-filed row
as unscheduled. Merged as PR 2470 (`e25946743`), docs tier, `gate ok`
green with nothing in flight. One conflict, on `S195.md`: `main` carried
the fix pass's new title with `status: review`, the orchestrator branch
carried the close under the old title. Resolved to the union — the new
title, which the body supports, and the closed state.

**The rule stands and is now this orchestrator's too: merge the
orchestrator branch at every seam.** Three lanes went out within the
hour, and they can see every row filed to date because of it.

### The regrouping, which is the one real judgement call at the handover

Four open rows are the **same ten lines of `eval/wire.rs` seen from four
angles**: the duplicated frame door
(`frame-plane-lane-and-axis-frame-are-one-door`), the deleted
classification (`the-is-this-a-frame-door-was-deleted-…`), the
hand-copied composed phrases
(`composed-expected-phrases-are-hand-copied-across-sites`), and the
refusals whose `found:` is the negation of their `expected:`
(`wire-refusals-answer-found-with-a-negation-of-expected`). Each item's
own body says a taker of one must read the others; none says they are
one unit.

They are dispatched as **one unit with a full review**, against
`plan.md`'s class column, which rates three of the four **E**. The
reason is this program's own record: the trap where **a unit closing a
duplication mints a fresh instance of it** has fired **six** times here,
most recently when PR 2435 deleted `frame_kind` — the door whose doc said
*"is this a frame" is answered once with one refusal vocabulary* — and
inlined the classification at three sites. Staffing four views of one
door apart mints it three more times. The class column is corrected in
`plan.md` rather than left to disagree with the dispatch.

The unit also carries `wire-rs-module-header-describes-five-sixths-of-the-file`
as a separate cheap deliverable, because it is a sentence in the same
file and a second PR against `wire.rs` buys nothing but a merge conflict
and a cancelled CI run.

### Three lanes out

| lane | rows | review |
| --- | --- | --- |
| `wire/operand-door` | the four above + the module header | **full** |
| `wire/placement-prose` | `placement-rs-states-its-exactness-rule-in-nine-paragraphs`; `frame-linear-generic-door-has-no-consumers` as a **measurement only** | light |
| `wire/names-vocab` | `interrogate-writes-the-family-vocabulary-a-third-time`, `emit-topo-destroys-the-edge-key-in-a-split-lineage-cycle`, and WIRE's `paths` | light |

**`frame-linear-generic-door-has-no-consumers` is deliberately not a
removal.** The row is a CLASS whose real question is whether this project
wants `crates/geom/src/scalar_lift.rs`'s *"one name, `map_scalar` on
every geometry type"* convention to **mint public doors ahead of
consumers** — and answering either instance alone leaves the other
unprincipled. So the lane runs the class's own exact instrument (drop
`pub`, compile, read the dead-code warnings) over every type the
convention names, checks whether `Frame` is reachable from the public
path or from Python at all — the existing evidence is a *workspace*
measurement, which is the right instrument for "nothing in-tree calls
it" and the wrong one for a claim about external users — and returns a
recommendation. The disposition is then a decision made on measured
ground, and it is the shape that may need Ev.

**WIRE's `paths` are corrected in the `names-vocab` lane, not globbed.**
WIRE has now landed in six `names/` files it does not claim, drawing the
fence one PR body at a time. The obvious glob
`crates/editor-core/src/names/*` is wrong: it sweeps in `role.rs`, which
is DOCM's, and one-file-one-item means the lane cannot write the
matching `keep_out` on DOCM's side — so the glob would mint an
unrecorded double claim to fix a fence problem. The files are
enumerated instead.

### What the briefs carry forward from this program's record

Every brief states the instrument rule as this program measured it:
**six of nine units had their central test claim corrected by an
instrument rather than a reading** — mutation ×3, a built fixture, the ε
matrix, and a fixture's own asserted precondition hiding the thing under
test, and not one came from reading a diff. The `operand-door` brief
additionally requires the lane's own adversarial re-read of its diff
before pushing, and requires it to say what that found: it is the only
thing that has ever caught the minting trap from inside a lane.

## 2026-09-12 — two units back, and a question that turned out not to be Ev's

`wire/placement-prose` (PR 2475) and `wire/names-vocab` (PR 2474) both
returned green and are under light style review, which is the posture
`plan.md` sets for them. CI verified at the check-run level here rather
than read off the reports: both heads show **0 checks in flight, 0
non-success, twelve `test (…)` jobs and five `k-lint (gate, …)` rows**.

### The `Frame::linear` decision is WIRE's, not Ev's — checked rather than assumed

`placement-prose`'s third deliverable was a **measurement, not a
removal**, deliberately: the row is a CLASS whose real question was
whether `crates/geom/src/scalar_lift.rs`'s *"one name, `map_scalar` on
every geometry type"* convention should mint public doors ahead of
consumers, and the lane filed that question on PROPS's slate as
`DESIGN.md`-shaped and *"probably Ev's"*.

It is not Ev's, and `CLAUDE.md`'s clause that landed today —
**"check that Ev ever agreed, before you wait for Ev"** — is what
settles it. Traced with `git log -S` over full history
(`git rev-parse --is-shallow-repository` → `false` in this session's
checkout, which the entry of 2026-09-12 above says to confirm before
trusting any dating claim from here):

- the convention was written by an agent in a **fix pass**, `b61d25ddc`
  (2026-09-02, CERT-N1, PR 1536), as a module doc-comment.
  `docs/DESIGN.md` does not mention `map_scalar` or scalar lifts at all,
  and neither `geom` README names it;
- the **opposite** rule — `crates/pncad/src/lib.rs`'s *"Re-export it the
  day a consumer needs it"* — was written in the façade skeleton commit
  `b43bb3e29` (2026-08-06). Also unratified, also a code comment.

So there is no ratified clause to change, nothing waits on Ev, and the
row records a real defect of a different kind: **two unratified
conventions pointing opposite ways at the same altitude, with neither
site aware of the other.** The evidence goes on the PROPS row through
`placement-prose`'s fix pass, because that row lives on the lane's
branch and one-file-one-item means it gets written once, where it lives.
The first attempt to append it from here created a second, front-matterless
copy and `work.py lint` caught it in one call — which is the rule working.

**The disposition is therefore an ordinary engineering call and it is
mine.** It is still held until the review returns, because the whole
decision rests on the dead-code table and the reviewer's brief requires
re-taking it with the instrument rather than reading it. The lane also
established that `Frame::linear` is **not a rung of the convention** at
all — `Frame` is a document-layer record in `editor-core`, spelled
`linear`, not a type `scalar_lift.rs` names — so the two halves of the
class are now cleanly separable and neither waits on the other.

### What the measurement found that the row did not predict

Of the **eight** `map_scalar` rungs, exactly **one** has a production
consumer outside its crate. Both top rungs (`Curve3`, `Surface`) warn
`never used`. `Vec2::map` has **zero** consumers workspace-wide, tests
included, and `Mat3::map`'s only consumer anywhere is the dead
`Frame::linear` this unit was measuring. The row opened as two instances;
it is nearer eight.

### Two disclosures the reviewers were pointed at by name

`names-vocab` disclosed that its own central fix is **unguarded**: the
refusal it repaired is defensive and unreachable from `editor-core`, so
no row evaluates a cycling body and *"a mutant that reverted the call
site to `map_err(|_| …)` would stay green."* Its three red mutants pin
the `Display`, the `From` and the Python tag inventory — the plumbing,
not the discard the item was filed about. That is an honest report of
the exact gap `reviewer-style-lane.md` Q6 governs, and the reviewer is
asked both to try to falsify the unreachability and, if it holds, to say
whether the owed *"unguardable, and here is why"* sentence is at the
claim site rather than only in a PR body.

`placement-prose` disclosed that the S9 ratio finding is **not** closed
and that the file got longer (430→458 lines). Same treatment: a
disclosed shortfall owes a schedule, and the reviewer checks whether it
has one.

## 2026-09-12 — PR 2475's review: mergeable, and the answer to the adversarial question was yes

**0 MAJOR, 2 MINOR, 4 NOTE, 7 style.** Verdict mergeable. Every one of
the three claims the brief said reading was not enough for came back
settled by an instrument.

**The measurement reproduced exactly, and the reviewer made it
stronger.** All five dead-code rows re-ran one mutation at a time in an
independent target: `Frame::linear` warns `never used` with **no errors
at all**, the `Frame::affine` control gives exactly three `E0624`,
`Vec2::map` is clean at `pub(crate)` with the full workspace compiling,
and `Mat3::map`'s only consumer anywhere is `Frame::linear` itself. The
lane had caveated the table as a lower bound because cargo stops a
crate's dependents at the first failure; the reviewer noticed that for
three of those rows **no crate failed**, so cargo never short-circuited
and **those are exact zeros, not lower bounds**.

**And it closed the stated blind spot with a differently-shaped sweep.**
The lane's excluded-roots check grepped `.linear::<`, which **cannot
match a turbofish-free call** — `let m: Mat3<f64> = f.linear();`, the
shape a demo would most plausibly write. Re-swept for the bare token
`\.linear\b`: zero hits across all four roots. That is the Q1 rule
working — *ask what the sweep's pattern could not match, then run one
shaped differently.*

**The `from_affine` inertness claim is now settled by execution.** The
lane filed it from reasoning about bit patterns. The reviewer wrote a
row comparing `from_affine` against a branch-free copy over six inputs
including `-0.0` and a subnormal, showed it goes **red** under a mutation
of the identity arm, and checked it is not vacuous. Adopted with
authorship kept; the row moves from an argument to a guard.

### The fix minted two fresh instances of its own defect

Asked directly, and the answer is yes — the eighth and ninth on this
program, and both inside a unit whose whole subject is "one rule, many
homes":

- **the guard census is a hand-written list.** `:102-110` names four test
  functions in backticks. They cannot be intra-doc links — you cannot
  link a test fn — so the rustdoc gate the row cites as proof cannot see
  a rename, and **one entry of the new table is already wrong** against
  the PR's own head.
- **the `#[must_use]` rule is held by nothing.** It is written in the
  tracker row only: not at the site, not as a lint
  (`clippy::must_use_candidate` is off workspace-wide), not as a test.
  The twelfth `Frame` method arrives without it and the file is back in
  S8's state. The reviewer found the sibling instance already open on
  WIRE's own ground —
  `work/wire/the-third-tag-vocabulary-macro-owes-a-unification-trigger.md`
  records the identical inconsistency one vocabulary over — which makes
  fixing one and leaving the other the half-fix shape by name.

**MINOR-1 is the sharpest single finding**: the new section's one
genuinely new assertion is the false one. *"Every claim above is
guarded"* is untrue of `Mat3::determinant`'s fixed-evaluation-order
claim, which has **no test anywhere in `editor-core`**, and whose only
guard in the tree asserts `1.0` and cannot distinguish evaluation orders
at all.

**"One home" is one home plus five survivors**, and the row overclaims:
four methods carry a cross-reference, not the eight it says.

### Two process notes

**The reviewer was rate-limited out of the PR body for its whole
session** and said so, working from the diff and the tracker rows
instead. That is the right call and the right disclosure — the diff is
the artefact of record — but it means no finding was made about the
body's wording, and the report says so rather than leaving it implied.

**A citation-rot class, found in passing and now owed a home.** Seven
line-numbered citations of `placement.rs` across `work/` point at the
wrong thing after this PR's 28-line growth, the code correct in every
case. `implementer-discipline.md` §7 already states the rule they
violate — *"cite by name; line numbers rot"* — so this is a stated rule
with no instrument. The fix pass gives it a `work/meta/` row rather than
fixing seven files from here, because one-file-one-item means those
seven are seven programs' edits.

### The `Frame::linear` decision, taken

**Delete.** The measurement is now exact rather than a lower bound and
was reproduced by two parties with the instrument; the blind spot that
could have hidden a consumer is closed; `Frame::linear` is not a rung of
the `scalar_lift.rs` convention, so that convention does not claim it;
and the only stated position in the tree at the same altitude is
`pncad/src/lib.rs`'s measurement-driven *"Re-export it the day a
consumer needs it."* Fail-loud says an unused public door is a claim the
library does not keep.

**It is not in this fix pass, deliberately** — the same reasoning PR
2375's fix pass used, one step on. A fix pass repairs what its review
found wrong; a public API removal gets its own unit so the deletion's
blast radius is reviewed on its own terms, including the finding it
exports to PROPS that `Mat3::map` is consumerless the moment it lands.

## 2026-09-13 — PR 2474's review: mergeable, and it falsified one of the unit's own premises with a fixture

**0 MAJOR, 5 MINOR, 3 NOTE, 9 style.** Verdict mergeable. The three
claims the brief said reading could not settle were all taken with an
instrument, and one of them came back **false**.

**Claim 3 confirmed the right way.** A scratch `ValuePayload` variant
produced exactly three `E0004`s, one of them `interrogate.rs`'s match —
so exhaustiveness really is preserved and the compiler really is the
door. That is the claim a reading would have passed on and an instrument
settled in one build.

**The sweep's two stated blind spots were swept differently and are
empty.** The reviewer went after the two the lane said could hide a
fourth reader — a word bound to a differently-named const, and a word
built at runtime — by shape rather than by word (every file with a
`ValuePayload::…` arm, nineteen of them, read individually). No fourth
Rust reader exists. A blind spot that is *stated* is one someone else
can close, which is the whole reason the rule asks for it.

**The Python tag word is not an instance of the census class**, checked
rather than assumed: the pin **parses `tags.rs` at test time**,
enumerates the literals and diffs them against the committed inventory,
naming the exact words added or gone. One spelling plus a census that
localises — the opposite of
`census-messages-assert-a-mismatch-without-naming-what-they-found`.

### The finding that matters: a premise was false, and the fixture proves it

The unit argued that deliverable 1 could carry **no test row at all**,
because `output_body` is reached only through `entity_of` after a name
resolves, so its six arms are unreachable through any public door.

`crates/editor-core/src/clearance.rs:1979` calls it with a
caller-authored `Selection` and no name, and `clearance::clearance` is
public. The reviewer ran it: the arm is reachable, and the refusal a user
reads is **`[selection] node 0's value carries no body at index 0`** —
which asserts a *false fact*, since no index of a datum carries a body
and index 0 is not the problem.

**This is the same discard, one caller away from the function the unit
rewrote** — and it is not a new row. `work/shell/clearance-reports-a-no-bodies-payload-as-a-bad-body-index.md`
has covered `clearance.rs:1979` since 2026-09-11, filed by this program
off PR 2378's sweep. What it gains is what it never had: **reachability
from a public door, and an executable repro.** The fix pass appends it
there with the reviewer's authorship, rather than opening a second row —
which is the "grep the owner's directory first" rule paying off.

The fix pass is told to be careful about *what* the false premise
overturns: through that door the family word is destroyed by
`map_err(|_| …)` immediately, so the conclusion may survive for a
different reason. Asserting the old conclusion under a new premise is
exactly the move a review exists to catch.

### The trap fired again, and the sibling is worse than what was fixed

**S1 — tenth instance on this program.** The unit that removed a
vocabulary's third spelling added a **second spelling of a sentence**:
`emit.rs:123` and `:130` now hand-write the same framing clause eight
lines apart, with a comment disclosing the copy. Nothing keeps them
equal, because the new suite asserts the substring against one arm only.

**The Q4 sweep found something worse than the unit's own subject.**
`emit_topo.rs:894-908`'s `chase_b` is a second hand-written
split-lineage walk with its own budget, and on exhaustion it **returns a
silently wrong root with no refusal at all**. The unit repaired a lost
*locator*; its sibling forty lines away loses the whole *failure*, on
the lane used for exactly the grafted operands that can produce a cycle.
`chase()` has the same silent fallthrough. Directed: sweep it here if
the repair is local and mechanical, otherwise file it and **label the
half-fix** — an unlabelled one is the only unacceptable answer.

### Two corrections against the lane's own honesty, and one against mine

The unit's unreachability sentence was the **broad** claim — *"no door
reachable from this crate constructs a cycling body"* — and the tree
contradicts it in shipped kernel prose (`topo/src/props.rs:877-881`)
plus a tracker row recording that `split_root`'s cycle arm **fired on
real assembly products**. The narrow claim about this chase survives and
is untested; it now says so.

Two of the four announced territory crossings were **convenient, not
forced**: `emit.rs` already contains `mod display_tests` in WIRE's own
file, whose existing rows are the shapes the new suite needed. Put there,
the new integration suite and its `all.rs` line — and two crossings of
another program's territory — do not happen. Worth recording as a
general lesson: *an announced crossing is still a crossing, and the
question "is there already a home inside my own fence" is cheaper to ask
than to announce.*

And the lane filed a row against another program for a header claiming
"every rung" while pinning five — then shipped a new instance of exactly
that in the file it was editing (`every_variant_names_its_subject` is now
five of seven). Recorded on the PR, not smoothed over.

## 2026-09-13 — PR 2480's FULL review: the byte-identity claim was measured, not argued

**0 MAJOR, 4 MINOR, 4 NOTE, 11 style.** Verdict mergeable. This is the
unit `plan.md` raised to a full review at dispatch against three **E**
class estimates, and the raise earned itself on the central claim alone.

### The instrument the claim needed, and the review built it

The unit claimed **exactly one refusal's text moves** and every other
message is byte-identical under a new compile-time composition. The brief
forbade settling that by reading the macro. The reviewer wrote
`zz_probe_operand_messages.rs` — one document, **19 miswirings**,
rendering `(expected, found, input)` for every document-reachable
`WrongOperand` — compiled it **unchanged on both trees**, and diffed the
two rendered sets.

**Exactly three lines differ, all `section_of`**; thirteen other rows are
byte-identical, covering 13 of the 15 converted sites. The claim holds,
and now it holds because it was measured.

The probe is kept and handed to the fix pass. It is **the shape-level
census the suite does not have**, which is the reviewer's phrase and the
right one: the unit's own suite pins six chosen miswirings, the probe
enumerates the road.

Also settled by counting both trees rather than by assertion: **17
`WrongOperand` constructions on the base, 2 on the head.**

### The finding that matters most: two stated rules with nothing behind them

*"Constructed at exactly two sites"* and *"no `expected:` is a literal at
a call site"* are **source-text predicates that nothing reds.** A third
construction ships green; a fresh literal ships green.

**The history is the argument, and it is this program's own.**
`wire-expected-phrases-spell-family-words-as-literals` was closed on
PR 2376; the literals came back as
`composed-expected-phrases-are-hand-copied-across-sites`, which is one of
the rows this unit closes. Closing it a second time with nothing guarding
it is an invitation to a third. A source census is idiomatic here —
`every_suite_file_is_aggregated` and a dozen suites already walk source
through `test_utils::source` — so "unguardable" is not available.

**The same shape one level up, and the trap's eleventh instance**:
`macro_rules! family_word` closes a hand-written list by minting a second
hand-written list. A const with no arm is a compile error, which is the
good half; **an arm with no const is silently dead**, because macro arms
are not dead-code-linted.

### What the sweep's stated blind spot was hiding

The unit's grep keyed on `NodeErrorKind::WrongOperand`, and said so. The
review went after the shape instead and found **four more same-shape
sites in the same file** — `resolve_open_faces` and `resolve_selection`
are *the same five lines*, and `wire_datum` carries a third copy inline —
against a PR sentence disposing of the area as *"one enum, two arms, no
duplication yet."*

The class as it actually stands has **six spellings in this crate**:
`expected/found`, `name/found` (three error kinds), `verb/found`,
`wanted/found`, `expected/found` again in `stackup.rs`, and the node road
already filed on DOCM. One row is owed for the entity-kind door, and the
"no duplication yet" sentence is owed a retraction.

### Two half-fixes of rows this very unit closes

- **The module header is better and still short.** It names wiring and
  declaration routing, not the ~150 lines of placement-rule arithmetic
  that is `pub(crate)` with a consumer *outside* the module. Since the
  unit closes `wire-rs-module-header-describes-five-sixths-of-the-file`,
  a header that still omits a job is a half-fix of the row it closes.
- **The stated goal is already false inside its own file.** Four sites
  spell a `phrase::` const plus a suffix by hand — `"datum frame x
  axis"`, `"datum frame y axis"`, `"datum plane normal"`,
  `DATUM_AXIS_ROLE` — none composed.

### A correction against the orchestrator, recorded

My brief gave `wire.rs` as 4861 lines on the head. That is the **merge
base**; the head is 4921, and the PR's own sentence had it right. Third
time on this program that a lane's or a reviewer's number beat the
orchestrator's — the earlier two were the shallow-clone datings. The
lesson is the same one: **the orchestrator's numbers are the ones nobody
re-takes**, so state where they came from.

### One design wrinkle worth keeping

`node_operand`'s two halves disagree about which node they answer for:
`found` comes from `node_value_kind`, which walks `Node::Transform`
chains, while the caller's `read` matches the unwalked node — so a
transform over a profile would refuse **`expected: "profile", found:
"profile"`**. Unreachable today because `wire_transform` poisons the loft
first, which is why it is MINOR rather than MAJOR; but a door whose two
halves disagree about their subject is the thing that becomes reachable
later, quietly.

## 2026-09-13 — PR 2475 MERGED (`9dafd4172`), and two process corrections against the orchestrator

Verified at the check-run level on the final head — the one with
`origin/main` merged in, not the one the fix pass reported — **0 in
flight, 0 non-success, twelve `test (…)` jobs, five `k-lint (gate, …)`
rows**. The distinction matters: the lane's green was on `650ab6bcd`,
and merging main moved the head to `f1e697baa`, which is a different
tree and got its own run.

**The fix pass wrote the guard rather than taking the escape.** The
review's sharpest finding was that the new section's one genuinely new
assertion was the false one. The lane could have written *"unguardable,
and here is why"* and been within the rules. Instead it pinned
`Mat3::determinant`'s **association** — `c0.dot(c1.cross(c2))`, `dot`
summed left to right — on a frame whose three summands are `1.0`,
`1e16`, `-1e16`, where the two groupings of one addition chain answer
`0.0` and `1.0`. It asserts **the groupings disagree** before asserting
which one the code gives, so it cannot pass vacuously, and it is
red-first verified. A D9 evaluation-order claim that had no test
anywhere in `editor-core` now reddens from one crate away.

**And it repaired the minted defect structurally.** The hand-written
census of test names is gone, by **inverting the reference**: the home
names no test, and each test row names the claim it keeps. Nothing
points at a test from outside, so nothing can rot. The one citation that
cannot be inverted — it lives in `asm2a_instantiate` because its claim
needs a whole document — carries the Q6 sentence at the site saying a
test fn is not an intra-doc link target, so a rename there is silent and
the rustdoc gate is blind to it.

**The row closed and its residue got a file, not a sentence.** S6 and S8
are discharged; S9, the ratio, is not, and
`placement-rs-frame-carries-51-doc-lines-over-a-7-line-struct` now
carries it — deliberately narrower than its parent, because "is 60%
prose a defect" is not answerable and a row that asks it sits open
forever. `work/README.md` is explicit that a residue disclosed only in a
closing section is invisible to the re-homing sweep.

### Two corrections against this orchestrator

**I committed a file with conflict markers in it.** Merging main
produced two tracker conflicts; I resolved one, then ran the whole-tree
marker grep and `git add -A && git commit` **in the same call**, so I
read the grep's output after the commit had already staged the second
file verbatim. That is the exact hazard
[[agent-lane-operations]] records — *"`git add -A && git commit
--no-edit` stages a conflicted file verbatim and prompts for nothing"* —
and knowing the rule did not help, because I ran the check and the
action together. **Check and act must be two calls.** Caught by the next
`work.py lint`, fixed forward rather than by rewriting.

**The upstream cause is mine too, and it is now fixed.** Both conflicts
existed because I had been writing interim `status=review` and `pr:`
onto item files **on the orchestrator branch** while the unit branches
own the same YAML block. The convention already says state-sync rides
the unit's own PR; writing it twice manufactured the conflicts.
**Stopping: from here the unit branch owns `status`, `pr` and `closed`
on its own rows**, and the orchestrator branch carries the log, the plan
and rows no unit is touching. Dispatch-time `status=dispatched` stays
here, because at that moment no unit branch exists yet.

### `Frame::linear` dispatched

`wire/linear-door`, off the merged main. The measurement is now exact
rather than a lower bound, reproduced by two parties with the same
instrument, and the sweep hole that could have hidden a consumer is
closed. The brief carries the grounds so the lane can recognise evidence
that would overturn them, and is explicit that the row is a **class with
two instances** and this unit disposes of one — closing the whole row
would mislabel a half-fix.

## 2026-09-13 — PR 2474 MERGED (`c6202101b`); 2480's fix pass and 2487 both to review

Both merges verified at the check-run level on the final head with
`origin/main` merged in — the tree the lane tested is not the tree that
lands, and this program has one entry per merge saying so.

### 2480's fix pass built a guard, so it gets a narrow delta round

The full review found the unit's two stated rules — *"constructed at
exactly two sites"*, *"no `expected:` is a literal at a call site"* —
were **source-text predicates that nothing reds**. The fix pass built
`operand_vocabulary_census`: three rows reading source through
`test_utils::source`, with **floors** (12 door calls, 10 macro arms, 17
rows) so a drifted scan reds rather than measuring nothing, and a doc
stating what the census cannot see. It also closed S2's hole — an arm
with no const was silently dead, because macro arms are not
dead-code-linted.

It went further than asked on two counts, both good: **S4 taken, so the
count is 17 → 1**, not 17 → 2; and the **reviewer's probe adopted as
assertions** rather than subsumed — 17 document-reachable refusals plus
3 asserted as edit-door defences, authorship credited in the module doc.
Seven mutants, each red for its own stated reason.

**A delta round is dispatched, narrow**, for one reason: the fix pass
built a **new guard mechanism and no review has ever seen it.** A census
with a hole reports green — `plan.md` says exactly that about `D364` —
and this one's discriminators are textual heuristics whose safe
direction is asserted rather than tested. The brief's headline question
is the one worth the round: **the fix closed "a hand-written list with
no census" by adding a census with hand-written sentinels, floors and
counts. Is that the trap one level up, or the point where hand-writing
is correct?** "Nothing further" is stated as a complete answer.

**One honest negative from the fix pass, worth keeping.** S3's sweep —
composing the direction role words from the phrase consts — **does not
compile**: `concat!` takes literals and a `const` is not one, so the
macro layer would have to be extended from words to phrases. Three
literals became named consts, so the one-home half is done, and
composition is filed as a residue with `eval::phrase`'s doc stating it
as a residue rather than a boundary. A sweep that was attempted and
failed, reported as failed, is worth more than one that was never tried.

### `Frame::linear` is deleted (PR 2487), and the censuses could not see it

The unit executed the decision and the grounds held. One correction to
the brief, already carried by the row: the test module had **one**
`.linear::<` site, not three.

**The finding that generalises, and it corrects my brief rather than the
lane's work.** I wrote that the two façade censuses "have twice caught a
public surface change neither a lane nor a review noticed" and to expect
one of them to have an opinion. **Neither did, and structurally neither
could.** `every_document_layer_root_export_is_carried_or_listed` reads
`editor-core/src/lib.rs`'s root `pub use` **names**; the Python census
reads `pncad.pyi`'s classes and attributes. `Frame` is still exported
and still carried, so **a method removed from a carried type is in
neither alphabet**. Verified here against the census body rather than
taken on report.

If that holds under review it is bigger than this unit: the two
instruments this program has been treating as the backstop for public
surface changes see **names, not signatures**, and cannot see a removed
or changed method at all. The reviewer is asked to confirm or refute it
and to say whether it deserves a row.

The lane also reported, unprompted, that it **never got a build slot**
for its entire life — one was held by the operand-door fix pass's
50-minute `doc-gate.sh` run — so nothing but `cargo fmt` ran locally and
everything rests on CI. It caught an early retry loop **exiting 0
without running** and declined to report that as a pass. That is the
waiter self-test rule catching exactly what it exists to catch.

**The half-fix is labelled**: the row stays `open`, retitled to say the
first instance is deleted and the two `profile` `map_scalar` rungs are
the half still live, with a `## Half disposed` section giving the reason
their dispositions differ.

## 2026-09-12 — `Frame::linear<T>` deleted; half the class disposed

`crates/editor-core/src/placement.rs`'s `pub fn linear<T: Real>` is
gone, with its one caller — the `linear::<f64>()` cross-check inside
`affine_at_f64_carries_the_stored_bits`. The test stays: its subject is
`Frame::affine`, and only the limb reading the deleted door was cut. The
`# Exactness` section named both doors as the read-back pair and now
names `affine` alone.

**The row does not close.** It is a class with two instances; this
disposes of one. `profile`'s two `map_scalar` rungs are the other, they
sit outside WIRE's fence, and their argument is the `scalar_lift.rs`
convention's rather than this one's.

**The delete exports a finding**, exactly as the row's own
counterargument predicted: `Mat3::map` had one consumer workspace-wide,
this door's body, and now has none. Filed as evidence on
`work/props/the-scalar-lift-convention-mints-doors-faster-than-consumers.md`
— PROPS's ground, PROPS's call, and `Mat3::map` was left in place.

No assertion was added. A deletion's guard is the compiler plus the two
façade censuses; a test asserting an absent function is absent is
documentation, and §2 of the implementer discipline says to delete it
rather than write it.

## 2026-09-13 — CORRECTION: `Mat3::map` was never consumerless, and this log said it three times

PR 2487's review returned **1 MAJOR** and it is against a claim **this
orchestrator relayed upstream and wrote into this log three times** —
in the 2475 review round, in the 2480 delta dispatch entry, and in the
lane's own `Frame::linear` entry immediately above, each saying
`Mat3::map`'s only consumer was the deleted door. Those sentences are
**wrong** and are
corrected here rather than edited in place, because this log is
append-only and the record of having been wrong is the useful part.

**`crates/geom-core/src/linalg/affine.rs:48` is `Affine3::map`'s body:**

```rust
Affine3::from_parts(self.linear.map(&f), self.translation.map(&f))
```

`self.linear` is `Mat3<T>`, so that call **is** `Mat3::map`; the second
is `Vec3::map`. Verified here, not taken on report. The function's own
doc comment two lines above says it in words — *"the linear part through
`Mat3::map`, the translation through `Vec3::map`"* — so the claim was
refutable by reading the documentation of the function that makes the
call.

**The irony is load-bearing**: `Frame::affine`, the door PR 2487 *keeps*,
is `affine_f64().map(T::from_f64)` → `Affine3::map` → `Mat3::map`.
Deleting `Frame::linear` orphaned nothing, and the test the unit kept
still exercises it.

### The instrument was right; the narration overstated it

This is the transferable part, and it is a **class**, not a slip.

The measurement table defines level **B** as `pub` → `pub(crate)`,
answering *"any consumer outside this crate"* — and `E0624` **cannot
fire for a same-crate caller**. `Mat3::map` was measured at level B and
then written up as *"its only consumer **workspace-wide**"*. Those are
different claims and the second does not follow from the first.

**The evidence that refutes it was inside the table the whole time.**
Every level-B row with no in-crate use reports a `warning: method … is
never used` beside its errors — `Vec2::map`, `Point2::map`,
`Point3::map`, `Affine3::map` all do. The `Mat3::map` and `Vec3::map`
rows report **no warning at all**, which is exactly what an in-crate
consumer looks like. Nobody read the absent warning as data.

**Three parties repeated it**: PR 2475's reviewer, who ran the level-B
mutation and got exactly one `E0624` (a true result); PR 2487's lane,
which inherited the narration; and this orchestrator, who relayed it to
Ev as *"`Mat3::map`'s only consumer anywhere"*. A correct measurement
narrated one notch too broadly survived two reviews and a merge, because
each party checked the number and none re-read what the number measured.

**Rule, stated for the rest of this program**: *a demotion measurement
answers the question its LEVEL asks. `pub` → `pub(crate)` answers
"outside this crate"; only dropping `pub` entirely answers "anywhere".
Write the verdict in the level's own words, and treat an absent
`never used` warning as evidence of an in-crate consumer.*

### What is NOT affected

**`Frame::linear`'s deletion stands.** Its own measurement was **level
A** — drop `pub` entirely, `never used` with no errors, whole workspace
compiling — which is the level that answers "anywhere", and the reviewer
confirms the deletion is correct and complete, the surviving test still
reds for real reasons, and nothing in code, docs, READMEs or `pncad.pyi`
still claims the door exists. The decision was sound; the side-finding
it exported was not.

The fix pass is directed to sweep **every** level-B row for the same
substitution rather than repairing the two named, and to label a
half-fix if it stops.

### The census blind spot is confirmed and sharper than reported

The reviewer settled it and found a second instrument the lane had not
named. The Rust census builds its alphabet with `module_pub_use_names`
over `editor-core/src/lib.rs` — leaf names of root `pub use`. The Python
side has **two** censuses, not one, and the member-level one
(`test_every_member_of_a_matched_type_is_spelled_or_listed`) draws from
`declared_members`, which reads enum variants and bare-`pub` struct
fields and **never reads `impl` blocks**. So `Frame`'s two `pub` fields
are guarded and every one of its methods is not.

**The class: the surface censuses guard names and members, and no
instrument in this tree guards a public *method*.** Every public method
added or removed here is unseen by both. Filed on `work/meta/`, and
cited from `crates/pncad/tests/all.rs`'s doc-comment as a fourth blind
spot beside the three it already lists.

## 2026-09-13 — the delta round on 2480: NOT mergeable, and it paid for itself twice

**2 MAJOR, 6 MINOR, 7 NOTE, one class.** The round was dispatched on one
argument — *the fix pass built a new guard mechanism and no review has
ever seen it* — and that argument was right.

### MAJOR 2 is the finding of the session

**The guard states one rule and asserts a proxy for it, and the thing it
was built to catch walked past it in the same diff.**

`operand_vocabulary_census`'s literal row asserts
`plain_string_literal(...).is_none()` while its failure message says *"an
`expected:` phrase comes from `eval::family` or `eval::phrase`."* Those
are different rules. The reviewer hoisted `family::SPLIT` into a local
`const` **at the call site** and the row passed **green** — a phrase in a
const in the wrong home is invisible to it.

And that is exactly the move the same commit made three times, minting
`FRAME_X_ROLE`, `FRAME_Y_ROLE` and `PLANE_NORMAL_ROLE` as call-site
literals. A guard written to stop a class, in a commit that then
performed the class, and passed.

**MAJOR 1 is the same defect in prose**: a new design-page sentence
asserts as fact that those four role words *are* composed from consts,
while the row filed **in the same commit** says "respell" in its title.
The lane's own honest negative — that composition does not compile,
because `concat!` takes literals and a `const` is not one — is the true
statement; the sentence is what has to change.

### The verdict on the adversarial question, which is a rule now

Asked whether closing "a hand-written list with no census" by writing a
census with hand-written sentinels, floors and counts is the trap one
level up, the answer came back **"yes, but only in its second half, and
the halves are cleanly separable"**:

- where the census computes a **set equality** — `built.len() == 1`, and
  `heads == sorted_used` for the macro-arm/const bijection — it is
  right, has no slack, and both were driven red. Hand-writing does not
  enter.
- where it computes a **hand-written floor over a hand-written roster**
  — `calls >= 12` over four door names — it is the defect one level up.
  Measured **18 against a floor of 12**, and the slack hides real drift:
  one door contributes 2 calls and is not in the roster at all, so
  renaming it takes the scan 18 → 16 and nothing reds.

**The decisive half of the verdict: a bijection was available and was
not taken.** The door set is enumerable from the region the row already
carves; the slot indices are derivable from the `fn` signatures the row
already locates. That is not "a guard must eventually be hand-written" —
it is a choice that went the wrong way.

**And the tree had already written this lesson down.**
`crates/test-utils/tests/reader_census.rs` carries a design note saying
*"a walk that matched nothing is not a pass, and the equality is what
says so … this row needs no separate count floor (an earlier one
asserted `found.len() >= 20`, which set equality had already subsumed
and which could not fail for the reason it stated)."* That paragraph is
about precisely the `>= N` floors this census wrote three of. The
project learned this, recorded it in the file the census imports from,
and the census did not read it.

**This is the standing guidance for the census block** that was waiting
on this round's answer: **prefer a bijection; a floor is what you write
when you have proved no bijection exists, and you write why.**

### Four holes in the floors, each instrumented

The reviewer forced each rather than reading: a braced import hides a
second construction site (a plain `use` is caught only *by accident*,
because the `use` line itself carries the matched text); `"fn operand"`
is a prefix of `"fn operand_refusal"`, so renaming the door cannot red
that check; row 2's split half matches a **field declaration** whose
"initializer" is a type, so it is non-empty even when the real
initializer moves — and `test_utils::source::sole_initializer` exists
for exactly that refusal and was not used.

### A correction against the orchestrator, again

**MINOR 6 is my error.** I directed the last fix pass to write that
`split.rs:218`'s `assert_eq!` is *"the only byte-exact pin any phrase in
that vocabulary has"*. The same pass then added a suite pinning 17
refusals byte-exact over 8 phrases, `"datum plane"` among them. I wrote
a uniqueness claim and then asked for the thing that falsified it, in
one message.

### The class the round turned up

Three hand-rolled sentinel-region extractors now exist — the new one,
a `split_once` pair in the same file, and one in `topo/tests/` that is
nearly line-for-line the new one — and **none is in
`test_utils::source`, which is the declared home for exactly this**.
The floor-message idiom is written three times in one file beside them.

**And a hole in a guard the tree relies on** (NOTE 6, to be filed):
`every_site_that_reads_rust_source_is_in_the_ledger` is keyed on the
**reader file**, so a second census inside an already-listed file
arrives invisibly. This census is the first instance.

## 2026-09-13 — PR 2480 MERGED (`585f2f62c`): the operand door, after a full review and a delta round

Five rows close on one unit. **`WrongOperand` is constructed at exactly
one site, down from seventeen**, and the door computes `found:` from the
value it was handed — which is why the negation row closed as a
consequence rather than as a text edit.

### No third round, and the reason is evidence rather than fatigue

The delta round broke the first census by hoisting a const to a call
site. The rewrite could have been waved through on the lane's report; it
was not. Three things settled it:

- **the reviewer's own breaking mutation is now the first row of the
  mutant table**, and six mutants each red for their stated reason;
- **the floors became set equalities**, which have no slack by
  construction — and the door roster is now *derived from the region's
  `fn` signatures*, so the two holes about renaming and adding a door
  died at the root rather than being patched;
- **the one vacuity a set equality does NOT rule out was checked here,
  directly.** `assert_eq!(a, b)` passes when both sides are empty, and
  both sides derive from one scan, so a scan that died reads as a pass.
  Read on the head: every equality is preceded by a non-emptiness
  assertion on its own derived set, each with a message naming the
  failure mode — *"eval/wire.rs mentions no `family::` or `phrase::`
  const at all — the vocabulary moved and this row is reading the wrong
  file"*. That is the check the whole census turns on, and it is there.

**One floor survives and is argued**: `doors.len() >= 2`. The door set is
what the row *computes*, not something it compares against a second set,
so there is nothing to equate it with. That is exactly the exception the
steer allowed — a floor is what you write when you have proved no
bijection exists, and you write why.

### What the lane did better than it was asked

**MINOR 5 was deleted rather than extended.** `DegenerateDirection`'s
doc carried a hand-written list of role words and the unit had added
three more without updating it. The obvious repair is to add them; the
lane removed the list and named the class instead, because a list there
is a second copy of a set those modules already hold. That is the right
instinct applied to the finding rather than to the instruction.

**The census moved out of `eval/mod.rs` into `tests/`**, where the
tree's other source censuses live, and `sentinel_region` was hoisted
into `test_utils::source` — the declared home — with two of its three
callers converted and the third announced with its own row plus the
untaken sweep recorded.

### Residues re-homed at the moment of closing, not disclosed

`wire-rs-module-header-…` carried **three** findings and only the header
is discharged, so the comment ratio and `wire_sweep`-exists-to-fail got
their own file. The entity-kind door's six spellings, the role words
that respell rather than compose, the three-homed sentinel extractor,
and the reader ledger keyed on the file rather than the census are each
a row. `work/README.md`'s rule was the operative one all evening: a
residue named only in a closing section dies with the directory.

### The tally this unit closes

Three full rounds on one unit — a full review, a delta, and a fix pass
that answered both MAJORs — and **every round found something the
previous one could not have.** R1 could not review a census that did not
exist; R2 broke the census R1's findings caused; the orchestrator's own
read caught the vacuity R2 did not name. The lesson stands for the
program: **a round is worth running when the diff contains a mechanism
no previous round has seen**, and is not worth running otherwise.

## 2026-09-13 — the second block: the census family, and three small rows

Four units merged, nineteen rows closed, nothing in flight — so the next
block goes out. Two lanes, and the split is by **whether the delta
round's verdict governs them**.

### `wire/census-localise` — three rows, one file, and the rule they inherit

`census-messages-assert-a-mismatch-without-naming-what-they-found`,
`wire-roundtrip-census-localises-nothing` and
`document-only-vocabulary-blind-spot` are one subject seen three ways,
all in `crates/editor-core/tests/switch_program_vocabulary.rs`. **This is
the block that was held pending the delta round**, and the verdict is now
its design rule, handed over in the brief with both corollaries:

> Prefer a bijection. A floor is what you write when you have *proved*
> no bijection exists, and then you write why, at the site.
> A set equality is **not** automatically safe — it passes when both
> sides are empty — so every equality owes a non-emptiness assertion on
> its own derived set. And assert the rule, not a proxy for it.

Deliverable 3 is where that bites and is the reason this unit is not
three small ones. Both censuses anchor on the **kernel** `ALL` — the
right anchor for the failure they exist to catch. The mirror is open: a
document-only variant of `ProgramArcData` or `ProgramTarget` is forced
through every exhaustive match that consumes it, **and every one of those
arms may legally resolve it into an existing kernel variant**, at which
point all five clauses stay green and the document form silently authors
something nobody wrote. The witness functions match on the kernel tag, so
nothing makes a document-only variant acquire a witness at all.

Deliverable 1's instance is small and load-bearing: the mode census's
laundering clause is, measured, **the only catch in the tree** for the
`res_spec` hop that #836 and S195 both call the hop that matters — and
it is the one clause that will not tell you where you landed. Its fix is
written fifteen lines above it, in the target census, which does the same
comparison and names what it got.

### `wire/small-batch` — three unrelated rows in one PR, deliberately

`from-affines-identity-fast-path-…`, `product-gate-says-verbatim-…` and
`placement-rs-frame-carries-51-doc-lines-…`. Each is a few lines, and
three PRs would be three CI runs; at the cancellation rate this program
measured on 2026-09-12 (three of five runs cancelled in thirteen
minutes, four programs) that is a cost paid by other programs, not by
this one. Separate commits, one PR.

Two steers in that brief are there because they are easy to get wrong:

- the justification for deleting `from_affine`'s branch is **not** that
  output is unchanged — `memories/output-stability-as-justification.md`
  says an argument of that shape justifies nothing — it is that the
  branch *cannot change the answer*, which is a property of the code
  that the adopted test pins. And after the deletion the lane must
  check whether that test still reds for a real reason rather than
  keeping it out of politeness to its author.
- `product.rs`'s gate calling itself the import rule **verbatim** is the
  exact self-declared-duplication tell `reviewer-style-lane.md` Q1 has
  reviewers grep for. The lane is told to check whether the two
  sentences are one rule with two spellings **before** calling it a
  prose fix; if they are, the row is a duplication wearing a
  documentation costume.

### Posture

`census-localise` gets a **full review** — a census with a hole reports
green, and this program has now proved that twice in one day, once by a
reviewer walking a const past a guard's own rule. `small-batch` gets a
light style review, with the bit-exact path as the one thing a reviewer
is told not to take on report.

## 2026-09-13 — the small batch returned, and it falsified a row this orchestrator wrote

PR 2499, full matrix green (12 `test (…)`, 5 `k-lint`, 0 in flight, 0
non-success — verified here at the check-run level, and `draft=false`
via REST after the GraphQL endpoint refused again). To a light review.

### Two checks in the brief paid for themselves, and one of them against me

**`product.rs`'s gate is a DUPLICATION, not prose.** The brief told the
lane to check whether the two sentences were one rule with two spellings
**before** calling it a prose fix, because `verbatim` at a copy site is
the self-declared-duplication tell. They are. `model.instances.len() > 1`
counts instances; an instance is one solid (`build_one_solid` makes one
body per `SolidSpec`, `transform_rigid` does not change the count,
`graft_disjoint` appends one per call, and
`the_assembly_record_indexes_the_shipped_solids` pins that the A7
record's per-instance `index` *is* the shipped body's `solids()` order),
so `instances.len()` **is** the shipped solid count and both sites say
the same thing. The row therefore **stays open, recharacterized**: the
policy has **no home** and is stated at **six sites across three
crates**, each citing another by prose. A row that would have closed as a
one-word prose edit is now a cross-crate finding.

**The doc-ratio row's premise was false, measured.** I wrote that row,
including *"part of the measurement will move without anyone writing
prose at all"* once `Frame::linear` was deleted. Measured across that
deletion: **the type doc moved by exactly zero lines**, and the file's
comment share went 42% → **43%** — *up*, because the door was more code
than prose. The parent row's *"fewer doors, not less prose"* does not
survive its own measurement.

Two of my numbers were also wrong: the type doc is **55** lines, not
~51, and the struct is a **4**-line declaration under 4 lines of field
doc, not "7 lines". **Fourth time this session a lane's or reviewer's
number has beaten the orchestrator's**, after the three shallow-clone
datings, the `wire.rs` line count, and the `Mat3::map` narration. The
pattern is now unambiguous and it is not about carelessness: **the
orchestrator's numbers are the ones nobody re-takes**, so they survive
in a way a lane's do not. Every figure I put in a row or a brief needs
its provenance written beside it, or it needs to not be a figure.

### The lane deleted a test rather than keep it out of politeness

Asked to check whether the adopted guard still reds for a real reason
after the branch it pinned was deleted, the lane found it did not:
`from_affines_identity_branch_agrees_with_the_branch_free_copy` was
comparing `from_affine` against a transcription of its own body, its
verified mutation no longer existed, and its one open scenario would stop
the fixture *compiling* rather than redden it. `implementer-discipline.md`
§2 makes deleting it the repair, and it was deleted — **and replaced with
the claim that does survive**: `from_affine` carries the affine's
coordinates and **snaps nothing**, which `mate/solve.rs` depends on.
Red-first verified against a tolerance-snapping `from_affine`.

That is the right disposition of a test written by someone else two
rounds earlier, and it is the disposition that is hardest to reach for.
The review is told to re-plant that mutation rather than take it on
report, because it is the one bit-exact path in the diff.

## 2026-09-13 — 2499's review: no MAJOR, and two MINORs that both land on citation

Verdict: nothing blocks the merge. Claim 1's reasoning was checked
against the type on the head, the deletion of the earlier guard was
confirmed as the repair with what replaced it **strictly stronger**, and
claim 2's chain holds — with **link 3 stronger than the lane stated**:
`graft_disjoint` hard-refuses `src.solids().count() != 1` with
`JoinDesync`, so an instance contributing zero or several solids is a
refusal rather than a silent miscount. The equivalence is enforced at
runtime, not merely by construction.

### The finding that matters: a replacement test with a hole where its consumer reads it

`from_affine_carries_the_affines_bits_and_snaps_nothing` has teeth in
**one direction only**. Every non-identity fixture perturbs the
**translation**; none is one bit from identity in a **column** with a
zero translation. The reviewer planted the mutant that closes that gap —
a `from_affine` snapping the **linear part** to `IDENTITY.columns`
within `1e-9` — and **all five placement rows stayed green.**

And that is precisely the mutant that matters, because of who reads the
answer: `mate/solve.rs` branches on `relative.is_identity_bits()` and the
`true` arm **discards the solved relative pose**. A linear-part snap
would read a gauge that rotated by a hair as *"did not move."* The row's
doc claims it would catch a frame one bit from the identity home; today
it would not.

**Seventh instance of the standing pattern**: a guard written against the
failure its author had in mind, blind to the neighbouring one. Found by
mutation, not by reading — the count of central test claims corrected by
an instrument on this program keeps rising, and not one has come from a
diff read.

### A citation that never existed, inside the PR that trims prose about citations

Claim 2's link 4 cited `the_assembly_record_indexes_the_shipped_solids`.
**No such test is in the tree and `git log -S` says the name never has
been.** The property *is* pinned, by two real rows in
`step-import/tests/freecad.rs`, so the chain's conclusion survives — but
the citation was taken on faith and propagated verbatim into the work row
and the PR body.

It is a live, already-rotted instance of the exact class deliverable 3
trims prose about, in the same PR that trims it. Directed to be fixed
**and recorded as an instance** rather than quietly corrected.

### The census the row now carries was wrong in count and incomplete in kind

The row says *"five sites"* over a table of **six**. And there is a
**seventh the review found, of a different kind**:
`step-import`'s `vertex_rest_contact` doc does not restate the policy, it
**relies** on it — *"the per-solid gate above sees only the pre-graft
copies and only when more than one instance ships"* is its premise for
refusing rather than passing over an unresolvable vertex. A **consumer
citing a rule by prose** is a worse position than another restatement of
it, and it was absent. `work/perf/assemble-aggregate-census-is-quadratic-in-solids.md`
has already drifted off the same policy, stating the trigger with the
wrong subject.

### And a lint gap, surfaced by my own bad date

The row I filed yesterday now reads `closed: 2026-09-12` under
`opened: 2026-09-13` — **closed before it opened, and `work.py lint`
passed.** The bad `opened` is mine. `lint` resolves every reference,
enforces the vocabularies and measures territory, and does not notice
that a row's dates run backwards; it is a cheap exact check over data the
parser already holds. Filed to `work/meta/`.

Fifth orchestrator-originated error caught by a lane or reviewer today,
and the second in one row. The standing correction stands: **a figure or
a date I write is one nobody re-takes** — it carries its provenance or it
does not go in.

## 2026-09-13 — the census unit returned, and it measured the blind spot rather than asserting it

PR 2501, full matrix green (12 `test (…)`, 5 `k-lint`, 0 in flight, 0
non-success, verified here). To a **full review**, per `plan.md`'s
posture and for the reason this program has now demonstrated twice: **a
census with a hole reports green.**

### The blind spot is measured, not argued

The row claimed a document-only variant could launder into an existing
kernel form and stay invisible. The lane built the mutant and recorded
the answer: **five of seven clauses stayed green**, and only the two new
censuses red. *"The compiler forced an arm everywhere — including the
new `spec_label` — and every arm was legally dischargeable without a
witness."*

That is the row's claim turned into a number, and it is the number the
review is told to re-take, because it is the entire justification for
deliverable 3 and it is exactly the kind nobody re-takes.

### The bijection took the shape the delta round's rule asked for

`declared_variants` reads `ProgramArcData`'s and `ProgramTarget`'s
variant names out of `program.rs` through `test_utils::source` — **that
is the document vocabularies' `ALL`, one home, still the declaration
itself**, without minting `ProgramArcMode`/`ProgramTargetKind`, which
would have been a third spelling of the vocabulary in a file the fence
says to announce rather than land in. **`program.rs` is untouched**, and
the brief's STOP condition — anything more than a derived constant in
DOCM's file — was respected by finding a shape that did not need one.

The non-emptiness corollary was applied **beyond where it was asked**:
the wire and slot clauses gained the same guard, not just the new
censuses. And the limit is stated rather than left implicit — a
macro-generated variant is invisible to a textual walk.

### Where the review will earn itself

The two `document_only_*` allow-lists are **both empty today**, which is
the honest state and also the classic vacuity: an empty list beside a set
equality is how a census passes over the thing it exists to catch. And
the lists are **hand-written**, in a unit whose own subject is
hand-written things going stale — so the adversarial question has a
specific target this time, and the brief names it.

The sweep's stated blind spot is the other one: the pattern read every
`assert!`/`panic!` for whether the message names the value that made it
fire, and **cannot see a clause whose message is fine but whose SUBJECT
is wrong**. One instance was found that way — by reading, not by the
pattern — which means the pattern's hit list is a floor on what is
there, not a ceiling.

### Nine clauses now say where they landed

Including the one the earlier review measured as **the only catch in the
tree** for the `res_spec` hop: it fired correctly and would not tell you
where you landed, and its fix was written fifteen lines above it the
whole time. Every message was captured from a planted mutant rather than
predicted, which is the form of evidence this program has settled on.

## 2026-09-13 — PR 2499 MERGED (`257d64d05`): five units in, twenty-one rows closed

Two rows close, one stays open **larger than it arrived**, which is the
better outcome of the two.

### The guard is now blind in no direction, and both halves were shown to fire

The review found the replacement test had teeth only where the
translation moves. The fix pass closed the other direction with two
zero-translation fixtures — a **subnormal off-diagonal** and the **next
`f64` below 1.0** — and added a **non-vacuity assertion to all four
near-identity fixtures**, so one that drifts onto the identity reddens
rather than passing quietly. That second part was not asked for and is
the part that keeps the row honest a year from now.

It then showed **both halves firing independently** against the
re-planted mutant: the carried-bits loop reds at *"from_affine moved a
bit at subnormal shear"*, and with the near-identity fixtures held out of
that loop, the `is_identity_bits` teeth red at *"from_affine snapped
subnormal shear onto the identity"*. The second is the one that matters,
because that is the same read `mate::solve`'s `reconcile` makes before
**discarding the solved relative pose**. The test's doc now names its
reader.

### A row that got bigger by being checked

`product-gate-says-verbatim-…` arrived as a one-word prose fix and
**stays open as a cross-crate finding**: the policy has **no home** and
is stated at **seven** sites across three crates, each citing another by
prose. The seventh is a different kind — a **consumer relying on the
rule** as the premise for a refusal, where a restatement that rots is
wrong documentation but a premise that rots is a refusal decided on a
condition that no longer holds. An **eighth** had already drifted within
two days of the row being written, which is the row's own prediction
happening in the wild. The sweep is grep-shaped over the rule's own
vocabulary, so **seven is a floor**, and the row says so.

### A citation that never existed, found inside the PR that trims prose about citations

The duplication chain cited a test name that `git log -S` says has never
been in the tree. The property *is* pinned, by two real rows, so the
conclusion survived — and the lane **recorded it as an instance** of
META's citation-rot class rather than quietly fixing the name. That is
the right disposition: an instance found inside the PR that touches the
class is worth more recorded than repaired.

### Three more orchestrator errors, and the counting rule that now exists

The ratio row was mine and it was wrong three ways: the premise (*"part
of the measurement will move without anyone writing prose"* — it moved
by **exactly zero lines**, and the comment share went **up**), two
figures (55 lines not ~51; a 4-line struct not 7), and the `opened` date,
which `git log --diff-filter=A` puts a day earlier than I wrote it.

The lane's own first counts were also one low in every row — the
trailing newline's empty string counted as a blank line — and the fix is
the generalisable part: **the counting rule is now written down beside
the table**, and the three classes are checked to partition the file.
Two independent counts of one quantity disagreed and neither stated its
rule; now one does.

**And the lint gap is filed with its code read rather than assumed**:
`work.py lint` types both dates, checks each one's *shape*, and checks
the `status`/`closed` coupling in both directions — but never reads the
two values together, so closed-before-opened, closed-in-future and
opened-in-future all pass. One `<` over data the parser already holds.

### One disclosure worth keeping visible

The lane **amended a CLOSED row on PERF's slate** — a factual citation
fix, authorised by §6, disclosed rather than buried, with the note that
someone may have preferred a new row. Recording it here so that
preference can be expressed against a visible decision rather than
discovered later.

## 2026-09-13 — 2501's full review: NOT mergeable, and the hole was found by adding one attribute

**3 MAJOR (one a confirmation), 2 MINOR, 2 NOTE, 7 style.** The strongest
round of the program, and it vindicates the full posture on a unit whose
three rows all read as tidying.

### MAJOR 1: the census goes green if you write an attribute

`variant_name` takes alphanumerics off the front of a variant's range,
and **an attribute sits in front of the name**. So `#[doc(hidden)] Chord`
yields `""`, the empty-name filter discards it, the variant vanishes from
`declared`, both sides of the bijection agree, and the census reports
green.

Measured twice over one mutant. The document-only laundering gives
`5 passed; 2 failed`. **Adding only `#[doc(hidden)]` to those same two
variants — nothing else changed — gives `7 passed; 0 failed`.** The
variants still launder; the census built to catch exactly that is silent.

The unit's stated limit was "a macro-generated variant is invisible to a
textual walk". True, and not the whole of it: `#[cfg]`, `#[serde]`,
`#[doc(hidden)]` are ordinary spellings in this tree. **And the
non-emptiness guard does not reach it** — that guard catches a *total*
scan failure and says so; a **partial** drop is the silent direction and
nothing watched it.

**This is the exact shape `plan.md` set the posture for** — *a census
with a hole reports green* — and it took an instrument to find, in a
census that was itself built to the rule a previous round produced.

### MAJOR 2: two of three vocabularies, and the header claims all three

`ProgramStep` is **the document vocabulary this file is named after**.
`res_step` is its construct hop, `chain_steps()` is a `Vec` so nothing
forces the verb side, and `Verb::ALL` stays fully witnessed. Measured:
`ProgramStep::Dash` laundered to `Step::Line`, discharged at four
compiler-named sites plus `step_label` — **7 of 7 green**. `step_label`'s
exhaustive match forced a *label*, not a *witness*, which is the row's
own finding one level up.

And the header this PR added says *"the mirror failure is a variant added
to `ProgramArcData` or `ProgramTarget` alone"* — a **false completeness
claim in the file's own contract**, with the row closed on it. The
class-not-instance rule in its exact form: swept two of three, and the
third is the one the module is named after.

### MAJOR 3 is a confirmation, and worth recording as one

The five-of-seven measurement was **re-taken independently** — own
mutation, own discharge across the nine sites the compiler named, every
one legally dischargeable by laundering — and it **holds exactly**, with
the rendered messages matching verbatim. Recorded as a finding because
the brief asked for it to be settled, not because it failed. A number
that survives an independent re-take is worth more than one that was
never doubted.

### The file's own words convict the fix

`switch_program_vocabulary.rs:430-435`, written before this PR: *"There
is no exception list … an empty escape hatch is a hatch that will be
used."* This PR adds **two empty escape hatches** to that file, 330 lines
below. Whichever is right, they cannot both be — and MINOR 4 is the
sharper form: an allow-listed variant is **declared, not covered**, since
it never enters `corpus()`, so the round-trip, the slot bijection and
both corpus-reach clauses stay blind to it.

Directed: **do not ship an empty exception list.** Refuse any unwitnessed
document variant, and add the list the day one exists, with its argument.

### The steer that was wrong was mine, and the reviewer took it on

My brief pushed away from minting an `ALL` on the document enums — a
third spelling in DOCM's file. The lane found a text-walk shape that
avoided it, and the reviewer's Q7 says plainly it would not have anchored
on text: **the walk bought avoidance of one spelling and paid with a
failure mode that is silent.** It also names the long "third spelling"
justification as the kind the style brief says to treat as *mild evidence
for flagging*, which is the rule working against the orchestrator's own
argument.

Re-taken: my STOP condition was *"more than adding a derived constant"*
in DOCM's file, and a derive- or macro-generated `ALL` **is** a derived
constant — inside the fence with an announcement, and it fails at compile
time. **Prefer the anchor that fails loudly** is the directive; patch the
lexer only if the compile-time anchor genuinely cannot be had, and then
file the residual class rather than leaving a silent mode undocumented.

### Two instances of the class inside the code that closes it

`declared_variants`'s own panic restates its expectation and never names
what it got. And the verb census **re-derives its own subject** — a
second freshly-built corpus agreeing with the first only by coincidence
of construction — which is word for word the reasoning this same PR
wrote fifteen lines away to justify doing the opposite.

## 2026-09-13 — the census took the compile-time anchor, and the roster is what is left

The fix pass chose the anchor that fails loudly. `program.rs` gains
`document_vocabulary!`, and all three document enums are declared through
it, projecting `ALL_NAMES` **from the same tokens that declare the
variants**. Verified here rather than on report: the macro captures
`$(#[$variant_meta:meta])*` ahead of each name, so attributes pass
through and the whole class the review found — `#[doc(hidden)]`,
`cfg_attr`, doc-comment-plus-attribute, raw identifiers — is closed **by
construction** rather than by a widened reader.

**The demonstration is the right one**: all three vocabularies mutated at
once, **each carrying `#[doc(hidden)]`** — the exact spelling that took
the text version from red to green — discharged everywhere the compiler
named, gives `5 passed; 3 failed`, one red per vocabulary.

**The reviewer's fixture was declined, correctly.** It pins a text walk
that no longer exists. Its finding is what forced the change and is
recorded on the row, which is the right disposition of a fixture whose
subject was deleted by the fix it caused.

**And the ledger line came back out**, because the file stopped being a
source-reading site when `declared_variants` and the `test_utils::source`
import went — the same discipline that put it there, applied in the
other direction, with `reader_census` verified green after the removal
rather than assumed.

**The empty exception lists are gone**, which disposes of MINOR 4
structurally rather than by disclosure: the censuses are anchored on what
`corpus()` carries, so *declared* now means *covered* by the wire
round-trip and the slot bijection too. The file's own sentence —
*"an empty escape hatch is a hatch that will be used"* — no longer has a
counterexample 330 lines below it.

### What is left, and it is the same defect one level up

`every_declared_variant_is_witnessed` is called at **three hand-written
sites**. `ALL_NAMES` closes *"a variant arrives without a witness"*.
Nothing closes *"a vocabulary arrives without a census"*: a fourth enum
declared through the macro gets its `ALL_NAMES` free, gets no census, and
nothing reds.

Found here by inspection, in two calls, which is itself the signal — the
first hole in a new mechanism has never been the only one on this
program. Sent back before the delta round rather than after, so the round
reviews the final shape rather than a version already known to be holed.

**Third time in two units that the ROSTER rather than the SET was the
soft edge.** PR 2480's delta produced the rule (*prefer a bijection; a
roster is what you write when you have proved no bijection exists, and
you write why at the site*); the operand door's door-list was the first
instance, its macro-arm list the second, and this is the third. The
macro is already the one place every document vocabulary passes through,
which is exactly the position a roster can be projected from rather than
typed — and if it genuinely cannot, the reason belongs at the call sites,
not in a PR body.

## 2026-09-13 — the roster became a bijection, and the lane caught the trap inside its own fix

The three document enums were already contiguous, so they are declared
through **one** `document_vocabulary!` invocation projecting
`DOCUMENT_VOCABULARIES` beside each `ALL_NAMES`. The census iterates the
roster; the three hand-written call sites are gone.

**What makes it a list rather than a convention is a compile error**:
the constant is emitted once per invocation, so a second invocation does
not compile. That is the property the delta round is told to test in one
build, because it is the whole load-bearing claim — without it,
*declared through the macro* is a habit.

**The reason is written at the site, in both halves**, which is what the
directive asked for: `DOCUMENT_VOCABULARIES`'s own doc says it closes
*"a vocabulary arrives without a census"* the same way `ALL_NAMES` closes
*"a variant arrives without a witness"*, **because a roster typed out on
the test side is a second list kept in step with this one by hand, which
is the defect the whole macro exists to remove**; and the test door's
header says three calls would have closed the variant question while
leaving the vocabulary one open, *"which is this file's own defect one
level up."*

**The witness sets cannot be projected and are bijected instead** — only
the suite knows which walk of `corpus()` answers for which vocabulary —
compared as sets in both directions, so a vocabulary with no witness set
reds with instructions. That is the honest version of "prefer a
bijection": project what can be projected, biject the rest, and say
which is which.

### The lane caught the trap inside its own fix, unprompted

Asserting per vocabulary **inside** the loop would have named only the
first offender — **this unit's own class, re-introduced by the loop that
fixed a different one.** Instead the door returns its complaint and the
caller collects, so all three are named in one run. That is the second
time on this program a lane has sprung the trap on itself and caught it
before pushing, and it is the outcome the standing lesson is meant to
produce rather than the one where a reviewer finds it.

### The residual is disclosed, filed, and correctly routed

An enum declared with a plain `pub enum` has no `ALL_NAMES`, is absent
from the roster, and nothing notices. **Closing it means asking "is this
enum a document vocabulary?" over the file's declarations — a text walk,
which this very PR removed after measuring it silently wrong on any
attribute.** So the honest disposition is to state it at the site and
file it, on **DOCM's** slate, because the declaration convention that
would close it is a design call about that file. Filed as
`work/docm/a-document-vocabulary-declared-outside-the-macro-is-uncensused.md`.

That is a residue named rather than a hole left quiet, and it is the
third time today a unit has correctly refused to close a class it could
only half-reach.

### The short delta round is dispatched, narrow

Subject: the macro and the roster only — every awkward variant spelling
(`cfg_attr`, doc-comment-plus-attribute, raw identifiers, discriminants,
named and tuple payloads), the single-invocation compile error, the
bijection's second direction, and whether the fence's condition held —
that what landed in DOCM's file really is a derived constant and doc
prose, with no derive, variant or payload changed under the macro.
"Nothing further" is stated as a complete answer.

## 2026-09-13 — the delta on the macro: no MAJOR, and a live instance of the class the disclosure only described

**0 MAJOR, 2 MINOR, 7 style.** The round earned itself on two findings
and confirmed the mechanism outright on the claim that mattered most.

### Claim 1 holds, and the way it was settled is the point

The attribute class is closed **by construction**, verified two ways:
the macro extracted verbatim and compiled against **eleven** awkward
spellings — `cfg`, `cfg_attr`, doc-comment-plus-attribute, deprecated,
raw identifier, tuple and named payloads — with every name present and
correctly spelled, **including `#[doc(hidden)] Chord`, the exact variant
the text walk dropped**; and again in-tree on two real `ProgramStep`
variants, where the green was shown to be **two-sided rather than
vacuous** (had `ALL_NAMES` dropped either name, the other direction of
the bijection would have fired). That is the standard this program has
converged on: not "the test passes" but "the test would have failed."

The fence held too, measured rather than asserted: with doc lines
stripped, the only things that landed in DOCM's file are the
`macro_rules!` and the invocation wrapper — no variant, payload, derive,
visibility or behaviour changed.

### MINOR 5: a disclosed class with an unswept live instance, four lines away

`LoopProgram` is declared with a plain `pub enum` **four lines below the
invocation's closing brace**, and `LoopProgram::resolve` is a **fourth
construct hop of exactly the shape the disclosure describes** — it
matches the document vocabulary and constructs `Step::Circle` /
`Step::CircleSplit`. A fourth variant of it laundering into an existing
kernel form leaves every clause, the roster and the census green, and
`corpus_vocabulary` explicitly declines to witness it.

The disclosure and the DOCM row state the class **faithfully and
entirely hypothetically** — *"or whatever a fourth one's would be"* —
while the instance sits in the same file. That is the class-not-instance
rule in its exact form, and it is the shape this program keeps meeting:
**the hypothetical is easier to write than the sweep, and reads as
completeness.**

### MINOR 2: the compile error is per MODULE, and that is the claim that carries everything

*"A second invocation does not compile"* is what makes *declared through
the macro* a complete list rather than a convention. `E0428` is scoped to
one module's value namespace — rustc's own note says so — and a second
invocation in a child module compiles clean, producing a second
`DOCUMENT_VOCABULARIES` the census never reads. Verified in-tree.
Reachability is low today; the claim still has to be exact, in all three
places it is written.

### S1: the macro traded a silent failure for an unformatted one

rustfmt does not format macro-invocation bodies, so **~185 lines of the
crate's central payload type are now outside the formatter**, with no
gate that would ever notice drift — already visible in the enums sitting
at column 0.

Adjudicated: **the trade still favours the macro.** It closes a *silent*
class by construction, and formatting drift is *visible* to any reader,
which is the whole difference this program has been paying for all day.
But the cost is real, was not flagged by the lane, and is plausibly a
class — any `macro_rules!`-wrapped declaration block in this tree has the
same property and nobody has swept for them. Disclosure at the site and a
filed row, not silence.

### And the trap, one more time, in the fix for the trap

**S6**: two hand-rolled bidirectional set differences, fifteen lines
apart, with near-parallel messages. The reviewer's sentence is the one to
keep — *a PR whose subject is removing a hand-kept second list added a
second hand-rolled copy of its own comparison.*

This is the last fix pass on the unit; the findings are repairs rather
than a new mechanism, so no further round follows it.

## 2026-09-13 — PR 2501 MERGED (`8a546fabd`): six units, twenty-four rows closed

The census unit lands after a full review, a delta round and three fix
passes — the most rounds any unit on this program has taken, and every
round found something the previous one could not have.

### `LoopProgram` was a fourth document vocabulary, and the evidence was the hop

Not the shape. `LoopProgram::resolve` matches the document enum and
constructs `Step::Circle`/`Step::CircleSplit` — a construct hop of
exactly the disclosed form — and a fourth carrier form laundering into
`Step::Circle` now reds the right clause and only it: *"1 of the 4
document vocabularies are short."* All three of its variants were
already in `corpus()`, so the witness cost the **decision**, not the
code, which is the tell that it always belonged.

**The membership test is now written at the site** — *does a variant
launder into an existing kernel form at a construct hop* — and one
clause disposes of the other plain enums rather than leaving a reader to
wonder: `ProgramRefusal` and `RecordedProgramError` fail it, having no
hop that builds a kernel form out of them.

### The claim that carried the design is now stated exactly

*"A second invocation does not compile"* became *"per module"*, with the
reason: `E0428` is scoped to one module's value namespace. The doc now
says the list is complete **because `program.rs` has no child modules**,
not because the macro forbids one. That is the difference between a
guarantee and a circumstance, and the file now says which it has.

### The rustfmt cost, disclosed and filed as a class

~185 lines of the crate's central payload type sit outside the
formatter's reach, because rustfmt does not format macro-invocation
bodies. Adjudicated as worth paying — a **silent** failure class was
traded for a **visible** one — but disclosed at the site and filed as
`work/ciw/rustfmt-does-not-reach-a-macro-wrapped-declaration-block.md`,
with `profile`'s three macros named as the obvious unswept neighbours.
A cost nobody flagged is a cost nobody will remember paying.

### The sharpest style finding got the same repair as the rest of the PR

*"A PR whose subject is removing a hand-kept second list added a second
hand-rolled copy of its own comparison."* The two bidirectional set
differences are now one `set_difference` door called twice. The trap
fired inside the fix for the trap, was named by a reviewer who did not
write it, and was closed with the move the whole unit is about.

### What this unit cost, and what it bought

Three rows that all read as tidying. Six rounds. What it actually found:
a census that reported **green** whenever a variant carried an
attribute; a blind spot covering **two of three** vocabularies while the
file's own contract claimed all of them; a **fourth** vocabulary nobody
had counted; an anchor whose failure mode was silent, replaced by one
that fails at compile time; and two empty escape hatches in a file whose
own words say *an empty escape hatch is a hatch that will be used.*

**The posture note for the record**: `plan.md` put this unit under a full
review for one sentence — *a census with a hole reports green* — against
three rows rated as prose fixes. That call was right, and the margin was
not small.

## 2026-09-13 — the third block: the entity-kind door, and two refusals that do not name their subject

Six units merged, twenty-four rows closed, nothing in flight — so two
lanes go out. Fifteen rows remain; these are the two coherent subjects
among them, and the rest of the slate is deliberately left for the block
after, because two of those rows are about the **macro layer** that PR
2501 just changed and their dust should settle first.

### `wire/entity-door` — the same job the value door had, one kind up

The crate answers *"read a thing, test its kind, refuse"* in **six**
spellings, and the field names are the census: `expected`/`found`,
`name`/`found` (three error kinds), `verb`/`found`, `wanted`/`found`,
`expected`/`found` again in `stackup.rs`, and the recipe road already
filed on DOCM. Three of them are **the same five lines in one file**,
sharing their *resolution* door already and sharing neither the kind test
nor the refusal after it.

PR 2480 did exactly this for **value** kinds a day ago and took 17
construction sites to 1. This unit is that job for **entity** kinds, and
the brief hands it 2480's shape *and its two review rounds' findings*,
because those were expensive: the door computes `found:` itself; a set
equality needs a non-emptiness assertion on its own derived set; assert
the rule and not a proxy for it; and **project a guard's subject rather
than typing a roster** — three times in two units the roster was the
hole.

One constraint is the interesting one: **three error kinds must survive
with their own identities.** `ShellOpenKind` carries a `name`,
`BlendSelectionKind` also a `verb`. A door that flattens three refusals
into one has changed behaviour rather than unified a spelling, and the
brief says to label that a half-fix if it cannot be avoided.

Also carried: PR 2480's hit list disposed of `Selected::faces` as *"one
enum, two arms, no duplication yet"*, which is **false of the class as it
stands and is retracted on the row**. Repeating a retracted disposition
is the cheapest mistake available here, so the brief names it.

### `wire/refusal-subject` — the third chase, and a refusal with no id

`emit_topo`'s `chase()` is the **third** of three bounded lineage walks.
PR 2474 made the other two refuse on a spent budget — a budget bounded by
the arena means the walk revisited a key, and a revisit is a corrupt
record — and did not sweep this one. On exhaustion it **falls out of the
loop and returns the key it happened to be holding**, which becomes a
group key and is handed to `upstream_name`. So a cycling fragment map
does not refuse: **it names faces after the wrong root.** For a naming
kernel that is the worst of the three outcomes, because a wrong name
beats a refusal only in the sense that nobody notices.

It travels with `frame-direction-refusal-lands-on-the-profile-without-naming-the-frame`
for a reason that is about cost rather than shape: **both decisions run
through `crates/pncad-py`'s error vocabulary**, where a new locator is
actually paid for — the chase needs a `FaceKey` where PR 2474 built an
`EdgeKey`, so the refusal it made does not fit and a new
`naming_error_tag` word is new public Python surface. A taker who reads
one should price the other at the same time.

**The sweep obligation is the sharp part.** Q4 says an invariant a bugfix
establishes sweeps its siblings *in the same PR*, and PR 2474's sweep
**missed this very function because it was keyed to one spelling**. So
the brief asks for a grep by *shape* — a bounded walk over a map that
falls out of its loop and returns its cursor — with the hit list and the
pattern's stated blind spot.

And the honesty requirement, which PR 2474 met and is the standard here:
its chases are **unreachable from `editor-core`** and it said so **at the
claim site**, not only in a PR body. If the same holds for this one, that
sentence goes where a reader of the refusal will find it.

### Posture

`entity-door` gets a **light style review** with the assertion sweep as
its one hard obligation — it is a structural unification of three
copies, and three user-visible refusals move. `refusal-subject` gets a
**full review**: it turns a silently wrong answer into a refusal, on a
naming path, and adds public Python surface to do it.

## 2026-09-13 — the entity door returned, and it proved its text claim instead of asserting it

PR 2517, full matrix green (12 `test (…)`, 5 `k-lint`, 0 in flight, 0
non-success — verified here). To a light style review.

### The technique that made the difference

The unit's central risk was the same as PR 2480's: **three user-visible
refusals move through a new door.** Rather than claim byte-identity, the
lane **compiled its new suite's four document rows, unchanged, on a
worktree at `origin/main`** and showed all four pass there. Seven
strings, including both `MeasureSelectionKind` arms whose `Display`
expression changed.

That is PR 2480's probe technique — render on both trees and compare —
applied **unprompted**, and it is now the third unit to settle a
text-preservation claim with an instrument rather than an argument. The
review is told to re-take it, and specifically to check *what had to be
removed to make it compile*, because that is where such a comparison
quietly weakens.

### Two dispositions argued rather than converted, which is the harder answer

- `names::interrogate`'s `kind_mismatch` is **not an instance**: already
  one home for four call sites, already computes `found.kind()` itself,
  a different error type, and it **keeps a fact this door has no room
  for** (`WholeBody`).
- `stackup.rs` is **not this class at all**: `ResultArm`'s two words are
  result arms of paired evaluations, not entity kinds.

Both were on the row's own census as spellings of one question. Saying
"this one is not the defect, and here is the fact that distinguishes it"
is harder than converting it, and it is the answer a hit list is for.

### A seventh spelling, and the blind spot that hid it

`assembly.rs`'s `RefusedRef::NotAFace` computes its own word, so there is
no correctness defect — the residue is a **third field name for one
answer**. Filed on DOCM's slate.

**And the lane named the reason its own census would miss an eighth**:
the census walks `eval/mod.rs`'s `NodeErrorKind` body, so it **cannot see
past that enum** — which is exactly what let the seventh hide from the
row's census. Stated in the suite's own doc, in its own words. The review
is asked for a judgement rather than a finding: is that an honestly
scoped guard, or does it re-arm the defect one level out?

### Two smaller things worth keeping

`EntityKey::vertex` was **deliberately not added** — no consumer — which
is this program's own `Frame::linear` lesson applied by a lane that did
not live through it. And the census **projects its subject**: set A
walked out of the enum's body, set B what the file constructs, equated
both ways with a non-emptiness assertion whose message names its own
failure mode, and a mutant aimed at each limb.

### An operational fact worth the log

The lane's first push was **rejected by the pre-push hook as
unformatted**, and its observation is the transferable part: **`cargo
test` is happy with formatting the hook rejects.** A lane that runs only
tests locally will meet the hook for the first time at push, which is the
worst moment to learn it. Its second push also **cancelled its own first
CI run** — the green run reported is on the final head, and the review is
told to confirm the `head_sha` rather than the run's colour.

## 2026-09-13 — 2517's review: mergeable, and the sharpest sentence of the session

**0 MAJOR, 2 MINOR, 11 style.** Verdict mergeable, and the round produced
a general statement worth more than any of its individual findings.

### The proof technique was re-taken, properly

The reviewer built a detached worktree at `origin/main`, installed the
suite's first 231 lines **verbatim** (`diff` IDENTICAL), mounted it and
ran it: all four document rows pass on main, seven rendered strings
including both arms whose `Display` expression changed. It then checked
the thing that could have undermined the claim — **what had to be removed
to make it compile** — and found it was only the source census, which is
necessarily false on a main with no door and contributes nothing to the
rendered-string claim.

That is now the strongest evidence any text-preserving refactor on this
program has produced, and the technique has crossed three units without
being re-invented.

### The sentence

> **A census that finds sites by the spelling it is normalising can only
> ever find the ones that already comply.**

That is the reviewer's, and it explains a pattern this program has been
circling since PR 2376: every vocabulary census built here keys on the
canonical spelling, so it is structurally blind to exactly the sites the
unit exists to find. The seventh spelling hid from the row's census that
way; the eighth and ninth hid from this unit's the same way.

### Four of nine, and the sweep had already printed them

The review found an **eighth** site — `DeclareUnsupportedPair`, inside
the enum this unit's census walks, in the door's own file, reporting
**the declared kind off the `StableName` rather than the resolved key**,
which is precisely what `entity`'s doc forbids a caller from doing,
thirty lines from where that doc says it. Safe today only because
`insert_ref` rejects a name whose kind disagrees with its key — an
invariant, not a construction.

And a **ninth** on the same road as the one converted, reached by the
same verb, naming no found kind at all and conflating *"not a face"* with
*"a face of the wrong body"*.

**The damning part is not that they were missed.** `rg 'EntityKey::'`
returns 156 lines and **both are in that output**. The sweep found them
and stopped before triaging them. *"What would find an eighth is the
sweep they already ran, triaged to the end."*

### The trap, in the guard, in a PR whose thesis is the trap

`line` and `boundary_before` are **byte-identical** to the operand door
suite's, and `door()` is the same function modulo two sentinel strings —
while `test_utils::source` is the declared one home and already hosts
three siblings, with a module doc arguing guards must not each write
their own. **A PR whose thesis is "three copies of five lines get one
home" shipped two more copies of two helpers.**

### And a vacuity that is the *partial* form of the one we knew about

`built` is populated by iterating **over `declared`**, so a carrier that
falls out of `declared` falls out of `built` too and the equality passes
at the lower count. Measured: re-spelling one field's type path — using
an import already in that file — left the suite **6/6 green with that
carrier covered by nothing**. The non-emptiness assertion does not reach
it, because that guards the **total** case, which is exactly what the
unit's own mutant exercised.

We have now met this vacuity three times in three shapes: both sides
empty, a scan that went to zero, and now **one member silently leaving
both sides at once**. The rule needs its third clause: *derive the two
sides independently, or the equality is only a statement about one of
them.*

## 2026-09-13 — the refusal-subject unit returned, and its best result is a measurement against itself

PR 2518, full matrix green (12 `test (…)`, 5 `k-lint`, 0 in flight, 0
non-success — verified here). To a **full review**: it turns a silently
wrong answer into a refusal on a naming path and adds public Python
surface to do it.

### M6: the lane measured that its own fix is unguarded, and said so

It reverted `chase` to the old silent fallthrough and ran everything:
**115 lib + 1231 integration, 0 failed — GREEN.** That is the evidence
behind its *"unguardable, and here is why"* note, and it is an honest
negative rather than a convenient one: the easy move was to assert the
fix mattered and never check.

**The note is at both claim sites**, not only in the PR body, with the
argument for why the refusal is unreachable *and* a pointer to the
cycling route that is real — a graft copying records with source keys,
which has fired the sibling's cycle arm on real assembly products — plus
why that route aliases **edge** records rather than face-fragment rows.
The review is told to try to falsify it, because *unreachable* and
*unreached* are different claims and only one of them is a property of
the code.

### The public-surface decisions are the interesting half

**Deliverable 2 adds no new Python word.** The new variant's tag arm
**delegates**, so the tag stays `degenerate_direction` /
`non_finite_direction` / `underflowed_direction` / `escalated` and a
caller matching the old word keeps matching. Additive at the kernel,
invisible at the binding — and the reason the lane could take that route
is that it carried the whole `DirectionRefusal` rather than flattening it
into loose fields, which is the shape PR 2435's delta round forced after
a flattened carry could not call its own door.

**And it declined to generalise `SplitLineageCycle`** for a reason worth
keeping: generalising would **retire `split_lineage_cycle`, a word
already on the wire**, and *adding is additive where renaming breaks
callers.* It also rejected the row's own other option — an id field on
all four direction variants — on a concrete ground rather than taste:
the fourth, `Escalated`, is the general escalation variant reached from
every `unit()` caller, so a frame id would sit on refusals that have no
frame.

Both are surface decisions argued from what the surface costs, which is
the standard this program reached the hard way on `Frame::linear`.

### The sweep found a sibling in another program's ground, and priced it

`KeyView::live_vertex`/`live_face` in `topo/boolean` answer `None` on a
spent budget — which is **also** their answer for a chain that simply
ends — so a cycle reads as *"genuinely consumed"* and a **declared
contact is dropped**. Milder than this unit's subject, same family,
filed on BOOL because the cost lands in `BooleanError`'s vocabulary and
that makes it their decision.

**Five blind spots stated**, with two named as the ones that could hide a
fifth instance: a budget spelled from something other than the map's
`len()`, and a chase written as recursion rather than a loop. And a
disclosed scope gap — `sweep/`, `geom-brep/` and `verbs/` were not
swept, because the brief said two crates. A gap the brief caused is the
orchestrator's to own, not the lane's.

### Two counts corrected against the row, again

The row said `descend_face` had two call sites; it has four arms, and the
mechanical total is **seven**, not four. That is the fourth row this
session whose own numbers were wrong and were corrected by the lane that
read them — the standing lesson holds: **a figure in a row is one nobody
re-takes until someone has to use it.**

## 2026-09-13 — 2518's review: the reviewer wrote the guard the lane called impossible, and it reaches back into merged work

**1 MAJOR, 1 MINOR, 2 NOTE, 7 style.** The strongest single finding of
the session, and the only one so far that indicts a unit already merged.

### The finding

The unit shipped *"unguardable, and here is why"* at two claim sites, on
the argument that no door reachable from `editor-core` can build a
cycling face-fragment map. **`topo::BooleanNaming` is a public struct
with public fields, `name_boolean` is `pub(crate)`, and `emit_topo.rs`'s
own test module was already minting synthetic `face_fragments_a` rows**
to guard two other emission refusals. The technique was idiomatic in the
file the lane was editing.

The reviewer wrote the guard — **~35 lines, at that door** — and it does
better than pass. Under the M6 mutant it **exhibits the defect**:
`name_boolean` returns `Ok`, publishes a total table, and **the two cap
faces are named after each other**, `built.top` taking bottom's operand
name and `built.bottom` taking top's. The row's claim, made visible
rather than argued.

### It is a class, and it reaches PR 2474, which this orchestrator merged

The identical sentence sits on `chase_edge_to_table` and `chase_b` from
**PR 2474**, and on `NamingError::SplitLineage`. Those shipped two units
ago on the same false reasoning, **and I accepted it at merge** — the
report said the refusal was unreachable and pinned by the type system,
the note was at the claim site as Q6 requires, and I took the argument
because it was honestly made and well placed. It was still wrong.

The lesson is not "distrust the lane". It is that **Q6's discharge is
available only when the guard genuinely cannot be built, and nobody had
tested that**. An "unguardable" note is a claim about a call graph, and
the strongest evidence against it was in the same file's own tests.
Directed: repair every copy, guard the siblings or argue per site, and
**say in the PR that this corrects a claim PR 2474 merged** — an error
found in merged work is worth more recorded than quietly overwritten.

### What held, and one of them is a technique worth keeping

**The tag-compatibility claim was executed, not read**: all four arms,
before → after, unchanged; inner tags `None` on both sides; the 85
`pncad-py` rows green. A backward-compatibility claim about a public
binding settled by running it.

**And the reviewer turned another program's row into a repro.** The BOOL
row's cycling-descendant claim had been read off the code path; the
reviewer built it — two mutually-referring dead face keys plus a declared
v-on-f contact — and measured **cycle → 0 records, dead end → 0 records,
indistinguishable**. BOOL inherits a repro instead of an argument.

### The style finding that converged with the MAJOR

**S2**: 35 doc-lines on a 4-line variant, 25 on a 9-line function, the
argument restated a third time in the PR body — **and that is the
justification that turned out to be wrong.** `reviewer-style-lane.md`
§1's rule is that unusual justification length is *mild evidence* a thing
is worth flagging. Here it pointed straight at the false claim, which is
the second time today the length tell has paid.

## 2026-09-13 — 2517's fix pass, and a lane that defeated its own first repair

Green on `ebf6bc46f` (12 `test (…)`, 5 `k-lint`, 0 in flight, verified
here). A short delta round is dispatched, and **the lane asked for it
itself** — which is the part worth recording.

### The repair that did not work, found by the lane that wrote it

MINOR 2 was "the sentinel comment states a rule no assertion checks: the
construction's **position**". The lane asserted position — the
construction must sit inside a door call's arguments — and then **planted
the reviewer's exact defeating shape** (`let found = /* own answer */;`
above the call, a closure ignoring the parameter) and watched the suite
**stay green**. Its first fix was inadequate and it found that out by
attacking it rather than by shipping it.

The rule it settled on is the one the door actually rests on: the refusal
must be the **whole body of a closure whose parameters bind `found`** —
a road that spells `found` without being handed it has answered the
door's question itself. That sentence is now the failure message.

### The partial vacuity, closed by independent derivation

`declared` now matches by the type's **tail** rather than one path
spelling — which is what R1's mutation exploited, since the enum already
mixes qualified and imported forms — and `built` is derived from **door
call argument ranges**, with the call sites themselves derived from the
`fn`s in the sentinel region. **Neither consults the other.**

And the lane did not stop at "the mutation now reds", which proves
nothing on its own: it built an instrument that **prints the derived
set** to show `ShellOpenKind` genuinely stays in `declared` under the
re-spelling, rather than both sides dropping it in step. That is the
difference between a test that passes and a test that is known to be
measuring something.

### The sweep, triaged to the end, and labelled partial

Three more sites dispositioned and **each filed on the slate that owns
it** — `DeclareUnsupportedPair` (WIRE, not converted: it tests a pair and
carries a `cross_operand` bit that `entity`'s single-key shape does not
fit, and its safety rests on `insert_ref`'s invariant rather than on
construction), `clearance.rs`'s `NotAFace` (SHELL), `assembly.rs`
(DOCM, already filed).

**Labelled a partial fix on the row and in the PR body: four sites of at
least nine.** And the reviewer's general sentence is now the suite's
**first blind-spot line**, with the known members named rather than left
implied — *a census that finds its sites by the spelling it is
normalising can only ever find the ones that already comply.*

### S8 was live, not hypothetical

The census was scanning `wire.rs`'s own `#[cfg(test)]` module, which the
suite's doc said it excluded — and **that module already builds
`DeclareUnsupportedPair` by hand.** A future inline test constructing one
of the four carriers would have reddened the equality for a reason the
message did not describe.

### Why the round runs

The lane disclosed, unprompted, that **the derivation changed in two ways
no review has seen**, and recommended a short round over a blind merge.
That is the standing rule applied by the party it costs, and it is the
second time today a lane has asked for the round that would scrutinise
its own work.

## 2026-09-13 — the delta on 2517: the closure rule is defeated, and the defeat is structural

**1 MAJOR, 3 MINOR, 7 style.** The round was dispatched because the lane
disclosed its derivation had changed; it came back having broken the half
the lane was proudest of.

### The defeat

`refuse_params` asks *"is the text immediately before this construction a
closing `|`"*. It never asks **which** closure, nor that the closure is
the door's `refuse` argument. So a road binds `found` on an **inner**
closure and feeds it its own answer — planted in-tree on the measure
road, **6 passed, 0 failed**:

```rust
|_door_found| (|found| NodeErrorKind::MeasureSelectionKind { verb, found })(
    match self.key { /* a hand-written match over EntityKey */ })
```

The door's answer is discarded and a hand-written match over `EntityKey`
is a second home for *"what an entity IS"* — the exact shape this unit
removed. **And the document rows cannot see it, because the road's answer
happens to be correct.** On the measure road, which the module header
says already spelled two correct words by hand. The regression the unit
exists to prevent, restored under a green suite.

Two further spellings, both compiling and both delivering the road's
word: a one-line IIFE, and a `let`-bound closure **inside the argument
list** — which is the shape the guard's own doc says cannot exist, and
which `clippy::redundant_closure_call` does not reach either.

### Why this is the same lesson a third time

All three defeats share one root: **the road can compute an `EntityKind`
and hand it over.** The textual rule is trying to enforce with a parser
what the door's *shape* could enforce outright.

This program has now met that exact trade three times in three days — the
census anchor that went green on an attribute until a macro projected it
from the declaring tokens; the roster typed on the test side until it was
projected from the invocation; and now a closure-form check standing in
for a type. The rule that keeps emerging: **stop policing the spelling,
make the wrong thing unspellable.**

Directed accordingly: find out whether `refuse` can take a value only
`entity` can mint, so the road passes a kind through rather than choosing
one — the three error kinds keep their identities because the road still
picks the constructor, it just cannot pick the kind. If that works the
guard shrinks to the equality and the parser disappears. **If it does not
work, the fallback is honesty**: delete the sentence asserting an
invariant nothing enforces, and put this blind spot in the list — which
is the reviewer's sharpest procedural point, that *the "what these rows
cannot see" list enumerates four blind spots and not this one, incomplete
in exactly the direction the round was dispatched to test.*

### And the adversarial question answered yes, in the same commit that closed it

**The commit that extracted `line`/`boundary_before` into
`test_utils::source` — on the stated grounds that two censuses had
written them byte-identically — minted two fresh byte-identical fragments
between those same two censuses.** `doors`, twelve of fifteen lines
identical; `door_calls`'s inner loop, identical modulo a `&`. Neither
declares itself in prose, so the `verbatim|ported from` sweep finds
nothing. One of the copies also **carried a dead guard** — a branch that
cannot fire here because both doors are generic — which is live in the
census it was copied from.

### Two censuses on one file with opposite rules, each documented as right

`built_in` **requires** the `NodeErrorKind::` qualifier; the operand
census **requires the token bare**, and argues at its own site that this
is what stops a `use` walking past it. Same file, same class of
construction, contradictory receipts — *"one question, many spellings"*
reappearing inside the pair of guards that pin it.

### What held

The independent derivation, verified the right way: the carrier stays in
`declared` under the re-spelling **and** the coverage was shown real
rather than both-sides-in-step, by moving a road's refusal into a helper
and watching `built` drop to 3 against `declared`'s 4. The tail check's
own hole fails **red**. P1 reds with an accurate message. Claim 4 holds
on both limbs of both sides.

## 2026-09-13 — PR 2518 MERGED (`56f18092e`): seven units, twenty-six rows closed

The unit that turned a silently wrong answer into a refusal, after a
full review that found the "unguardable" note false and wrote the guard
itself.

### What the fix pass did better than comply

**It re-measured the exhibit rather than quoting the review.** With the
refusal removed, `name_boolean` returns `Ok` with a **total 27-row
table** in which the two cap faces carry **each other's names** —
`top = FaceKey(1v1)` taking `FromA([Cap(Start)])` and `bottom` taking
`FromA([Cap(End)])`. Nothing missing, nothing refusing, the document
wrong about which face is which. Observed, then written on the row as an
observation.

**And it found the family's actual dividing line, which is writer
access.** `chase_b`'s identical note was wrong too and is now guarded —
one synthetic `graft_edges` row closes a loop provenance records alone
cannot, because that walk advances in two steps and only the first is the
caller's data. `chase_edge_to_table` genuinely cannot be reached, and the
argument is now **checkable rather than a survey**: it advances only on
`Body::edge_provenance`, `pub(crate)` to `topo`, whose one writer records
a parent on a child it has just minted — so every chain is **strictly
decreasing in age**.

`grep Unguardable` over `editor-core/src` now returns **exactly one
hit**, at the site that earns it. The PR body says plainly that this
corrects text PR 2474 merged.

**The rule that generalises**: *a bounded walk is guardable exactly when
something outside the crate can write a step of it.* That replaces three
copies of a survey with one property.

### One row went further than it was asked

`FrameDirection` names **both** nodes, not one — because
`profile_plane_f64` is also called from `section_of`, where the error
lands on the loft or sweep and **neither node in the sentence is the one
it attaches to**. The row asked for a frame id; the lane found the
three-node case and carried the profile too.

### The red on the state-sync commit, and why it did not block

My docs-only state-sync run went **red** on
`review_gui1_r1::random_integer_rays_match_the_exact_oracle` — the
**anti-vacuity guard**, not an oracle mismatch: *"no draw hit the cube"*
at `CAD_FUZZ_SEED=0x1a9e0f26198e881b CAD_FUZZ_EFFORT=1`.

Established rather than assumed, in this order: the reddening commit was
**two markdown files**; the previous run on the same code was green; and
**re-running the failed job on identical code passed.** The variable is
the seed, not the tree.

**And a row already existed** —
`work/docm/pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1.md`,
opened 2026-09-08 from S-MESH's landing run on a *different* seed. So the
occurrence went onto that row as a **second instance** rather than into a
duplicate, which is the "grep the owner's directory first" rule paying
for itself — I was one command from filing a second copy.

Worth stating on the row and here: **one bad seed reads as bad luck, two
read as a distribution.** And the tree already carries the convention the
fix would land on — `test-utils/src/vacuity.rs` says an anti-vacuity
claim is *"stated against the floor of the dial, never"*.

The PR carries the annotation the standing rule requires before merging
over an inherited red.

### An operational note against myself

My PR comment came back through the away channel as an event, because I
did not **lead it with the `(WIRE orchestrator)` tag** — which is both
the thread subscription and the self-suppression key. The etiquette is in
`memories/orchestration-model.md` and I simply did not follow it.

## 2026-09-13 — the entity door took the structural route, and the defeats stopped compiling

The lane was asked whether `refuse` could take something a road cannot
manufacture. **It can, and it does.**

```rust
pub mod entity_door {
    pub struct Found(EntityKind);        // field private to this module
    pub(crate) fn entity<R>(key, read, refuse: impl FnOnce(Found) -> NodeErrorKind)
        -> Result<R, NodeErrorKind> { read(key).ok_or_else(|| refuse(Found(key.kind()))) }
}
```

The roads are **unchanged** — `|name, found| NodeErrorKind::ShellOpenKind
{ name, found }` — and cannot mint the token, because they are not
descendants of that module. The road still chooses the constructor; it
cannot choose the kind.

**All three of R2's defeats, plus the plainest fourth, now fail
`cargo build`** with `error[E0308]: expected Found, found EntityKind`,
and rustc's own note on every one is *"try wrapping the expression in
`eval::entity_door::Found` (its field is private…)"*. Not "the census
reds" — the program does not compile.

**And the whole textual guard is deleted**: `refuse_params`, the form
check, the `.kind()`-read-once row, the sentinels, `doors`,
`door_calls`. The guard is now the equality plus a form the compiler
enforces.

### This is the third time the same move has paid, and now it has a name

The census anchor that went green on an attribute, until a macro
projected `ALL_NAMES` from the declaring tokens. The roster typed on the
test side, until it was projected from the invocation. And now a closure
shape policed by a parser, until a private field made the wrong call
unspellable. **Stop policing the spelling; make the wrong thing
unspellable** — and each time, the guard that replaced the policing was
*smaller* than the policing it replaced.

### The lane made a mistake inside the round and reported it

Its first census rewrite asserted *"a `found` field but no `Found`"* and
**redded on `WrongOperand`** — the *value* door's answer, which is
correct and not its business. The claim is now that every refusal
answering *what was it instead* is built in `eval/wire.rs` exactly once
**and by one of the two doors**, with the doors told apart by declared
field **type** or by sitting inside the operand sentinels, **never by
name**. A third `found:` vocabulary reds, and so does a token type
re-spelled out of recognition.

### The adversarial answer was deletion

R2 found that the commit extracting `line`/`boundary_before` had minted
two byte-identical fragments **between the same two censuses**, one
carrying a branch that could not fire. Both went out with the census that
held them, and the lane's own sentence is the one to keep: **hoisting two
helpers does not make a file free of the defect it hoisted them for.**

`S4` swept into two more files, with one non-instance correctly
identified (`pncad/tests/all.rs` counts a whole text, not a prefix) and
one remaining instance **labelled rather than swept** —
`shell_tolerance_chain.rs` hand-rolls `sentinel_region`, a different
shared function and not this PR's.

### A public-surface change, priced

Four `NodeErrorKind` variants change a field's type. `NodeErrorKind`
derives only `Debug` — no serde, no `PartialEq` — so nothing persisted or
compared moved and no rendered string changed. One hand-minted refusal in
`pncad-py`'s tests **was deleted because it can no longer be minted**,
and the real path already reaches the same tag; that module's doc already
made this argument for a sibling and now makes it for both.

### The round runs because the lane said it should

*"The door's shape changed, so this is a mechanism no review has seen"* —
the lane's own words, against its own work, matching the condition I set.
Third time today a lane has asked for the scrutiny that costs it.

## 2026-09-14 — the token door round: the forgery moved down a level, and it corrects MY lesson

**1 MAJOR, 2 NOTE, 5 style.** The finding is the deepest of the session
and it lands against a sentence **this log wrote three times**.

### From outside the crate the token is airtight, and that was verified properly

Eight attack shapes, eight compile errors — `Found(k)` E0423,
`Found::default()` E0599, struct-update **E0451 checked in isolation so
other errors could not mask it**, pattern-match E0532, a foreign `From`
E0117, a child module of the caller E0423, the door itself E0603, and
`transmute` refused by `-F unsafe-code`. R2's three defeats plus a bare
construction all fail to **compile**.

### And inside the crate, the road can still choose the kind

`entity` computes the token from a key the **caller supplies** —
`read(key).ok_or_else(|| refuse(Found(key.kind())))` — and `EntityKey`'s
variants are `pub` with slotmap payloads that have `Default`. So a road
hands the door a synthetic key of whatever kind it wants plus a `read`
that returns `None`, and **the door mints the lie for it.**

Compiled, in the sharp form: `resolve_open_faces` rewritten to resolve
the **real** key, project through `ent.key.face()` so the success path is
byte-identical, and hand the door a **forged `EntityKey::Vertex`**. It
compiles, and **the rewritten census passes on it.** Only the byte-exact
document rows catch it — which is exactly the textual-guard dependency
the token was adopted to replace. And the deleted `.kind()`-read-once row
was the thing that used to cover it.

### The correction, and it is to my own sentence

I wrote **"stop policing the spelling, make the wrong thing
unspellable"** into this log three times today, as the lesson the census
anchor, the projected roster and this door all taught. The reviewer's
closing line is the correction:

> what the token makes unforgeable is the **word**, while the **key the
> word is computed from** is still the road's to pass. That is a
> spelling **moved one level down**, not eliminated.

The rule survives and is narrower than I stated it: **making a thing
unspellable moves the forgery to whatever the unspellable thing is
computed FROM, and the move is only complete when that input is not the
caller's to choose either.** A guard that looks total because its own
type is airtight is the most expensive kind of partial guard, because it
retires the guard that covered the rest — which is precisely what
happened here.

Directed: narrow the claim to the truth at the site, **then** decide what
covers the key path — the realistic defect is not an adversarial forgery
but a road that resolved one entity and handed the door another,
yielding a confidently wrong refusal. What is not available is leaving
the doc claiming the gap does not exist.

### Two notes worth their own lines

**The row's own closing text is stale and it is the done-state of
record** — it still describes the `ENTITY-DOOR` sentinels that a later
commit deleted, with the type-change section appended below a paragraph
describing an arrangement that no longer exists.

**And the `sentinel_region` hand-roll in `topo` is NOT byte-identical**:
it *includes* the opening sentinel where the shared function excludes it.
That divergence is the exact drift the shared function exists to stop, so
calling it "a second instance of a different shared function" undersold
it. A copy that has already drifted is worth more than a copy that has
not.

### What the token did buy, kept rather than deleted in the narrowing

`Found` is **not** a new spelling in the sense the row means — a spelling
is a second thing that can *answer differently*, and `Found` computes
nothing, forwarding `kind`/`article`/`noun` unchanged. It made three of
the six spellings' failure modes unspellable rather than merely
unwritten, and out-of-crate the result is total. The narrowing takes back
the last mile, not the move.

### A process note: the mutex was starved and the reviewer routed around it

The main build slot was held by neighbouring lanes for ~90 minutes —
seven lanes were contending at one point, two CURVED, two MDOOR, two
TRIM and mine — and the express lane's 600 s cap cannot finish
`editor-core`'s test binary. The reviewer lifted the census **verbatim
into a standalone rig against `test-utils`** and ran its mutations in
seconds each. That is the right response to a starved mutex, and it is
worth knowing the technique exists: a source census has no dependency on
the crate it reads.

## 2026-09-14 — usage wind-down at 90% of the 5h window, and an instrument that is not there

`usage-watch.sh` warned that **this session's own account** is at 90% of
its 5h limit, resetting ~22 minutes out. The alert names my account
(resolved at session start from the agent dir), so it is mine to act on
rather than informational.

**The prescribed instrument does not exist here.**
`memories/orchestration-model.md` says not to infer from the event but to
read `<agent-dir>/events/claude/usage/events.jsonl`, whose last line
carries `rate_limits.five_hour` and `.seven_day` **together** — because a
reset on one window while the other is still full is how that rule was
learned. **There is no such file under this agent's directory**, and no
usage jsonl written in the last two hours. So the two-window check the
memory exists to make possible could not be made, and I am acting on the
alert's single number knowingly rather than believing I checked.

Worth a row on `work/meta/` when the box is not rate-limited: the memory
prescribes an instrument by path, and the path is empty for at least this
agent layout. A rule that cannot be followed is worse than one that is
merely unwritten, because it reads as having been followed.

**What I did, and the reasoning.** The one live lane — `entity-door`'s
final fix pass — **has pushed** (`13253a396`, *"carry `Found` through the
document facade"*), so its work is durable and a death is recoverable by
resume. Against that, killing it mid-pass discards the in-context
reasoning about the key-path decision, which is the substantive question
of the round. With the reset inside half an hour I let it run and stopped
spending on my own side instead: **no new dispatches, no further polling
loops, state committed now.**

The lever that would actually have cut consumption is stopping the
subagent, and I am choosing not to pull it — stated plainly so the choice
is visible rather than implied by silence.

## 2026-09-14 — PR 2517 MERGED (`c6a723a82`): eight units, twenty-seven rows closed

The entity door lands after **three review rounds**, the most any unit on
this program has taken, and every round found what the previous could not
have.

### The arc, because it is the unit's real result

A textual guard over the roads' shape → **defeated three ways**, all with
the suite green because the road's answer happened to be *correct* → a
token whose field is private to the door, so all four defeats **stop
compiling** and out-of-crate the result is total (eight shapes, eight
compile errors) → **and then the last mile**: the token makes the *word*
unforgeable while the *key it is read off* was still the caller's, proven
by a compiled attack with a byte-identical success path that the census
passed on.

Closed in two honest moves rather than one overstatement:

- **the claim narrowed at the site, and measured**: `EntityKey` is
  constructed in about **150 places** crate-wide, `EntityRef` in 30,
  nearly all legitimate naming-layer mints — so full closure is a
  naming-layer redesign, not this unit. The narrower `Resolved`-token
  option fails today on module privacy, and the row says so, so a taker
  does not re-derive it.
- **`read` became `fn(EntityKey) -> Option<R>`, not a closure.** Verified
  here by reading the door rather than on report: a `fn` **cannot
  capture**, and both `read(key)` and `Found(key.kind())` take the same
  binding — so a road that substitutes a key substitutes it for its own
  success path too and **stops working**. It does not make the key
  unforgeable; it makes a forgery **self-defeating instead of
  invisible**, which is exactly what the compiled attack relied on. That
  is a language guarantee, not a design claim, which is why no fourth
  round ran.

### Two instrument lessons against myself, both from my own polling

**A background waiter's completion is not evidence of the thing it
waited for.** My CI loops break on the condition *or* on exhausting their
iterations and exit 0 either way, so "task completed" reads identically
to "run concluded". I nearly merged on one. The fix is a real
until-condition whose *event* is the evidence — which is what I armed,
and which is what the harness had been pointing at.

**And the accumulation was mine too.** The entity-door lane diagnosed
itself spawning seventeen concurrent shell waiters because each timeout
moved a loop to the background and it re-issued a fresh one instead of
re-reading the runner's own output path. My pattern all session was the
same shape. The cheaper rule, stated for whoever reads this: **when a
runner or an API already records its result, re-read the record in a
short call; do not block again.**

### A re-home caught in the act

Merging main brought a conflict on
`pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1.md`: I had appended
the second-instance evidence to it in `work/docm/`, and **DOCM closed
while this unit was in review**, moving the row to `work/tint/` on the
argument that a probabilistic test guard is test-suite integrity. Both
sides were additive, so the resolution is the union with the re-home note
last, where it says the directory is the claim. My evidence survived to
its new address.

Worth noting the same sweep re-homed a row the operand-door lane filed —
`work/docm/…mate-member` is now `work/door/…` — and that lane, waking
from a killed background job, **traced it rather than reporting it
missing.** That is the tracker's own promise working: a row moves, it
does not drop, and the id is what finds it.

## Announced seam from TOPO (2026-09-14): the stamping call, with the typed-absence unit

TOPO's `geom-source-absence-conflates-four-origins` (branch
`topo/geom-source-typed-absence`) makes provenance absence say which
absence it is on the identity channel (`crates/topo/src/source.rs`).
WIRE's `crates/editor-core/src/eval/wire.rs` `stamp_minted` is read;
if the stamping door's signature moves, that one call is edited by
this seam and the PR names it. The reader that would answer Ev's
PR-2404 question is reported to WIRE, not built. Signed (TOPO
orchestrator).

## Announced seam from TOPO (2026-09-14): the edge-side kind read's document twin

TOPO's `edge-carrier-kind-has-no-readback-door` (branch
`topo/edge-carrier-kind-readback-door`) gives the edge side the shape
Ev ratified for the face side on PR 1948: `readback::edge_carrier_kind`
is the one reading of an edge's stored carrier tag and
`query::edge_carrier_kind` flattens it. The seam on WIRE's ground is
the document-layer twin: `crates/editor-core/src/names/interrogate.rs`
gains `edge_carrier_kind(ev, node, name)` beside `face_carrier_kind`,
the same node ladder, `WrongKind` for a non-edge name, the wrapped
`ReadbackError` — the delegate-and-re-export shape the face twin
already has, no new refusal arm and no signature moved. Its name rides
`names/mod.rs`'s interrogate list. `names/geompred.rs` is untouched:
`GeomPred::CurveKind` still reads through `query::edge_carrier_matches`,
which is unchanged in behaviour. Signed (TOPO implementer lane).

**Addendum (TOPO fix pass, same PR): the interrogate ladder collapses.**
Both blinded reviews named the same thing about the seam above — the
new door was the FIFTH body in `interrogate.rs` of one shape
(`entity_of` → two-arm match on `EntityKey` → `kind_mismatch`), so a
lane closing a duplication one layer down minted one a layer up. The
fix pass collapsed all five onto one private reader, `read(ev, node,
name, door)`, taking the kernel door as a `fn` pointer and the wanted
kind from a private `Denoted` trait implemented for `FaceKey`,
`EdgeKey` and `VertexKey`. `face_frame`, `face_carrier_kind`,
`edge_frame`, `edge_carrier_kind` and `vertex_position` keep their
names, signatures, docs and refusals exactly; each is now one delegate
line. Nothing else in `names/` is touched, no public item moved, and
the whole editor-core suite is green unchanged (1238 rows). The
projections are exhaustive with no wildcard arm, so a fifth entity kind
fails to compile there rather than refusing at run time. A sixth read
door is now a delegate line rather than a sixth copy, which is the
point. Signed (TOPO fix-pass lane,
`edge-carrier-kind-has-no-readback-door`).

## Reported from TOPO's fix pass (2026-09-14): `wire.rs`'s absence vocabulary, unedited

The seam announced above is discharged without a diff — `stamp_minted`'s
signature did not move, and `crates/editor-core/src/eval/wire.rs` was
read, not edited, in the unit or in its fix pass. Two facts for WIRE,
reported rather than changed, because the file is WIRE's:

- `stamp_minted`/`stamp_minted_from` and `compose_placed` still say
  "unsourced" in their docs, with no pointer to the door that now says
  which absence an unsourced description is (`topo::GeomOrigin`, read
  through `Body::surface_origin` and its siblings). The word is still
  exact — those functions speak of `GeomSource`, and
  `Body::surface_source` answers exactly what it answered before — so
  this is a vocabulary gap, not a defect: a reader of `wire.rs` cannot
  find the channel from there.
- `stamp_minted_from`'s selector (`body.surface_source(k).is_none()`)
  and `compose_placed`'s `filter_map` are byte-identical in behaviour
  after the change: `surface_source` is now the projection of
  `GeomOrigin`'s `Recipe` arm, and every other arm projects to `None`.
  A description that carries `Imported` or `Cleared` is therefore
  stamped by `stamp_minted` exactly as an unmarked one used to be; the
  stamp overwrites the mark, and over `Imported` that is lossy (see
  `work/exch/log.md`'s addendum of the same date).

The reader that would answer Ev's PR-2404 question is still WIRE's to
place; `GeomOrigin` is the door it would use.

Signed (TOPO fix-pass lane, `geom-source-absence-conflates-four-origins`).

## 2026-09-15 — orchestrator handoff, and a residue the last session disclosed but did not file

A new orchestrator takes WIRE. There was no handoff entry to read: the
previous session wound down at 90% of its 5h usage window on 2026-09-14
and the log ends with three TOPO announcements that arrived after it, so
the resting state had to be re-derived from the tree rather than
inherited. It is clean, and that is worth saying plainly.

**The resting state, measured rather than assumed.** 22 open rows, 1
parked (`axis-shaped-identity-channel`, on TOPO's and EXCH's step-1
rows, and `work.py lint` confirms neither trigger has fired), 2
deferred, 28 closed. **Nothing dispatched, nothing in review, no open
`wire/` PR, no `needs_ev: true` anywhere on the slate, lint green (0
problems).** No local `wire/` branches on this box.

**The `[ev]` channel is clear, and the clearing is the interesting
part.** PR 2555 asked whether five of this program's working rules
should be promoted into `docs/prompts/reviewer-style-lane.md`. It was
**declined** and merged with its doc edits reverted — the doc on main
carries neither insertion, which is the state the PR body promised for
a no. Ev's reading pointed past the rule it was aimed at: a scan that
returns an honest zero over a file that *must* contain its markers has
a broken premise and should refuse **at the scan**, which is a code
change rather than a paragraph. That became
`a-source-census-scan-that-matches-nothing-should-refuse-at-the-scan`
and is the better result the exchange produced. `plan.md`'s "What
eight units taught" section still said the promotion "goes out as an
`[ev]` PR"; corrected here to say it went, and what came back.

Also corrected: the slate header said twenty-seven rows closed, and the
twenty-eighth (`reviewer-discipline-owes-the-census-failure-rules`)
closed with 2555 on 2026-09-14.

### The residue that was disclosed and not scheduled — filed now

The 2026-09-14 wind-down entry names a finding and says it is *"worth a
row on `work/meta/` when the box is not rate-limited"*: the usage-alert
bullet in `memories/orchestration-model.md` prescribes the two-window
check by a path (`<agent-dir>/events/claude/usage/events.jsonl`) that
was not there for that agent. **The row was never filed**, which is
exactly the failure `work/README.md` describes — a residue disclosed
inside prose is invisible to the re-homing sweep and dies with the
directory. Filed as
`work/meta/orchestration-model-prescribes-a-usage-instrument-with-no-file.md`.

It carries a second data point taken here: on this hosted box there is
no agent-directory layout at all — no `~/.local/share/cad-work`, and a
bounded `find` for a usage `events.jsonl` returns nothing. So the rule
is unfollowable on two layouts, not one, and the fix is `memories/`,
which makes it Ev's call and the row the right vehicle.

### Posture

Ev re-affirmed it in-chat today in the same two terms the opening set:
**no A/B protocol** (band 3700–3799 stays bookkeeping, no ordinal
drawn), and **style reviews generally, a full review kept for the
hardest units**. Recorded in `plan.md`'s posture section beside the
2026-09-11 direction it repeats.

### What this orchestrator inherits as the first real decision

The seven DOCM rows are still **unread against the tree by this
program**, and `plan.md` is emphatic that this program has twice paid
for dispatching on a row's prose. That read is the next move, before
any grouping.

Signed (WIRE orchestrator).

## 2026-09-15 — the seven DOCM rows read against the tree: five are rulings, two are units

`plan.md` said to read them before grouping, and the reason it gave was
that this program has twice paid for dispatching on a row's prose. It
paid a third time in the other direction: the prose here **understated**
what the rows are.

**Every one of the seven is still live.** Nothing was discharged by
adjacent work — unlike the 2026-09-11 cut, where two rows estimated H
turned out three-quarters done. Each site was opened and read:
`emit_topo.rs`'s seam-vertex arm, `emit.rs`'s shared-edge walk,
`look_through_merges`, `docm7_union_declare.rs`'s asymmetry assertion,
`product.rs`'s tie-row `finish()`, `role.rs`'s `BandSlit`,
`emit_blend.rs`'s `CornerArc` keying.

**But five of the seven are not dispatchable at all**, and that is the
result. They pose a question that decides what a document MEANS, and a
lane cannot close one by implementing it:

- `member-space-look-through-…` — a membership test cannot answer which
  fragment a member face's material ended in, and the geometric
  re-measurement that could is the one DM4's routing step forbids.
- `the-pair-verbs-declared-merge-is-asymmetric-…` — the symmetric answer
  moves `Fragment` rows in every existing declared-merge golden.
- `product-refuses-naming-…-two-roots` — qualify by root, or refuse in
  the recipe's vocabulary; either changes what a document's names are.
- `blend-slit-name-collides-…` and `cut-off-arc-persists-as-a-corner-arc`
  — persisted `RoleSeg` vocabulary, so a format change with a migration.

All five re-kinded `issue` → `ruling`. **Three of them said so in their
own bodies** — *"a design ruling"*, *"for Ev"*, *"Ev's call, since it is
persisted vocabulary"* — **and were filed `kind: issue` anyway**, which
put them on the board as available work. That is worth naming as a
tracker failure mode rather than a clerical one: a row's kind is what the
board reads, its prose is not, and DOCM's sweep re-homed the files
faithfully without re-reading what they were. The two remaining
conversions (`blend-slit`, `product-…-two-roots`) are my call and each
row says so at the point it says it.

### The two that ARE units, and the order between them

- **`nobodyroots-classification-has-two-homes` — E.** One predicate on
  `ProductError`. Cleanest row on the slate.
- **`two-emitter-refusals-a-legal-declared-union-reaches` — M.** The half
  that needs no ruling: `Emission` means *a kernel bug by definition* and
  these are reached from legal documents, so the classification is false
  today whatever naming rule eventually lands. A typed refusal naming the
  construction is better under either outcome. **It runs before any
  look-through work**, because its emitter refusal is what makes the
  fragmented-merge shape unreachable in the first place.

### Three corrections a taker would otherwise have inherited wrong

1. **`nobodyroots`'s owed sweep is discharged, and the count is two, not
   four.** Both candidates were read and neither is an instance:
   `pncad-py`'s `E::NoBodyRoots | … => (none(), none(), none())` groups
   arms by which payload fields they carry, and `pncad/tests/all.rs`
   asserts the refusal. Different partitions for different reasons. The
   row had been carrying *"if either does, the count is four and the
   predicate is overdue rather than merely tidy"* since 2026-09-04; it is
   tidy.
2. **`nobodyroots`'s two copies are no longer copies of one rule.**
   `frame::product_badge` now declines four arms and declines three of
   them because the Features pane already badges them at the node — not
   because they are not faults. The shared classification is the
   `NoBodyRoots` arm alone, so a predicate written against the filter's
   current shape would get the partition wrong. Separately,
   `ProductError` has grown `kind()` → `ProductErrorKind`, exhaustive
   with no wildcard; that answers half the row's cost argument and is not
   the predicate, because `kind()` says which arm and not what it means.
3. **A citation rotted into the thing that makes its row a ruling.**
   `the-pair-verbs-…` sends a reader to `docs/DOCM-REFERENCES-DESIGN.md`,
   DM4's bullet. That file is gone (`docs/DOC-LEDGER.md`: replaced by
   `crates/editor-core/REFERENCES.md`, present tense, DM1–DM6 kept), and
   DM4's sentence now sits in a page `docs/DESIGN.md`'s companion table
   lists as **Ratified** — which is exactly the document class CLAUDE.md
   reserves to Ev. Also confirmed dead as predicted: `docs/DOCM-7-SPEC.md`.

### Two smaller things found while reading, recorded rather than swept

- `emit.rs`'s shared-edge walk carries **four** `bug(...)` refusals, not
  the two the row measured: `"unmated half-edge"` and `"dangling mate"` /
  `"dangling loop"` sit beside them and DOCM-8's reviews did not measure
  whether a legal document reaches them. Noted on the row as the taker's
  measurement, not assumed either way.
- `look_through_merges` has a fifth shape the row does not mention — the
  two-matches arm refusing `MEMBER_FACE_IN_TWO_MERGES` — folded into the
  same ruling.
- `blend-slit` cites `BandSlit(Box<StableName>)`; the payload is now
  `NameRef`. The shape the finding rests on (one field, no
  discriminator) is unchanged, so the rot is harmless and is noted at the
  row rather than repaired silently.

Signed (WIRE orchestrator).

## 2026-09-15 — the persisted-vocabulary pair: a ruling from Ev, and two rows that were never WIRE's

The `[ev]` PR I was about to open never went out, and that is the result
rather than a shortcut. CLAUDE.md's rule — *check that Ev ever agreed,
before you wait for Ev* — applied to the premise both rows share, and the
premise did not survive it.

### The premise that dissolved

Both rows treat a `RoleSeg` change as *a document-format change with a
migration story*, and I passed that on as the reason it was Ev's. Three
ratified texts say otherwise:

- **`crates/editor-core/src/persist/mod.rs`**, ratified at M4 PR 6:
  *"No schema version, on purpose… Schema breaks are not at all a
  problem, because this is not released yet: no document exists outside
  this repository, and every checked-in document is a regenerable
  artifact."* Versioning returns as Band-4 work the day a document ships.
- **N1** (`names/README.md`, Ratified #74) ratifies the STRUCTURE —
  closed enum grouped by op, role arguments are themselves names, no
  floats, no arena keys — and lists variants illustratively with
  ellipses, never enumerating the blend group at all.
- **V3** (`sweep/README.md`, Ratified #992) keeps this vocabulary
  *fillet-named on purpose*: the fence is against renaming it
  BLEND-ward, not against growing it.

No serialized document in the tree carries `BandSlit` either — every hit
is Rust source. So there was no migration to cost and no clause to amend.

### And then the ownership, which neither row's own prose mentions

`work.py territory --files -` on the three files a fix touches:
`crates/sweep/src/blend/naming.rs` is **BLEND's**,
`crates/editor-core/src/names/role.rs` is **EDIT's**, and
`crates/editor-core/src/names/emit_blend.rs` is claimed by **no program**.
WIRE owns none of them.

**DOCM's exit sweep sent both here on a glob.** Its re-home boilerplate
reads *"the file it names is WIRE's (`names/emit*.rs`, `eval/wire.rs`,
`product.rs` are in WIRE's paths)"* — but this program's `paths` is an
explicit file list naming `emit.rs` and `emit_topo.rs`, and
`emit_blend.rs` was never in it. A pattern was matched against a list
that does not contain the thing the pattern matched. Worth naming as a
sweep failure mode: a re-home is a claim about ownership, and this one
was written once and applied to seven rows.

`blend-slit` → BLEND (the kernel half is theirs, and V3 — the fence over
this very vocabulary — lives in their ratified README).
`cut-off-arc` → EDIT (the persisted recipe and the edit vocabulary is its
charter). Both announced on the receiving programs' logs with the whole
read attached, including, for `blend-slit`, the one design question left
deliberately undetermined: whether the band's identity must come from
`rec.slits` or can be derived from the `BandFace` row the emitter already
mints. That decides one crate or two, and guessing it would have been
worse than leaving it named.

### The ruling, taken in chat because Ev was here

`cut-off-arc` asked for a new `CutOffArc` variant or a ruling that the
corner family is right. Ev ratified a third shape, **(b′)**: keep one
role, repair what it CLAIMS.

The argument that decided it is the vocabulary's own, at `RimSupport`:
*"A pair of structural ROLES, not a geometric classification."*
`TransverseCap` against `ThreeConvexEdges` is a geometric classification,
so a separate variant would bake a classifier's verdict into a persisted
identity and re-spell the name of an entity that is structurally
unchanged — the arc where this band closes at this source vertex, either
way.

**A discriminator field was considered and rejected**, and the reason is
worth keeping because it looked like the obvious move: it mirrors
`BandTrim { edge, support: RimSupport }` and the `BandSlit` fix exactly.
It is not the same shape. On `BandSlit` the discriminator is needed for
UNIQUENESS — two bands genuinely collide. Here a source vertex is either
a corner or a transverse cap and never both, so `(vertex, edge)` is
already unique and the field would be derived data inside an identity,
carrying the instability and buying nothing. **Two rows that look like
one class are two classes**, and the tell was asking what the field does
for uniqueness rather than what it says.

I also had to weaken my own first answer mid-conversation: I leaned to a
new variant until reading V3's `OpGroup::Fillet` precedent — a name kept
*"whose name under-describes what it groups… the minting node tells the
two apart"* — which is the same move one level up and argues the other
way.

### One thing split off rather than ridden along

The rod-with-a-flat editor fixture. `editor-core/tests` has no ruled
fixture and no `TransverseCap` anywhere, so those three roles have never
been minted through the document layer — and that gap is what FILLET-H7's
original decision to leave the vocabulary alone rested on. It is filed as
its own EDIT row rather than as the price of the correction: the (b′)
work is a rename and three comments and does not need it, and had the
ruling gone the other way the test would have been dead on arrival. It is
the only row that would have caught the mis-description, and the only one
that would notice if the rename breaks an emitting arm.

Signed (WIRE orchestrator).

## 2026-09-15 — PR 2629 open, CI verified first-hand, style review dispatched

Lane `wire-n1` returned the `ProductError` predicate as **PR 2629**
(`wire/nobodyroots-predicate`, head `dae85987`, 389+/24−, 12 files). Not
merged; a light style review is running per `plan.md`'s posture.

**CI checked against the API rather than on report**, which is this
program's own standing lesson about background waiters. Run
`34938629986`: **39 check runs, every one `success` except six expected
`skipped`** (two cache primes, `interval backend crate`, `interval
oracle`, `step import (freecad)`, `corrupt input (release profile)`).
Counted by name: **twelve `test (…)`** — `{default, interval}` ×
`{default, 1e-6, 1e-12}` × `{1/2, 2/2}` — and **five
`k-lint (gate, …)`**: `dev-default`, `release-default`, `release-budget`,
`dev-budget`, `dev-probe`. `python suite (wheel + guide + north-star)`
ran green. Nothing narrowed the matrix.

**The PR is `mergeable_state: dirty`** — main moved under a branch cut at
`e4dcc5c7`. Lane sent back to merge `origin/main` (merge commit, never a
rebase), with the header-conflict resolution spelled out.

### What the lane did beyond its brief, and it was right to

The brief named **two** consumers. The sweep found **four**:
`viewer/src/session.rs` and `pncad-py/src/product_memo.rs` also
re-derive the partition, and neither was in the row or in my read of it.
A predicate cited by three of four leaves the fourth re-deriving the rule,
which is the defect — so the fourth seam (`work/lib/log.md`) was
announced too. **My read section was incomplete and the sweep corrected
it**, which is the outcome §5's *assume it is a class* exists to produce.

It also sited the predicate on `ProductErrorKind` rather than
`ProductError`, and the deciding argument is the one the brief warned
about: a `ProductError::is_empty_document` delegating to
`self.kind().is_empty_document()` would be a public door whose only
production caller is its own delegate — a fresh instance of
`frame-linear-generic-door-has-no-consumers`, open on this same slate.
The lane found the trap the brief named, in the shape the brief did not
predict. Whether the fix mints a fresh instance anyway is the first thing
the reviewer was pointed at.

### One correction to the lane's report

It recorded working around *"an uncommitted edit to my own item file"* in
`/home/user/cad`. The edit was committed and pushed before the lane
started; what it saw was the working tree between the `work.py set` call
and its commit. The decision to use a separate worktree was still right,
for the reason that actually matters — never share a checkout or a
`CARGO_TARGET_DIR` with another lane.

### A tooling trap the lane caught, now filed

`work.py territory --base main` returned a ~200-path answer. Not a
territory crossing: **the local `main` ref is stale.** Confirmed
first-hand here — local `main` at `0312083a` (2026-09-12), `origin/main`
at `385c01b3` (2026-09-15), `rev-list --count main..origin/main` =
**12068**, and `origin/main..main` = 0, so it is a strict ancestor and
simply old. Agents work in ephemeral worktrees and never check out
`main`, so this is the normal state, not an accident of this box.

`work/README.md` documents the invocation with the bare ref, twice
(`:190`, `:257`). The failure is the expensive kind: a three-day-old base
makes a branch look like it contains everything that landed in between,
so the output reads as *"your branch crosses everyone's territory"*
rather than *"your base is wrong"*. Filed as
`work/meta/territory-base-main-reads-a-stale-local-ref-and-answers-confidently-wrong.md`
with the three fix shapes and a note that the guard belongs in
`work.py`'s `--selftest`, which the per-PR gate runs.

Signed (WIRE orchestrator).

## Dispatched: `nobodyroots-classification-has-two-homes` (2026-09-15, PR #2629)

Lane `wire-n1`, branch `wire/nobodyroots-predicate`. **Open for review;
not merged.**

**What landed.** `ProductErrorKind::means_no_body`
(`crates/editor-core/src/product.rs`) — the empty-document reading of a
gather refusal, argued once, in the crate that owns the enum.

**On the kind and not the error, deliberately.** `NoBodyRoots` carries no
payload, so nothing beyond the class informs the answer; `kind()` is
already the one exhaustive projection, so a predicate on the kind adds no
second exhaustive match over `ProductError` and makes a tenth arm a
compile error TWICE by name — once to give it a class, once to classify
it. A delegating `ProductError::means_no_body` was considered and
refused: its only production caller would be its own delegate, which is
`frame-linear-generic-door-has-no-consumers` with a new instance. Every
consumer spells `fault.kind().means_no_body()`.

**The row's count was two. It is four.** The read section of 2026-09-15
discharged the two candidates it inherited (`py/assembly.rs`,
`pncad/tests/all.rs`) and both judgements survive re-checking — neither is
an instance. But its sweep looked only where the row pointed. A sweep of
the SHAPE found two more production consumers re-deriving the same
partition, both unexamined by the row:

- `crates/viewer/src/session.rs` — `DocSession`'s landing, which runs the
  registry over `Subject::NoBodyRoots` for this arm and argues *"has no
  product and no failure either"* in its own words. **CHROME's and VIEW's.**
- `crates/pncad-py/src/product_memo.rs` — `checks_report`, the same
  routing as `run_checks` written again for the memoized gather. **LIB's.**

All four now cite the predicate. Seams announced in `work/fix/log.md`,
`work/chrome/log.md`, `work/view/log.md` and `work/lib/log.md`;
`work.py territory --base origin/main` names exactly those four paths.
No signature moved and no routing decision changed.

**Filed outside the fence**, on FIX's slate:
`work/fix/subject-refused-accepts-the-one-refusal-that-must-not-go-through-it.md`
— `checks::Subject::refused` is public and accepts `NoBodyRoots`, turning
an empty document into a `ChecksError::Product`. Three in-tree callers
route around it by hand; nothing states the precondition. The predicate is
what makes the guard a one-liner, so the row is newly cheap rather than
newly true.

Signed (WIRE implementer lane `wire-n1`).

## Review pass on #2629 (2026-09-15)

Three stated claims were false and are repaired; the code's behaviour
did not change.

- **The new census test promised a red it cannot deliver.** Its doc said
  a tenth arm read as no-body *"reds here, naming the arm"*. It does
  not: `means_no_body` is exhaustive over the KIND, so an arm added
  under an EXISTING kind and left out of `every_arm` reads no-body with
  nothing red anywhere. No bijection is available to assert — stable
  Rust cannot enumerate an enum's variants, which is why the roster is
  hand-written at all — so the claim is narrowed to the floor it is and
  the residue is written at the site, citing the census's own caveat
  rather than restating it. The predicate's own closing paragraph
  carried the same overclaim (*"a compile error twice over"*) and is
  narrowed with it.
- **The predicate's doc overclaimed in the `true` direction.** It read
  the class absolutely — *"nothing is wrong … asking it for one is not a
  failure"* — while `eval/parts.rs`'s `product_fault` falls the arm
  through to `PartFault::PartProduct`, because instantiating a body-less
  part document IS a fault of the instantiate node. The
  consumer-freedom paragraph now covers both directions and names that
  case. This lane's own sweep called `parts.rs` "a payload extraction",
  which is right about what the function does and misses what its
  fall-through decides.
- **The row filed on FIX's slate named three callers of
  `Subject::refused` from memory and got all three wrong.** Re-derived
  with `git grep`: two production callers (`run_checks`,
  `checks_report`) and two test callers; `viewer::session` is not one —
  its test decides whether to run the checks, not which subject to
  build. The row's option (a) said "all three callers" and now says
  both production ones. The `pncad-py` literal it cited as evidence of
  reachability is filler in a tag-stability test, and is dropped.

**Renamed `is_empty_document` → `means_no_body`.** A kind is not a
document, and a document holding sketches and datums is not empty; the
doc's first line was doing repair the name should not need. Naming the
shared fact after one consumer's reading is this row's own defect one
size smaller, so the code says what the class means and the chrome goes
on calling it the empty-document reading.

**Asked and answered: is `session.rs`'s `at_rest` refusal over
`NoBodyRoots` constructible?** **Yes**, derived from the tree rather
than built: `roots::is_sink` makes the root set exactly the sink set;
`Node::Measure`'s `inputs()` are the nodes its references are read AT;
`product::sources_of` returns `None` for a measure. So a document of an
`InstantiatePart` plus a `Measure` whose refs are read at it has the
instantiate de-sunk, the measure as its only root, and gathers
`NoBodyRoots` — while `session::assembly_shaped` is true, because it
scans `order()` for any `InstantiatePart` and does not care about roots.
That reaches `AtRestBadge::Refused` carrying the no-body refusal: the
"deleting the last feature looks like a failure" outcome, three lines
below the line that avoids it. Not fixed here — pre-existing, and
CHROME's and VIEW's. Two routes were closed on the way and are worth
recording so nobody re-walks them: a `BooleanValue::Empty` root still
sets `any_body_denoting` (it returns `Some(vec![])`), and
`Node::Mate`'s `inputs()` is empty, so a mate never de-sinks an
instance. What was NOT checked is whether the GUI will author a measure
over the only body-producing node.

Signed (WIRE implementer lane `wire-n1`).

## 2026-09-15 — PR 2629 MERGED (`5d34c629`): the first unit of this orchestrator's tenure

`nobodyroots-classification-has-two-homes` closed. One light style
review, one fix pass, no second review round — the diff that came back
contained a rename, three narrowed prose claims and a corrected tracker
row, and **none of those is a mechanism a previous round had not seen**,
which is this program's own test for whether a further round is worth
running.

### What the review bought, and it was the sharpest finding of the unit

The reviewer caught that the new test's doc **promised a red the census
cannot deliver**: a tenth `ProductError` arm projected onto the existing
`NoBodyRoots` kind and left out of `every_arm()` compiles, reads as
no-body, and reds nothing — because the predicate is exhaustive over the
KIND, not the error. Two things make it the good kind of finding. The
sibling test forty lines above **already states that hole in as many
words**, and the new test rode its census without carrying the caveat
forward. And it is **working rule 1 landing on the unit that quotes it**:
*a floor over a hand-written roster is the defect one level up.*

The fix pass then followed rule 1's actual procedure rather than skipping
to the floor: it tried for the bijection, established it is not available
(stable Rust cannot enumerate an enum's variants, which is *why*
`every_arm` is hand-written), and wrote the residue **at the site**,
citing the sibling caveat instead of restating it. It also found an
overclaim in the predicate's own closing paragraph that neither the
reviewer nor I had named.

### Two things the lane got right that were not asked of it

**The sweep found four consumers where the row and my read of it found
two** — `viewer/src/session.rs` and `pncad-py/src/product_memo.rs` also
re-derive the partition. A predicate cited by three of four leaves the
fourth re-deriving the rule, which is the defect. **My read section was
incomplete and the sweep corrected it**; the row now carries that
correction from the lane's side, so the file agrees with itself from both
directions.

**And it renamed `is_empty_document` → `means_no_body`** on an argument
better than the objection that prompted it: *"empty document" is one
consumer's READING of the fact*, so naming the shared fact after one
consumer's vocabulary is this unit's own defect one size smaller. The
chrome goes on calling it the empty-document reading, which is the
chrome's to call it.

### A claim-level error caught before it cost another program

The row the lane filed on FIX's slate said three callers route around
`Subject::refused` correctly and named `viewer::session` among them.
`git grep` says otherwise: two production callers
(`checks.rs` `run_checks`, `product_memo.rs` `checks_report`), two test
callers, and `session.rs` **does not call the door at all**. That changed
the row's own option (a) from "deletes it from three callers" to two.
Wrong evidence on another program's slate is worse than no row, so it was
a must-fix rather than a nit.

### The merge treadmill, named because it is structural

The PR conflicted **three times** before landing, and never once in code:
every conflict was a program log — `chrome`, then `view`, then `chrome`
and `wire` again — where both sides append to an append-only narrative.
That is not bad luck. **A unit that announces four seams races four
programs**, each of which appends at end-of-file, and this tree moves
about 12,000 commits in three days. Each resolution is mechanical (the
union, main's entry first) but each costs a full CI round (~35 min), and
the conflict window is shorter than the round.

Two things learned, worth the next lane's attention:

- **Check whether the base merge is prose or substance before spending a
  round.** The second merge brought 248 changed files including a
  **+420-line change to `crates/viewer/tests/frame_policy.rs`** — the
  suite testing `product_badge`, one of the four functions this PR edits.
  That is a real interaction, so it was compiled and run locally first
  (45 `frame_policy` rows green) rather than discovered by CI.
- **Close the green→merge gap mechanically.** A real until-loop on the
  check-runs API, waking on conclusion, is what let the last round merge
  before main moved again. GitHub auto-merge would be better and is **not
  enabled for this repository**; not changed, because a repo setting is
  Ev's.

### Filed out of this unit, all on other programs' ground

- `work/fix/subject-refused-accepts-the-one-refusal-that-must-not-go-through-it.md`
  (by the lane, corrected in the fix pass).
- `work/chrome/viewer-states-the-empty-document-rule-in-four-places-and-the-one-that-gates-cannot-red.md`
  — the structural half: three restatements the citation did not reach,
  and `product_badge`'s remaining `matches!`, which is the one construct
  in the arrangement that cannot red when an arm is added. The
  `editor-core` side now reds twice by name; the viewer side, where a
  user sees the consequence, reds not at all.
- `work/chrome/at-rest-badge-reports-an-empty-document-as-a-refusal.md`
  — the behavioural half, and the answer to the one question I put to the
  lane: **yes, constructible.** An `InstantiatePart` plus a `Measure`
  whose refs are read at it de-sinks the instantiate, leaves the measure
  as the only root, gathers `NoBodyRoots` — while `assembly_shaped` stays
  true because it scans `order()` and never looks at roots. `at_rest`
  then badges "Refused" for exactly the empty document the line three
  above classifies as an absence. Two dead routes closed on the way and
  recorded so nobody re-walks them.

Signed (WIRE orchestrator).

## 2026-09-15 — Dispatched: `two-emitter-refusals-a-legal-declared-union-reaches` (lane `wire-e2`)

Branch `wire/emitter-refusals-a-legal-union-reaches`. **Raised to FULL
review at dispatch**, against the M estimate, and `plan.md`'s posture
table carries the reason.

**Why full.** The deliverable is a MEASUREMENT — which of the emitter's
`bug(...)` refusals a legal document can actually reach — and a wrong
measurement fails in the dangerous direction. `NamingError`'s own doc
says every variant is *"an emission BUG … never a normal modeling
outcome"*, so re-classifying a refusal that is really body corruption as
a legal outcome would silently downgrade a genuine kernel bug's signal.
That is a correctness question a light review is not looking for.

**What reading the sites added to the row, and it is the unit's real
content.** `unique_shared_edge` carries **four** `bug(...)` refusals,
not the two the row measured, and they are visibly **two categories
wearing one word**: `"unmated half-edge"`, `"dangling mate"` and
`"dangling loop"` read as structural corruption of the body, while
`"two shared edges where one expected"` and `"no shared edge"` read as a
legal shape the combinatorial derivation has no answer for. The row
names only the second pair. The fifth site, `emit_topo.rs`'s seam-vertex
arm, is a `_ =>` **catch-all** after a case analysis — which is where a
shape nobody enumerated lands, a different thing from an inconsistent
fact. The brief says to measure the line rather than assume it, and that
a site no legal document reaches KEEPS `Emission` and is reported as
such.

**The precedent is in the same enum, twice, and the brief points at
both.** `Band(BandError)` was carved out of `Emission` with exactly this
argument — *"which this is not: nothing about the result body is wrong
here"* — and `SplitLineage` / `FragmentLineage` were carved out because a
variant can carry *"the one thing the repair needs that a sentence cannot
supply: WHICH edge / WHICH face."* `FragmentLineage`'s doc also argues
why it is a **sibling word rather than one generalised** over
`EntityKey`, which is the same question this unit faces about how many
variants it needs. So the shape of the answer is established; only where
the line falls is open.

**Fenced explicitly.** This unit re-classifies and does **not** invent a
naming rule: what a member-space declaration MEANS for a face that is no
longer one face is `member-space-look-through-…`, a `ruling` on this
slate. The change must be correct under either answer, and the brief says
to stop if the lane finds itself designing the name.

Three traps named in the brief, all earned: a variant with no caller
(`frame-linear-generic-door-has-no-consumers` is the open row for that
shape); a new variant whose payload is a free-text `&'static str`, which
rebuilds `Emission` one level down; and re-classifying a site the lane
did not actually reach because its message reads legal-ish.

### Three rows arrived from other programs while this session worked

`declare-door-refuses-a-tie-before-it-asks-the-pairs-kinds`,
`interrogate-read-answers-a-tie-before-the-door-s-kind`, and
`select-refusal-coverage-is-not-compiler-enforced-from-the-test-crate`.
Unread against the tree by this program; the 2026-09-11 lesson applies to
them as it did to DOCM's seven.

**The first two look like one class and should be read together**: a door
answering a tie or an `Ambiguous` *before* it has resolved the kind the
answer depends on. That is the same shape as
`the-declared-pair-refusal-reads-the-authored-kind`, already on this
slate — which would make it three instances, not two, and a class rather
than a pair. Noted rather than acted on; the read comes before the
grouping, as it did for DOCM's rows.

Signed (WIRE orchestrator).

## 2026-09-15 — Dispatched in parallel: `wire/tie-before-kind` (lane `wire-t1`), three rows as one class

Three rows, one unit, **no file overlap with `wire-e2`** — that lane holds
`names/emit.rs` and `names/emit_topo.rs`; this one holds
`eval/wire.rs` and `names/interrogate.rs`. The brief fences each lane off
the other's files by name.

**The class, in PORT's words:** *a site that asks how many entities
answer to a name before it asks what the name denotes.* A caller whose
reference happens to be TIED is told to narrow it, when narrowing could
not have helped, because what they actually did wrong is answerable
without resolving at all.

- `declare-door-refuses-a-tie-before-it-asks-the-pairs-kinds` — ordering
  in `resolve_declarations`.
- `the-declared-pair-refusal-reads-the-authored-kind` — **the same two
  lines**; authored kind or resolved key.
- `interrogate-read-answers-a-tie-before-the-door-s-kind` — the one body
  five public read doors delegate to.

**Taken together because PORT says they must be, and it checked the
interaction rather than leaving it.** If the "found" word must come off
the resolved key, a tied name has no single key, so the kind-first answer
has to come off the NAME — which is what `resolve_face` took. The brief
says to land that consistently and not re-litigate it.

**The precedent is merged, so the shape is copied rather than invented:**
PORT-DOORS-1, PR #2635 (`0f2667888`), closed this defect in
`assembly.rs`'s `resolve_face`. `eval/wire.rs`'s own `face_name` is a
second counter-example in the file being edited.

**Posture: light style review with one named correctness claim to
falsify** — the premise that every candidate of an `Entry::Tied` carries
the name's kind. Not raised to full: the premise is already verified by
another program against the same table, the fix shape is landed one door
over, and no variant changes shape — only which variant a document gets.
That is a narrower blast radius than
`names-flush-and-select-discard-a-refusal-with-map-err-underscore`, which
was raised to full because two public enum variants **gained a field**.
The brief still requires the lane to re-verify the premise itself and to
stop if it holds only by an invariant a module away.

Named in the brief: PORT's **first** instrument missed two of these three
(the `Entry::Tied`-adjacent grep could not see them), so the lane owes a
sweep shaped for the caller-side pairing and a stated blind spot. A
fourth instance is likely.

### Not dispatched, deliberately

`select-refusal-coverage-is-not-compiler-enforced-from-the-test-crate`
(S-TINT's TINT-1 residue, `names/geompred.rs`, WIRE's ground) is real and
takeable and does not collide with either live lane. Held anyway: three
concurrent PRs on a tree moving ~12,000 commits in three days means the
session spends itself on merge conflicts rather than on work — the last
unit took three conflict rounds to land, none of them in code. Two lanes
is the level this tree supports. It is next in the queue, not deferred.

Signed (WIRE orchestrator).

## 2026-09-15 — RULED: N3's retire-loudly generalises, and it settled four of five compositions

PR 2677 merged (`2835e57b`). Ev, in the PR's comments:

> *"refuse and offer where a unique best offer exists" is good. for
> context on n3, the decision that was made against was not even
> refusing, just silently taking the merged descendant*

**Asking the framing first was worth it, and this is the evidence.** One
comment settled three rulings' worth of ground; answered separately they
would have been three decisions with three chances to disagree with each
other, which is how this vocabulary accumulated two spellings of things
before.

### The half that did the work was the half I did not ask about

I asked whether N3 generalises. The answer that made the hard cases
decidable was the **context**: N3's rejected alternative was *"not even
refusing, just silently taking the merged descendant."* So the value the
rule protects is **never silently re-point** — refusing is the floor, and
the offer is the courtesy above it.

That is what unsticks the cases where no offer can be computed. I had
been carrying "refuse with no offer is a weaker promise than N3 makes" as
a reason the generalisation might strain; under the real reading it is
not a weakening at all, because the thing being ruled out is silence, not
absence of an offer. `ResolutionFailure::offers` is already a
`Vec<StableName>` documented *"Empty when nothing structural offers
itself"*, so zero, one and many were always expressible and nothing needs
building.

**A framing question is worth asking when the answer might reframe the
question** — not merely when it might say yes or no. That is the
transferable version.

### What it settled, and what it deliberately did not

Four of the five compositions, and the fifth explicitly not:

- **split**, **containment**, **fragmented merged row** → refuse, no
  offer. `member-space-look-through-…` re-kinded `ruling` → `issue`: it
  is now the application of a decision.
- **two roots** → refuse, which `ProductError::Naming` already does, so
  the refusal's KIND was never the question. Qualifying by root is not
  available (it is a naming scheme, not an offer, and neither root is
  *best*). `product-refuses-naming-…` re-kinded `ruling` → `issue`: what
  remains is refusing EARLIER and in the recipe's vocabulary. Its real
  severity is untouched — the solve still accepts a document the gather
  cannot represent.
- **operand seat (A/B)** → **not settled**, and the row now says why at
  length so nobody applies the rule here by analogy: nothing is
  re-pointed. No name vanishes, nothing resolves wrongly, no refusal is
  owed — two orders produce two VALID documents whose names differ. The
  rule governs a reference whose entity went away, and this is not that.
  It is now WIRE's only ruling.

### One residue decided rather than escalated

Ev said *unique best*; N3's own offer is **plural** (*"the merged name
vanishes with its constituents offered"*), so a reading exists under
which a split offers its fragment SET. Took the narrower reading —
refuse, no offer — on the ground that N3's plural case is an exact
DECOMPOSITION of what the merged name covered, whereas a split's
fragments are CANDIDATES for what the reference meant, and offering
candidates is one step from the silent pick the rule exists to prevent.
Written at the ruling row and at `member-space-look-through-…` with the
reasoning, so a lane that disagrees argues in a PR rather than silently
taking the other reading.

### Reported to the live lane

`wire-e2` was told, because the ruling names its unit: the
fragmented-merge shape is unreachable *because* its emitter refusal
fires first. Its fence is unchanged — it does not get to make that shape
reachable or name it — but if its re-classification moves which refusal
that shape hits, a row on this slate is waiting on exactly that.

Signed (WIRE orchestrator).

## 2026-09-15 — `wire-t1` — the tie-before-kind unit, in review (PR 2681)

Three rows of one class, closed together on `wire/tie-before-kind`:
`declare-door-refuses-a-tie-before-it-asks-the-pairs-kinds`,
`the-declared-pair-refusal-reads-the-authored-kind` and
`interrogate-read-answers-a-tie-before-the-door-s-kind`.

**The premise holds by construction**, which is the answer the brief
asked for and the stronger of the two it offered. `NameTable::forward`
is a private field with exactly two writers — `insert_ref` and
`insert_tied_ref` — and both refuse a row whose `name.kind` disagrees
with a candidate's `key.kind()`. `insert` / `insert_tied` are wrappers;
`project` reaches `Entry::Tied` only through `defer::narrow_into`,
which is one of those two calls. So it is not "safe today by an
invariant one module away": the module that holds the invariant is the
module that holds the only door through which a row can exist.

That decided the authored-kind question with it. The ordering makes the
kind question precede resolution, a tied name has no single key, so the
word comes off the NAME — and `the-declared-pair-refusal-…`'s own guess
("read `k1` and `k2`") is the wrong direction. What the resolved keys
keep is the fallthrough arm, reachable only on a broken table, under a
`debug_assert!`: the `resolve_face` split, guard off the name and
projection answering what the key IS. The same split landed at
`interrogate::read`.

**Nothing asserted the old behaviour.** All 1239 editor-core rows,
`pncad` and `viewer` pass unchanged; `m4_pr5_declare`'s existing
`Ambiguous` row declares a CROSS-operand face pair, which is supported,
so its tie still refuses. No `.py` row asserts an `ambiguous` outcome at
either door. No tag string changes — `declare_resolve`,
`declare_unsupported_pair`, `ambiguous` and `wrong_kind` all exist
already; what moves is which one a document gets. Two `pncad.pyi`
docstrings (LIB's path) were re-worded because the change moved what
they describe.

**The sweep found the fourth instance the brief predicted, and a
fifth.** The instrument was the caller-side pairing — a multiplicity
token and a kind token in one non-test `fn` across all of
`crates/editor-core/src/` — 30 candidates read by hand. Filed:
`work/wire/the-designation-road-resolves-before-it-asks-the-kind`
(`named_entity` and the measure reference, one row because all four
refusals carry `entity_door::Found` and the change is one in
`entity_door`) and
`work/shell/clearance-window-selection-asks-how-many-before-what`
(`clearance::windows_of`, SHELL's ground — a tied face name refuses
`Unresolved`, and `SelectionRefusal` has no word for a tie at all).

The instrument's own blind spot is stated in the PR and is worth
carrying forward: **it segments by `fn`, so a resolve in one function
paired with a kind test in another is invisible to it.** The measure
site is exactly that shape and was found by reading
`entity_door::entity`'s call sites instead. A later sweep of this class
should not reuse the grep alone.

Signed (`wire-t1`).

### Delta after the style review of PR 2681 (`wire-t1`)

No MAJOR, nothing blocked merge; the fix pass is recorded here because
two of its items correct claims this log itself made.

**The fix minted a fresh instance of its own class, and the review
caught it.** `DeclareBothOperands` is a multiplicity question raised at
the door I rewrote, above the kind question, with a 25-line doc comment
arguing why *rung 3* stays above it and nothing at all about
both-operands. The argument now written at that site is that
`DeclareUnsupportedPair` carries `cross_operand`, so the kind refusal
**cannot be built** over a name that landed in two operands — a field
of it has no value — where a tie leaves no field empty. That is a
different shape of reason from "this outranks that", and it is the only
one available: the kind question there is genuinely unanswerable, not
merely deferred.

**The disclosure was one quarter of the truth.** I reported one moved
outcome (`NodeGone` on the second name) and justified it as "the
ladder's own stated ranking". The ladder ranks within ONE name's walk
and says nothing about one name's rung 3 against another's rung 2, so
the justification did not cover the case it was attached to, let alone
the three it omitted (`Vanished`, both-operands, and `step_diagnosis`'s
`UnionDeclareStep`). All four change a Python tag. Two are now pinned;
the rule is restated as this door's own.

**Three "one home" claims that were not.** `declared_pair_supported`
said "the list, once" beside a `match` that re-enumerated the same
three shapes; it is now `DeclaredStep`, an enum the door projects from,
so a fourth shape fails to compile rather than diverging.
`ladder::vanished`'s "rather than a second spelling" was false while
`route_declarations` built the same payload inline 1150 lines below;
that call now goes through it. And two rung-order statements — the
module header and the ladder's own doc — still described 1, 2, 3 for a
door that now asks 1, 3, kind, 2.

**For future lanes on this program:** a comment asserting a property the
code does not have is the shape this program keeps paying for, and a
one-home unit is exactly where it is least affordable. Writing "once"
is a claim to check with `rg`, not a summary of intent.

Signed (`wire-t1`).

### Delta round 2 on PR 2681 (`wire-t1`) — the claim, not the code

One MAJOR, on a **sentence**. The delta review ran the experiment my
prose asserted and it failed: adding a fourth *variant* to
`DeclaredStep` gave `E0004`, but adding a fourth *pair shape* reusing
an existing variant compiled clean and silently filed a cross-operand
vertex-vertex contact under operand 1's list. My PR body and this log
both stated the property over pair shapes. It held over variants.

**And the paragraph one above it in this log told future lanes that
"once" is a claim to check with `rg`.** "Fails to compile" is the same
kind of claim and I wrote a fresh unchecked one directly below the
warning. That is the finding worth carrying forward, not the enum.

**Why the code allowed it.** `DeclaredStep` carried the SHAPE and none
of the facts `declared_step` established to pick it, so all three
orientations were discarded at the return and re-derived at the
projection from the raw `o1` / `n1.kind` — agreeing with the
classifier only because both sides happened to read the same inputs.
Rule 1 one level down, inside the fix for rule 1.

**The bijection was available and is taken.** A private `mod sides`
holds three witnesses — `SameOperand`, `CrossOperand`, `VertexAndFace`
— whose fields are unreachable outside it and whose only constructors
are comparisons of the two sides. `DeclaredStep`'s variants carry
them; the projection reads orientation off the step and re-derives
nothing. Experiments, run rather than asserted:

| spelling of the mistake | result |
| --- | --- |
| fourth variant, projection untouched | `E0004` non-exhaustive |
| variant named without its witness | `E0308` mismatched types |
| reaching past a witness constructor | `E0603` constructor is private |
| asking the constructor honestly | compiles, returns `None`, refuses |
| calling a comparison with one side twice | **compiles** — the residue |

The last row is named at the site and in the doc, at that resolution,
because this door has now shipped one over-stated claim and will not
ship a second. The other uncaught case — an arm pairing the wrong
KINDS with a variant — reaches `broke` and fails loud, which is the
floor and is called the floor.

**A second lesson, cheaper.** My account of why one disclosed outcome
was unpinnable was wrong, and the tree said so in a doc comment I had
already cited for something else: `NameTable::project` keeps a
straddling tie's row verbatim in both halves. I reported a blocker I
had reasoned to instead of probing. The probe takes four minutes and
its numbers are on the row now. **Report what you measured, or report
that you did not measure.**

Signed (`wire-t1`).

## 2026-09-15 — PR 2681 MERGED (`228b076d`): the tie-before-kind class, three rows, four rounds

`declare-door-refuses-a-tie-before-it-asks-the-pairs-kinds`,
`the-declared-pair-refusal-reads-the-authored-kind` and
`interrogate-read-answers-a-tie-before-the-door-s-kind` closed together.
Review → fix → **delta** → fix, and the delta was worth running because
the fix pass introduced `DeclaredStep`, a mechanism no round had seen.

### The delta's MAJOR was a claim, and the way it was found is the lesson

The fix pass replaced a `bool` with an enum and wrote, in the PR body and
in this log, *"a fourth **pair shape** is added by adding a variant and
fails to compile."* **The reviewer did not read the code to check it —
it ran two experiments:**

- a fourth **variant** → `error[E0004]: non-exhaustive patterns`;
- a fourth **pair shape** reusing an existing variant → **compiled
  clean, zero warnings**, with the cross-operand V–V contact landing in
  operand 1's list carrying operand 2's vertex.

So the property was true of variants and false of pair shapes. And the
paragraph directly above that claim in this log told future lanes
*"writing 'once' is a claim to check with `rg`, not a summary of
intent."* **The claim was written one paragraph below the warning against
writing it.** The lesson now on the row: *"fails to compile" is the same
kind of claim as "once"*.

### One row closed by being shown wrong

`the-declared-pair-refusal-reads-the-authored-kind` asked whether
`kinds: (n1.kind, n2.kind)` should read the resolved key instead. It
should not, and the reason is forced rather than preferred: once the kind
is asked BEFORE resolution — which is what the other two rows require — a
tied name has no single key, so the word must come off the name. The row
closes because its proposal is refuted, not because it was implemented.

### The both-operands argument, settled

The first round found `DeclareBothOperands` raised before the kind
question — a multiplicity question outranking the kind question where the
tie does not. The lane argued rather than moved, and the argument holds:
`DeclareUnsupportedPair` carries `cross_operand`, a plain `bool` with one
construction site and no way to say "neither", **so the kind refusal
cannot be built over a name that landed in both — a field of it has no
value.** A tie leaves no field empty. Verified by the delta reviewer down
to the side pick never distinguishing `Unique` from `Tied`.

### What the second fix pass built, and how it was adjudicated

A private `mod sides` whose three witnesses have unreachable fields and
whose only constructors compare the two sides. The variants carry them;
the projection re-derives nothing; `declared_step` matches on `(ka, kb)`
alone, so **the operands left the match entirely** and an arm pairing
kinds with a wrong operand assumption cannot be written.

**Adjudicated by re-running the experiment rather than reading the
report** — the whole finding was that a compile-time claim had been
asserted instead of checked, so accepting one would have repeated it:

- naming the variant without its witness → **`E0308`**, as reported;
- the **disclosed** residue (`SameOperand::of(oa, oa)`, the comparison
  called with one side twice) → **compiles**, as reported.

The second check is the one that mattered. It confirms the lane was
honest about what it could not close, rather than narrowing the claim to
whatever it happened to achieve. Both residues are named in
`DeclaredStep`'s own doc — rule 1's procedure completed: take the
bijection where it exists, and where it does not, write why at the site.

### Two things the lane did that nobody asked for

**It corrected its own blocker with measurements.** Having reported row 3
unconstructible, it probed and found `NameTable::project` does keep a
straddling tie in both halves (`in-BOTH=2` at one plane, `4/4/0` at
another) — exactly as that function's own 40-line doc says. The real
blocker is narrower: the fixture's two ties are symmetric about one
plane. Reported as **not measured** rather than dressed up, with the
numbers and the required fixture shape on the item.

**It withdrew a row it had filed.** The third fault it added to SHELL's
slate was already there (`named-face-scope-…`, finding 2, 2026-09-13), so
per `work/README.md` it cross-linked and stated the split instead of
leaving a duplicate for someone else to reconcile.

### The standing pattern, third instance today

The evidence against a claim was in the tree, beside the thing claimed
about: `NameTable::project`'s doc for row 3's blocker, as
`emit_sweep.rs`'s `UNRESOLVED` const is for PR 2688's MAJOR 1, and as
`each_kind_has_an_arm_…`'s caveat was for PR 2629's test claim. **Three
units in one day, three times the refutation was already written down.**

Signed (WIRE orchestrator).
---

## 2026-09-15 — `wire-e2`: two emitter refusals a legal declared union reaches

`work/wire/two-emitter-refusals-a-legal-declared-union-reaches.md` →
`review`.

**The measurement is the unit.** `NamingError::Emission` says a
mint-time fact disagreed with the result body — a kernel bug by
definition. I took the row's five candidate sites (the four `bug(...)`
arms of `unique_shared_edge` plus `emit_topo`'s seam-vertex catch-all),
re-took the row's reproducers on `origin/main`, and swept the whole
workspace test suite with each arm temporarily instrumented. Two sites
are reached from ordinary declared unions; one is refuted on a body that
`topo::validate_closed` accepts; three are dangling-key arms nothing in
the tree reaches.

**Landed vocabulary: two sibling variants**, on
`FragmentLineage`'s own test (a sibling word when the two name different
structures to go and read). `SeamVertexParentage { vertex }` and
`SharedRim { node, face, other, found: RimShare }` — the second's two
failure modes are ONE fact with a typed discriminant rather than two
words, which holds because the classification lives at the CALLER, so
the variant has one construction site and one premise. Both open with a framing sentence written once,
`UNRULED_FRAMING`, which is deliberately not a reworded `EMISSION_FRAMING`:
they say opposite things about whose fault the failure is.

**What I did not do**: no naming rule. What a member-space declaration
means for a face that is no longer one face stays with
`member-space-look-through-stops-at-splits-containment-and-fragmented-merges`,
and the re-classification is correct under either answer. The
fragmented-merge shape still refuses at step 2 of the same fold — it now
refuses as `SharedRim`, not as `Emission`, which is the fact that row is
waiting on.

**Filed elsewhere**:
`work/wire/seam-junction-vertex-name-cannot-be-collapsed-by-the-union-fold.md`
— a third reachable refusal the sweep turned up, and the one that is
NOT a misclassification: `emit_topo` mints a seam-junction vertex name
whose path is k ≥ 2 `Seam` segments, and `emit_union::collapse` refuses
any tail segment that is not a `Fragment`. Two halves of one emitter
disagreeing about a shape one of them mints. Left `Emission`, correctly.

Signed (`wire-e2`).

### 2026-09-15 — `wire-e2`, review round 1 (two MAJORs, both upheld)

**MAJOR 1 — the classification belonged to the predicate, not to the
caller.** `shared_rim` (was `unique_shared_edge`) has three production
call sites under TWO premises. `emit_topo`'s chord derivation did not
build the body it asks about: it descends two result faces into an
operand and GUESSES the pair carries the chord's rim, which a later
split can legitimately refute. `emit_sweep`'s cap rims did build theirs,
where a wall meets each cap along one edge by construction and any other
answer is a contradicted key bundle — `emit_sweep`'s own `UNRESOLVED`
const argues exactly that for the sibling derivation, in the same file
(at the module head, ~100 lines above the call sites and for a different
function's refusal; round 3 corrected "ten lines above the call", which
was wrong on both counts). Round 1 shipped the missing-rule sentence over
both, so a corrupt extrude or loft would have rendered *"the result body
is sound"* and nothing would have gone red.

**The repair, which is a mechanism no round has seen**: the walk now
RETURNS the cardinality (`Result<Result<EdgeKey, RimShare>, NamingError>`)
and each caller classifies under its own premise —
`CAP_RIM_CONTRADICTED` (an `Emission`, `UNRESOLVED`'s twin) in
`emit_sweep`, `SharedRim` in `emit_topo`. Structural corruption still
refuses inside the walk, because that IS about the body and not about
anybody's premise. Chosen over a premise parameter or two predicates
because it is the only one of the three where the walk cannot express a
classification at all.

Same defect, same cause, at the seam-vertex `_ =>`: a nine-arm match's
catch-all is the preimage of every unenumerated shape, and round 1 gave
the whole preimage a sentence witnessed on one member. The witnessed
shape is now its own arm (`([_], [], _, _)` — one A-descended edge, none
on B, nothing else naming the missing parent) and the residue keeps
`Emission`. The mirror is deliberately NOT included: the fold is not
symmetric in A and B, so it is a separate claim nobody has reached.

**MAJOR 2 — `UNRULED_FRAMING` shipped with two runs of 32 spaces in it**,
in the one sentence this unit exists to write, and all three assertions
were `contains(UNRULED_FRAMING)` — the constant against itself, which
passes for any content. The literal is a `concat!` now, one test pins
both framings as WORDS, and every rendered sentence is asserted free of
a padded run.

**S8 — the census had a hole where it was looking.** `shared_rim` opens
with `face_half_edges`, so its refusal set is eight arms, not five, and
the missing one — `"face walk: dangling face"` — is the arm a stale face
key from a caller would hit, the exact hypothesis under test. Re-measured
all eight.

**S1** — category membership was held by hand at three places with
nothing tying them. An exhaustive `expected_framing` match in the test
now ties variant → framing and asserts the other framing is absent, so a
not-a-bug variant that writes `EMISSION_FRAMING` no longer compiles-and-
passes.

Verified green on hosted run `35023772205` (39 jobs, 33 success, 6 skipped,
twelve `test (…)`, five `k-lint (gate, …)`, python suite green).

Signed (`wire-e2`).


### 2026-09-15 — `wire-e2`, review round 2 (three MAJORs, all upheld)

**The finding behind the findings**: the code converged and the CLAIMS
did not. Round 2 shipped five sentences that are false, and a reviewer
found them by experiment where I had found them by rereading. Round 3's
deliverable was therefore the claims: every declarative sentence in the
diff and the PR body re-checked by `rg` or by running something. What
each false claim cost:

- *"unstatable"* — false. `let bug = |what| …` was in scope over the
  cardinality returns, and `Emission` takes a `&'static str`. **Made
  partly true and the rest of the claim withdrawn**: the closure is gone
  (three named consts, which cannot be applied to a new subject), and
  the doc now says what the design buys — a caller must WRITE its
  classification, never inherit one by saying nothing.
- *"the fold is not symmetric in A and B"* — a symmetry claim standing
  in for a measurement, and **the measurement refutes it**. Censused
  17,381 seam vertices: `a=0,b=1` (the mirror I excluded) is the
  COMMONEST shape at 10,865, against 5,305 for the shape I treated.
  Replaced with the census, and the mirror stays `Emission` on the only
  ground that survives — nothing reaches it (the residue arm fired zero
  times).
- *"exactly one construction site"* — two (`emit_topo`'s two chord
  arms). The argument survives, because both are arms of one derivation
  under one premise; the sentence did not.
- *"the tag leg is the only remaining gap"* — two legs.
- an enum doc citing a test that does not exist.

**MAJOR A — two fresh instances of a row this program CLOSED.**
`map_err(|_| CAP_RIM_CONTRADICTED)` at both `emit_sweep` call sites is
exactly `names-flush-and-select-discard-a-refusal-with-map-err-underscore`
(PR 2378), whose repair was structural: *"the closure is gone
entirely"*. Repaired the same way — the walk returns a `Rim` enum, every
call site is an explicit `match`, and **there is no `map_err` on this
path anywhere**; `emit_sweep`'s arm calls a named function that CONSUMES
the `RimShare` to pick its sentence instead of dropping it.

**MAJOR B** — above.

**MAJOR C** — above. Also acknowledged: the missing-rule half is
unstatable in the walk only because `SharedRim` requires a
`RecipeNodeId` the walk has no access to, a field added for an unrelated
reason (S5), not by design.

**S1 was a half-fix and I proved it by experiment.** Adding a `Probe`
variant with a category and a `sampled` arm but no row leaves
`every_variant_names_its_subject` GREEN, because `covered ==
(0..rows.len())` only proves the rows cover a contiguous prefix. Ran
`NamingError` through `tests/display_contract.rs`'s census harness and
measured the whole chain, twice — once against the hand-written
`(token, roster)` pair, where a variant with neither roster entry nor
case still passed, and again after merging forward onto
`f6_variants!`, which landed on `main` mid-round and writes the
wildcard-free `match` and the roster from ONE ident list. Under the
macro no step is green: the probe stops that file compiling, the only
fix is to add the ident, adding the ident adds it to the roster, and the
set difference reds until it has a case. That is what is written where
the claim is.

Also: the dropped `!contains(" { ")` assertion is back and applies to
EVERY variant rather than being hand-spelled per row; `EMISSION_FRAMING`
said "two variants speak it" and it is three; `emit_topo`'s chord
comment said the descent finds "the unique operand edge" ~80 lines above
the arm that exists because it is a guess.

**Filed**: `work/wire/the-b-side-contact-record-rescue-arm-never-fires.md`
(the census's own finding — `partner_a` is `Some` at zero of 17,381
vertices, so one rescue arm is dead in front of the larger population)
and `work/wire/three-emission-bugs-do-not-speak-the-framing-written-once-for-them.md`
(`EMISSION_FRAMING` claims "every emission-inconsistency refusal opens
with" it; three of six do not).

Verified green on hosted run `35039975831` (39 jobs, 33 success, 6 skipped,
twelve `test (…)`, five `k-lint (gate, …)`, python suite green).

Signed (`wire-e2`).

## 2026-09-16 — PR 2688 MERGED (`f096ee39`): four rounds, and the lesson is about claims

`two-emitter-refusals-a-legal-declared-union-reaches` closed. Full review
→ fix → delta → fix → delta → fix → a claims-only round. The most rounds
any unit on this program has taken, and every round found something the
previous could not have.

### What the unit actually did

Measured which of the emitter's `bug(...)` refusals a legal document can
reach, and re-classified only those. The line came out asymmetric, which
is the useful result: of `unique_shared_edge`'s eight arms (**eight**,
not the five the row named — `face_half_edges` contributes three), six
are corruption and keep `Emission`; two are legal shapes the
combinatorial derivation has no answer for.

The final shape is better than anything the brief proposed. The walk —
now `rim_between`, returning `Result<Rim, NamingError>` — **reports the
cardinality and classifies nothing**. The outer `Err` is corruption under
every caller; `Rim::NotOne(RimShare)` is a fact each caller classifies
under its own premise. `emit_sweep` (asking about a body its own mint
built) says `Emission`; `emit_topo` (guessing about an operand body) says
`SharedRim`.

### The pattern that cost three rounds

**The code converged; the claims did not.** Every round shipped prose
asserting more than the diff delivered:

- *"unstatable"* — false: the `bug` closure was in scope for the whole
  walk and took an arbitrary `&'static str`.
- *"the fold is not symmetric in A and B"* — false, and **measured**
  false: the excluded mirror is the most common seam vertex in the tree
  (10,865 of 17,381), and the real asymmetry runs the other way
  (`partner_a` was `Some` **zero** times, `partner_b` 1,441).
- *"exactly one construction site"* — two. *"the tag leg is the only
  remaining gap"* — two legs. An enum doc citing a test that does not
  exist.

The instruction that broke it was not "be careful": it was **check every
declarative sentence by experiment or `rg` before pushing, not by
rereading**. That found three more false claims of the lane's own,
including a sweep count (*"44 hits"*) that had excluded the very file it
was about and was several hundred commits stale.

**The standing form of this, earned across three units today:** *"fails
to compile", "once", "unstatable" and "not reached" are all the same kind
of claim, and none of them is checked by rereading the code you just
wrote.*

### Two traps sprung and caught

A fix for a structural finding mints a fresh instance of it — **twice**
in one unit. `map_err(|_| CAP_RIM_CONTRADICTED)` at two sites was a
literal re-instance of `names-flush-and-select-discard-a-refusal-with-map-err-underscore`,
**closed by this program in PR 2378, in this directory**, whose repair
was deliberately structural because *"`map_err(|source| …)` is one
keystroke from `map_err(|_| …)`"*. Repaired the same way: the closure is
gone, the walk returns an enum, every call site is an explicit `match`.

And the variant census was a half-fix until `f6_variants!` landed on
`main` mid-round — a reviewer proved it by adding a `Probe` variant with
no row and watching the test pass.

### What the lane did right, and it is the durable half

It **withdrew** a claim rather than defending it. It **re-took** a census
rather than citing the reviewer's. It **deleted** the false symmetry
sentence and replaced it with the measurement. And it kept the mirror arm
unre-classified on the one ground that survived — the residue fires zero
times, and treating an unreached shape is the trap this unit exists to
avoid — then filed the dead rescue arm rather than leaving it in prose.

Closing line worth keeping: *"if a sentence in there is still wrong, it
is one I checked and got wrong, not one I did not check."*

### Filed out of this unit

`seam-junction-vertex-name-cannot-be-collapsed-by-the-union-fold`,
`the-b-side-contact-record-rescue-arm-never-fires`,
`three-emission-bugs-do-not-speak-the-framing-written-once-for-them`
(all WIRE), and `work/ciw/an-unmergeable-pr-is-silently-ungated-not-visibly-red`
— the lane hit twice a head that carried **zero** check runs because
`main` had outrun `refs/pull/N/merge`, which reads as green unless you
count jobs. On this tree that is a normal state, not an edge case.

Signed (WIRE orchestrator).

## Rows arriving from FIX, 2026-09-20

Ev ruled in chat on 2026-09-20 that **FIX carries no design decisions**:
*"can you kick all the design decisions back to the track they actually
belong to, leaving fix design-free?"* FIX is the program for rows whose
fix is already written; three waves closed those and left a slate that
had drifted into decisions. Fourteen rows moved out by `git mv` to the
track owning the surface each decision is about, each carrying a
`## Re-homed` section stating the question, the routing basis, and what
was NOT decided for the receiver. Routing was taken from
`python3 scripts/work.py territory --files -` over every path the rows
cite, not from FIX's `keep_out` prose — two of that clause's fence
claims were stale and are corrected in the moved rows.

**One row: `node-error-kind-has-no-fieldless-projection`.**
`NodeErrorKind` exists (`crates/editor-core/src/eval/mod.rs`, yours) and
is payload-carrying; there is no **fieldless** projection of it, so three
doors that have the value in hand render it away to prose
(`e.kind.to_string()` into a `cause: String`).

It lands on WIRE because the decision is about the type's shape and the
type is declared here — and your slate already carries a row on it,
`node-error-kind-renders-the-slot-id-through-debug`.

**The decision, and it is the row's whole point:** whether a fieldless
projection is the right answer at all, or whether the three doors should
simply carry `NodeErrorKind` itself — it is already an enum the consumer
could match on, and the reason the other mirrors are fieldless (a payload
the consumer must not depend on) may not apply. *Answer that before
minting a sixth hand-written mirror.* PR 2344 established that a
hand-written mirror's pairing direction is closable by a derive and by
nothing else, and three of them now carry a copied two-part guard; a fifth
hand-rolled pair should not land just because four already have.

**The row's own fence claim was stale and that is part of why it moved.**
It sat on FIX because `crates/editor-core/src/mc.rs` was *"in no open
program's `paths`"*. Territory now says `mc.rs` is PROPS's — as are the
other two consumer doors, `drive.rs` and `stackup.rs`. All three renderings
are PROPS's ground; the declaration is yours.

Separately, CENSUS took `a-new-kind-pair-arrives-unguarded-by-default`,
which asks whether your `product.rs:963` guard earns its keep having never
fired. That half wants your assent, not an announcement.

Signed (FIX orchestrator).

## 2026-09-22 — a note from VGEOM: one more row waits on `need_count`'s siting

(VGEOM orchestrator, announced rather than left to be found.)

`work/vgeom/a-count-slots-cast-still-saturates-for-a-finite-value-too-large`
is now **parked on `need-count-spells-every-failure-as-a-pattern-count`**.
The viewer's `props::SlotValue::of` saturates a finite out-of-`i64`
Count to `i64::MAX` and has no word to refuse with; the variant that
WIRE's row needs minted is the variant VGEOM's row needs raised, so
they are one decision. VGEOM holds the chrome-side evidence and may
not write the `editor-core` door itself (`work/vgeom/program.md`'s
`keep_out`: *"a numeric door the viewer consumes is a hand-off and
never a diff from here"*).

**Nothing here asks WIRE to schedule anything.** When the siting lands,
VGEOM's row is a small chrome-side diff. The evidence VGEOM gathered
for it — that `Expr::count` is total, and that `eval/wire.rs`'s
pattern loop has no ceiling before `names::output_body` inside it —
is already on WIRE's two rows.

- 2026-09-29 — Seam note from ORIGIN: PR 3419 lands step 3 of
  `axis-shaped-identity-channel` (the per-component axis source,
  `crates/topo/src/source.rs` `AxisSource`, and the propagate half of
  P2 in `eval/wire.rs` `place`/`compose_placed`, keyed on the placement
  map rather than the output ordinal). `axis-per-component-source-…`
  is closed and pruned from that row's `blocked_on`; the row stays
  parked on step 2 (EXCH). Two residues on ORIGIN's slate bear on step
  4: `axis-source-lowered-bytes-carry-no-minter-namespace` (before step
  2 stamps), and a sphere row must be stamped with a line through the
  centre. (ORIGIN orchestrator)
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)
- 2026-09-29 — Seam note from TOPO: PR 3467 (`topo/sense-reads-same-chart`, not yet merged) implements Ev's D1 ruling (PR 3480): `FaceSurface::New { surface, sense }` and `Shared { key, sense }` state the new face's bit; on the parent's chart `mef` derives the parent's bit and `mfkrh` its negation, and a contradicting stated bit is refused (`EulerOpError::SenseContradictsChart`); `set_face_surface` takes the same spec and `set_face_surface_and_sense` is gone; `Body::mvfs` and `Body::mfkrh_plug` take the seed's provisional bit. Paths: `editor-core/src/eval/wire.rs`, `editor-core/src/names/emit.rs`, `editor-core/src/names/emit_topo.rs`. In your files every `New`/`Shared` spec, `mvfs` and `mfkrh_plug` call states the bit it carried before; no expected value moved. The PR also files `emit-topo-says-every-face-mints-sense-true` on this slate: a doc here says every face is minted `sense: true`, which has not held since M5 S11. (TOPO implementer)
- 2026-09-30 — Seam note from TOPO: PR 3513 (branch `topo/every-escalation-names-its-decision`) drops the F7 gate's same-operand pair from `BooleanError::UndeclaredCoincidence`: the gate now raises `BooleanError::CoplanarNeighbours`, which `eval::wire::refusal_menu` does not lift, so a same-operand pair no longer becomes an `UndeclaredContact` finding. Two comments in `refusal_menu` that named the F7 case were trimmed. (TOPO implementer)

- 2026-10-01 — Seam note from AUTHOR: Ev ruled on #3551 that an extrude's distance is a positive depth with a structural `side`, and that a negative depth refuses with a recourse naming `side`. The work is filed on EDIT as `extrude-distance-is-a-depth-and-a-side`. Its kernel half is `sweep::Extrusion`/`ExtrudeError` (CARVE/STRUT) and its eval wiring `wire_extrude` (WIRE). The rule's follow-ons are `carve/revolve-angle-is-a-signed-size-beside-a-directed-axis` and `edit/pattern-spacing-is-a-signed-size-beside-a-direction`. (AUTHOR orchestrator)

- 2026-10-01 — Seam note from AUTHOR: Ev ruled on #3587 (A2) that a boolean's and a union's declared pairs become the node's own payload and that `Node::Declare` goes. The work is filed on EDIT as `declared-pairs-are-a-booleans-own-payload`. `eval/wire.rs` `declared_pairs` / `wire_boolean` / `wire_union` and `UndeclaredContactFinding::recourse` change with it. (AUTHOR orchestrator)
- 2026-10-10 — Seam note from CONTACTHOLD: evidence for `a-boolean-drops-its-operands-own-contact-records`. In topo, an operand's records re-enter a boolean only as `BooleanDeclarations::carried_a`/`carried_b` (author declarations), and `ContactRecords::carried()` drops face-granularity records outright. INTENT's F deletes `CarriedContacts` with no replacement named (note left on INTENT's log). The at-rest record fork (`[ev]` PR 4547) would make the carry one function over one list. (CONTACTHOLD orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4811, merged at `abff76bb25`): editor-core's undecided sentences render through `Indeterminate::undecided`. Evaluation, naming and selection are ended by one closed `decision::Decision` table with a kind column; evaluation constructors take a `Decision`, and an unknown name ends loud (`MissingRecourse`). D10 ground keeps main's text through held variants: `NodeErrorKind::MeasureEscalated` (measure distance/gap parallelism), `PlacementAxisEscalated` (`EditError::PlacementAxis`) and `PoseDirectionEscalated` (datum directions and frames, pose `degenerate`). All three classify as Escalated, and pncad-py `tags.rs` and viewer `tree.rs` gained arms. Left for after D10, on the ENCL row: the placement route compares the role word, the naming door's `&str` feed, `placement.rs`'s stale `# Errors` doc, and `datum_unit_norm` vs `eval_direction_norm` ending two ways. (ENCL orchestrator)
- 2026-10-12 — Seam note from SYM: SYM-17 Phase 2 (Ev's answer on PR 4902; `docs/SYM-17-SPEC.md` §Phase 2) replaces `OrthoFrame` with `RigidMotion<T>`, a proper rigid motion carried by construction. `topo::transform_rigid` will take it and ask nothing about rigidity; `check_rigid` becomes its one decided door, for raw maps. The type acts on `UnitVec3`, and pose inverses become transposes. Branch `sym/17-rotation-readout`. Touches linalg, `transform.rs`, `eval/wire`, placement, the mate solve's representatives and inverses (not `member_of` or `check_offsets`, which stage 3 retires), and the revolve, sweep and pattern rigid maps.

## 2026-10-17 — an orchestrator picks up the track

WIRE had no orchestrator since 2026-09-20 and nothing in flight (no open
`wire/` PR, no `wire/` branch on the remote). Status `active`.

The slate is **101/30** and was last read against the tree a month ago,
before INTENT built D10 stages 2, 4 B/E/F (the declare surface is gone)
and stage 3 A/A′. Nobody parked WIRE's rows under the D10 hold. So the
first work is a triage, not a dispatch: each live row read against the
tree as STALE (close), HELD (park on the most specific INTENT row that
fires it, and move to a held successor so this track does not wait on
its slowest blocker), or LIVE; then the live slate is cut to budget on
its priority seam. Three read-only lanes are doing the per-row reads.

Signed (WIRE orchestrator).

## 2026-10-17 — the triage, and the cut it produced

Three read-only lanes read every live row against `fd218851f9`; I
spot-checked the stale verdicts (the declare symbols match nothing in
`crates/*/src`; `topo/src/source.rs` is gone). Of 50 live rows:

- **12 closed as overtaken**, each with a dated `## Closed` section.
  INTENT stage 4 F deleted the declare surface, which took
  `route-declarations-…`, `a-declared-pairs-kinds-…`,
  `select-pair-in-band-…` and `refusal-menu-stamps-…`; D10 retired
  the axis channel and N6's stamps (`axis-shaped-identity-channel`);
  Ev's DM4 fold-order ruling (PR 4323) answered
  `the-pair-verbs-declared-merge-…`; the anchor rewrite went with
  `ProfilePieces` (three rows); `fused_partners` already walks the
  whole chain; SEAT and `crates/verbs/README.md` §5 already record
  `two-verb-seats-…`; the seam-vertex residue row had found no
  witness of its own.
- **11 parked on the D10 hold and moved to WIREHOLD** (opens
  `blocked`), each on the most specific INTENT row: six on
  `transform-retires-into-a-placement`, two on
  `patterns-are-index-variables`, one each on `a-mate-relates-two-poses`,
  `mates-declare-no-contact` and `the-at-rest-census-is-a-check`. The
  P0 `guided-lift-…` is among them: its refusal reads the authored
  frame stage 3 D deletes.
- **1 re-homed**: `a-boolean-drops-its-operands-curve-and-patch-records`
  to CONTACTHOLD, parked on the stage 5 census. Its title is overtaken
  and what is left is topo's carry.
- **26 live, 48 points.** Cut on the priority seam: WIRE keeps P0–P2
  and the four E rows that ride with its first bundle (30/30), and the
  P3/P4 tail goes to WIRETAIL (18/30). Unpriced rows were priced and
  legacy `D` re-priced; four rows were narrowed in place.

Bands: WIREHOLD 13200–13299 and WIRETAIL 13300–13399, recorded in
`docs/MODEL-AB-LOG.md`. Review posture answered in `plan.md`.

Signed (WIRE orchestrator).

## 2026-10-17 — wave 1 dispatched

- **`wire-chord`** (`wire/merged-chord-rule`):
  `a-merged-face-with-several-same-side-constituents-has-no-chord-rule`,
  P0 H. Review tier: FULL single review (a confident wrong name is the
  failure mode). Told to stop on a naming fork or on held ground
  (`PlacedUnion` witness excluded).
- **`wire-endings`** (`wire/refusal-endings`): `need-count-…`,
  `eval-kernel-bug-…`, `wire-profile-spells-…`, `lane-profile-…`, plus
  S40's stale `M6` citations and the dead declare-door prose the triage
  found. Review tier: STYLE single review (refusal text and two
  internal-break refusals; readable and believable). The variant it
  picks is the one VGEOM's parked count-cast row raises.
- **Designer pairs**, blinding bytes on `analysis/design-fork/wire-2026-10-17`:
  FORK-PINCHNAME (the two pinch-naming rows) and FORK-ENTITYDOOR (the
  entity door pair). Neither is yet known to be Ev's; a converged,
  non-ratified-text answer is built without an `[ev]` PR.

`a-union-fold-publishes-no-pinch-row` waits for a lane slot (4-core
box, two builders live); it may also bear on FORK-PINCHNAME.

Signed (WIRE orchestrator).

## 2026-10-17 — FORK-ENTITYDOOR converged in round 1; dispatched

Both designers (A and B) independently rejected the framing (which comes
first at evaluation, the kind or the tie; what key the door takes) and
recommended one final state: the kind is authored data on the name, so
it is the edit and load doors' to refuse, and evaluation reads typed
keys with no entity door. Both found the same off-question defect, the
blend and shell arms' silent `filter_map` kind drops. No ratified page
is contradicted (`Select::fault`'s doc and the stage 2 spec are agent
text), so this is not an `[ev]` fork and draws no fork-log row. The
decision is on both rows; seam note on INTENT's log.

Dispatched as `wire-kind` (`wire/selection-kind-at-the-door`), both rows
in one unit. Review tier: FULL single review — it deletes a door and
moves refusals across layers, but is reversible (one predicate arm).

Signed (WIRE orchestrator).

## 2026-10-17 — FORK-PINCHNAME converged; the rule goes to Ev as PR 5258

Both designers converged in round 1, measuring rather than arguing:
- **The union half is stale.** PR 4881's vertex-run rule and TANG's PR
  5178 made the pinch name order-free: one 99-row table in all six
  member orders. `a-pinch-vertexs-name-depends-on-the-unions-member-order`
  is re-priced `E`; it closes with a pinning test.
- **The split half is a gap in N2.** Both propose that a corner one
  Split half holds as several vertices is `OnToolVertex{of} +
  Fragment(Keeps)` over the corner's own edges each copy ends. That
  extends a ratified N2 sentence and binds future naming, so it is
  Ev's: `[ev]` PR 5258 edits N2 and carries both `For Ev` sections as
  A and B; design-fork log row 156. `needs_ev` is set on the row there.

Acted on from the `For the orchestrator` sections:
- filed `a-mirrored-split-through-a-pinch-refuses-emission-at-an-on-plane-vertex`
  (WIRE, rides with the split row: the runs-below mirror refuses
  `Emission` before the naming question is reached);
- filed `on-tool-vertex-may-be-a-segment-n1-already-makes-redundant`
  (WIRETAIL, a design question on N1);
- `work/issues/a-split-touching-at-a-notch-apex-mints-a-vertex-twice`
  was the same naming defect on a smaller fixture: its naming half is
  cited on the split row (on PR 5258's branch, so the two edits don't
  collide), and the file moved to BIND retitled to its pncad-py half
  (the prose gate panics on a `Debug`-rendered refusal).

WIRE reads 31/30 until the union pin closes its row.

Signed (WIRE orchestrator).

## 2026-10-17 — PR 5262 (refusal endings): style review, mergeable; fix pass sent

The review found no MAJOR. Taken into the fix pass:
- **C2 MINOR, the sharpest:** `CountOutOfRange.admits` stated the
  integer cast's range, not the slot's domain, so a stations count of
  −3 told the author "it admits at least 0", and 0 still refuses
  (Stations ≥ 2). A test pinned the false domain. The fix: each caller
  passes its slot's own domain. One variant, one meaning.
- **C5 half-fix:** `witness.rs` carries the same stale `M6` premise one
  file from the stubs the PR re-cited.
- **C6:** the declare-door prose sweep missed the phrase "declare door"
  and every non-`src` file. The repeat lesson is the discipline's §5:
  name what the pattern cannot match, then search that gap.
- **S1:** `ProfileAnchor { loop_: 0 }` is the fake-coordinate shape the
  PR removed from `wire_profile`, left in an arm it edited.

Class, filed by the fix pass rather than fixed: eval/ handles internal
breaks three ways a few lines apart (`unreachable!`, typed refusal,
silent default), and `run_op` carries node-kind facts as loose Options.

Signed (WIRE orchestrator).

## 2026-10-17 — PR 5262 MERGED (`90a7974563`): four rows closed

`need-count-…`, `eval-kernel-bug-…`, `wire-profile-spells-…` and
`lane-profile-…` closed. Merged over two slow-set reds that are main's
(FUSE's `boss_union` Interval census, #5227; CLEAVE's `review_pick_r2`
tally), commented on the PR per Ev's standing ruling. VGEOM's count-cast
row unparked (its trigger fired); its word is not `CountOutOfRange`.
Residue filed by the unit: `wire-rs-defaults-two-missed-records-to-empty`,
`eval-handles-internal-breaks-three-ways-…`, DOCTAIL's
`hit-test-unnamed-entity-ends-in-a-kernel-bug-tag`, and (from its fix
pass, filed here) `a-fold-steps-coincidence-rows-are-named-without-the-end-passes`.

Signed (WIRE orchestrator).

## 2026-10-17 — PR 5263 (merged-chord rule): full review NOT mergeable, one MAJOR

The reviewer built a legal recipe the rule mis-names: a chord that crosses
the merged-away boundary between the two constituents. The pick read only
the chord's middle, so it cited one piece confidently (and flipped with the
cut's extent), and the vertex pass then refused a bug-framed `Emission`
where main had refused typed `MergedChordConstituents`. Its sibling
`rim_holding` reads the whole chord. Fix pass sent: pick only a constituent
that holds the whole chord, otherwise keep the typed refusal; propagate
containment errors; restore a guard that a legal recipe never reads as a
kernel bug. The lesson is the brief's: "the fixture passes" is not "the
rule is stated", and the reviewer's mutants proved the fixture caught wrong
picks but not wrong premises.

Filed `a-chord-spanning-two-merged-constituents-has-no-name` (design): what
a spanning chord is named, and whether N2 should state the chord-descent
rule at all (new ratified text, so Ev's if so).

Signed (WIRE orchestrator).

## 2026-10-18 — dispatched `wire-pinch` (`wire/union-fold-pinch-row`)

`a-union-fold-publishes-no-pinch-row` with the union pin that closes
`a-pinch-vertexs-name-depends-on-the-unions-member-order`. Review tier:
FULL single (which rows a fold publishes is a confident-wrong-record
failure mode). Told to stop on held ground or a fork over which pinch rows
a fold publishes.

Signed (WIRE orchestrator).

## 2026-10-18 — FORK-PARTCACHE converged in round 1; dispatching

Both designers rejected the framing (how to make the lock safe) and
recommended one final state: the top evaluation enters its parts in a
serial pre-pass, as nested evaluations already do, carrying rows across
evaluations from the prior; the cache becomes an immutable lock-free
table. The decision is on `part-cache-lock-held-across-a-nested-rayon-join`;
WIRETAIL's `part-cache-miss-samples-…` moved here to ride with it. Both
struck the row's fresh-pool sketch as unsound by reading rayon-core's
`in_worker_cross`. Not Ev's (agent-written text only); no fork-log row.

Review tier: FULL single — a new evaluation stage, but reversible.

FORK-FOLDPINCH: designer B reported that ratified DM4 (PR 4323) already
answers the fold-pinch question (publish the judgement's pinch rows whole,
in member space; a covered contact keeps its pair's row). Waiting on A.

Signed (WIRE orchestrator).

## 2026-10-18 — correction: FORK-PINCHNAME is design-fork log row 157

INTENT recorded its own row 156 concurrently; the merge of main into PR 5258 renumbered this program's row to 157. The entry above saying "row 156" means 157.

Signed (WIRE orchestrator).

## 2026-10-18 — PR 5263 MERGED (`6bf23859b7`): the P0 merged-chord rule

Three rounds: full review (one MAJOR: the middle-point pick named a
chord spanning both constituents confidently, and the pick flipped with
the cut), fix pass (pick only an unjoined chord whose middle one
constituent holds; ends checked), delta review (mergeable; every wrong-pick
mutant caught, the join premise unbroken), a last small push. CI fully
green on the head. P0 row closed; the spanning chord's name stays on its
own design row. Merged before it: PR 5258 (`5be81cde65`, the N2 Split
vertex `Keeps` rule Ev approved) and PR 5270 (`56a650efc7`, the union
pinch-name pin and the FORK-FOLDPINCH decision).

Signed (WIRE orchestrator).

## 2026-10-18 — PR 5264 MERGED (`a3a5bf6d05`): a selection's kind is the doors'

FORK-ENTITYDOOR built; both rows closed. Full review approved with fixes
(the author-facing refusal had lost the verb and the name); fix pass gave a
kind fault its own sentence and checked every name against an `Is` seat's
kind. Merged over CLEAVE's `review_pick_r2` red, commented on the PR.

Signed (WIRE orchestrator).

- 2026-10-18 — PR 5275 fix pass adjudicated by the orchestrator's delta
  read (the closed in-plane edge is credited; restoring the guard reds
  the revolved-apex pin; Keeps pinned as literal name vectors, two
  mutants red); merged on green; the tip-copies and mirrored-split rows
  closed. Filed by the lane: `split-keeps-has-two-builders-that-disagree`,
  `a-revolved-pinch-split-refuses-with-its-normal-flipped`,
  `split-and-boolean-copy-chains-are-chased-three-ways` (all P3).
  (WIRE orchestrator)
- 2026-10-18 — PR 5276 fix pass adjudicated by the orchestrator's delta
  read (dedup runs before any record cites a row index; `fold_cell`
  refuses an unstripped minted head as it does a member head); merged on
  green; `a-union-fold-publishes-no-pinch-row` closed. (WIRE orchestrator)
## 2026-10-18 — FORK-CIRCLEPHASE to Ev (PR 5278); PR 5273 reviewed

- **FORK-CIRCLEPHASE converged:** both designers found the circle-split
  row's premise reads a name as a point set, which N1 rules out; a piece is
  its index, a phase edit moves it continuously, and a whole-arc shift is
  not a re-aim. No code change; the one sentence stating it in N1's
  `circle_split` bullet is Ev's (`[ev]` PR 5278, fork-log row 158). The row
  closes with a pinning test after.
- **PR 5273 (part pre-pass):** full review approved with fixes. The
  design held under every executed claim (schedule-independence at 1–16
  threads, cancel mid-pass, carry keyed on content). Taken into a fix pass:
  the schedule row could not go red at its own fixture (a nested fixture
  does), two carry widenings undisclosed (resolver `None` and an unrepinned
  store now evaluate on a pose edit), ASSEMBLY A2/A11 prose not matching
  the code, and the stale "lazy part cache" prose the sweep missed in
  `mate/`.

Signed (WIRE orchestrator).

## 2026-10-18 — FORK-SPANCHORD to Ev (PR 5279); it would retire PR 5263's pick

Both designers converged in round 1, and both reached past the question:
the pair boolean spells one merged face two ways (its `Borders` walls cite
the parent `Merged` set, its seam pass descends to a constituent), and
Ev's N3 ruling (PR 3222) already has a union's seam cite the parent. The
recommendation is to widen N3's seam sentence to every boolean and delete
the chord descent — including the constituent pick PR 5263 built and
merged yesterday. That is not a waste to defend: PR 5263 made legal
recipes name that refused, and its review proved the pick sound where it
picks; the parent rule names those and the spanning ones too, with a name
stable as the cut moves. `[ev]` PR 5279, fork-log row 159.

Signed (WIRE orchestrator).

## 2026-10-18 — PR 5273 MERGED (`7e636c2bca`): a part is entered once, before the node map

FORK-PARTCACHE built; both rows closed. Full review approved with fixes;
the lane's own schedule row could not go red at its fixture, and the
reviewer's nested fixture could (red on the old code); the carry's two
widened behaviours are now disclosed; ASSEMBLY A2/A11 describe the code.
CI fully green. Seam note on GUARD's log (the census roster line).

Signed (WIRE orchestrator).

## 2026-10-18 — PR 5275 (split vertex Keeps): full review NOT mergeable, one MAJOR

The reviewer split 43 documents at head, at the merge base and under four
mutants. No existing name moved; `pair_original` is right; the in-plane
chord reading never credits the wrong edge. The MAJOR: a closed in-plane
edge (a revolved notch's apex circle) is never credited, so a curved pinch
gets a confident name that is not N2's — the guard `o_far != corner`
discards every closed edge, uncommented. Two mutants survived (Keeps
order reversed; `chord_within` replaced by `true`): the pins checked
counts, not spellings. Fix pass sent; three class rows to file
(two Keeps builders, a −y orientation that refuses where +y names, three
hand-written lineage chases).

Signed (WIRE orchestrator).

## 2026-10-18 — PR 5276 (fold rows in member space): mergeable, two duplicate classes taken into a fix pass

No MAJOR; every pinch row it now publishes is true; the lint's findings equal
the row count over 11 fixtures in every order. Two duplicate-row classes,
mostly pre-existing: identical fold rows pushed twice (the citation only
searched judged rows), and a repeated read making two identical judged
rows. One keyed table over (unordered cells, relation) for judged and
already-published fold rows closes both. The pin only asserted presence;
it now asserts the multiset.

Signed (WIRE orchestrator).

- 2026-10-18 — PR 5303 fix passes adjudicated by the orchestrator's
  read. One further item came from that read: two judged rows merged
  whatever their sites, PR 5276's own dedup, the same class. Each
  mutant reds its test. Merged on green; the fold-minted row closed.
  (WIRE orchestrator)
- 2026-10-18 — PR 5298 fix pass adjudicated by the orchestrator's read.
  `NodeStanding`'s Failed and Poisoned sentences match main again, and
  their entries stay filed against the WIREHOLD row. One category map
  now drives both the `NamingError` endings and the framing test. Kept
  labelled: `NotEvaluated` and `NotInDocument`, which the WIREHOLD row
  does not dispute. Merged on green; the row closed. (WIRE orchestrator)
- 2026-10-18 — Ev approved PR 5278 ("sounds good!"): N1's `circle_split`
  sentence merged (34583bffff); fork-log row 158 filled (byte 40, even:
  A = Opus, B = Fable). Dispatched `wire/circle-phase-pin` (pinning test,
  closes `a-split-circles-phase-edit-re-aims-its-piece-names-silently`);
  review tier: orchestrator's read (a test and a row close, no kernel
  change). (WIRE orchestrator)

- 2026-10-18 — PR 5294 (circle-phase pin) merged on green and the
  orchestrator's read; `a-split-circles-phase-edit-re-aims-its-piece-names-silently`
  closed. The fillet-on-a-seam row was skipped: a split circle's seams
  join one cylinder to itself with no crease, so a blend has nothing to
  carve. (WIRE orchestrator)

- 2026-10-18 — Merged PR 5276 (fold rows in member space) and PR 5275
  (split vertex Keeps) on green after main merges. Dispatched
  `wire/prism-crossing` (now unblocked: 5275 merged in the same file)
  and `wire/fold-minted-cells` (DM4's member-cell spelling for
  fold-minted cells, plus one margin per unordered pair); both FULL
  single reviews (naming units). (WIRE orchestrator)

- 2026-10-18 — The recourse lane opened PR 5298 (`wire/refusals-state-their-recourse`),
  green; STYLE review dispatched with four claims (new recourses true;
  no held family or D10 ground touched; no double endings, through the
  shared `NodeStanding` too; `NamingError::ending`'s category split).
  `MeasureRefUnreadable/Readback` stays filed: its sentence is
  `topo::ReadbackError`'s, which held `FaceFrameReadback/*` also
  renders. The lane filed `naming-refusal-census-skips-ten-arms` (P3 E).
  (WIRE orchestrator)

- 2026-10-18 — PR 5298 style review adjudicated: MAJOR upheld. The PR
  labelled `NodeStanding`'s repair sentences as `Recourse:`, and
  `wirehold/kernel-standing-names-a-cluster-refused-node-as-its-own-failure`
  records them as false for a node a mate cluster refused, which is held
  ground. They go back to unlabelled, and their census entries stay filed
  against that row. Minors sent too: VarKindMismatch half-recourse;
  Vanished doubling after a diagnosis with its own repair, and Cascade
  pointing downstream; the ProfileLaneReplay point-pass premise. Class
  note: a unit that adds a category-keyed ending can mint a second
  hand-written category map beside the test's (`NamingError::ending` vs
  `expected_framing`); the fix is one map. (WIRE orchestrator)

- 2026-10-18 — Ev approved PR 5279 ("that recommendation sounds good!").
  N3's seam sentence was widened and merged (da3f0bf530). The fork-log
  row was renumbered from 159 to 160: INTENT's row 159 landed first. It
  is filled with byte 197 (odd): A = Fable, B = Opus. The build unit
  (spanning chord rebuild; delete chord descent, `constituent_holding`
  and `MergedChordConstituents`) is held until PR 5298 merges. That
  fix pass is rewriting the naming category map that lists the variant
  (`emit.rs`, `wire_legal_union_refusals.rs`). Sequencing call;
  the alternative was dispatching now and rebasing the map twice.
  (WIRE orchestrator)
- 2026-10-18 — PR 5302 (prism crossing): already fixed on main by PR
  4816, as bisected, so the row's reading was refuted. The PR is the
  order test alone, so its review tier dropped from FULL to the
  orchestrator's read: no naming change is left to review. Merged on
  green and the row closed as overtaken. (WIRE orchestrator)

- 2026-10-18 — FORK-INPLANE: `a-split-names-an-operand-edge-in-its-plane-as-a-section-chord`
  goes to a designer pair (Opus + Fable, problem only). It asks how
  an in-plane operand edge and its copies are named, and whether N2's
  face `Keeps` cites it. N2 is ratified, so a change goes to Ev. The
  blinding draw is on `analysis/design-fork/wire-2026-10-17`.
  (WIRE orchestrator)

- 2026-10-18 — FORK-INPLANE, round 1: the designers agree the upper
  copies stay `SectionEdge` chords, and that today's name for the lower
  leftover edge is build order (`frag_rows.root` of a section face),
  probably also the `-y` revolved refusal. They split on two points.
  The leftover edge: own name, or merge the coplanar section faces at
  the split's output so it goes. Keeps: strike N2's in-plane clause, or
  extend it to faces with a recorded trace. Round 2 dispatched, each
  given the other's report. (WIRE orchestrator)

- 2026-10-18 — FORK-INPLANE converged after five rounds and three
  crossovers. Agreed: the split finishes halves maximal; chords stay
  chords; `Keeps` reads descent, with N2's in-plane clause struck; every
  merged group is `Merged`. `[ev]` PR 5306 edits N2/N3, fork-log row
  162. Filed `a-section-faces-index-is-its-completion-order-not-its-identity`
  (P3, design). The build unit waits on Ev. (WIRE orchestrator)

- 2026-10-18 — PR 5303 FULL review adjudicated: approve with fixes. The
  orchestrator upgraded one finding to MAJOR. Spelling a fold-minted
  crossing as its member edge lets `stated_at` (cells plus relation,
  site ignored) merge two crossings of one edge into one row. That is
  latent (0 hits in the probe), but it is the defect class this unit
  closes. The fix makes the site part of a statement. Minors sent: no
  test pins re-cited `cites` (a mutant citing row 0 stays green on 288
  tests); the overlap test is restricted to straight colinear edges;
  `fold_cell`'s swallowed `Err`; stale docs; mint-ordinal A disclosed.
  Class note: canonicalising a spelling to make rows order-free can
  merge distinct facts unless the dedup key keeps what told them apart.
  (WIRE orchestrator)

- 2026-10-18 — PR 5298 (refusals state their recourse) merged on green
  with main merged in. Dispatched `wire/seam-cites-parent`: N3's seam
  sentence, approved on PR 5279, is built, and chord descent with
  `MergedChordConstituents` is deleted. It gets a FULL single review.
  (WIRE orchestrator)

- 2026-10-18 — PR 5303 (fold-minted cells) merged on green with main
  merged in; its row closed. PR 5313 (seam cites parent) opened green,
  and a FULL review is dispatched. Dispatched `wire/small-breaks`, one
  STYLE-reviewed bundle of four rows:
  - the seating door's kind-mismatch refusal;
  - `wire.rs`'s two defaulted records;
  - the cancel check before the mate solve (mate/* untouched, D10);
  - the fold-row guard, which closes
    `a-fold-steps-coincidence-rows-are-named-without-the-end-passes`
    now that 5276 and 5303 removed the unheld cells.
  `naming-refusal-census-skips-ten-arms` waits for 5313, which deletes
  one of its arms. (WIRE orchestrator)

- 2026-10-18 — PR 5313 FULL review adjudicated: mergeable with fixes.
  - Fix pass sent: tests that reach the `(true,true)` arm and pin
    `of_published`, since both stay green under revert; a refusal that
    does not depend on key order (it is a pncad-py tag); the `holds`
    residue in `merged_side`.
  - The side-order choice gets disclosed and filed. Row 160's
    recommendation leaned to name-ordered seam sides, Ev approved "as
    recommended", and N3's text is silent. So it is a row, not a
    breach, and no switch happens in this PR.
  - PR 5316 (small breaks) opened green and a STYLE review is dispatched.
  (WIRE orchestrator)

- 2026-10-18 — PR 5316 STYLE review adjudicated; no MAJOR. Fix pass:
  - The fold-row guard checked only member cells. No order published a
    fold-minted cell, so the case the fold-steps row was about went
    unexercised. The fix needs a fixture that publishes one, or an
    honest Resolution; the check also has to read the entity a name
    denotes, not merely that it exists.
  - One spelling for the `lost` flag, through a `try_recited` if that
    is small.
  - Kind checked before collision at every seating door.
  - The cancel test asserts its premise.
  Class note: a guard over a corpus can be vacuous on the half it was
  written for; measure that it fires. (WIRE orchestrator)
- 2026-10-18 — PR 5313 fix pass adjudicated by the orchestrator's read
  of `rim_among`: one chooser, decided by name. A sole holding rim wins
  over an escalated pair, which is sound because two distinct rims of a
  valid body cannot overlap along a chord. The `(true,true)` arm is
  pinned by unit rows on its extracted decision, since no model
  reaches it. Merged on green; the spanning-chord row closed.
  (WIRE orchestrator)

- 2026-10-18 — Sequencing: `naming-refusal-census-skips-ten-arms` waits
  for PR 5316, which adds `KindMismatch` to the same census lists
  (`refusal_concision_chains`). The alternative was dispatching now and
  resolving the list twice. (WIRE orchestrator)

- 2026-10-19 — PR 5316 fix pass adjudicated by the orchestrator's read.
  - The strengthened guard found the fold-steps defect still live. The
    prism-crossing union with the plate publishes two fold-minted
    `Crossing` vertices that its table does not hold: `Crossing { edge:
    Seam{..}, face: plate Cap(End) }`.
  - So `a-fold-steps-coincidence-rows-are-named-without-the-end-passes`
    stays open with the measurement. Its acceptance is adding that
    fixture to `published_row_cells_denote_entities_of_their_reads`.
  - `topo::ContactRecords::try_recited` was added (CLEAVE ground,
    15 lines).
  - Four source-scanning tests moved to the slow set when `topo` entered
    the change scope.
  - Merged main to resolve one slow-set conflict; merge on green.
  (WIRE orchestrator)

- 2026-10-19 — PR 5316 merged on green; three of its rows closed. The
  fold-steps row stays open. Dispatched:
  - `wire/naming-census`: the nine unrostered `NamingError` arms plus
    `a-short-tie-refuses-as-a-duplicate-name`, STYLE review.
  - `wire/fold-steps-end-pass`: publish the two fold-minted seam
    crossings under the names the finished table holds, per DM4, and
    add the fixture to the guard as its acceptance. FULL review.
  (WIRE orchestrator)

- 2026-10-19 — PR 5324 STYLE review adjudicated: mergeable with a fix
  pass.
  - The 75-word cut took facts out of three missing-rule sentences (who
    tied; what failed; "between them"), so the words come back out of
    the shared framing instead.
  - `ShortTie`'s Display wording is fixed.
  - The held-ground `FILED_NO_RECOURSE` entries get parked homes.
  - Two rows filed rather than fixed: entity collisions reported as
    "minted twice", and the hand-kept payload lists.
  Class note: a word budget met by cutting content, not framing, loses
  the fact the reader acts on. (WIRE orchestrator)

- 2026-10-19 — Ev approved PR 5306 ("sure, sounds good"). N2/N3 merged
  (2b49303ad9). The fork-log row was renumbered 162 → 163 because an
  INTENT row landed first; it is filled with byte 153 (odd): A = Fable,
  B = Opus. Dispatched `wire/split-maximal-halves`:
  - the kernel merge at the split's finish, which crosses CLEAVE's
    `splitting/finish.rs` (no CLEAVE branch touches `splitting/` today);
  - the in-plane edge rule;
  - one descent `Keeps` builder.
  It closes or verifies the two-builders, `-y` revolved, section-root
  and CLEAVE split-half rows. FULL single review; not dual, since one
  call at the finish reverses it. (WIRE orchestrator)

- 2026-10-19 — Container restart stopped three lanes: fold-steps, the
  census fix pass and split-maximal-halves. Their worktrees survived;
  fold-steps and split-maximal had pushed nothing. Disk was at 93%, so
  I deleted the census lane's target dir and two stale `[ev]`
  worktrees, which brought it to 79%. All three lanes resumed, told to
  push early and build with `CARGO_INCREMENTAL=0`. Friction note: three
  lanes' target dirs (12 + 5 + 4 GB) nearly exhaust the allowance.
  (WIRE orchestrator)

- 2026-10-19 — PR 5324 (naming census and short ties) merged on green
  with main merged in; its two rows closed. The fix pass restored the
  dropped facts by shortening the shared `UNRULED_FRAMING` from 33 to
  13 words. PR 5328 (fold-steps end pass) opened green, and a FULL
  review is dispatched. Its first claim is that the junction row
  appears in only 2 of 24 orders, a possible DM4 order-dependence the
  lane disclosed but did not check. (WIRE orchestrator)

- 2026-10-19 — PR 5328 FULL review adjudicated: not mergeable as is.
  - The blocker, sent back to the lane: `fold_vertex_forward` refuses
    `Settle(Dead)` where `Links::judged` treats the vertex as consumed,
    so a fifth member covering the junction makes two orders refuse
    that built before. The fix pass also covers forwarding only cited
    rows, a site check in the guard, the cone-aware settle,
    `name_of` for the published name, and one helper with
    `in_member_space`.
  - The review's M1/M2 are pre-existing: row PRESENCE depends on member
    order in `crossing_one_wedge` and `flush_onto_an_edge`, on main too.
    That measures the three-member design row for unions. FORK-ROWPRESENCE
    goes to a designer pair (problem only; blinding draw on the analysis
    branch). DM4 says "one row in every order", so a rule change there
    would be Ev's.
  (WIRE orchestrator)

- 2026-10-19 — FORK-ROWPRESENCE, round 1.
  - The designers agree: rows spelled as sets of member cells (k-ary),
    never fold-minted names; DM4's "one row in every order" sentence
    changes.
  - They split on presence. One says fold decisions are rows whose
    presence follows the author's fold order. The other says only
    order-free sources count: judged rows; implied fold decisions are
    not rows; irreducible ones are published where the finished body
    holds them.
  - Round 2 sent with an exchange. The second designer must measure its
    premise that the four order-dependent statements are implied by
    judged rows.
  - PR 5328's merge is held until the fork settles: both answers retire
    its published-name mechanism. Its fix pass (the BLOCKER) continues.
  (WIRE orchestrator)

- 2026-10-19 — FORK-ROWPRESENCE, round 2 crossed.
  - The second designer measured that every order-dependent statement
    is implied by judged rows, but only 2 of 6 through lineage; 4 need
    line and plane theorems. Every step decision is cited (106/0). The
    pencil records nothing even in `topo`.
  - It then moved to "every decision is a row, presence follows order".
    The first designer moved to "order-free via a closure over judged
    carrier identities", with a proof that surviving coincidences are
    met in every order.
  - Round 3 asks each to answer the other, plus the question beneath
    both: who reads published coincidence rows, and is it one product
    (the door's per-evaluation log) or two (that log, and an order-free
    fact set)?
  - Filings once the fork settles: `topo` records no coincidence for a
    face through a seam (the pencil); the (a, c) pair records no row for
    colinear apex edges; `cite_rows`' citing filter; the guard pins
    `minted > 0`.
  (WIRE orchestrator)

- 2026-10-19 — FORK-ROWPRESENCE converged in round 3: a node's rows are
  its evaluation's decision log, with the judged rows as the order-free
  subset, marked on the row. Every reader of the rows reads per
  evaluation. `[ev]` PR 5329 edits DM4; fork-log row 167; the blinding
  byte is on the analysis branch.
  - Filed `cleave/a-boolean-records-a-zero-decision-only-where-a-record-cites-it`
    (P1, design): the pencil, and the (a, c) pair with no row. It is the
    prerequisite either way.
  - Filed `wire/the-fold-row-guard-pins-a-row-naming-a-fold-minted-cell`.
  - PR 5328 waits on Ev: both converged answers retire its
    published-name mechanism, though its BLOCKER fix may survive into
    the build.
  (WIRE orchestrator)

- 2026-10-19 — PR 5328 fix pass, green at 67b0dcd469.
  - The blocker is fixed: forwarding follows `Links::judged`'s `place`,
    with `settle_cones`, and the 120-order cover test builds in every
    order.
  - Deviation accepted. A cited fold row whose vertex a later step
    swallows keeps its step's spelling, as main does, so no order
    refuses. Filed as
    `a-cited-fold-row-on-a-vertex-a-later-step-swallows-keeps-its-step-spelling`
    (P2).
  - Merge held pending Ev on `[ev]` PR 5329. Both converged answers
    replace 5328's published-name mechanism with member-cell spelling.
    If Ev declines 5329, 5328 merges as is; if Ev approves, the 5329
    build carries over 5328's cover test and guard site check.
  (WIRE orchestrator)

- 2026-10-19 — Ruling for `wire/split-maximal-halves`. A split's merge
  group with no section face (a cylinder's two half-walls on one
  carrier, merged across the profile seam once the cut opens the turn)
  is `Merged` of the parent operand faces, with `Keeps` for several
  pieces of one parent, as the boolean names the same shape
  (`a_slot_across_a_sunk_boss…`). This is a faithful elaboration of N3:
  a merged face is `Merged` of its constituents, and the boolean already
  does it. Rejected:
  - merge only section groups: DESIGN's maximal faces;
  - name pre-merge pieces: no finished body holds them.
  The lane adds one N3 sentence and re-baselines the two edge-lineage
  pins whose pieces now merge. The `Joined::Seam` spelling is flagged
  for review. The lane crossed CLEAVE (`finish.rs`, `mod.rs`) and TOPO
  (`euler_ring.rs`, `merge_faces.rs`); the TOPO crossing was forced by
  the revolved half's `RechartStrandsDescriptions{RingMove}`.
  (WIRE orchestrator)

- 2026-10-19 — Second ruling for `wire/split-maximal-halves`. Once a
  cylinder's half-walls merge, the join joins the rim across the
  profile seam, covering pieces of several operand edges and none
  whole. It is named as the union names a flush rim along several
  member edges: `Merged` of those edges, with `Ends` to tell pieces
  apart.
  - The README's "pieces of several operand edges, is refused, not
    named" is agent text (100587fd5d, an implementation commit; `git
    log -S` finds no Ev ratification). It was a stop case, so it is
    re-worded, not waited on.
  - The refusal stays for a chord joined to a piece.
  - Faces are linked per half, not across halves, because the merges
    are recorded per half.
  (WIRE orchestrator)

- 2026-10-19 — Ev approved PR 5329 ("sure, i don't have much intuition
  about this but it seems fine"). DM4 merged (72a204ff73). The fork-log
  row was renumbered 167 → 169 and filled with byte 128 (even):
  A = Opus, B = Fable.
  - PR 5328 closed unmerged as superseded, with a comment. Its two filed
    rows were copied here, and its cover test, site check and
    `settle_cones` reading carry into the build.
  - Build unit 1 dispatched: `wire/boolean-rows-every-decision` (CLEAVE's
    `cite_rows` row, crossed by WIRE as the row invites), FULL review.
  - Unit 2, `wire`'s k-ary rows with an origin mark plus re-pinned order
    tests, follows it.
  (WIRE orchestrator)

- 2026-10-19 — PR 5334 (split maximal halves) opened. `gate ok` is red
  on two eps-1e-12 sweep rows:
  - `band_turn_overrun_fillet::the_needle…`, also red on the 10-18
    nightly;
  - `reach_cone_split::steep_frusta…`, which panics before any split
    runs. Its presence on main is unconfirmed.
  A FULL review is dispatched, with the CI-origin check as claim 6. The
  review also covers:
  - the lane's assumption that a joined edge runs with the operand edges
    it covers;
  - the re-pinned tests whose subject the merge removed;
  - the seven gui render cells the lane `git rm`'d as stale renames.
  (WIRE orchestrator)

- 2026-10-19 — PR 5334 FULL review adjudicated: mergeable after a fix
  pass, with no wrong answer shown. Both 1e-12 reds are not this PR's:
  they appear identically on the 2026-10-19 nightly. `steep_frusta` was
  filed on FLUX (likely PR 5038); the needle was already filed.
  - MAJOR: the skip record was dropped, unlike the boolean.
  - MAJOR: a new `PinchCornerEscalated` acceptance with no row.
  - MAJOR: the TOPO design row and door census are stale for
    `ring_move_unasked`.
  - Minors: `SetPiece` sense when operand edges run opposite ways; bare
    `Seam` joins colliding; re-pinned test names; a lost non-vacuity
    guard; the `volume_pad` ceiling; a possible lineage `minted`
    mislisting.
  - The dispatch premise was corrected: the gui cells were main's own
    stale copies, retired by the `ciw` procedure.
  (WIRE orchestrator)

- 2026-10-19 — Build unit 1, PR 5352 (`wire/boolean-rows-every-decision`),
  opened green. Every margined decision is now a row: the pencil's step
  records 2, and the (a, c) wedges publish 2.
  - It also fixed a latent bug the filter had hidden: a sphere cut-in's
    rows named clone-only cells.
  - Policy provenance is agent text (`53ec9087fe`, `cdafc97bad`).
  - Four CLEAVE rows filed: the `glue::touched` filter; `SplitOn`'s
    two-run filter; the re-chart arm; a pre-existing Villarceau fuzz
    seed.
  - FULL review dispatched.
  - Unit 2 must drop `wire_combine`'s #4323 citing filter: the pencil
    union still publishes only judged rows.
  (WIRE orchestrator)

- 2026-10-19 — PR 5352 FULL review adjudicated: not mergeable as is.
  The filter and its renumbering hold.
  - MAJOR: two clone-to-input cell maps (`ChartCuts::input_cell`'s
    meridian arm, and `ops::input_cell` for minted vertices) widen a
    cell but keep the relation. So the PR publishes false rows: a cap
    face OnCarrier a brick face, twice, and the pencil's seam OnCarrier
    rim, where the truth is a crossing at a point.
  - Fix pass: publish only rows whose input spelling is true. Hold
    widened decisions as typed "decided, not yet spelled" entries, for
    unit 2's k-ary meeting-dimension spelling to publish. Put both maps
    in one function.
  Class note: a cell map that changes a cell's dimension must own the
  relation, or it turns a crossing into a coincidence. (WIRE orchestrator)
- 2026-10-19 — PR 5334 fix pass adjudicated by the orchestrator's read;
  merged over the two known main reds (the needle, `steep_frusta`, both
  on the 2026-10-19 nightly), with a comment.
  - The skip record is shipped. No pose moved on the pinch corner: only
    the finding changed, and that is filed.
  - The TOPO row was updated.
  - One digest (seat7/seat8 `cut_cylinder`) was re-blessed. The lane
    found identical edge descriptions and no moved point, but did not
    isolate the cause; noted here.
  - Five rows closed (the in-plane, two-builders, `-y` revolved,
    section-root and CLEAVE split-half rows).
  (WIRE orchestrator)

- 2026-10-19 — PR 5352 fix pass adjudicated by the orchestrator's read;
  merged on green.
  - One spelling door; widened decisions are held, not published, so the
    cap's face-face and the pencil's edge-edge rows moved to held.
  - Exception accepted: a decision a surviving record cites is still
    published with its old spelling, as on main, because the record
    needs a row. That is pre-existing and filed on CLEAVE.
  - Twins still arise on coincident lines (filed). CLEAVE's citing-filter
    row is closed.
  - Next, unit 2 (`wire`):
    - k-ary member-cell rows with an origin mark;
    - spell the held decisions;
    - drop `wire_combine`'s #4323 citing filter;
    - re-pin the order tests;
    - carry over the closed PR 5328's 120-order cover test and site
      check.
  (WIRE orchestrator)

- 2026-10-19 — PR 5352 merged; CLEAVE's citing-filter row closed.
  Dispatched unit 2, `wire/union-decision-log`:
  - k-ary member-cell rows with an origin mark;
  - spelling the held `UnspelledDecision`s;
  - dropping #4323's citing filter;
  - the door proving k-ary rows;
  - re-pinned order tests, and 5328's carried tests.
  Re-triaged to the DUAL review tier: it changes the published
  `NamedCoincidence` type (Python's `coincidences`) and the door's proof
  shape, which is broad and hard to reverse. The lane stops if a
  truthful meeting relation needs more than `CoincidenceRelation`
  states. (WIRE orchestrator)

- 2026-10-19 — Ruling for unit 2. The lane stopped at the brief's step 2:
  `OnCarrier` needs a distinguished point cell, and a fold-minted
  point's member-cell spelling has none. Ruled option A: a new
  `topo::Relation::Meet`, unordered and k-ary ("the cells' carriers
  share one point"). It applies where the point was widened, or where a
  set has three or more cells; two-cell rows with a member vertex keep
  `OnCarrier`.
  - Why the orchestrator rules it: DM4, approved on PR 5329, already
    says "the set of member cells that meet". No design page lists the
    relation set (checked DESIGN.md and the crate READMEs). B, keeping
    the point apart, spells one point two ways by order, against DM4.
  - `Meet` also publishes topo's held `UnspelledDecision`s and CLEAVE's
    15 cited misstatements truthfully.
  (WIRE orchestrator)

- 2026-10-19 — Unit 2, PR 5378 (`wire/union-decision-log`), opened
  green at 998ee55c3f.
  - `Relation::Meet`; k-ary rows with `origin`; the citing filter
    dropped; `member_cells` spelling.
  - Deviation: `Meet` proves by `MarginIdentity`, with no k-carrier rung.
  - The lane filed a residue: a Meet on a boolean-minted vertex omits the
    cell that minted it.
  - DUAL review dispatched under protocol `24b76fff0e`. Class H, so the
    CONCURRENT arm: R1 and R2 on frozen head 998ee55c3f, with identical
    briefs (`brief-review-5378.md` in the session scratchpad). No
    method relaxations.
  - The lane also reports main's slow-set entry
    `census::…review_5006_delta::d1_tabs_a_few_bands_wide` listed three
    times. That reds the slow-set name check for any topo-touching PR;
    the PR dedups it.
  (WIRE orchestrator)

- 2026-10-19 — PR 5378 dual review delivered. R1 and R2 are both
  APPROVE-WITH-FIXES with no MAJOR, so there are no tally candidates.
  The pair is flagged: each glimpsed only the other's directory or probe
  names, after or apart from its findings. The pre-note is in the
  session scratchpad (`rev5378-prenote.md`); the dual-review row rides
  the PR as its last commit at merge.
  - Fix pass on the union: `restated` grouped by point, not by any
    shared cell, and projected to Python and the viewer; a completeness
    guard that a dropped cell turns red; the collapse requires the site;
    the catch-all refuses; one `Meet` constructor; the vacuous
    determinism test fixed or deleted.
  - Q4 adjudicated: naming `Meet` in DM4 stays as the orchestrator's
    elaboration of the approved "set of member cells that meet". The PR
    body flags it for Ev's retroactive read.
  (WIRE orchestrator)

- 2026-10-19 — PR 5378 (unit 2, DM4's decision log) merged (248bf8cfaa).
  - The concurrent dual (R1, R2) were both APPROVE-WITH-FIXES with no
    MAJOR. The blinded coding (byte 214, even: A = R1) found 17 groups:
    9 bilateral, 4 unilateral A, 4 unilateral B. 0 tallied.
  - The row `docs/dual-review-log/rows/PR-5378.md` rode the PR as its
    last commit.
  - The `FoldRow::Unnamed` hold-out was narrowed to the table-miss error
    before merge.
  - Seven rows closed: the three-member, fold-steps, guard-pin,
    cited-fold-row, minted-seam and two-member-margin rows, plus CLEAVE's
    cited-misstatement row.
  - Open residues filed on CLEAVE: the Meet that omits the minting cell;
    the ring-tie pierce vertex; twins; the in-place Zeros.
  (WIRE orchestrator)

- 2026-10-19 (WIRE orchestrator) Triage: should-pair-seams-take-name-ordered-sides (P3 M, design), a-chord-between-two-merged-faces-within-no-rim-refuses (P3 M), a-split-merging-a-two-piece-wall-reads-its-seam-vertex-as-a-pinch-corner (P3 M). a-literal-placement-frame-crosses-scalars-by-closure… is D10 ground (placement): moved to work/wirehold, parked on transform-retires-into-a-placement.

- 2026-10-19 (WIRE orchestrator) Dispatched wire-p4 (branch wire/p4-refusal-bundle): a-seated-entity-collision-refuses-as-a-name-minted-twice + editor-payload-lists-other-than-naming-are-kept-by-hand, one PR, STYLE review.

- 2026-10-19 (WIRE orchestrator) Dispatched wire-chase (wire/one-chain-chase; STYLE review), for split-and-boolean-copy-chains-are-chased-three-ways. Dispatched wire-breaks (wire/eval-internal-breaks; FULL review), for eval-handles-internal-breaks-three-ways…. The breaks lane applies only a ratified rule for typed kernel-defect refusals vs D9 row 4 `unreachable!`. If none decides, it reports and a designer fork follows.

- 2026-10-19 (WIRE orchestrator) PR 5405 (wire-chase) is green at 05e271e8da.
  - One `names::chase::to_root` helper now runs four chases. A cycle past the bound is D9 row 4, consistent with ORIGIN's stated acyclicity argument on `Provenance::SplitEdge`.
  - `Fused::origin` is total by `Minted` and stays as it is.
  - Filed hone/split-null-pair-rows-…-minted-list (P4).
  - STYLE review dispatched on the frozen head.

- 2026-10-19 (WIRE orchestrator) PR 5407 (wire-p4) is green at 2369d63db3.
  - The new arm `EntityNamed { name, held }` is ruled to cover an entity repeated inside one offered list. `insert_tied`'s silent dedup is gone.
  - Six editor families are welded to f6 rosters. Swept: ParamBox SymbolsOverABox.
  - Filed: kernel concision lists; the product carry messages.
  - STYLE review dispatched; it presses the dedup removal as a behaviour change.

- 2026-10-19 (WIRE orchestrator) PR 5406 (wire-breaks) is green at c9fe0c1b5e.
  - Part 2 is done: `OpInput`; `ProfileRecordMissing.loop_` narrowed to `u32`; silent defaults removed.
  - Part 1 came back as a question, which is right: D9 row 4 (Ev engaged via PR 4006) against an agent-written `KERNEL_DEFECT_ENDING` typed-defect family, about 44 files.
  - Filed a-kernel-defect-is-a-panic-or-a-typed-refusal-and-nothing-says-which (P2 M, design).
  - Drew FORK-DEFECT on the analysis branch, unread; the designer pair is dispatched.
  - FULL review of 5406 dispatched. 5406 closes only Part 2 and the defaults, and Part 1 moves to the new row.

- 2026-10-19 (WIRE orchestrator) PR 5405 STYLE review at 05e271e8da: MERGEABLE AFTER MINORS.
  - MINOR-1 (demonstrated): `reached`'s seen-stop absorbed a within-walk cycle. To be fixed.
  - MINOR-3/4 and NIT-1/2/4: to be fixed.
  - NIT-3, the helper's shape, is taste and is skipped.
  - MINOR-2, a panic in editor-core vs typed `SplitLineageCycle` in topo for one fact: the PR follows D9 row 4 as ratified and notes FORK-DEFECT, which may move topo's arm.
  - Fix pass sent back to the wire-chase lane.
  - The reviewer's off-scope find, a mate Flip-chain walk with no guard, is filed parked in wirehold on a-mate-relates-two-poses.
  - Class note: a dedup in a walk's stop closure can absorb a cycle the walk itself was meant to refuse.
- 2026-10-19 (WIRE orchestrator) FORK-DEFECT crossover round 1: Designer O converged on the rule, the at-rest ending, the rename and the provenance. It reconciled the Emission count at ~135 non-test raises; most samples are row 0/4. Waiting on the other designer's round.

- 2026-10-19 (WIRE orchestrator) FORK-DEFECT reconciled in one round. The rule converged: proof decides, a typed shortfall row 6, a `pub fn` precondition is row 1, `held_out` goes. Two residual splits go to Ev: the rename, and "or file" on an imported body's geometric gate. [ev] PR 5411 opened, editing D9, with fork-log row 177. Subscribed.

- 2026-10-19 (WIRE orchestrator) PR 5406 FULL review at c9fe0c1b5e: MERGEABLE.
  - Content keys and result hashes are unchanged at f64 and Interval (a probe run at head and at the merge base).
  - Fix pass sent:
    - M1, a wrong premise for keeping the assertion slice let-else: try a typed pair;
    - M2, split the u32::MAX row into authored and kernel counts, and name `names::to_u32`;
    - N1, the re-resolve comment's lost argument;
    - N2, World in Part 1's list;
    - S1, pairwise spelled two ways.
  - M3 (row bookkeeping) is mine at merge. S2–S4 are taste.
  - Class note: a 'keeps the key bytes' reason must be checked against how the key is fed. A hand-fed key is not a layout.

- 2026-10-19 (WIRE orchestrator) PR 5405 merged (8d59072d88). The fix pass was read: a chain-local set gates `stop`, and a cycle test reds on the old order. Row closed.
  - Parked a-split-merging-a-two-piece-wall… on the FORK-DEFECT row: its question is which ending a carrier-rounding pinch corner gets.
  - Dispatched wire-mchord, an investigation (no PR), to find whether any door reaches MergedChordOffRim before a design fork is spent on its sides' order.

- 2026-10-19 (WIRE orchestrator) PR 5407 STYLE review at 2369d63db3: mergeable.
  - Fix pass sent:
    - the in-list (held == name) sentence states the wrong fact;
    - demonstrated: the census's own roster list is hand-kept (deleting a roster entry with its row stays green), so make forwarded_arm exhaustive over NodeErrorKind and weld or list the unwelded forwarders;
    - the product row moves to wirehold;
    - NITs.
  - CI skipped demos, and this PR turns insert_tied's dedup loud, so the lane runs demos/tour before merge.
  - Class note: a weld that fixes a hand-kept list must not add its own hand-kept list. Mutate the weld, not just the rows.

- 2026-10-19 (WIRE orchestrator) PR 5406 merged (6bb0052fc3).
  - I read the fix pass. The assertion payload is a typed pair whose key is fed the same bytes; `ProfileForm` decides the profile's form once, before the key; `pairwise` has one spelling.
  - Part 2 is closed. Part 1 continues as FORK-DEFECT ([ev] 5411).

- 2026-10-19 (WIRE orchestrator) Ev on 5411 asked for a principled reason not to convert the typed kernel-bug refusals to panics. I found none, and replied:
  - row 6 rested on reading 'some input triggers it' as D9's input-reachable;
  - the tree already types limits (KERNEL_LIMIT_*) and unbuilt cases (NOT_YET_ENDING) apart;
  - defect-ending sites state that only a bug reaches them.
  - Revised 5411 in place: row 6 withdrawn. Two clarifications: a defect-ending site panics or takes its true row's ending; a load-bearing re-deriving check panics in release.
  - Class note: two designers converging is not a check against the ratified principle. Test the shared premise against D9's own words before sending.

- 2026-10-19 (WIRE orchestrator) PR 5407 merged (3e04c6a0ac) over main's red python suite.
  - The red: test_checks.py TestAProfileJunctionDecidedTangent, which broke when #5377's door refusal reached main. It is filed P1 on PATHS (a-python-junction-test-builds-a-junction-the-door-now-refuses), with a PR comment.
  - The fix pass widened the census over all 74 NodeErrorKind arms, so forward is wildcard-free; it found and added 5 arms with no row.
  - Demos tour 94/94 and wild 8 cells, run locally.
  - Both rows closed.

- 2026-10-20 (WIRE orchestrator) wire-mchord reported.
  - A plain two-prism Union, Subtract, folds and split-then-union all refuse MergedChord, every hit zip-listed. MergedChordOffRim has no witness.
  - The chord row is raised to P2 with the witnesses, and it joins should-pair-seams-take-name-ordered-sides (also P2) in one fork, FORK-PAIRSEAM. The byte is drawn, unread; the designer pair is dispatched.
  - A three-member fold's borders refusal goes on borders' held-edge row as a possible witness.

- 2026-10-20 (WIRE orchestrator) FORK-PAIRSEAM first reports.
  - Both recommend name order everywhere and retiring Seams, of_published and the MergedChord arms.
  - Divergent premise: one assumes the sides carry their read, while the other measured bare operand names (a Transform-copy subtract keeps distinct seams apart only by slot, and its four Crossings tie). The other's design is node-space sides, then name order.
  - Filed a-subtract-of-a-transform-copy-ties-its-seam-crossings (P2). Crossover round 1 sent.

- 2026-10-20 (WIRE orchestrator) FORK-PAIRSEAM converged in one round.
  - B verified the sides are bare and adopted A's node-table sides; A adopted B's vertex and one-operand rules.
  - [ev] PR 5423 edits N3 and N2's parenthesis, with fork-log row 178. 177 is 5411's, and a contacthold PR also cites 177, so renumber on merge.
  - One small split for Ev: a chord within both inputs' rims is SharedRim or a defect.
  - Before sending, the shared premise was checked against N3's ratified words ('an unmerged face's own name'): the clarified reading is consistent with it, and the order clause is new.
  - Subscribed.

- 2026-10-20 (WIRE orchestrator) Dispatched:
  - FORK-SECTIONID (designer pair; row raised to P2, since a held SectionFace{k} silently rebinds);
  - FORK-BLENDEMPTY (designer pair, smaller brief);
  - wire-census2 (wire/census-tail; STYLE review), for the three P4 census rows, with Part kept on D10.
  - Triaged list-positions-… as P3 M.
  - The Transform-copy crossing tie waits on 5423.

- 2026-10-20 (WIRE orchestrator) FORK-BLENDEMPTY: both designers converged on their first reports. The doors refuse an empty Edges selection through Select::fault, and BlendSelectionEmpty and the viewer's NoEdges go.
  - The 'unfinished recipe' premise is agent text (e2985861f0); no flow grows a selection.
  - No ratified text changes, so no [ev]. Each designer's optional doc sentence is left out.
  - Build dispatched: wire-blend (wire/empty-blend-at-the-door; STYLE review). The fork-log row lands with the build PR, its A/B filled at merge.

- 2026-10-20 (WIRE orchestrator) FORK-SECTIONID first reports converged on the shape: bare when alone, Borders when several, ties per N4, the index gone.
  - O measured that a 0.06° tilt swaps indices on NOTCHED with no topology change.
  - Divergent: a merged group's spelling (one plane-piece constituent, or each section by name), and the kernel record (O: absorbed sections lose their walls, so record them at completion).
  - Round 1 sent.
  - Found: fork-log row 163's Ev/match columns are a copy of row 165's. Fixing them in the next WIRE PR to touch the log: Ev 'sure, sounds good' (PR 5306, 2026-10-19), match both.

- 2026-10-20 (WIRE orchestrator) FORK-SECTIONID converged in two rounds.
  - The shape: bare when alone, Borders when several, ties per N4.
  - The merged group cites the plane once (O moved to F's principle, F to O's outside-the-set form).
  - The kernel record is the ring-to-island export plus the emitter's plane group (the orchestrator checked emit_topo ~503: the split records its other groups).
  - Premise checked against N4 and N2's ratified words.
  - [ev] PR 5426, fork-log row 179. Subscribed.

- 2026-10-20 (WIRE orchestrator) PR 5427 (wire-census2) is green at 0072cfea6e.
  - A multi-level weld over 18 kernel rosters adds ~45 rows.
  - A new ROWLESS list holds back two over-budget rows.
  - Two recourses added. 8 rows filed, D10 ones parked.
  - STYLE review dispatched; it presses ROWLESS as a possible second hand-kept list.

- 2026-10-20 (WIRE orchestrator) PR 5427 STYLE review at 0072cfea6e: MERGEABLE with MINORs.
  - Demonstrated: three forward-arm mutations stay green (if-let levels act as wildcards, and nothing checks a roster is read).
  - ROWLESS is a second hand-kept list. Replace it with name_words_corpus's OVER_BUDGET ceiling precedent.
  - Stale header; check_findings' inline weld; NITs.
  - Fix pass sent.
  - Class note: an exhaustive roster proves the enum is listed, not that the list is consulted. Mutate the forward arms too.

- 2026-10-20 (WIRE orchestrator) PR 5427 merged (f1a197be45).
  - The fix pass was read. A ROSTERS table reds on any roster no row reaches, every level is an exhaustive match, and the reviewer's three mutations are now red. ROWLESS is replaced by OVER_BUDGET ceilings.
  - The ResultEscalated row is pinned at margin 1e-20 for one length across eps. The longer tolerance-offering form is unmeasured, noted on the hone row.
  - Three rows closed.

- 2026-10-20 (WIRE orchestrator) PR 5436 (wire-blend) is green at aa71518c41, after resuming across a container restart.
  - SelectionCount sits on the kind, with one Count fault. BlendSelectionEmpty and the viewer's BlendError go; the commit is disabled at 0 edges.
  - Filed: band's kernel empty blend returns its input; wire's loft TooFewSections.
  - STYLE review dispatched.
  - Orchestrator error: blend-opus.md held the designer prompt (my extraction matched a phrase in the brief). Re-extracted; the build followed my brief's description, which matched the report.

- 2026-10-20 (WIRE orchestrator) PR 5436 STYLE review at aa71518c41: mergeable after a fix pass.
  - M3, demonstrated, is an input-reachable panic: the ungated pub selection_into mints an empty Edges var, the insert door's read check reads kind only, and evaluate hits the unreachable!. The fix is for the door to re-check Select::fault for a selection read by id, and to cfg-gate selection_into.
  - Also: M1, a count fault needs its own recourse; M2, stub drift; S2, prose sweep; V1, the hover text; N1. Taste skipped.
  - Fix pass sent.
  - Class note: 'test-only' must be checked on the function's own cfg, not its caller's. An unreachable! behind an ungated pub door is row 1.

- 2026-10-20 (WIRE orchestrator) PR 5436 merged (a0ac0b4ddd).
  - The fix pass was read: the insert door re-checks Select::fault for a selection read by id, selection_into is cfg-gated, and a count fault gets its own recourse.
  - Fork-log row 180 (BLENDEMPTY, decided without [ev], A/B filled) and row 163's Ev and match cells restored rode the PR's last commit. Row closed.
  - Orchestrator slip: a failed 'worktree add' left a chained commit on local wire/orchestrator. It was never pushed; I moved it to the PR branch by cherry-pick and dropped it locally.
  - Class note: chain a cd into a new worktree as 'cd X || exit', never after an && that can fail silently into the wrong checkout.

- 2026-10-20 (WIRE orchestrator) Ev approved 5423 ('i still have no real opinion on the order but this seems ok'). Ev read the split as a question of fact, and I agreed in reply; the build settles it. Merged (7beba6f7ae), with fork-log row 178 filled.
  - Slip, repeated: a main merge on the [ev] branch conflicted in the fork log and I committed over the markers. Caught before the merge and fixed. Commits are now gated on a marker grep for files lint does not cover.
  - Dispatched wire-seamorder (wire/pair-seam-name-order; FULL review). It closes the order row, the merged-chord row and the Transform-copy tie row.

- 2026-10-20 (WIRE orchestrator) Ev approved 5426 ('sure, this definitely seems like an improvement'), and it is merged (3b687e1652) with fork-log row 179 filled. The section row is back to open for the build.
- Ev's broader point on 5426 (no consistent naming strategy) is filed as naming-has-no-single-strategy-and-each-fork-patches-one-entity (P1 H, design).
  - It cites WIRE's six naming forks this week.
  - A read-only survey lane (wire-namesurvey) is dispatched: a defect census grouped by what leaked, the clauses' generality, and an invariance-check sketch. The strategy itself will go to Ev as an [ev] PR.
  - Not replied on 5426, since it is merged.

- 2026-10-20 (WIRE orchestrator) Ev approved 5411 after asking to rewrite the misleading sentence: 'looks good!'. Merged (86f868aeb2), fork-log row 177 filled (match: neither). Fork row closed. Follow-ups:
  - Filed scalar/kernel-defect-ending-sites-convert-to-panics-or-their-true-ending (P2 H), with the 28 outside sites by owning program.
  - Raised CLEAVE's ring-tie row to P1, since it is a known-reached defect.
  - WIRE rows:
    - eval-typed-defect-arms-go-by-row-zero-or-panic (dispatched as wire-evalarms, FULL review, staying out of names/ while seamorder runs);
    - names-emission-raises-panic-or-take-their-true-ending (after seamorder);
    - held-out-goes-once-the-ring-tie-row-is-fixed (parked on CLEAVE).
  - The pinch-corner row is unparked: it is a limit, row 1.

- 2026-10-20 (WIRE orchestrator) wire-namesurvey reported, and the survey is saved on the strategy row.
  - About 120 naming defects in six groups. The candidate principle fails as worded, since it needs declared symmetries, locality, reads, totality and birth records.
  - The member-order probe on corpus::documents() found 0 differences: the wrong corpus.
  - Filed intent/a-placement-adds-no-name-segment-is-said-three-ways (a RoleSeg::Placed inconsistency, D10).
  - Dispatched FORK-NAMESTRATEGY (designer pair; byte drawn, unread).
  - Dispatched wire-nameinv (wire/name-invariance-checks; STYLE review), six perturbations over a contact-heavy corpus with findings filed and admitted both ways.

- 2026-10-20 (WIRE orchestrator) FORK-NAMESTRATEGY first reports. They converge on five principles, a derivation for new entities, no reversal of 158–179, and dropping OrderAlong.of (filed P2).
  - O found that the principle is ratified already, in Ev's words: DESIGN.md [intrinsic.contingent], 436489e58b, 2026-10-17. Naming is missing from its 'Instantiated in' list, and N4's agent-written invariant predates it and is looser.
  - Crossover round 1 sent on: the framing; the grammar (4 forms vs a 6-case ladder plus a type gate); checks P8 and the denotation verifier; the coincidence-row scope; the one-namer code-level cause.
  - Passed perturbations 7 (insertion history) and 8 (arena permutation, only if a helper exists) to wire-nameinv.

- 2026-10-20 (WIRE orchestrator) FORK-NAMESTRATEGY converged in one round.
  - The result is N0: five principles NS1–NS5, a four-form grammar (Carried, Merged, Met, Made), a check per principle, and no reversal of rows 158–179.
  - The premise was checked: [intrinsic.contingent] is ratified (via [ev] 5230), and naming never instantiated it.
  - [ev] PR 5455 adds N0, rewrites N4's invariant, and lists [identity] in [intrinsic]. Fork-log row 183.
  - Open: whether a crossing's sense may read an edge's stored orientation; it waits on the arena-permutation check. The Placed question goes to D10.
  - Subscribed.

- 2026-10-20 (WIRE orchestrator) Ev on 5455: 'this is looking good!'.
  - Ev is sure of no frame and no value, but less sure of slot and order, and asked what slot means. Ev also asked for a usage example and how ties are ruled out for identical operands; 3 and 5 'yes'.
  - Added a definition of slot (sequence vs set) and a worked example to N0 (26abb74f8d).
  - Replied:
    - slot and order explained, with the measurement under way in wire-nameinv;
    - the example;
    - identical operands never collide (the read is in every carried name), but N4 ties remain by design where geometry doesn't distinguish, plus D10's Placed question.

- 2026-10-20 (WIRE orchestrator) PR 5460 (wire-nameinv) is green at 93d1cf0c37.
  - Seven invariance tests over a contact-heavy corpus.
  - Found:
    - a body and its copy named from the copy minted first;
    - a pinch named by the leg minted first;
    - one pair-union order refusing EntityNamed.
  - Filed 3 new wire rows, a P3 topo arena-helper row, and a cleave at-rest refusal.
  - Perturbations 3 and 4 find nothing today.
  - STYLE review dispatched; it presses whether the set-sort normalisation hides content leaks.

- 2026-10-20 (WIRE orchestrator) PR 5457 (wire-evalarms) is green at 8f279e27fc.
  - VerbArity and ProfileAnchor go by row 0; ProfilePieces and ProfileRecordMissing by row 4. MissingSlot is row 4 except on D10 readers (parked).
  - Two rewordings forced by moved code: the bounds-allowlist pin goes 1→5, plus the real.rs SEAT-4 and the READMEs.
  - Four rows filed.
  - FULL review dispatched; it presses input-reachability per panic and the five-copy header.

- 2026-10-20 (WIRE orchestrator) PR 5460 STYLE review at 93d1cf0c37: mergeable once M1 is fixed.
  - Demonstrated: planted leaks red tests 1 and 3, and the admissions red both ways.
  - M1, demonstrated: many cases are vacuous (both sides empty or refusing). Fix with a must-compare assertion and a VACUOUS admission.
  - m1: the hand set-sort copies canonical.rs partially; use production canonicalisation.
  - m2: per-witness admissions. m3: tilt refusals were skipped. m4: the copy row's remedy conflicts with N0. m5: wrong row cited. m6: add an OrderAlong case. m7: the arena row. m8: test 5 is a subset of 7.
  - Fix pass sent.
  - Class note: an invariance test over a corpus that mostly refuses or is empty passes by comparing nothing. Assert that each case compared something.

- 2026-10-20 (WIRE orchestrator) PR 5457 FULL review at 8f279e27fc: approve with fixes.
  - No input-reachable panic found arm by arm. The anchor mutations go red. Key bytes unchanged.
  - MAJOR-1: Verb<T> left as an enum nothing in production builds, so the V1 rewording would have changed what V1 decides. The fix routes production through Verb with the typed doors as payloads and deletes Arity if unread. No [ev] needed if V1 keeps its decision.
  - Also: one supertrait for the 5-copy bound (pin back to 1); profile_edges_of's caller-reachable assert; stale citers; derive_naming's silent no-Outer; NITs.
  - Filed a-doc-deserialized-directly-skips-the-load-doors-checks (P2).
  - Class note: a rewording 'because code moved' must be checked against whether the code still does what the clause decides.

- 2026-10-20 (WIRE orchestrator) 5457: the lane showed that MAJOR-1's fix (route production through Verb<T>) is impossible without bringing the mismatch back. With no GADTs, a variant can't be tied to its operand type.
  - V1 is ratified (#1388, #1870, #1983, #1997), so the question went on 5457 itself, titled [ev]. Recommended (a): delete Verb<T> and Arity, keep VerbKind closed. (b) keep a vocabulary-only wrapper; (c) one unreachable arm per lowering.
  - The lane pushes items 2–6 meanwhile (DoorScalar, pin 2). Subscribed.

- 2026-10-20 (WIRE orchestrator) PR 5460 merged.
  - The fix pass was read: VACUOUS lists both ways; production canonicalisation via remap_name; per-count admissions; tilts compared not skipped; in-plane rotation added; twins merged into history.
  - New rows: bridge-last fold refusal; CITED_GROUP_NOT_ONE_SEAM.
  - Admitted counts recorded in the PR body.
  - OrderAlong is unreachable with planar and cylindrical carriers, noted on its row.

- 2026-10-20 (WIRE orchestrator) PR 5469 (wire-seamorder) is green at 253aefec22.
  - Seam sides and crossings now come from the boolean's table with their read, in name order. Seams/of_published retire, as do the dead RankRule arms and MergedChord/OffRim.
  - The both-rims case is a typed Emission: a probe never fired, and nothing citable rules it out.
  - The vertex rule is narrowed from the designers' version, disclosed.
  - Names moved: all 30 subtract tables; 5 of 21 unions, via embedded subtracts only.
  - die_composed_tour.pncad is respelled by a scratch replay. The file is stale on main, filed with intent.
  - Filed the Borders bare-names row.
  - FULL review dispatched.

- 2026-10-20 (WIRE orchestrator) PR 5469 FULL review at 253aefec22: changes requested.
  - M1, demonstrated: the head predates 5460, so merging reds four name_invariance tests on 'two prisms' VACUOUS; it now compares with zero differences. Remove the entries.
  - m1: the new typed Emissions misread D9. Correcting my own brief: the ratified rows 4/5 test is 'current best understanding', not a citation (that was 5411's withdrawn first draft). BOTH_RIMS becomes unreachable!; ONE_OPERAND and BESIDE_OFF_RIM become row 2 or row 4 by D9.
  - m2: the zip-rim path is unit-only. m3: a silent unwrap_or_default. m4: RankRule (true,true) given a reading.
  - Also: witnesses to assert agreement across symmetries; the RimPick ladder once; stale prose; rows.
  - Fix pass sent.
  - Class note: re-read the ratified text, not my memory of a draft, before briefing a D9 rule. A withdrawn draft's test leaked into a brief.

- 2026-10-20 (WIRE orchestrator) PR 5469 merged (747279992d).
  - The fix pass was read. BOTH_RIMS is now unreachable!. Distinct faces off their rim at a subtract or intersect are row 4 (a valid operand's faces meet only along edges); one face on both sides is row 2, the new ChordOffRim, Unruled.
  - Name agreement is asserted across symmetries, and the RimPick ladder is written once.
  - Three rows closed; the Transform-copy row is closed here.
  - Dispatched (emit_topo is free now):
    - wire-secface (wire/section-face-pieces; FULL review), the row-179 build;
    - wire-mintfirst (wire/union-reads-not-mint-order; FULL review), for the copy-minted-first and pinch-leg-minted-first leaks, staying in emit_union.

- 2026-10-20 (WIRE orchestrator) PR 5486 (wire-secface) is green at dd014186b8.
  - SectionFace is a unit variant; pieces carry Borders walls; merged groups are Merged{SectionFace, floor}.
  - The kernel exports SplitNaming::islands, via a second nest run.
  - Goldens moved for the 3 split documents.
  - The python suite was skipped, and the old swap was not re-measured on main.
  - FULL review dispatched; it presses the kernel verdict log, the python suite, the motivating defect and the three Emissions.

- 2026-10-20 (WIRE orchestrator) Ev on 5455 asked how a boolean gets its inputs.
  - Answered from main: operands are BodyReads (a VarId, plus a CopyKey or indices); placement happens upstream (Transform, or a placement's copies); the boolean takes no placement info.
  - What a placement is, and the read-vs-Placed question, are D10's (INTENT).
- 2026-10-20 (WIRE orchestrator) PR 5488 (wire-mintfirst) is green at 3ec6022478, titled [ev].
  - Flush stretches, vertices and crossing lines held by several members are now Merged of them in name order.
  - It retires ratified flush clauses rows 74 and 121 (first-minted / least).
  - The name_invariance history admissions go to empty, with no golden moved.
  - Rows needs_ev. Subscribed. FULL review dispatched in parallel.

- 2026-10-20 (WIRE orchestrator) PR 5486 FULL review at dd014186b8: NOT MERGEABLE.
  - MAJOR 1, demonstrated: a pocket+pin+moat split that evaluated at base now refuses an Emission. The island record is taken pre-merge, and end_maximal dissolves the ring.
  - MAJOR 2, demonstrated: test_north_star 0 != 14, a stale one-segment pattern against the approved spelling.
  - Q1: the second nest run adds verdicts to five existing fixtures. Q3: the witness doesn't red on the old index (the fixture is drawn in the xy frame; the swap needs u=z), and split_pinch_keeps' tilt row compares name sets only.
  - The Emissions are reclassed under D9; tied-flag fold; limits row; NITs.
  - Goldens confirmed spelling-only by a full dump. Fix pass sent.
  - Class note: a lane's 'motivating defect is fixed' needs the witness to red on the base. 'Now passes' is not 'would have caught it'.

- 2026-10-20 (WIRE orchestrator) Ev approved 5455 ('this seems alright though i don't know if the intent refactor will affect it'). N0 merged (0c9d84779c); fork-log row 183 filled. Strategy row closed.
  - Ev's caveat is recorded on intent's Placed row: N0's NS1(b)/NS4 are re-checked when the placement rebuild lands.
  - Filed N0's follow-ups:
    - clause pointers (P3);
    - the RoleSeg→form map (P2);
    - the denotation verifier (P2 H);
    - the coverage fuzzer (P2 H).
  - Already filed: the arena-permute helper (P2) and OrderAlong.of. The one-namer program is left to the slate.

- 2026-10-20 (WIRE orchestrator) Ev on 5457 asked where the verb/door mismatch could happen before.
  - Answered: never in shipped code. At the merge base every Verb door method took any variant and refused Arity at runtime; only the verbs crate's own door-matrix tests made the mismatched call, on purpose. editor-core's correspondences always matched.
  - So the (a)/(b)/(c) choice is API shape only. (a) is the smallest.

- 2026-10-20 (WIRE orchestrator) Ev approved 5488 ('this looks good'): retiring flush clauses rows 74/121 is ratified. needs_ev cleared; the merge waits on the FULL review's adjudication.

- 2026-10-20 (WIRE orchestrator) PR 5488 FULL review at 3ec6022478: the code is right, and a planted least-pick reds two tests.
  - MAJOR, demonstrated: line_sense/line_head still read mint order, on a pinch with one leg reversed (Leaves vs Enters by authoring order). The README sentence Ev approved stated 'its least edge' as the rule.
  - Handled by disclosure: told Ev on the PR. The fix pass rewords the sentence to 'one of its edges; which is open', raises the row to P2 with the fixture measured and admitted, and fixes the citation (row 124, not 121).
  - Also: line_head's unchecked flush assumption; stale docs; cites_both on the head; a locality assert; the tied flag; three rows (N5 two-hop offers, subtract seats via the least, the duplicated grouping helper).
  - Class note: before merging a text Ev approved, check that the review didn't find the approved sentence itself false. If it did, narrow the text and tell Ev; don't merge it as ratified.

- 2026-10-20 (WIRE orchestrator) Ev on 5457: '(a) works though the stakes seem low'. The lane finishes item 1: delete Verb<T>/Arity/VerbKind::arity, keep VerbKind closed, reword V1. Merge follows.

- 2026-10-20 (WIRE orchestrator) PR 5457 merged (8428fbb043) as option (a). Verb<T>, Arity and VerbKind::arity are gone, VerbKind stays closed, and V1 is reworded per Ev.
  - The fix-pass items are in: DoorScalar (pin 2), profile_edges_of's caller-reachable assert gone, derive_naming typed or row 4, stale citers fixed.
  - Eval-arms row closed. MissingSlot stays typed on the D10 readers (parked row).

- 2026-10-20 (WIRE orchestrator) PR 5488 merged (59c00412db) and its two mint-order rows closed.
  - Fix pass: the least-edge sentence is narrowed to 'which edge orients it is open', and the orientation row is raised to P2 and measured (18 names, admitted as LINE_ORIENTED_BY_LEAST). Ev was told before the merge.
  - line_head now checks it is a flush line; cites_both checks the head reads; a locality assert added; tied flag pinned.
  - Rows: doctail N5 two-hop offers; one flush-grouping helper; pair-seam seat trace (no cross-seat least).
  - [ev] taken off the title.

- 2026-10-20 (WIRE orchestrator) PR 5486 fix pass at 4bddb8ab69 is green, including python.
  - topo now defines a half's plane pieces (SplitNaming::plane_pieces); record_islands runs post-merge with new verdicts on ringed sides; the moat witness evaluates.
  - The three Emissions are now unreachable!. A completion-index mutant reds both tilt rows. The tie flag is per wall set.
  - Because the kernel part was rebuilt, a focused FULL re-review of the fix delta is dispatched.
- 2026-10-20 (WIRE orchestrator) PR 5486 re-review (FULL, fix delta): MERGEABLE.
  - Both MAJORs are closed by execution: the moat witness evaluates, including a two-pin variant, and the python count is exact (14/14).
  - Mutant reds re-confirmed. No golden moved beyond the disclosed spelling change.
  - Merged at 4ea2aa382f. The section-face row is closed.
  - Filed `a-split-touching-a-wall-at-a-vertex-refuses-a-crossing-vertex-emission` (P2; pre-existing, legal input reaching an Emission) and `a-halfs-plane-has-two-definitions-and-n3-linking-two-homes` (P3; S1–S3, m1, m2 and nits).
  - topo's `every_public_mutation_path_preserves_tier1` red is inherited from main and already filed (encl).
- 2026-10-20 (WIRE orchestrator) Dispatched `names-emission-raises-panic-or-take-their-true-ending` as three lanes split by file. The brief is the D9 per-site ladder: row 0, then known reach (stays typed and filed), then row 1/2, then row 4.
  - `wire-emis-topo` (wire/emission-topo): names/emit_topo.rs.
  - `wire-emis-union` (wire/emission-union): emit_union.rs, join_names.rs, borders.rs, canonical.rs, defer.rs.
  - `wire-emis-rest` (wire/emission-rest): emit.rs, emit_sweep.rs, emit_shell.rs, emit_blend.rs, eval/*, coincide.rs, verbs/mod.rs.
  - Wire is at 69/30 load, so a split along the priority seam is owed (work/README.md "Track size"). This sitting is doing work first; the split is noted here.
- 2026-10-20 (WIRE orchestrator) `wire-emis-topo` opened PR 5503 (head 368af77673). emit_topo's 76 raises: 8 to row 0, 48 to row 4, 3 to row 2 (SeamVertexParentage), 6 kept typed as reached, 11 survivors.
  - It filed two rows: ten survivors have no row-1/2 variant, and a tied-set defect.
  - PR CI ran only a narrow change-filter set; the lane's wide suites were local. A FULL review (`wire-emis-topo-rev`) re-runs the slow set and corrupt-input, and samples the row-4 claims for reach.
  - Its emit.rs edits overlap `wire-emis-rest`'s file; that lane was told.
- 2026-10-20 (WIRE orchestrator) `wire-emis-rest` opened PR 5502 (head b4ed55a10e). Of its raises, 57 went to row 4, 2 stay typed as reached, and 2 survive filed (coincide MEET_OF_ONE; union verdicts disagreeing). The rest wait elsewhere: 3 on the verbs row, 4 on D10 ground, and 12 in emit.rs's shared helpers, left because emit_topo, emit_union and others also call them. A FULL review (`wire-emis-rest-rev`) is dispatched; it also merges the 5503 head to check the two together. A follow-up for the 12 helpers is owed once all three lanes land.
- 2026-10-20 (WIRE orchestrator) `wire-emis-union` opened PR 5501 (head 29b1e8985b). 2 raises went to row 0 and about 64 to row 4.
  - 13 stay typed: 6 because legal input reaches them, and 7 a correct kernel may reach, which wait on a variant (filed).
  - It edits `eval/wire.rs`'s `member_cells` caller (5502's file).
  - A FULL review (`wire-emis-union-rev`) is dispatched, with a three-way merge check of 5501, 5502 and 5503.
  - Rows owed after all three land:
    - one NamingError row-1/2 variant set for the 17 deferred "correct kernel may reach" sites (topo 10, union 7; filed rows);
    - the 12 shared helpers in emit.rs.
- 2026-10-20 (WIRE orchestrator) PR 5503 FULL review: NOT MERGEABLE, one BLOCKER, DEMONSTRATED.
  - `Track::runs_with`'s Zero arm panics under an accepted K=1.5: its row-4 argument needs K > 4, and `Tolerance` accepts any K > 1.
  - Fix pass sent to `wire-emis-topo`:
    - the Zero arms go typed;
    - UNCLASSIFIED_SIDE goes back to typed (its union twin is reached);
    - track_caller is useless inside closures;
    - message values; the stale merged.rs doc; PR body corrections.
  - **Class note:** a row-4 argument resting on a band or length bound must hold for every K and ε that `Tolerance` accepts, not the defaults. Both other reviewers were told to probe with CAD_AMBIGUITY_K=1.5 and CAD_TOLERANCE_EPS=1e-6.
  - The stored-curve row (`naming-edge-curve-ladders-can-read-through-body-stored-curve`) closes when 5503 merges.
- 2026-10-20 (WIRE orchestrator) PR 5502 FULL review: MERGEABLE. The low-K/ε probe fired no `unreachable!`, and the head merges clean with 5503.
  - A small fix pass is sent for: stale "refuses" prose in sweep/blend and wire.rs, the RowCell::Result premise, emit_sweep's empty-run ordering and messages, track_caller, the new rows' frontmatter, and a PR body citation.
  - Filed `emit-rs-shared-helpers-raise-emission-for-torn-reads` for the 12 shared-helper raises no lane owned (review M4).
- 2026-10-20 (WIRE orchestrator) PR 5502 fix pass at 43ec29c0cb, read by the orchestrator: prose, the RowCell::Result premise (topo doc corrected), emit_sweep's message order, closures out of the panic paths. CI green; merged at 3b93410f6d. The parent row stays dispatched until 5501 and 5503 land.
- 2026-10-20 (WIRE orchestrator) PR 5503 fix pass at 6a346388cb, read by the orchestrator.
  - Both runs_with Zero arms refuse typed, with K=1.5 regressions. UNCLASSIFIED_SIDE is typed again. The helpers use match, so track_caller works.
  - A local merge with main (which carries 5502) passes `cargo check -p editor-core --all-targets`. CI green; merged at 864aa4e786.
  - The stored-curve row is closed. The parent Emission row waits on 5501.
- 2026-10-20 (WIRE orchestrator) PR 5501 FULL review: MERGEABLE WITH MINORS. No panic reached under the defaults, K=1.5 or ε=1e-6, including on the three-way merge (one conflict, in wire.rs in_member_space).
  - Ruling: the three row-4 sites whose argument rests on independent tolerance decisions agreeing go back to typed: name_by_parents' one-parent seam, Flush::line_sense, and sense_against/meeting.
  - The class note now reads: a row-4 argument holds for every accepted K and ε, and does not rest on two independent decisions agreeing.
  - Fix pass sent: merge main with the conflict resolution; fold the two variant-less rows into one; split the panicking test; stale prose (including a quote in a perf row); orient's message.
- 2026-10-20 (WIRE orchestrator) **Cut along the priority seam** (work/README.md "Track size"). WIRE measured 69 budget points against 30. WIRETAIL, the P3/P4 tail, already stood at 25, so two new programs were opened. Rows moved by `git mv` with ids and bodies unchanged, each with a dated "Moved" section.
  - **NAMECHECK** (14200–14299, 17.5 points, P2): N0's five check rows (verifier, fuzzer, form map, arena helper, pointer rewrite). They are checks, not naming defects.
  - **WIREREST** (14300–14399, 27.5 points, P3/P4): thirteen rows.
  - WIRE keeps its P2 naming-defect rows, plus the two P3 D9-class rows its lanes filed (the tied joined set, union verdicts disagreeing): 28.5 of 30.
  - Priced on the way: the tied joined set P3/M, the loft count P3/E, and the variant-less emit_topo raises P2/M.
  - Recorded in docs/MODEL-AB-LOG.md, with four live bands (13800–14199) the roster had missed.
- 2026-10-20 (WIRE orchestrator) Dispatched `wire-docserde` (wire/doc-deserialize-door: Doc's public Deserialize goes through the load door; STYLE+correctness review) and `wire-splitvertex` (wire/split-wall-vertex-crossing: root-cause the vertex-touching moat refusal; FULL review). State sync PR 5509 opened.
- 2026-10-20 (WIRE orchestrator) PR 5501's fix pass was read by the orchestrator. Five band- and pair-dependent sites went back to typed, the variant rows were merged into one (24 raises), and the test was split. CI green, merged at 43d115ee1a. The parent Emission row is closed. The 4 D10-held emit.rs raises are noted on the emit.rs helpers row.
- 2026-10-21 (WIRE orchestrator) `wire-docserde` opened PR 5514 (head 073b2812b6). Doc derives Serialize only, and a private SnapshotWire is the load door's parse type. Its witnesses are a compile_fail doctest and the two id-shape faults refusing typed at the load door. No direct-serde users were left outside persist. The sweep filed no rows. A STYLE+correctness review (`wire-docserde-rev`) is dispatched; it checks byte-identical load over the corpus and that test-support is confined.
- 2026-10-21 (WIRE orchestrator) PR 5514's review says MERGEABLE; load is byte-identical over the tree's saved files and 66 mutations. A small fix pass is sent: keep the 'expected struct Doc' text, reword the runtime witness, fix the comments. Dispatched `wire-namevariants` (wire/naming-limit-variants) to give the 24 correct-kernel-reachable raises row-1 or row-2 variants.
- 2026-10-21 (WIRE orchestrator) `wire-splitvertex` opened PR 5515 (head 3a5495b9f3).
  - Root cause: the naming read a subtract's SplitEdge birth record as this split's crossing. The table is now checked first.
  - The diamond and kite witnesses evaluate. The row's second document did not reproduce.
  - Filed a cleave SliverSector row and added evidence to a refs row.
  - A FULL review (`wire-splitvertex-rev`) is dispatched.
- 2026-10-21 (WIRE orchestrator) The orchestrator read PR 5514's fix pass: `expecting = "struct Doc"` keeps the refusal text, and it is pinned. Merged at d1e5e7f3e3; the row is closed.
- 2026-10-21 (WIRE orchestrator) PR 5515 FULL review: MERGEABLE.
  - Instrumented over both suites, only the new fixtures' 5 vertices take the changed branch.
  - The row's second document was recovered from 5486's probe. It is the same pocket-wall cause, and it evaluates at head.
  - A small fix pass is sent: the above-half check counts tied rows, the recovered probe becomes a third witness, and the cleave row gains refs.
  - D9 note: split_sense now sees only vertices the split minted. The reviewer leans toward it being unreachable, because Joined::Whole/Set cover whole operand edges and Seam lies along no operand edge. Not proven, so it stays typed.
- 2026-10-21 (WIRE orchestrator) PR 5515 fix pass read by the orchestrator. Tests only: tied rows are counted, the third witness is the recovered probe, and the closure join is now reached (a ring meets the exterior at a point). Merged at 3b8c43a57b; row closed.
- 2026-10-21 (WIRE orchestrator) **Main red from this sitting's cut.** name_invariance's admission lists read row files by path, so the git mv to wirerest/namecheck left three slow-set tests failing. `wire-namevariants` found it. Fix branch wire/moved-row-paths repoints the three cited paths. **Class note:** before moving a row, run `rg work/<prog>/<id>` over crates/ and scripts/.
- 2026-10-21 (WIRE orchestrator) PR 5518 review: two MAJORs. (1) The Geometry and K levers end in KERNEL_LIMIT_LAST_RESORT, which geom-core's contract reserves for the ε lever. (2) The new row-2 variants lack D9's Unsupported* prefix.
  - Ruling: compose geom_core::lever_recourse with no qualifier. The new variants (and the reshaped SeamVertexParentage) take Unsupported*. Older Unruled variants get one class row, not renamed here.
  - Fix pass also covers locators, per-shape recourse, README's Outright-join rule (check provenance), prose counts and full-shape rendering.
  - Note: most such Unruled variants already break the prefix rule. That is a class, now filed rather than spread.
- 2026-10-21 (WIRE orchestrator) PR 5523 (repoint the moved rows' citations) is green and merged at fa65d8fb52. Main's three name_invariance failures are fixed.
- 2026-10-21 (WIRE orchestrator) Dispatched `wire-orderalong` (wire/order-along-drops-its-count). It drops OrderAlong's `of`, and the witness is part of acceptance: three same-sense crossings on a torus or a bumped revolve. If no such witness can be built, the witness is filed.
- 2026-10-21 (WIRE orchestrator) `wire-orderalong` stopped at a fork before writing any code.
  - canonical's Reverse rule reads `of` through the refactor remap (ASM-4 ground). No three-entry carrier is buildable today (NURBS×plane booleans refuse), so the defect is unreachable.
  - The row now records options A/B/C, is re-priced P3+design, and moved to WIREREST. The path grep found no code citations.
  - Not sent to Ev now: the case is unreachable, and a design fork goes to the designer pair first, once a witness exists.
- 2026-10-21 (WIRE orchestrator) Dispatched `wire-bridges` (wire/bridged-wedges-pair-booleans): the two bridged-wedge pair-boolean refusals as one unit. Their witnesses come off name_invariance's VACUOUS lists. FULL review.
- 2026-10-21 (WIRE orchestrator) The orchestrator read PR 5518's fix pass. The levers compose lever_recourse, the variants are Unsupported*, the locators are in subject position, and every shape is rendered in display_tests. Outright traces to agent text 7b6d56df65, so it stays row 2. Merged at 9fbf113ba1; row closed.
- 2026-10-21 (WIRE orchestrator) Dispatched `wire-emithelpers` (wire/emit-helpers-row-four): emit.rs's 12 shared-helper raises go to row 4, and callers on D10 ground are left alone.
- 2026-10-21 (WIRE orchestrator) `wire-emithelpers` opened PR 5530 (head b1083ca470). 11 of 12 helper raises went to row 4.
  - `to_u32`'s bound is SlotMap's u32 slot cap. `output_body` stays fallible: pattern counts are uncapped and on D10 ground. `loop_coordinates` stays fallible and was filed.
  - Four rows filed, two of them in other programs (seamrank, doctail: track_caller inside closures).
  - Review dispatched (`wire-emithelpers-rev`).
- 2026-10-25 (WIRE orchestrator) After the usage reset, `wire-bridges` and `wire-emithelpers-rev` resumed.
  - PR 5530 review: mergeable, four MINORs. Fix pass sent:
    - section()'s closures defeat track_caller;
    - first-hop dead-key arms need a proven-live reason (chord_faces guards a dead face);
    - output_body joins the loop_coordinates row;
    - stale docs, vestigial Results, borders' hand copy of edge_ends.
- 2026-10-25 (WIRE orchestrator) FORK-ORIENT opened (draw 28, even: Opus is A, Fable is B) on `a-unions-line-of-several-edges-is-oriented-by-its-least-edge`. Opus and Fable designers are dispatched. The brief names the seamrank axis Ev turned down, and the canonical Reverse rule, as known traps. PR 5700 (state sync) merged.
- 2026-10-25 (WIRE orchestrator) `wire-bridges` opened PR 5535 (head 727d143153).
  - The union refusal was a kernel record duplicate: `seam_edges` listed an edge that two zips glue twice (ops.rs dedup, NS5).
  - The intersect refusal was emit_union re-spelling N2's corner copies bare; the new `name_corner_copies` follows the rule N2 already states.
  - `ADMITTED_OPERAND_ORDER` is empty. A sliver-below-default-K row was filed.
  - A FULL review (`wire-bridges-rev`) is dispatched.
- 2026-10-25 (WIRE orchestrator) FORK-ORIENT converged in one crossover round: an orientation is read, never chosen. Sense is `Through { entering }` on a line with no own direction. Ranks run along the entering side. Reverse, flip_senses, RefuseRun and `of` retire, and a seam in a run gains a sense. B moved to A on spelling and on rank-vs-tie. The residue is A's NS1 sentence, which B rejects and the PR does not add. [ev] PR 5705 is opened with the README edit and fork-log row 195; the row needs_ev.
- 2026-10-25 (WIRE orchestrator) Ev on PR 5705 asked whether the method is simple, consistent and stable, or will need patches, and whether a simpler rule with a few worse cases exists. Both designers were asked to count the cases, weigh a no-orientation rule and a tie-instead-of-rank rule, and recommend one. The answer goes on the PR once reconciled.
- 2026-10-25 (WIRE orchestrator) Reply on PR 5705 to Ev's simplicity question. Both designers recommend dropping ranks: sense along each thing on the line, else tie (two rules), replacing the joint method's 9–10 cases. This reverses row 73's rank half; no test reaches it, and the change is additive later. The 18 seam-run groups are unmeasured; a build checks them first. Waiting for Ev's yes before switching the diff.
- 2026-10-25 (WIRE orchestrator) Ev on PR 5705: "this sounds good but do measure first". Dispatched `wire-orient-measure` (read-only instrumented probe over the suite and corpus). It counts every ranked or tied group, whether per-thing sense separates it, the tie count under the no-rank rule, and the 18 seam-run groups. The diff switches only after the numbers are reported to Ev.
- 2026-10-25 (WIRE orchestrator) PR 5535 FULL review: mergeable with fixes.
  - The ops.rs dedup is right. Instrumented, duplicates also appear in five topo families, and no reader depended on them.
  - MAJOR: the tied-corner arm still collapses copies, against N4's divider rule. The pair boolean does it right.
  - MINOR: a new unreachable! rests on two decisions agreeing, so it goes typed per the class rule.
  - Fix pass sent: tied arm, typed refusal, dedup helper, territory line, row wording.
  - The reviewer also reports main red at 4b7d569 on about 30 slow tests, goldens included, identical at base. The orchestrator is checking a sample on current main.
- 2026-10-25 (WIRE orchestrator) Checked main (63230e1077). die_name_tables_are_golden, node_standing READERS and intent_s2_c_world are red, and name_words_rows' timing test failed under load. These are already filed by INTENT, against PR 5613 (D′2): `intent/main-red-the-seat-and-name-table-goldens-moved-at-dprime2` and `intent/main-red-node-standing-readers-census-trails-the-dprime2-mate-split`, including wire.rs's 9-vs-10 reader count. Not WIRE's; no action. Lanes are told not to count these as theirs.
- 2026-10-25 (WIRE orchestrator) Measured on main 4a44083275 (probe only, no name changes). All 18 seam-run groups are told apart by per-side sense, including NO_START's 6 ties. The integration suite and corpus have no other ranked or tied group, so new ties are 0 there and 8 in rank unit tests. Posted on PR 5705. The diff is switched to the no-rank rule (head 473c5e886c), the body is updated in place, and the fork-log row is renumbered 197 after a collision with main's 195/196. Waiting on Ev's merge.
- 2026-10-26 (WIRE orchestrator) PR 5535 fix pass at 198b36a630.
  - The tied-corner arm was fixed by regrouping on member corners, with no witness (no fixture found).
  - The Tied unreachable! is gone by construction. The SeamEdges helper is in, and the rows are re-worded.
  - The lane filed a pair-boolean tied-corner row.
  - Because the grouping key changed without a witness, a focused re-review of the delta is dispatched (`wire-bridges-rerev`), targeted tests only.
- 2026-10-25 (PRED orchestrator) Seam note: branch `pred/full-revolve-cavity-per-axis-gap` adds one paragraph to `crates/editor-core/src/names/README.md`'s "N1, the revolve poles". A full revolve meeting the axis in several runs now builds one cavity per gap. Its inner run ends are cavity poles, and its cavity walls, meridians and rims are named by the profile pieces that sweep them, through loop 0's maps, with no new role. `emit_sweep.rs` is unchanged except for one new test, `a_full_revolves_axis_cavities_are_named`.
- 2026-10-26 (WIRE orchestrator) The orchestrator read PR 5530's fix pass.
  - Closures are gone, and there are two why-texts (TIER1, LIVE_ARGUMENT); row 0 was answered no.
  - chord_faces' dead guard was removed, since its only caller iterates body.faces().
  - Vestigial Results are gone, and borders and plane_pieces use the shared helpers.
  - The test-only point-removal door sits under sweep-testing beside its sibling.
  - Merged at c8a25fe0b4. The helpers row now holds only its 4 D10 raises: parked on patterns-are-index-variables and moved to WIREHOLD. No code cites its path.
  - The lane filed a cross-program row (issues/track-caller-fns-panic-inside-closures-in-topo-and-sweep, 61 sites).
- 2026-10-26 (WIRE orchestrator) PR 5535's focused re-review says mergeable: untied grouping is identical, SeamEdges matches the old behaviour, and the targeted suites are green. Merged at 354012bb8c, and both bridge rows are closed. Filed `a-unions-tied-fold-rows-at-a-member-corner-bypass-the-corner-rule` (P3/M) and corrected the pair-boolean tied-corner row's no-fixture claim.
- 2026-10-26 (WIRE orchestrator) Dispatched `wire-paircorner` (wire/pair-boolean-walls-and-tied-corners) for the pair boolean's bare-name walls and its by-name tied-corner pass.
  - Ruling: walls cite the boolean's own From name. N3 ('a wall cites its parent') with NS1(b) settles it; no fork.
  - The end-to-end wall witness may wait on D10 copies.
  - Priced the two unpriced lane rows (both P3).
- 2026-10-26 (WIRE orchestrator) `wire-paircorner` opened PR 5732 (head 5b3c7c9649).
  - Walls cite the boolean's own From. There is a unit witness only; a collision needs D10 copies.
  - The corner pass is keyed by candidate, and one name_corner_copies body is now shared with the union.
  - This also fixes the REFS carried-pinch refusal: CARRIED_PINCH admissions are removed.
  - A FULL review (`wire-paircorner-rev`) is dispatched, including python, which CI skipped.
- 2026-10-26 (WIRE orchestrator) PR 5732 FULL review: the code is right.
  - Instrumented table dumps over 2700+431 tests show untied keying identical and the shared body identical; python 927 OK; the witnesses go red.
  - MAJOR: two slow corpus goldens (lib_g16 digests, name_words_corpus) move with the wall spelling and were not disclosed. A fix pass re-pins them with a what-moved note.
  - The REFS row's pair-subtract witness still refuses; Intersect [prism, cube, big] is fixed.
  - A missed by-name grouping (the split's edge_groups) is sent too.
- 2026-10-26 (WIRE orchestrator) Ev approved PR 5705 ("cool, seems good!"). Fork-log row 197 is filled: Ev took the round-3 joint no-rank answer, neither first recommendation; A/B was Opus A, Fable B. [ev] is dropped from the title, and the PR merges on green CI. The build is next.
- 2026-10-26 (WIRE orchestrator) PR 5705 merged at 964083e53a. The row's needs_ev is cleared. Dispatched `wire-norank` (wire/nothing-is-ranked) to build the approved rule. It closes the orient row, the seamrank sided-axis row and the wirerest `of` row. FULL review.
- 2026-10-26 (WIRE orchestrator) Ruling for `wire-norank`: a union-minted seam in a run has no recorded class, and fold rows read order. Its sense is read from which of its pieces the result keeps at the vertex (order-free, off the finished body, NS2). Member and pair seams read edge_classes as stored. The rule is stated in N2's text, and the zero-pieces case goes typed or is argued impossible.
- 2026-10-26 (WIRE orchestrator) The orchestrator read PR 5732's fix pass. Four corpus goldens were re-pinned, spelling only, checked against the base over 782 slow tests. The REFS claim is narrowed. The split's by-name chord grouping is filed (`a-splits-chords-of-tied-faces-pool-as-one-parent`), not fixed, since it would turn unique names into ties. Merged at 94f46f49ab; both rows closed.
- 2026-10-26 (WIRE orchestrator) wire-norank: the seam-scene authoring cases go red to green. The subtract history case is green on both sides, because name_invariance's remap_name ran flip_senses on base and hid the raw flip. It is accepted as a pin, with the reason in the PR body, and an optional raw-table check if it is cheap.
- 2026-10-26 (WIRE orchestrator) `wire-norank` opened PR 5758 (head 9acaee665e). CI is green, including python.
  - Added: Through{entering}, the edge_reading home with a typed UNSIDED refusal, and Flush::seam_sense read from kept pieces.
  - Deleted: OrderAlong, every ranker, RankRule and flip_senses.
  - Witnesses: ADMITTED_HISTORY is empty; the two seam history cases go red to green; there is a raw-table check.
  - Goldens re-pinned: lib_g16, perf2, seat4, seat7 and name_words.
  - Seven open rows cite deleted code and need triage.
  - A FULL review (`wire-norank-rev`) is dispatched.
- 2026-10-26 — (sweep follow-up, Ev's ask) The tracker sweep's (PR 5747) WIRE recommendations, each checked against main at `d621f67e`:
  - `a-unions-line-of-several-edges-is-oriented-by-its-least-edge` gains a `## Ruled` section: FORK-ORIENT (fork-log row 197, Ev 2026-10-26 "cool, seems good!", PR 5705, `964083e5`), the no-rank rule as N2 now reads. It stays open as the build: `Flush::line_sense`, `rank_along_seam`, `RankRule::{Reverse, RefuseRun}`, `flip_senses` and `rank_crossings` all still stand. It never carried `design: true`. The rows parked on it are untouched.
  - Closed as fixed by PR 5732 (`94f46f49`): `a-pair-booleans-borders-walls-cite-bare-operand-names` (the `wall` closure wraps the operand's name as this boolean's `From`, `emit_topo.rs:1823-1833`) and `a-pair-booleans-tied-corner-is-split-as-one-corner` (`corners` keyed by `(StableName, Candidate)`, `emit_topo.rs:2816`; fixture `a_tied_corners_candidates_are_named_apart_wherever_they_lie`, `:8604`).
  - Duplicates merged. `a-sliver-beside-a-rest-stands-outside-the-band-below-the-default-k` is closed into `three-names-tests-fail-at-ambiguity-k-1-5`, which survives because it holds all three reds. All three are still red at `CAD_AMBIGUITY_K=1.5` on `d621f67e` (re-run today). The survivor carries the sliver's diagnosis and fix, and is priced P3 M.
  - The names README is unchanged. Its four rank sentences (companion table line 20, "the crossings ranked along it"; NS3 line 83, "a rank along a shared line"; line 423, "the least of several flush edges"; the group-size rung, lines 749-751, "it ranks the same-sense group") describe code that is still built that way. That code is `canonical.rs`'s `RankRule` and `rank_by`, the resolver's `rank_by` over-tie, and `emit_topo`'s `least_per_line`. Line 423 is not about ranks: it is `wirerest/a-pair-booleans-seam-vertex-cites-the-least-of-several-flush-operand-edges`. Each sentence retires with the build of the FORK-ORIENT row, and that build's PR re-words them with N2.
- 2026-10-26 — **Moved here from CLEAVE** by `git mv` in CLEAVE's priority-seam cut (Ev, in chat, 2026-10-26: CLEAVE at 270.5 budget points against 30), ids and bodies unchanged, each row with a dated `## Slate triage` note giving its band: `a-split-lineage-lists-a-joined-edge-over-chords-and-an-operand-rim-as-minted`, `a-boolean-zero-decided-and-consumed-in-place-reaches-no-row`, `a-re-charted-operands-rows-may-name-cells-the-re-chart-minted`, `identical-twin-coincidence-rows-arise-on-coincident-lines`, `a-boolean-records-a-face-pair-decision-only-where-the-two-faces-meet`, `a-split-records-an-on-vertex-only-where-it-leaves-two-runs`. These are the record and lineage rows WIRE's lanes filed (`wire-citerows`, `wire-splitmax`), per Ev's direction; the code they cite is topo's (`boolean/ops.rs`, `boolean/glue.rs`, `splitting/mod.rs`, `splitting/finish.rs`), so announce the seam on CLEAVE's log. `a-meet-row-on-a-vertex-a-boolean-minted-on-an-edge-omits-the-cell-that-minted-it` stays on CLEAVE for now: PR 5753 edits it. (CLEAVE orchestrator)
- 2026-10-26 — **Seam (CLEAVE):** the ring-tie row is spelled, in PR 5753 (`cleave/ring-tie-spells-its-pierce-vertex`).
  - `boolean/mod.rs` `Mints` records each vertex a held pierce minted inside a face, beside the edge splits. `ops::input_end` reads such a vertex as that face.
  - `spell` spells a ring tie whose two cells both widen as the `Meet` of both cells and the pierced face, carried in `topo::Coincidence::third`.
  - The corner boss logs that row in every member order and in every pair boolean. No fold row in the swept tests trips `held_out` now, so WIRE can take `held-out-goes-once-the-ring-tie-row-is-fixed`.
  - On WIRE's ground in the same PR: `coincide::name_rows` names a row's third cell, `eval/wire.rs` `in_member_space` takes a three-cell `Meet`, and `test_support::FoldRowNames::cells` is a `Vec`. (CLEAVE implementer)
- 2026-10-26 — **Moved here from CLEAVE** by `git mv` in PR 5753, on the CLEAVE orchestrator's ruling: `a-meet-row-on-a-vertex-a-boolean-minted-on-an-edge-omits-the-cell-that-minted-it`, banded P1 (a cell the row omits is a misnamed row).
  - PR 5765's triage left it unbanded on CLEAVE because 5753 edits it.
  - Its measured pinch case has one spelling after 5753. What stays open is the minting cell outside ring ties, which needs `Coincidence` to hold a list of cells. (CLEAVE implementer)
- 2026-10-26 (WIRE orchestrator) PR 5758 FULL review: MERGEABLE.
  - Instrumented tables, head vs base, over the ci profile and the slow naming set: 0 new ties and 0 outcome changes. The goldens are sense-spelling only. The raw-sense check goes red on base. UNSIDED never printed under K=1.5 or ε=1e-6. Python passes 927.
  - Merged at bd36ab7b53.
  - Closed: the orient row and the wirerest `of` row. Three other programs' rows are moot and closed with notes (doctail rank-rewrite, seamrank rank-extents, scalar K>2 feet).
  - Re-pointed or partly moot: coincide, splice, issues/seam-anchored, seamrank null-copy, three-names.
  - The seamrank no-first-side row is likely fixed, unwitnessed; it re-parks on D10.
  - The pair-boolean least-edge row returns to WIRE at P2, the one place the approved rule is unbuilt.
  - Filed the Through-walker residue row (P3/E).
- 2026-10-26 (WIRE orchestrator) Dispatched `wire-pairflush` (wire/pair-flush-line-reads-each-edge). The pair boolean's flush line takes the set and Through, finishing the 5705 rule; it also repoints the README's citation of the moved row. FULL review.
- 2026-10-26 (WIRE orchestrator) `wire-pairflush` opened PR 5778 (head 5043b1e764).
  - The pair's flush line is Merged+Through via a shared line_of_several. least_per_line is deleted, and the union's grouping is an adapter, which closes the twin row.
  - No published table reaches the pair arm (fold steps only), so the witness is emitter-level and goes red on base.
  - gate ok is red, attributed to main's RESTREAD P0 CoincidentWedgesOverlap.
  - A FULL review (`wire-pairflush-rev`) is dispatched; it verifies the red first.
- 2026-10-26 (WIRE orchestrator) PR 5778 FULL review: mergeable once main's P0 clears.
  - Reachability is confirmed (pair Union fold steps only). Every published table is identical, and the witness goes red.
  - Fix pass sent:
    - whether the Merged fold keys in the FragmentGroups grouped record move anything;
    - the new fold-step refusal routes (coincident twins on the face, touch agreement);
    - vocabulary; line_of_several's one-thing case, and one line-naming home.
  - A state sync is owed so the README's work/wire path exists on main.
- 2026-10-26 (WIRE orchestrator) The pairflush fix-pass lane got a security flag for 'irreversible local destruction'. Its transcript shows rm -rf only of its own build target, logs, probe files and scratch copy, all lane-owned and deleted as the standing rules require. Nothing else was touched; the orchestrator checkout is clean. Removed the merged norank worktree and its logs.
- 2026-10-26 — **Seam (CLEAVE):** the row-cell door check (`rows_name_input_cells`, at `boolean_op_with`'s exit) left PR 5753, on the CLEAVE orchestrator's ruling. It is parked on `cleave/rows-name-input-cells-guard` (commit e14e5f7355) and waits for `a-re-charted-operands-rows-may-name-cells-the-re-chart-minted`. The reason: main still builds the re-chart's rows, 72 cells across four `sweep` tests (two per-PR, two nightly), so the check cannot land green before that row is fixed. The row's new Acceptance section makes landing the check its close. (CLEAVE implementer)
- 2026-10-26 — **Seam (CLEAVE):** PR 5753 merged and closed `a-fold-steps-ring-tie-row-names-a-pierce-vertex-its-operand-does-not-hold`. `held-out-goes-once-the-ring-tie-row-is-fixed` was parked on it and is now open: nothing else gates it. (CLEAVE orchestrator)
- 2026-10-26 (WIRE orchestrator) The orchestrator read PR 5778's fix pass.
  - Coincident flush edges cannot disagree: both read On via vtxfac, argued in the doc.
  - There is one line_name / sense_along_line helper, and no shortcuts.
  - The grouped record's size reads now move in some member orders. That class is already on main (1803 vertex names), so it is filed (`a-unions-vertex-group-size-reads-the-fold-steps-spelling`) rather than blocking.
  - Main's P0 had cleared, and CI is green. Merged at 7eb29ffbe0.
  - The pair-flush row and the wirerest twin-helper row are closed. The 5705 rule is now built at every door.
- 2026-10-26 (WIRE orchestrator) CLEAVE's split moved two WIRE-filed P1 rows back to WIRE (meet-row minting cell; re-chart rows), and unparked held_out because the ring-tie fix landed in 5753. Dispatched `wire-meetrow` (spell/EdgeSplits) and `wire-rechart` (through_the_join, then held_out), split by function within ops.rs. Priced the group-size row P3/M.
- 2026-10-26 (WIRE orchestrator) `wire-rechart` opened PR 5795 (head 5c38d62542).
  - Recharts maps re-charted cells to the input shell's LEAST face, so Meet and OnCarrier keep their relation. Other relations refuse typed.
  - CLEAVE's door check is cherry-picked; there is a moved-keys witness; held_out is deleted.
  - The full suites were not run locally (disk).
  - A FULL review (`wire-rechart-rev`) is dispatched. It presses whether 'least face' is a mint/storage-order choice, and whether the BooleanNaming blind spot misnames anything today.
- 2026-10-26 (WIRE orchestrator) `wire-meetrow` opened PR 5798 (head 6d2203440a).
  - EdgeSplit carries the minting cell `by`, and spell or input_read widen to it. Coincidence goes from `third` to `more: [Option;3]`.
  - The witness is `Held::unfixed` counts on flush_onto_an_edge (red on base). The named pinch tests were already green after 5753's ring tie.
  - Filed the residual two-spelling row on the flush pair (P3). The lane filed a CLEAVE row (in-face minted vertex).
  - The FULL review waits for disk: 5795's reviewer is building.
- 2026-10-26 (WIRE orchestrator) PR 5795 FULL review: approve with fixes. Builds and goldens are unchanged, and the base reds are identical.
  - MAJOR-1: 'least face' picks by build order. Ruling: option (a), name every face of the input shell. This needs Coincidence's wider `more` from PR 5798, so 5798 merges first.
  - MAJOR-2: OnCarrier face rows pass onto the least lune as face pairs, against coincidence.rs's meet invariant.
  - The door check is cfg(debug) with a bare assert; per D9 it must run in release with row-4's why.
  - Pre-existing and wrong today: emit_ball_cut_by_brick publishes a pole name nowhere near a pole. The ORBIT row moves to WIRE at P1, and rechart takes it next.
  - PR 5798's FULL review is dispatched.
- 2026-10-27 (WIRE orchestrator) PR 5798 FULL review: the code is sound.
  - MAJOR: the witness is green on the real merge base be2b7a1, because #5545 (CLEAVE fate-cut) already spells the flush case with Vertex@m0. The lane's red was on an older base.
  - Lane told to measure whether the row is live on current main. If it is, witness a live case. If not, close the row as fixed by #5545 and keep only what 5795 needs (Coincidence `more`), plus `by` if it still widens something.
  - The 5795 conflict and its hidden `third: None` compile break are noted for whichever PR lands second.
- 2026-10-27 (WIRE orchestrator) The container restarted. Both lanes (meetrow fix pass, rechart fix pass) were stopped and resumed. Worktrees and targets survived. Disk is at 7 GB free.
- 2026-10-27 (WIRE orchestrator) PR 5795 fix pass, items 2–5, at 7e097a6f71, gate ok green.
  - New variant RechartedRowUnstated; the face-pair refusal follows an exhaustive states_a_point.
  - The door check runs in release as unreachable! with row-4's why.
  - Item 1 (the face-set reading) is built on wire/re-chart-rows-on-more over 5798's `more`. Census: 806 two-face shells, 1676 set meets, 0 refusals.
  - Item 1 lands after 5798 merges. 5795 waits for it; it is not merged with the least-face reading.
- 2026-10-27 (WIRE orchestrator) PR 5798's fix pass found the row live on main: the minting cell was missing from about 2,900 topo rows and about 1,050 editor-core rows. The new witness, glued blocks with a pyramid out a corner, was confirmed red on main by the orchestrator. Merged at bfc6f1ddbc; row closed. 5795's lane is landing the face-set reading on `more`. Pre-existing reds reported by the lane: fused_into_live_cells, the corner_boss logs and revalue_boss_union (main).
- 2026-10-27 (WIRE orchestrator) PR 5820 (corner-boss main-red) was read by the orchestrator: a test-helper fix, from crossed merges of 5753 and 5636. Merged and the row is closed.
- 2026-10-27 (WIRE orchestrator) The orchestrator read PR 5795's item 1, the face-set reading on 5798's `more`. Merged at 423a53eddc; the re-chart row and held_out row are closed. The stale side branch wire/re-chart-rows-on-more (9d51114e8a) is left on the remote: the lane's delete push failed, and deleting another branch is outside this session's push scope. Next: the sphere re-key P1 row, to the rechart lane.
- 2026-10-27 (WIRE orchestrator) **Self-inflicted:** a main merge into wire/orchestrator conflicted on the sphere re-key row (my move against 5795's appended evidence). My chained command piped the merge through tail, which hid its exit code, so the conflict markers were committed. They were fixed in the next commit, before any PR. **Rule for this branch:** a merge step is `git merge … || exit 1` followed by a marker grep, never piped.
- 2026-10-27 (WIRE orchestrator) Ruling for the sphere re-key row: re-charted faces read as their shell's faces, and re-charted seam edges and poles refuse typed as row 2 (Unsupported*, NOT_YET_ENDING), pointing at ORBIT's retire-the-re-chart row. A wrong name is worse than a loud refusal, BORDERS already recommended this interim, and NS2 has no true form. The lane measures flips first; any corpus or python flip goes to Ev before landing.
- 2026-10-27 (WIRE orchestrator) `wire-rechart` opened PR 5829 (head 49a7fc083c).
  - BooleanNaming carries Recharts, and refuse_turned_cells gives UnsupportedTurnedCell (row 2).
  - The face reading was argued dead and not built.
  - Measured flips: only emit_ball_cut_by_brick's two subtracts; no corpus, python or tour flips.
  - An ATREST row was filed. A FULL review (`wire-rechartname-rev`) is dispatched; it tries to break the dead-face argument.
- 2026-10-27 (WIRE orchestrator) PR 5829 FULL review: MERGEABLE. The dead-face argument held (a two-brick band is caught by its turned meridian), and only the expected flips occur. A small fix pass is sent: KilledSplits lineage, the redundant step guard, operand and key in the message, test specificity, Recharts::contains. The stale BORDERS sentence is annotated.
- 2026-10-27 (WIRE orchestrator) The orchestrator read PR 5829's fix pass: one chase_edge_until with the KilledSplits hops, one GraftInverse, the redundant step guard dropped, the read and key in the message, and the test asserting Cell::Edge. Merged at cfa35248b8; sphere re-key row closed.
- 2026-10-27 (WIRE orchestrator) Two prose-census main reds, both from this session's merges:
  - RechartedRowUnstated's Debug relation (5795), filed twice as P0/P1.
  - UnsupportedTurnedCell's untypeable local keys (5829).
  Fixed directly in PR 5851 (Relation::noun; render the variant's cell field), with local census, concision and display runs green; the rows are closed in the PR.
  **Class note:** a new error variant's Display must pass pncad-py's prose_census (slow set). Lanes adding a variant run `-p pncad-py -E 'test(/prose_census/)' --run-ignored all`.
- 2026-10-27 (WIRE orchestrator) PR 5851 is green and merged at 66bfc1c6d7. Main's two prose-census reds are fixed, and both rows are closed in the PR.
- 2026-10-27 (WIRE orchestrator) Dispatched the last two P2 rows.
  - `wire-splitchords`: key the split's chord groups per candidate. Ruling: NS3 locality and N4's 'undivided candidates stay bare' settle it, no fork.
  - `wire-seamselect`: seam (and other N0-set) SegPats match as sets. Ruling: N3 and NS1(a).
  WIRE is at 35/30 load because of re-homed and filed P3s; these two P2s take it down.
- 2026-10-27 (WIRE orchestrator) `wire-seamselect` opened PR 5854 (head 67830ffe91).
  - Seam, Merged, EdgeCrossing, BandFace and BandCross/BandSlit's band match as sets via bipartite matching. all_match memoizes per (pattern, name).
  - The two witnesses are red on base, and the positional wording was unsigned agent text.
  - A review (`wire-seamselect-rev`) is dispatched; the lane ran python only in CI.
- 2026-10-27 (WIRE orchestrator) PR 5854 review: mergeable.
  - Kuhn matching is correct, the address memo is sound, and positional tags are unchanged. Python 944 OK. Three witnesses are red on base.
  - Fix pass sent: the junction-run sibling (NamePat::path over a sorted Seam run, the same class one level up) to fix or file; the re-authoring test must assert that the flip happens; the stale audit doc; reuse nest.rs's address().
- 2026-10-27 (WIRE orchestrator) PR 5854's fix pass was read by the orchestrator: junction runs now match as a set, the flip premise is asserted, and nest's address() is reused. Merged; row closed. The 7 revalue_corpus failures the lanes saw locally are on INTENT's main-red list.
- 2026-10-27 (WIRE orchestrator) `wire-splitchords` opened PR 5857 (head 8103315fe3).
  - Split chord groups are keyed by parents' operand-entity sets. The witness is u_cutter_tie, via the shared fixture that places with `against`: using a placement in a fixture is not editing D10 ground, so that is accepted.
  - A table dump over 210 split tests shows them identical apart from the witness.
  - The lane filed the seam-group sibling row (P2).
  - A FULL review (`wire-splitchords-rev`) is dispatched.

- 2026-10-26 — seam from ATREST (stage 5 C, `the-at-rest-census-is-a-check`, branch `atrest/the-at-rest-census-is-a-check`): `product.rs`: `AssembleGather::gather` holds a `ProductRefusal`; doc lines naming the gate and `Separation` restated; `eval/*` paths re-spelled.
- 2026-10-27 (WIRE orchestrator) PR 5857 FULL review: MERGEABLE. A 217-test dump shows identical tables apart from the witness, and the witness is red on base. Merged; the split-chords row is closed. The sibling seam-groups row gained the review's measurement (the pair arm misnames today) and the key-reconciliation notes.
- 2026-10-27 (WIRE orchestrator) Dispatched `wire-seamgroups` (wire/seam-groups-keyed-by-candidate). The pair and union seam groups are keyed per candidate, with one 'per candidate' spelling across split, corner and seam passes, plus Seam/SetPiece split witnesses.
- 2026-10-27 (WIRE orchestrator) WIRE was at 34/30. Moved two P3 rows to WIREREST (three-names K=1.5 calibration, identical-twin coincidence rows); no path citations. WIRE comes back under budget once the seam-groups row closes.
- 2026-10-27 (WIRE orchestrator) Dispatched `wire-throughwalk` (wire/through-names-are-walked): walkers see Through's names and are made exhaustive; Sense::other_half computes a complement.
- 2026-10-27 (WIRE orchestrator) `wire-throughwalk` opened PR 5870 (head cf2a3cb9a8).
  - embedded walks Through names as partners, and name_args and embedded are exhaustively destructured.
  - other_half returns Option; None refuses Unwalked::Misplaced, because it would otherwise rest on two independent decisions agreeing.
  - Departure accepted: select classifies the sense rather than walking it, since a pattern has no sense axis.
  - A residue row was filed. Review dispatched (`wire-throughwalk-rev`).
- 2026-10-27 (WIRE orchestrator) PR 5870 review: mergeable. The Include-walk consumers are unchanged, other_half's None cannot be reached by a legal split, and the witness is red on base. Merged; row closed. Two MINORs and the nits were appended to the residue row.
- 2026-10-27 (WIRE orchestrator) `wire-seamgroups` opened PR 5876 (head eec1685386).
  - One ParentEntities key now serves split, pair seam, union seam and the pair corner.
  - Pair and union witnesses are red on base. The split Seam/SetPiece witnesses were unbuildable and are filed.
  - The dump shows 643 tests emitting tables and 7 differing, all tie conversions.
  - A FULL review (`wire-seamgroups-rev`) is dispatched. Also freed 7 GB of stale review scratch.
- 2026-10-27 — **Seam (ATREST):** `atrest/an-instance-carries-its-parts-coincidences` touches this program's ground. `eval/wire.rs`: `wire_instantiate_part` fills the new `CarriedFindings::coincidences` (each part row with this instance as its first hop).
- 2026-10-27 — **Seam (ATREST):** `atrest/an-assertion-quiets-an-operations-unproven-row` touches this program's ground. `eval/wire.rs`: `select_one` resolves its one name through the new `resolve_name` (the ladder, at the name's kind), and `wire_measure` reads a reference's body through the new `reference_body`; both `pub(crate)`, re-exported from `eval`, and read by the quieting rule's `checks::at_rest::contact_quietable` so a row's cell resolves as a measure's reference does. `ladder::live`/`resolve_key` take any `Doc<P: ProfilePayload>`. No refusal changes.
- 2026-10-28 (WIRE orchestrator) PR 5876 review: mergeable, no BLOCKER or MAJOR.
  - The keys are exactly the candidate partition at all four sites. The reviewer's own dump agrees: only tie conversions move.
  - Witnesses are red on base, and the union key is shown to matter alone.
  - MINOR: the PR body credited the README clause to PR 4221. It came from PR 3184, an agent's commit with no Ev ruling, so the reword stands. The body is corrected.
  - MINOR: the tie-summed decline lost its end-to-end witness. Filed `a-tie-summed-seam-decline-has-no-end-to-end-witness` (P3, E).
  - NITs: Debug-string matching in the new test helpers; ParentEntities is singleton-shaped at four of its six sites.
  - Merged at eec1685386; row closed; worktree removed.
- 2026-10-28 (WIRE orchestrator) Budget: main's merge plus the priced PR 5876 rows put WIRE at 35/30.
  - Priced: split tie-witness row E; union fold row M.
  - Moved to WIRETAIL (P3, 5 points): `the-older-missing-rule-naming-variants-lack-the-unsupported-prefix` and `a-body-value-consumed-by-k-nodes-is-gated-at-rest-k-times`. The measuredoor citation is repointed.
- 2026-10-28 (WIRE orchestrator) PR 5887 (state sync) merged.
- 2026-10-28 (WIRE orchestrator) Dispatched two lanes:
  - `wire-unionrecord` (wire/union-tied-groups-record-their-own-count): `a-unions-tied-groups-go-to-the-fold-though-formed-per-parent`. It decides whether to record or fold; FULL review.
  - `wire-tiewitness` (wire/tie-summed-decline-witnessed): the tie-summed decline witness and the split Seam/SetPiece tie witness, tests only; STYLE review.
- 2026-10-28 (WIRE orchestrator) `wire-tiewitness` opened PR 5891 (head fc5bf60b1f), tests only.
  - A tied pair seam group's own count is pinned; it goes red with name-only keys.
  - The split `Joined::Seam` tie witness is built; it goes red with `(slot, name)`.
  - The redundant assertion is dropped.
  - The end-to-end tie-summed decline turns out unreachable: `group_resized` returns at `fragment_base` before reading the record, and neither `record_by_name` site leaves a fragment-tailed row. A proposal to retire `record_by_name` is on the row.
  - `SetPiece` is still unwitnessed. Both rows stay open with findings.
  - Review dispatched (`wire-tiewitness-rev`).
- 2026-10-28 (`wire-tiewitness`) Tests only.
  - Pinned a tied pair seam group's own count, 2 → 0, which reads 4 with the name-only key.
  - Witnessed the split's `Joined::Seam` key with a vertical split of a pocketed U-tie. It goes red on revert.
  - Argued that the `record_by_name` tie-summed decline cannot be reached, and measured it: zero `sizes() == None` across the suite.
  - `SetPiece` has no witness; what was tried is on its row. Both rows stay open.
- 2026-10-28 (WIRE orchestrator) PR 5891 review: mergeable. Both reverts went red when the reviewer ran them.
  - MINOR: the row's closed-carrier claim was wrong. A tied joined set on a closed carrier would decline needlessly, because it is recorded tie-summed; the case is still unbuilt.
  - MINORs: fixture copies (U profile, prism, a private run, twin prong fixtures).
  - A fix pass shared them through `fixture` and corrected the row.
  - Merged at df2462ff36. Both rows stay open, with the `record(.., Parent::Elsewhere)` proposal for joined sets on the tie-summed row.
- 2026-10-28 (WIRE orchestrator) `wire-unionrecord` opened PR 5902 (head e2888ac188).
  - A union records its tied parents' face and seam groups with `record`; `Rederived`/`by_fold` are deleted.
  - Dump over 11,648 union evaluations: 0 rows and 0 sizes moved; 101 tied bases are now recorded as the union's own.
  - The witness passes on base too: no document was found where the fold and the union's own count differ.
  - Filed `doctail/a-unions-seam-piece-on-a-divided-parent-diagnoses-as-cascade`. FULL review dispatched (`wire-unionrecord-rev`).
- 2026-10-28 (WIRE orchestrator) PR 5902 FULL review: the code is correct, and the README reword needs no Ev sign-off (f457b08861 is an implementation commit).
  - MAJOR: the PR claimed the readings never differ, which is false. The reviewer's two-bar document has unequal tied groups. Base read `GroupResized` in one member order and declined in the other; the branch declines in both, which is right by N5. The PR owes this as a red-on-base pin.
  - MINORs: a stale `Rederived` cite in a WIRE row, and `groups.rs` tie_summed docs.
  - Fix pass sent to the lane.
  - Filed `wirerest/the-judgement-naming-fault-seam-refuses-other-tests-unions` (P4, E) for a pre-existing threaded-cargo-test flake.
- 2026-10-28 (WIRE orchestrator) PR 5902 fix pass: the unequal-tied-groups witness is red on base, the PR body is corrected, and the docs are fixed. Merged at e341b9d915; row closed; worktree removed.
- 2026-10-28 (WIRE orchestrator) Main re-weighted the cost tiers (2f96694d12: E 1, M 3, H 8), which put WIREREST at 37/30 and WIRETAIL at 34/30.
  - Split on the refusal seam into the new track WIREREFUSE (ab_band 18200–18299): five rows from WIREREST and four from WIRETAIL, by `git mv`. Nothing cited their paths.
  - Corrected both plans' slates: closed flush-helper rows dropped; moved-in rows listed.
  - Loads: WIRE 29, WIREREST 26, WIRETAIL 24, WIREREFUSE 21, WIREHOLD 23.
- 2026-10-28 (WIRE orchestrator) Dispatched two lanes, both with FULL review.
  - `wire-unionvertex` (wire/union-vertex-groups-from-the-finished-body): union vertex group sizes come from the finished body, not the fold steps' spelling. Takes the member-corner tied-fold row if it falls out of the same pass.
  - `wire-joinedsets` (wire/joined-sets-record-per-candidate): joined sets record per candidate, and `record_by_name`/`tie_summed` are retired or their docs made exact. Also the joined set over two tied operand edges. D10 constructs are kept out of fixtures.
- 2026-10-28 (WIRE orchestrator) PR 5916 (`wire-joinedsets`) is reviewed and merged.
  - Joined sets record per candidate. Seam vertices record no group, since they are always bare.
  - `record_by_name` and `tie_summed` are deleted. Dump: no table moved; only the count-1 seam-vertex groups went.
  - The FULL review found no code defect. I fixed its MINORs myself, as docs only: the stale README group-size clause, the groups.rs wrap and the closed row's "would answer".
  - Closed: `a-tie-summed-seam-decline-…`.
  - `a-joined-set-over-two-tied-…` is back to open, with no D10-free witness yet. The review's gap (a non-transversal touch) and the line-group decline guard are noted on the row.
  - Merged at 50b65794a4; worktree removed.
- 2026-10-28 (WIRE orchestrator) `wire-unionvertex` opened PR 5930 (head f1c3dffe9b).
  - A union vertex group is one member corner's copies, recorded over the finished body. The order-dependent probe went from 1806 to 0. Published digests are identical; 51,688 bare-vertex size lines moved.
  - It re-introduces `Rederived::by_fold` for fold `Tied` rows at member corners and for non-seam member edges.
  - The witness runs all 24 member orders.
  - The corner row was not taken: routing its tied rows through the corner rule refuses two building documents. Filed `a-unions-member-edge-group-size-reads-the-fold-steps-spelling` (P3, M).
  - Answered the lane's D10 question: test documents may use existing Place nodes unchanged. The lane rule is amended.
  - FULL review dispatched (`wire-unionvertex-rev`).
- 2026-10-28 (WIRE orchestrator) PR 5930 FULL review: mergeable. The witness is red on current main (16 of 24 orders) and no other diagnosis changed (probe).
  - MINORs: `by_fold` takes any Fragment-tailed kept vertex, not just fold-tied member corners; "0 disagreements" has no guard; the witness doc is wrong about which orders fail; the `group_resized` docs are stale; the member-edge row's numbers are pre-5916.
  - Fix pass sent.
- 2026-10-28 — **Filed by SPLITDOOR:** `a-split-in-the-document-drops-its-operands-contact-records` (P1). `wire_split` gives each side an empty `Carry.contacts`. Found by SPLITDOOR's split-door-records designers. (SPLITDOOR orchestrator)
- 2026-10-30 (INTENT tail retriage) Received from INTENT by `git mv`, ids and bodies unchanged, priority and cost set (PR body has each row's reason): `an-assemble-of-a-family-counts-it-one-part` (P1). INTENT's charter is D10 and these are follow-ups found while building it.
- 2026-11-01 (WIRE orchestrator) The PR 5930 fix pass resumed after the usage limit.
  - `by_fold` is narrowed to fold-Tied rows at member corners. The suite-only assumption is stated at its site. Witness and rung docs are corrected; NITs are done.
  - Merged at 712e1e7c46. The vertex group-size row is closed; the member-corner row is back to open. Worktree removed.
- 2026-11-01 (WIRE orchestrator) D10 is built: INTENT closed 2026-10-30, and the hold is released. The lane rule is updated.
  - INTENT's tail brought two P1 rows: `an-assemble-of-a-family-counts-it-one-part` and `a-split-in-the-document-drops-its-operands-contact-records`. WIRE stood at 32/30.
  - Moved `a-unions-member-edge-group-size-…` to WIREREST (29/30). WIRE is now 29/30.
  - Dispatched `wire-carries` for both P1 rows, as two PRs. The split row gets the interim carry only; the HELDRECORDS split-gate fork is still open.
  - Dispatched `wirehold-triage` (read-only): every WIREHOLD row checked against post-D10 main as moot, still real or changed.
- 2026-11-01 (WIRE orchestrator) WIREHOLD re-read after D10 (triage): 2 rows closed as moot, 10 re-scoped. Kept as a track (WIRE is full, and tests cite its row paths), retitled to WIRE's post-D10 residue. Load 25/30.
