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
