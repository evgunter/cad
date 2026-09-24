# PORT log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/port/plan.md`.

## Opened (2026-09-11)

Opened in the tracker cut of 2026-09-11 (Ev's direction, in-chat;
`docs/WORK-TRACKS-2026-09.md` addendum 3). Seven rows moved in by
`git mv`, each with a `## Re-homed` record: three from `work/issues/`,
four from `work/code-quality/`.

This is the track most likely to be dissolved by its own seams: every row
is on LIB's, EXCH's or DOCM's ground, and any of them may take a row at
any time. The plan says so — a row claimed away moves by `git mv` and
this program does not track it afterwards, which is code-quality's rule
applied to a program that is its descendant.

## First sitting: two rulings answered, two calls taken (2026-09-15)

The program had never dispatched anything — eight rows, all `open`, no
`port/` branch, no PR, and a log with one entry. This sitting did no
code; it cleared what stood in front of the code.

**Two rows were waiting on Ev and neither had been asked.** `S107`
carried `needs_ev: true` since 2026-08-20 and `msrv-floor-…` was called
an Ev question by the plan, but `git log --all --grep` finds no `[ev]`
PR was ever opened for either. They were flagged, not asked. Both were
put to Ev in chat and both were answered the same day:

- **`S107`: a defect**, on the general principle that *Python should
  always match Rust where it can*. The compatibility reading had no
  installed base to protect — `publish = false`, no name yet (Q9). The
  ruling closes `S107` carrying its verdict; the work it releases is
  `python-dimensionerror-names-the-quantity-check-not-the-dimension-check`,
  filed the same day, because a residue disclosed in prose is not
  scheduled (`work/README.md`). That row and
  `load-path-stringifies-structured-refusals` are one door and spec
  together: the first frees the name `DimensionError`, the second
  decides what the load door's refusals arrive as.
- **`msrv-floor-…`: a check that the two strings are equal, and call it
  a day.** That is the row's reading (2) made mechanical. It is a better
  answer than it looks: pinned equal, the channel *is* the floor, so
  every job that already compiles on the channel compiles at the floor —
  the promise becomes the one CI already proves, with no new build row.
  The row now names its cost (the floor can never lag the channel; the
  day it should, the gate is deleted deliberately and the original
  question returns with a consumer attached) and its shape: a
  `scripts/gates/` row, not a Rust `#[test]`, with the substitution
  disclosed for Ev to reject. Class drops **M → E**.

**The two assembly rows were the orchestrator's call and are decided.**
Ev declined to weigh in and left it here on a confidence test. Both
went the way the tree already pointed:

- Widen. `AssemblyError::AtRest` already carries every finding and
  publishes that as a guarantee; the two mint arms drop all but the head
  twenty lines away in the same function. Contracting would mean
  deleting a guarantee the type publishes to make three arms agree at
  the weaker answer. This was never really an open question — the door
  had answered it.
- Kind-first in `resolve_face`. MSOLVE-5's premise for `operand_answer`
  — a non-face never mints anywhere — is about the name, not the table,
  so it holds at the root too. It is also the more answerable refusal,
  which is this program's standing review question.

They dispatch as **one unit**, same file, same door.

**Review posture rewritten** (Ev, in-chat): style review is the default,
full review reserved for the hardest units. The section had asked for a
correctness arm on every unit changing a public refusal or a binding
signature — which is nearly every row here, so it was a full review on
almost everything. The one full review on this slate goes to
`load-path-stringifies-structured-refusals`: found by execution rather
than reading, carries a two-sweep class obligation, and a wrong fix
there is invisible from Rust.

**Charter corrected.** `program.md`'s `keep_out` and `plan.md` both
routed three rows to **DOCM**, which closed at sweep 14 (2026-09-13);
`assembly.rs`, `node.rs` and `persist/wire.rs` are **EDIT's** now, and
EDIT's own `keep_out` names this program back for the first two. The
slate table was also two rows short — `load-path-…` arrived at DOCM's
exit sweep after the table was written, and `S107`'s successor was
filed today.

**Filed outside the fence:** `work/edit/edit-charter-counts-a-row-that-went-to-port`.
EDIT's charter counts the load door's structured refusals among its
rows, but that row came to PORT in the same sweep; and EDIT's `keep_out`
names PORT for `assembly.rs` and `node.rs` but not for `persist/wire.rs`,
where PORT's row actually lands. Worth noting for the double-claim
rule's eventual promotion to an error: **PORT claims no paths at all**,
so it can never appear in an overlap pair, and its announcement surface
lives entirely in prose that nothing checks.

Nothing on this slate now waits on Ev. `S415` is still the opener.

## 2026-09-15 — S415, the three boundary residues (implementer lane)

All three closed on `port/s415-boundary-residues`. None of the files
are PORT's: `crates/step-import/*`, `crates/step-export/*` and
`crates/stl/*` are **EXCH's**, `crates/pncad-py/*` is **LIB's**. The
unit is announced to both in the PR body; either may take the row's
successors instead.

**Three of the row's premises were wrong**, two of them corrected in
the spec and one found here. The spec's two held: the printable-ASCII
band is **two formats** (Part 21, read and written) and not three, and
`no_minted_id` **does** have a kernel enum behind it. The third is the
spec's own: it said the two `no_minted_id` paths differ on the wire
because `declare_err` "carries the arm's fields" — it does not for
that arm, which passes `(None, EditPayload::NONE)` exactly as
`boundary_edit_err` does. The two were byte-identical to a Python
caller, and only the human message differed. That changed the verdict
from "a defect a caller can hit" to "a synonym nothing would have
reported", which is the shape the LIB filing below carries.

**Judgement calls taken**, since the spec left all three open:

- The scaffold guard is a **post-condition on the minted endpoint**,
  not a pre-condition on `p`: it states the rule the two sites share
  (a strut with coincident endpoints cannot certify) rather than
  today's arithmetic for reaching it, and it survives a change in the
  offset. Both sites share it outright, as two free functions in
  `assemble.rs` — `coincide` and `strut_endpoint`.
- `plant`'s coincidence check moved from **bitwise to value
  equality**. Bitwise is narrower than the hazard: `0.0` and `-0.0`
  are different bits and the same point, and the chord between them
  is still zero-length.
- **The Part 21 duplication stays.** A shared home would need a new
  crate or a dependency edge between the two STEP crates, and the
  kernel is no place for a text-format constant (`docs/DESIGN.md`'s
  `## Layering` section — prose, carrying no D-number, so it is cited
  by name rather than by an id it does not have). A leaf crate below
  both is the workspace's own precedent (`test-utils`), and a
  `step-import` -> `step-export` edge already exists as a
  DEV-dependency for the round-trip oracle — neither can carry a
  shipped constant as it stands, and both are heavy answers for one
  range. What
  changed is that it is disclosed at all three sites — the two STEP
  sites as a mirrored pair, the STL site as a coincidence deliberately
  not shared — and that both bounds are pinned by a test at each STEP
  site, which neither had.

**Filed outside the fence** (§6), both in the same PR:

- `work/exch/step-scaffold-strut-offset-is-absolute-in-a-unit-free-format`
  — the second half the row did not name: `1.0` is absolute in a
  format whose coordinates carry no unit contract. The degenerate
  half is closed here; the magnitude is a design call on EXCH's
  ground.
- `work/lib/boundary-minted-refusal-tags-are-pinned-nowhere-and-share-the-kernel-namespace`
  — the tag inventory re-derives itself by reading `src/tags.rs`, so
  the three words minted at raise sites (`name_serialize`,
  `not_utf8`, `wireframe`) are outside its reach: their values are
  pinned nowhere and nothing would have reported the `no_minted_id`
  collision.

The **M** estimate held: four crates, three judgement calls, two
filings, no ruling in front of any of it.

### Fix pass after the style review (2026-09-15)

Five must-fix, one sweep receipt, five rows filed. What moved:

- **The threshold was wrong and it was wrong three places deep** — the
  original row, the spec, and this lane's own `strut_endpoint` doc all
  said the offset vanishes at `|p.x| >= 2^53`. Checked: it is not a
  threshold on EITHER side. In `[2^53, 2^54)` the offset is exactly
  half a step, so round-half-to-even decides on the value's own
  mantissa parity — `2^53` loses it, `2^53 + 2` keeps it. From `2^54`
  out it is always lost. The negative side is the same picture shifted
  one binade, because a positive offset moves a negative coordinate
  TOWARD zero into the finer binade: `-2^53 + 1.0` is exact. The test
  now pins both bands on both parities and both edges, so the claim
  cannot be restated wrong without reddening.
- **The site-level row the spec asked for was written, and the branch
  turns out to be dead on the corpus.** Instrumenting the arm and
  running the whole `step-import` suite reaches it **zero** times: the
  corpus always takes the plain `mef` fan order. So the row drives the
  arm directly, and its ordinary-coordinate control proves the guard
  is not unconditional.
- **`mev_line` does refuse a coincident chord, and the message is the
  argument.** Probed: *"certification: the stored parameter interval
  is not forward … a degenerate zero-span interval is refused by the
  same gate"*. Loud, but in the certification gate's vocabulary about
  parameter intervals — which is why the guard's own `Topology` arm is
  worth having, and the PR says so with the real text rather than a
  guess about a NaN direction.
- **`D-layering` was an invented decision id** carried from the spec
  into this log. `docs/DESIGN.md`'s `## Layering` is prose with no
  D-number and is now cited by name. Two related claims were also
  looser than stated and are fixed in the code comments: a leaf crate
  below both is the workspace's own precedent (`test-utils`), and a
  `step-import` -> `step-export` edge already exists as a
  dev-dependency.
- **The spec's ledger entry was missing.** `docs/DOC-LEDGER.md` now
  carries the `## Per-merge deletion` row, with the recovery SHA and
  the four statements of the spec the unit corrects.

The **band class was not swept** in the first pass, which is the class
this unit is mostly about — the disclosure covered three of eleven
statements of `0x20..=0x7E` across the three crates. Swept now: the
eleven-site hit list is in the PR, and the per-crate restatements that
had no reason to spell the numbers now point at the site that decides.
Nine remain, each with a stated reason (three deciding matches, two
user-facing refusal texts, three public docs, one bounds argument).

**Filed outside the fence in the fix pass** — LIB:
`persist-err-projects-fourteen-arms-through-a-fifteen-slot-positional-tuple`,
`doc-module-header-promises-a-door-it-does-not-hold`; EXCH:
`signed-zero-module-hand-counts-the-bitwise-readers-it-exists-for`,
`three-crates-fold-signed-zero-with-one-body-and-three-unlinked-arguments`.
The 76 tag words restated in `py/doc.rs` prose went as **evidence on
the row already filed** rather than as a second row — same fence, same
blind spot. That row's proposed remedy was also rewritten: it had
offered a hand-maintained roster of words nothing derives as the
answer to "nothing derives this set", which is the trap, pre-committed
into the next unit's brief. It now states the two properties owed and
leaves the mechanism open.

## 2026-09-15 — PORT-DOORS-1 in review (PR 2635)

Both assembly-door findings landed as one unit, on
`port/doors-1-refusal-order`.

**Part A, widen.** `AssemblyError`'s two mint channels each carry every
refusal the gather recorded: `Mint { refusals: Vec<MintRefusal> }`
replaces the flattened `Reference` / `NoAtRestRecord` arms, and
`CarriedMintRefusal` takes `Vec<CarriedRefusal>`. Two arms, not one, so
the carried-before-own precedence the doc-comment calls load-bearing
stays visible in the type. The shape is `AtRest { findings }`'s, which
is the argument the row made: one enum answering one question one way.
The two follow-up comments are deleted.

**Part A's façade.** `MintRefusal` and `CarriedRefusal` left
`crates/pncad/tests/all.rs`'s `NOT_CARRIED` — they were held out on the
ground that the gate's answer named no row type, which the widening
ends — and cross to Python as two frozen classes under
`AssemblyError.refusals`. The two words a caller branched on
(`mate_reference_refused`, `no_at_rest_record`) moved one level in, onto
the rows, under a new `mint_refusal_tag`; the gate's own tag for its own
mates is `unminted_mates`. Nothing stringified.

**Part B, kind before tie.** `resolve_face` asks the NAME's kind before
multiplicity, so a tied non-face answers `NotAFace { kind }` exactly as
`operand_answer` already does, and only a tie AMONG FACES is
`Ambiguous`. The premise the whole part rests on holds against the
tree: `NameTable::insert_tied_ref` refuses any candidate whose
`key.kind()` differs from `name.kind`, and every path to `Entry::Tied`
(including `project` via `defer::narrow_into`) goes through it.

**What moved.** One of the two msolve5 control assertions, as the
orchestrator read it: the tied-EDGE row's second assertion, now
`NotAFace { kind: Edge }` at the pattern as well as below it, with the
row renamed and its divergence-as-a-rule doc-comment deleted. The
tied-FACE row is unchanged and its `Ambiguous` still stands.
`display_contract`'s `Ambiguous` row is a hand-built `RefusedRef`, not a
`resolve_face` answer, so Part B does not reach it; Part A re-homed
every `AssemblyError` row in that file onto the arm that now carries it,
and added one pinning the list rendering of both channels.

**Filed outside the fence:**
`work/wire/interrogate-read-answers-a-tie-before-the-door-s-kind` — the
sweep's one hit, `interrogate::read` asking `entity_of` (which refuses
`Ambiguous` for any tie) before the door's kind question, so a tied edge
name at `face_frame` answers `Ambiguous` where a unique one answers
`WrongKind`. WIRE's ground, and a public error-channel change of its
own.

**Territory announced:** `crates/editor-core/src/assembly.rs` is EDIT's
and the pncad-py façade is LIB's; PORT claims no paths.

## 2026-09-15 — PORT-DOORS-1 fix pass (PR 2635, full review)

Full review returned no MAJOR, one MINOR, nineteen style findings. The
MINOR was a **falsified claim**: `crates/editor-core/ASSEMBLY.md`'s A3
still cited `AssemblyError::NoAtRestRecord`, a path the change retired.
Repaired to `MintRefusal::NoAtRestRecord`, and a SECOND stale sentence
the review did not name was found beside it — the same page said the
carried refusal is raised *"on the head row in gather order"*, which
the widening makes false. Both are repointings of a ratified page that
an approved code change moved, not decisions (CLAUDE.md's git-workflow
test), so they land with the change that caused them; what the clauses
DECIDE is untouched.

**The class letters were wrong twice.** The lane reported `L`, which is
not in this program's vocabulary — `plan.md` defines `E`/`M`/`H` and
no plan in `work/` carries an `L`. Both cells are now **H**, on the
scale's own "spanning several programs' territory" clause: the widening
retires two public `AssemblyError` variants and reaches EDIT's
`assembly.rs` and LIB's whole façade. `plan.md`'s review-posture
sentence is corrected in the same PR too: the full review is no longer
"that row alone", because this unit asked for the second arm under the
escape hatch the same section defines — and the escape hatch earned
its keep, since the style lane's sweep would not have asked after a
ratified page's citations.

**The list-renderer class, all three sites disposed.** The review was
right that a point fix was a half-fix: `finding::render_list`,
`product.rs`'s disclosed-but-unrouted closure and the lane's own
`render_refusals` were three copies of one loop. The unification is in
`finding.rs`: `render_lines` is the loop, over anything that renders as
ONE line, and `render_list` is `render_lines` over a `Composed`
adapter. Both mint arms and `ProductError` now call it. The reason the
lane gave for not routing — a `MintRefusal` is already a composed
sentence — was a reason not to COMPOSE it, never a reason to copy the
loop, and that distinction is now what the sink's doc says.

**A second sweep, shaped for the shape the first one missed.** The
review observed that two of the three known instances are CALLER-SIDE
kind/tie pairings, which the `Entry::Tied`-adjacent grep can only find
by luck, and asked whether the instrument was structurally wrong. It
was. The second instrument walks every fn body in `crates/*/src`, finds
those that both reach a tie-refusal producer and test an entity kind,
and reports which comes FIRST. 23 functions do both; 8 reach the tie
first; 5 are projection or tag matches where arm order is not
semantics, 1 is a test fixture, 1 is `resolve_face` matching its own
signature line — and 1 is new:
`work/wire/declare-door-refuses-a-tie-before-it-asks-the-pairs-kinds`,
the declare door refusing `Ambiguous` before `DeclareUnsupportedPair`
can name the pair's kinds. Third instance of the class, filed on WIRE's
slate beside the second, and it sits on the same two lines as WIRE's
existing `the-declared-pair-refusal-reads-the-authored-kind`.

**Also filed:**
`work/lib/route-fields-builds-both-python-objects-to-return-one` —
the lane added the third and fourth caller that indexes a helper
returning a pair, deliberately (a consistent wart over an inconsistent
fix in LIB's file), and disclosing it without a row would have been the
thing `work/README.md:117` forbids.

Everything else the review raised was taken: the Python row that
computed its expectation from the answer's own length (both mates and
both words are written out now), the two probe names that asserted
"first bad mate wins" over a diff that made both win, the sweep-receipt
line that gave an untrue reason for a true conclusion (`select.rs`
DOES refuse over a tie — it is kind-first because `NamePat::matches`
tests `name.kind` and `continue`s before a candidate is read), the two
`TiedDisagrees` sites absent from the table, the non-empty "guarantee"
that was prose on a `pub Vec` (now stated as what the door does, with
the demo's vacuous `all` fixed), the `display_contract` sentence
claiming a guard that could not exist, `MintRefusal`'s type doc naming
one of its two destinations, the carried arm repeating its recourse
once per row (now once, in the header), and `debug_assert!(false, ...)`
restated as a positive predicate.

### Render drift on this branch is main's, not this unit's

Run 34953058313 posted `render drift (kernel)` and `render drift
(freecad)` neutral checks naming `s_duct.png`, `twisted_tube.png` and
both montages. §§3 says a frame that changed is telling you the kernel
changed, so it was chased rather than waved through.

It is not this unit's. The evidence: the branch's PREVIOUS head
(`ef4d485e2`) posted NO drift check at all, and drift appeared only
after merging `origin/main` at `bad35258d`; that range carries
`crates/sweep/src/loft.rs` and `skin.rs` from PR #2466, the s393
start-frame door, and `s_duct`/`twisted_tube` are sweep-and-loft
scenes; and this branch's own diff against main touches no geometry
crate, no render scene and no cell. Main's own re-baseline for the
kernel lane, `cba1c4e8b`, landed AFTER the commit this branch had
merged — merging the newer main cleared the kernel drift exactly as
that explains.

The freecad lane's re-baseline has not landed on main yet, so
`render drift (freecad)` still reports `twisted_tube.png`. It is the
same scene and the same cause, and the check says a PR run does not
re-baseline: main's own run commits the cells after the merge. Nothing
to commit by hand here, and nothing about this unit.

### Merge-forward before landing (2026-09-15)

One conflicted path, `crates/editor-core/tests/display_contract.rs`,
resolved by keeping BOTH sides. TINT-1 (#2648) re-homed `assert_f6`
into `test_utils::f6` and added `assert_f6_every_variant`; this lane
added the `mint()` helper that every assembly row in that file now
builds its subject with. The local `assert_f6` this lane was calling is
SUPERSEDED, not dropped: main's replacement has the same name and the
same signature and passes exactly the field-punctuation roster the
local copy hard-coded (`node:`, `name:`), so every rewritten row keeps
its meaning while the predicate moves to its one home. Taking this
lane's copy instead would have re-minted the third spelling that
TINT-1 exists to delete. No other path collided; every assembly row
this lane rewrote survived the auto-merge and was re-read to confirm
it.

**S415 (#2633) was already an ancestor** of this branch before the
conflict arose, so its `py/doc.rs` and `tags.rs` work had already been
merged silently. Re-verified against it rather than assumed: the
pncad-py tag-inventory test, the binding census and the full python
suite are green.

**The render drift is closed, re-read rather than re-asserted.** Main
now carries BOTH re-baselines and both are ancestors of this branch:
`cba1c4e8b` for the kernel lane and `29fdea8d6` for the freecad lane,
the latter touching exactly the two cells the drift check named
(`montage-freecad.png`, `twisted_tube.png`). Checked independently
that this lane could not have caused either: `s_duct` and
`twisted_tube` are built in `demos/tour/src/skinned.rs`, and this
branch's only change under `demos/` is an import line and one
assertion in `demos/tour/src/assembly.rs`, which renders no cell. The
run on the merge-forward posts NO drift check on either lane.

## State-sync at merge: the first two units land (2026-09-15)

`S415` (#2633) and `PORT-DOORS-1` (#2635) merged; both, and the two
findings PORT-DOORS-1 carries, are closed. `PORT-DOORS-1-SPEC.md` is
deleted with its ledger row (`docs/DOC-LEDGER.md`); `S415-SPEC.md`'s row
landed in its own PR after the review caught that the first push had
deleted the spec without one.

**Three of the four premise corrections were to this program's own
text**, not to the rows it inherited, which is the thing to carry
forward. The orchestrator's `S415` spec asserted that the two
`no_minted_id` paths differ on the wire (they are byte-identical — both
pass `EditPayload::NONE`), invented **`D-layering`** as a decision id
that exists nowhere in the tree, and repeated the row's `|p.x| ≳ 2⁵³`
threshold, which is not a threshold on either side: inside
`[2^53, 2^54)` the offset is exactly half a step, so round-half-to-even
decides on mantissa parity, and the negative side is that picture
shifted one binade. The `PORT-DOORS-1` spec read the widening as a shape
change when it is a vocabulary change, and said two control assertions
move where only one does.

Each was caught by a lane or a reviewer reading the spec against the
tree, which is the arrangement working — but a dispatch that sounds
authoritative is read as authoritative, and `D-layering` reached a
committed log line before anyone checked it. **`git log -S` is cheap and
the lanes now run it**: the `ASSEMBLY.md` A5 sentence the widening
falsified turned out never to have been ratified at all (`#2482`, a DOOR
lane closing a stale citation, not an `[ev]` PR), so the page's
companion-table row saying *Ratified* was not evidence about that
sentence. CLAUDE.md says exactly this; the habit is to run the check
before the argument, not after.

**Review posture, as exercised.** `S415` took the style lane and the
five must-fix items it returned were all real. `PORT-DOORS-1` asked for
the second arm under the escape hatch and was granted it; the full
review's one falsified claim was a **citation on a ratified design
page**, which neither of that lane's sweeps was shaped to look for,
because a lane's instruments are shaped by its own diff. Whether
"changes a ratified design page's subject matter" becomes a fourth
trigger for the second arm is with Ev — it binds future units. The
narrower habit needs no ruling and the lane has adopted it: `git grep`
over `crates/**/*.md` and `docs/**/*.md` for any public item a unit
renames.

**Two sweep instruments, and the first one was wrong.** PORT-DOORS-1's
`Entry::Tied`-adjacent grep found one of four sites in its class, and
only because it was the site the lane was already fixing. Challenged, the
lane conceded and wrote a caller-side instrument — a function that both
reaches a tie refusal and tests an entity kind, ordered by which comes
first — which found a third instance immediately (`eval/wire.rs`, filed
on WIRE). Both instruments are name-based and neither is a proof; the
lane says so in the PR.

Nine rows were filed outside this program's fence across the two units:
three on EXCH, four on LIB, two on WIRE. PORT claims no paths, so every
one of them went to the program whose ground it landed on.

Next: the Python pair, `python-cannot-set-options-structs` then `D341`.
`msrv-floor-…` is specced and unclaimed. The two naming rows —
`python-dimensionerror-…` and `load-path-…` — are one door and one spec,
and `load-path-…` is the row this program gives a full review.

## 2026-09-15 — `python-cannot-set-options-structs` in review (#2678)

Four doors bound, one census grown from one options struct to five.
The unit's claim was measured rather than inspected, twice: a probe
field added to each struct before the diff, and again after. **Before,
two of the four were silent at both sites** — `ImportOptions` (the door
called `::default()`) and `EvalOptions` (the literal ended in `..`) —
and the other two red only at the door, because no struct had a census
roster. After, all four red at both. The census half also reds on the
default no-Python build path, which is where the ordinary `test (…)`
jobs compile, so three of the four are guarded where nothing guarded
them before.

**Two of the spec's premises were wrong, and the lane found both by
reading the tree.** The spec said the STL doors' work was small because
they already take `solid_name=` / `header=`; they took them defaulting
to `""`, which is neither struct's `Default` — so `to_stl_ascii()` from
Python wrote `solid ` where Rust wrote `solid part`, and
`to_stl_binary()` wrote 80 zero bytes where Rust wrote the producer
text. That is the re-spelled-constant half of the same class, sitting
at the two doors the spec waved through. And the spec said a `NotBound`
entry's decay was already checked; the decay half read the stub's
DECLARED NAMES, which an options keyword never is, so the first options
entry to be declined would have stayed green forever. The alphabet is
per-roster now. Both corrections are in the ledger row that retired the
spec.

The sweep found a **fifth instance** and filed it: `ChecksConfig`'s
Python constructor re-spells all four of the kernel struct's defaults
in its `#[pyo3(signature = …)]` instead of forwarding them, and neither
it nor `McConfig` has a census anchor. Two rows went to LIB — that one,
and this unit's own residue, `ImportOptions::declared_contacts`, which
cannot be a keyword until `ImportContact` has a Python value class.

Class `M` was right and stands. Style review, as the posture says; the
correctness arm is the compiler, and the four probe measurements are
what say the anchor is actually there.

Next: `D341`, the `match Node` census this unit's instrument is the mold
for. `msrv-floor-…` is specced and unclaimed.

## 2026-09-15 — `python-cannot-set-options-structs` fix pass (#2678)

The full review held all eight correctness claims and re-ran the four
probes itself rather than reading the table, including restoring main's
two files with the probe still in place to reproduce the **before**
state for `EvalOptions`. Its own differently-shaped sweep — every
`#[pyo3(signature = …)]` in `py/` carrying a non-`None` keyword default
— found exactly the one hit the lane had already filed.

**The posture section is corrected above, and the correction is the
unit's own finding.** The converse clause — "a compiler-enforced claim
does not earn the second arm" — was being applied PER UNIT when it only
ever holds PER CLAIM. This unit's field-presence claim is as guarded as
a claim gets and its forwarded-value claim was guarded by nothing, and
**a presence anchor cannot guard a forwarded value**: that is what a
destructure is. Two STL doors sat under a green census writing `solid `
and eighty zero bytes for their whole existence. The section now says
so and cites this unit.

**The `NotBound` reason half is unguarded, and the first two entries to
use it were wrong.** The decay check falsifies the SPELLING, never the
REASON, and the review caught `param_box` claiming there is no f64
evaluation behind a box when `editor-core`'s MC lane runs one and
`monte_carlo` is its Python door — what Python cannot ask for is a box
with WIDTH. `profile_lift`'s reason was broader than what holds for the
same reason, one entry over. Both rewritten, and
`the_not_bound_roster_decays` now says in its own docs that the reason
is a reader's job.

Three further instruments landed in the pass: the alphabet is carried
on the roster instead of chosen at four call sites (the failure this
unit had just fixed, one level up), `every_options_type_in_py_is_rostered`
makes the roster LIST derivable from a source scan instead of
hand-kept, and `test_mesh.py` pins forwarding rather than
non-emptiness.

Four more rows filed, bringing this unit's total to six: LIB gets the
stub-signature gap, the `ImportReport.eps_in` quantity asymmetry and
the absent box-with-width evaluation door the three `EvalOptions`
reasons defer to; CIW gets the `step import (freecad)` job whose name
is the export fixtures' row. PORT claims no paths, so all six went to
the owner.

## 2026-09-15 — `msrv-floor-is-declared-and-never-compiled` → review (PR 2676)

Ev's answer of 2026-09-15 landed as `scripts/gates/msrv-floor-equals-channel.sh`:
`Cargo.toml`'s `[workspace.package] rust-version` and
`rust-toolchain.toml`'s `[toolchain] channel` must be the same string. Both
read `1.97.0` today, so the gate is green from its first run and has nothing
to say until a PR moves one of them — which is the moment it exists for.

**Shape as the item proposed it, not as Ev spelled it.** Ev said "unit test";
this is a `scripts/gates/` row, because the subject is two root manifests
rather than a crate's behaviour and `gate-roster.sh` proves a gate in that
directory is wired into both halves of CI. The deviation is disclosed in the
item and again in the PR body, and the row moves if Ev wants a Rust test.

**The cost is in the gate's header, where a future lane will read it.** The
gate forbids the floor from ever lagging the channel, so the day this
repository wants "we build with 1.99 and still support 1.97" — real once **Q9**
lands and something is published — the gate is wrong and must be deleted
deliberately. The header names Q9 and says so, and the non-version-channel
arm (`channel = "stable"`) refuses with the same pointer.

**Equality is exact.** `1.97` against `1.97.0` fires. The declaration is a
copy of the pin, character for character; a tolerance is a second rule to
maintain, and the only way the spellings diverge is an edit to one file made
without reading the other.

`local-scripts/ci-local.sh` needed **no edit** — verified, not assumed: its
`discipline()` loops `scripts/gates/*.sh` and self-tests each gate before
running it, so the new file is picked up by existing code. `gate-roster.sh`
went from 21 gates to 22 and is green.

Territory: `.github/workflows/ci.yml` is **CIW's** and `scripts/gates/` is
**GUARD's** (the item and the dispatch brief both guessed META; `work.py
territory` says `guard`). Both announced in the PR; either may take the row.
`Cargo.toml` and `rust-toolchain.toml` are unchanged.

Class **E** confirmed. **Four rows filed**, on CIW's and GUARD's slates —
the fix-pass section below names them.

This entry said *"No rows filed — nothing turned up outside the fence"*
when it was written, and that was false: the sweep behind it looked for the
wrong shape and reported a negative result rather than a blind spot, and
the style review found what it missed — a MAJOR inside the fence and three
rows outside it. The sentence is corrected here rather than annotated,
because a reader scanning this log for what a unit filed reads the claim,
not the footnote. What the wrong sweep was and what would have caught it is
in `work/guard/gate-directory-counts-in-prose-go-stale-on-every-new-gate.md`.

### CI (2026-09-15)

Run `34997969247` (head `2f33783bb`) was green in 37 of 39 jobs. The two
that spoke to this change — `CI half parity + gate wiring (every tier)`
and `discipline (evaluation-code)` — were both green, as were all twelve
`test (…)` jobs bar one and all five `k-lint (gate, …)` unifications, so
the run was the full matrix and nothing narrowed it.

The one red was `test (eps = 1e-12, 1/2)`, at
`crates/editor-core/tests/review_gui1_r1.rs:498` — *"no draw hit the cube
— generator shape broke"*, the searched anti-vacuity guard S-TINT already
carries a row for, on a third seed and a third eps row. **Filed as
evidence on the existing row** rather than as a second one:
`work/tint/pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1.md`. This
diff is a shell script and a YAML step; it compiles nothing and cannot
reach `editor-core`.

Run **`35001465730`** (head `ee7127d48`, which adds only that markdown
file) is **green in all 39 jobs**, `test (eps = 1e-12, 1/2)` included —
same Rust tree, new seed. That is the run of record.

### Fix pass after the style review (2026-09-15, PR 2676)

The review returned one MAJOR and fifteen style findings. All are
addressed on the same branch; what follows is what actually changed and
which of this lane's earlier claims were wrong.

**MAJOR — the gate held one of three declarations.** Three manifests in
this tree declare a literal `rust-version`: `Cargo.toml`'s
`[workspace.package]`, and `[package]` entries in `benches/Cargo.toml`
and `interval-transcendentals/Cargo.toml`. The last two are in
`Cargo.toml`'s `exclude` list, inherit nothing, and were unheld — so the
gate would have forced the workspace floor up on a channel bump and left
them behind, which is the failure it exists to prevent, one directory
over. `interval-transcendentals` is in the kernel's build closure through
`geom-core`'s `interval` feature, so its floor is live.

The gate now scans **every `Cargo.toml` in the tree** (`target/` and
`.git/` pruned) and holds every literal floor equal to the channel;
`rust-version.workspace = true` parses as a table and is skipped as an
inheritance rather than a declaration. `scripts/doc-gate.sh --print-roots`
returns `.`, `benches`, `demos/tour`, `demos/wild`,
`interval-transcendentals`, `tools/k-lint`, `tools/tess-lint`,
`tools/tess-meter`; the walk is a **superset** of that (26 manifests,
exactly what `git ls-files -- '*Cargo.toml'` lists), so a fifth root is
covered the day it lands and so is the first member that stops
inheriting. The deviation from the review's suggested derivation is
deliberate and argued in the gate's header: `--print-roots` needs `cargo`
and `git`, which would put both in a row that is otherwise greps and
would make every self-test fixture an initialised repository.

**A refusal and a dead reader now get different sentences**, which
`lib.sh`'s `gate_reader_died_refusal` block draws the line for. The
reader exits 3 for a refusal about documents it read (missing key,
non-string floor, non-version channel, rustup's legacy bare-name file)
and 2 for a reader that could not run (parse failure, unreadable file, no
`tomllib`). Six self-test cases assert the framing with `--also`, so the
two cannot quietly become one again.

**Both `gate_require_file` guards now have a case.** Only the toolchain
twin had one; deleting the manifest guard left the self-test green while
its own summary claimed a missing subject file was diagnosed. The
reader's blind `except` was the reason it stayed invisible — an absent
file arrived as "does not parse" — so `OSError` and `TOMLDecodeError` are
caught apart now.

**Nine mutations, all red**: loosen the compare to major.minor; `if
False:` for the comparison; scan only the workspace manifest (reds on the
excluded-root case — the MAJOR, held); drop either `gate_require_file`;
collapse the refusal framing into the dead-reader one; drop the `target/`
prune (reds on the stale-manifest near-miss); plus the two from the first
pass.

**Rows filed — four, on two other programs' slates:**

- `work/ciw/seal-oracle-refuses-toml-spellings-the-msrv-gate-blesses.md` —
  `local-scripts/seal-oracle.sh`'s sed requires a double-quoted `channel`
  at column 0 and refuses an indented or single-quoted one; the new gate
  plants a single-quoted channel as a **must-pass** fixture. The two
  readers of one field now disagree about what a well-formed pin looks
  like. Three ways out are written down; PORT took none — it is CIW's
  file and the choice is theirs.
- `work/guard/gate-require-file-guards-with-no-gone-case.md` —
  `viewer-module-kinds.sh:664` and `:665`, both deletable with its
  `--selftest` green, proved by mutation.
- `work/guard/gate-directory-counts-in-prose-go-stale-on-every-new-gate.md`
  — the two stale counts this PR creates, in GUARD's own files.
- `work/guard/embedded-python-readers-are-copied-between-gates-and-unlinted.md`
  — three gates embed a python reader; the plumbing is copied rather than
  shared, and none of the 297 lines is in `ruff.toml`'s population (two of
  the three fail it, `BLE001` among them, which is the same conflation
  that hid the guard above).

**What PORT changed in another program's file, rather than filing:**
`local-scripts/seal-oracle.sh`'s header stated *"The two carry the same
string today and nothing requires them to"* — a premise this PR abolishes,
in shipped code. Reworded to say that the two remain different claims, that
a gate now holds them equal, and that the distinction is what keeps the read
correct on the day the gate is deleted. Announced to CIW in the PR body.

**Two claims of this lane's that were false:**

- *"No prose in the repo carries a hardcoded gate count"* — it carries
  two, and the sweep that said otherwise was gate-count-shaped
  (`21 gates`) where the drift was directory-size-shaped (`(24 files)`).
  The blind spot and a pattern that would have caught both are in the
  GUARD row.
- *"No rows filed — nothing turned up outside the fence"* — four did.

**Also:** the ci.yml step name lost its `(ratified 2026-09-15)` suffix,
which elsewhere marks a membership Ev ratified into a design page and here
labelled an in-chat answer whose only written record is an item file that
is deleted when PORT closes; the gate's header quotes the step's real
name. The header's *"every hosted job … builds on the channel"* is now the
hedged form the `gate_ok` line always had — the `discipline` job runs no
cargo, `ci.yml` sets `RUSTUP_TOOLCHAIN: stable` for tool installs, and a
TIER=docs run has no build jobs. `GATE_SCAN_FILES` is the walk's own count
rather than a hand-held 2, and the embedded python passes `ruff.toml`.


### Merge-forward (2026-09-15)

`origin/main` moved a long way while this unit ran — `S415` (#2633),
`PORT-DOORS-1` (#2635) and `python-cannot-set-options-structs` (#2678)
all landed, with their state-syncs. Merged forward; **one conflicted
path, this file**, and every entry on both sides is kept. The merged
entries sit in date order ahead of this unit's, which are the newest
work. Nothing else collided — the gate, its ci.yml step and the four
filings touch no path those units reach.

### The floor is not inert (2026-09-15)

Added after Ev cleared the resolver question. `Cargo.toml` sets
`resolver = "3"`, which makes `rust-version` an **input to dependency
version selection** — cargo prefers versions whose own `rust-version`
the floor clears. The item told Ev the floor is documentation nothing
computes with; that premise was incomplete, and it is a reason the floor
tracking the channel is coherent rather than merely tidy.

`Cargo.lock` is committed, so the preference is consulted when the
lockfile is **regenerated** — a deliberate act — and not on an ordinary
build. Recorded in the gate's header and beside the declaration itself,
because the repository recorded it nowhere.

## The options doors land; the posture is corrected per-claim (2026-09-15)

`python-cannot-set-options-structs` merged (#2678) and closes. Six rows
filed outside the fence — five on LIB, one on CIW.

**The unit disproved the posture clause written the same morning.** It
was dispatched style-only under the converse rule *"a compiler-enforced
claim does not earn the second arm however public its surface"*, on the
reading that `surface_census.rs`'s destructure anchor is the correctness
arm. The lane's four probe measurements — required by the spec precisely
because that reasoning is only worth as much as the anchor — showed
`ImportOptions` and `EvalOptions` silent at **both** sites before the
change, and the `NotBound` decay half reading declared *names* where an
options field is bound by becoming a *keyword*, so the first declined
options entry would have stayed green forever. The spec asserted that
machinery was sound and told the lane to reuse it.

The reviewer then gave the mechanism the correction needed: **a presence
anchor cannot guard a forwarded value; that is what a destructure is.**
The converse clause was applied per-unit when it only ever holds per
claim. `plan.md`'s Review posture now says so, with this unit as the
worked example and the cost named — two STL doors sat under a green
census writing `solid ` and eighty zero bytes where the kernel's own
defaults say a part name and a producer line, for as long as they had
existed. The spec had waved those two doors through as "the visible work
is small".

**What the review bought, beyond the finding.** It ran the four probes
itself rather than reading the table, reproduced every line number,
built the wheel and observed the bytes, restored main's two files with a
probe still in place to confirm the *before* state, and ran five
mutations against the census. It also swept a different shape — every
`#[pyo3(signature = …)]` for a non-`None` keyword default — and found
exactly one hit, the row the lane had already filed. Two MINORs, both
sentences rather than behaviour.

**One written reason was false on the first entries to use the device.**
`param_box`'s `NotBound` reason claimed no f64 evaluation could carry a
box; `mc.rs` builds one and `monte_carlo` is its Python door. The decay
check falsifies a **spelling**, never a **reason**, and that gap is now
stated at the check itself.

**The fix pass made the roster derivable** rather than hand-listed —
`every_options_type_in_py_is_rostered` scans `src/py/` for a constructed
`*Options` type and fails if it is unrostered, measured with a probe,
with its scanner checked by a second test — and the module header stopped
claiming "every" in favour of the narrow rule it actually enforces, with
`ChecksConfig`/`McConfig` named as the blind spot and their row cited.

**A lane habit worth carrying.** The fix pass's first push went red on
`test-utils`'s `reader_census`: the new source scan put `surface_census.rs`
into the class of files that read Rust source, which owes a ledger line.
Every local check the lane ran was package-scoped (`-p pncad-py`), and a
tree-wide census living in another crate is invisible to those. CI found
what no local check could.


## The MSRV floor is held, at every manifest that declares one (2026-09-15)

`msrv-floor-is-declared-and-never-compiled` merged (#2676) and closes.
Ev's ruling — hold the two strings equal and call it a day — is a
`scripts/gates/` row with a ten-case selftest, wired into both halves by
the roster with no hand-maintained list.

**The style review returned a MAJOR and it was the completeness of the
fence.** Three manifests declare a literal `rust-version = "1.97.0"`:
the workspace root, `benches/` and `interval-transcendentals/`. The last
two are in `Cargo.toml`'s `exclude` list, so they are not members and
inherit nothing. The gate held one of the three, and its header fenced
the gap with a sentence about *members* — true, and about the case that
does not occur. The day the channel moved, the gate would have forced
the root up and left two behind: the drift it exists to prevent, one
directory over. `docs/prompts/implementer-discipline.md` §2 names that
trap by name, listing those excludes, and the lane had read it.

**The lane's remedy is better than the one prescribed and says why.**
The orchestrator asked for the root set from `scripts/doc-gate.sh
--print-roots`; the lane walks **every `Cargo.toml` in the tree**
instead. That is a strict superset — it covers a fifth root the day it
lands, and also the first *member* that stops inheriting, which a root
list structurally cannot see because a member is not a root. It also
keeps `cargo metadata` and `git ls-files` out of a discipline row that
is otherwise greps, and keeps every fixture from having to be an
initialised repo with a resolvable workspace. Argued in the header
rather than left as a preference, which is what a better-than-spec
deviation owes.

**Nine mutants, all red**, including the one for the MAJOR itself
(scanning only the workspace manifest reds on the new
`plant_outside_root_drifts`). A guard green from birth — the three
manifests already agreed — is a decoration until something shows it
would fire.

**The resolver fact is now written down.** `Cargo.toml`'s
`resolver = "3"` makes `rust-version` an input to dependency version
selection, so the floor is not documentation-only, which is the premise
the item had put to Ev. `Cargo.lock` is committed, so the preference is
consulted when the lockfile is **regenerated** — a deliberate act — not
on an ordinary build. Ev cleared it; the header and a comment beside the
declaration now carry it, because nothing in the repo did.

Four rows filed outside the fence: one on CIW (`seal-oracle.sh`'s reader
refuses a `rust-toolchain.toml` spelling this gate blesses — two readers
of one field drifted in opposite directions), three on GUARD (a
`gate_require_file` guard with no gone-case, proved by mutation in a
second gate too; gate counts in prose that go stale on every new gate;
embedded python readers copied between gates and outside `ruff.toml`'s
population).

## PORT-DIMS-1 — the load door's structure and the vacated name (2026-09-15)

**PR #2702** on `port/dims-1-load-door-and-name`. Both rows
close: `load-path-stringifies-structured-refusals` and
`python-dimensionerror-names-the-quantity-check-not-the-dimension-check`.
**`H` was right**, and for the reason the plan gave — the decision, not
the diff.

**Where the structure goes.** A per-parse refusal slot
(`persist/refusal.rs`), read back by `parse_body` and routed by
`parse_err`. serde's `Deserialize` hands an impl one error type and it
is the format's, so a typed value can only leave by a channel beside the
error. The two alternatives the spec sketched were priced and rejected
in the PR: a deserializer adapter with our own `Error` type costs
`Unreadable`'s line/column, which is a tested payload, and validating
the wire type ahead of serde costs a parallel wire tree for the whole
document. Both are bigger than this unit; the slot is ~70 lines and
carries its own hazard statement.

**The name.** Reading 1 — vacant. `QuantityOpMismatch` crosses under its
Rust name; the document layer's `DimensionError` keeps reaching Python
under DOOR names, because it reaches it at four doors and two of them
carry payload the type has no room for. Part A's refusals therefore
arrive as `PersistError` / `variant == "dimension"` / `inner_variant`
from `expr_dimension_error_tag` — the shape `ParseError` already had.

**Three of the spec's premises were wrong**, all in its own "confirmed"
list or adjacent to it, and all corrected in the deleted spec's ledger
row: the load door's tag was `unreadable` and not `parse` (and had been
since PR 1553), which five doc comments, the binding census and both
rows had copied onward; the `persist/` half of the row's first sweep was
exact rather than stale; and the side-channel direction was the only one
of the three sketched that does not cost something already tested.

**Two assertions in the tree were not reaching their subject**, both
found by executing them rather than by reading:
`m4_pr6_refusal::corrupt_payloads_refuse_typed`'s ill-dimensioned case
needled a literal spelling that lost its match when `unit` was added to
the wire, and fell to an `else` branch that probed `serde_json::from_str`
instead of `load`; and `pncad-py`'s seven-arm load-door probe wrote its
literals without `unit`, so five of the seven refused as a MISSING FIELD
and never reached the dimension checker at all — they passed on
`unreadable` for the wrong reason. Both are now structural tampers that
fail if the slot they aim at is gone.

**Rows filed outside the fence**: one on EDIT
(`load-door-is-the-construction-door-for-expressions-and-not-for-profile-programs`
— the reachability sweep's finding: `wire.rs` rebuilds expressions
through their constructors and profile programs structurally, and states
only the first; `ProgramRefusal::Validate` has no stated disposition),
and a second sighting added to LIB's existing
`persist-err-projects-fourteen-arms-through-a-fifteen-slot-positional-tuple`
rather than a duplicate.

**Territory**: `crates/editor-core/src/persist/*` is EDIT's and
`crates/pncad-py/*` is LIB's; both announced on the PR.

### Fix pass (2026-09-16)

The full review returned no MAJOR, two MINORs and one style finding the
orchestrator escalated to a must-fix because the repo has a written
standard for it. Six things moved.

**The refusal channel is a type now, not a paragraph.**
`docs/PERF-SCAN-2026-08.md` §2.4 sets the bar for a production value
delivered by thread-local side effect — RAII, loud re-entrancy,
type-enforced thread confinement, coupling visible at both ends — and
`work/scalar/D283.md` records `k_stats`'s outcome in one line: *"The
thread-local stays; its correctness is now a type."* The first draft's
slot met none of the four; its clear-before-parse made re-entrancy safe
by silently discarding the outer refusal, which is criterion 2
inverted. It is now `refusal::Parse`, a `!Send`, `Drop`-closed frame
stack — `Bracket`'s shape one frame deep. Re-entrancy composes rather
than overwrites, and `record` outside a parse is a no-op, which is
strictly better than the clear-before it replaces: a `from_str` of one
wire type can no longer arm anything. **All four met**, with one
qualification: the `compile_fail` doctest that pins `!Send` for
`Bracket` cannot run for a private module, so what stands is the bound
and the type's single construction site.

**One fault, one arm.** An off-table display-unit symbol on an
expression literal arrived typed with no recourse; the same symbol on a
document PARAMETER went through `UnitSym`'s own `Deserialize` and
arrived `unreadable` with *"regenerate the file from its source
recipe"* — advice that reproduces the refusal. The unit's own sweep had
dispositioned that site **(b), correct**, which was defensible before
this diff and is exactly what this diff changed. `UnitSym` now records
into the frame, so both routes are `PersistError::Dimension` with
`inner_variant == "unknown_display_unit"`. A `test_notation.py` row had
pinned the split; it now pins the union and says what it used to say.

**The door count was wrong three ways** — four in one file, three in
two others, four implied in a fourth. It is **six doors, four classes,
three attribute spellings**, and the count is now stated ONCE
(`ErrorClass::DIMENSION_DOORS`) with the other three pointing at it,
because each carrying its own is how they diverged.

**The defect survived at a sibling route through this unit's own
door.** `py/doc.rs`'s `EditReplay` arm projects `edit_error_tag` only,
so a replayed `SetExpression` refusal reaches Python as
`inner_variant='dimension'` with the actual check surviving in the
message — and the PR called that arm *"the model the new one copies"*.
It is one rung short. Filed rather than fixed (`persist_err`'s tuple
has one `inner_variant` slot and this arm nests two levels, so the fix
is a public payload decision), with a kernel-side row proving
reachability: it has to be a TAMPERED file, because `save` replays the
log through the same doors.

**Two premises in this crate's own files were false after the diff**
and are corrected: `persist/mod.rs`'s header said the arm is decided by
serde_json's classification *"and by nothing else"*, and framed
`Unreadable` as the one door for `Data`.

**The "first refusal wins" premise is a gate**, not a reading:
`scripts/gates/persist-no-backtracking.sh` refuses `untagged`, `other`,
`flatten`, an unallowlisted `deserialize_with` and a
`serde_json::Value` intermediate across `editor-core`, with nine
self-test cases. The review named two blind spots the first draft's
prose had missed (`flatten` and a `Value` intermediate — plus a
backtracking `deserialize_with` and a retrying visitor); all are in the
matcher or in the stated gaps.

**And the disclosed cost named the wrong precedent.** `product.rs`'s
gather counter is a `cfg(debug_assertions)` counter and the least like
this of the nine thread-locals in `crates/`. The family is
`geom-core/src/sym/report.rs` and `sym/profile.rs` — the same
install/record/take scaffold spelled twice, each saying so at its copy
— plus `k_stats`. `refusal.rs` names them.

Rows filed: two more on LIB
(`persist-inner-variant-stops-one-rung-above-the-check`,
`literalerror-publishes-its-tag-under-two-names`), bringing this unit's
§6 total to four.
## The edit log has one wire shape; the pre-rows migration is gone (2026-09-23)

Surfaced by PORT-DIMS-1 (#2702) and decided by Ev in chat. #2702's
refusal slot rests on nothing on `editor-core`'s wire asking serde to try
one shape and fall back to another, and the gate that holds that
(`scripts/gates/persist-no-backtracking.sh`, on #2702's branch) fired on
`LoggedEdit`'s `#[serde(untagged)]`, which main added while the branch
sat. The two shapes existed for compactness and so that *"a log from
before rows were recorded reads as a log of bare entries"*. Ev: backward
compatibility with older logs is a red flag, and worry about churn or a
format change is never allowed to prevent a change to a better final
state.

So `LoggedEdit` is a derived `{edit, maintenance}` with both fields
always present, and the migration doors that existed only to read
pre-rows logs — `persist::load_with`, `Doc::replay_with`, and
`replay_entry`'s `migrate` argument — are removed, with their re-exports.
A bare-shaped file now refuses `Unreadable` with the regenerate recourse.
`load` returns the log as saved rather than as re-derived.

Two fixtures regenerated (`tests/golden/golden.cad` by its bless path,
`tests/corpus/tour/die_composed_tour.pncad` by `demo-tour die-corpus`),
both verified to change only the entry wrapping. The nineteen `bool13`
goldens are frozen older-build bytes asserted to refuse (v1–v4 at the
header, the rest inside the snapshot before the log is reached), and are
untouched.

The full review (triggers 1 and 2) falsified two claims, both minor, and
both closed here rather than filed. An empty entry did not mean "no
maintenance": `Maintain::Never` still derived `Join` and `Drop` rows from
the documents alone, `save` never compared an entry's rows with its
replay's, and so once `load` stopped re-deriving, `Loaded::edits` and the
viewer's `History::replayed` gave two answers for a mate insert logged
bare. Now replay performs exactly an entry's rows — a non-empty list
verbatim, an empty one refused `MaintenanceUnrecorded` if its edit
performs any — so the log has one answer and the viewer commits the entry
itself. That also made the refusal's stated reason true; it had said
"the save door always writes the rows", which `save` never enforced. One
hand-edited corpus row (`msolve10`, a contradictory rider logged bare)
now records its join. The review's S8, MSOLVE-9's spec prescribing an
untagged `MateFrame` on this retired precedent, is filed on MSOLVE's
slate (`msolve-9-spec-prescribes-an-untagged-wire`).

## PORT-DIMS-1 closes; the program is ready again (2026-09-23)

PR 2702 merges with PORT-DIMS-1 and both of its rows
(`load-path-stringifies-structured-refusals`,
`python-dimensionerror-names-the-quantity-check-not-the-dimension-check`)
closed. It landed after PR 3123, which removed the `untagged` read its
backtracking gate refused; merging main forward also needed the gate's
`plane_ref` allowlist to match the full path `program.rs` spells, and
`m4_pr6_refusal`'s tamper to reach the expression through the entry's
`edit` — by `pointer_mut`, so a vanished slot fails the test instead of
`[..]` inserting it. The orchestrator releases the program: `status:
ready`, with the remaining open rows dispatchable.

## `port/wrap-a`: three rows to review (2026-09-24)

One lane, three rows, one PR. `refusal-messages-render-floats-through-f64-display`:
`geom_core::Readable` is the one rendering of an `f64` in a message
(positional inside `[1e-4, 1e16)` and at zero, scientific outside — `{:?}`
less a trailing `.0`), and the spline stack's refusals route through it:
the row's three `Display`s, nine sibling arms the sweep found
(`SplineError::DomainInvalid` among them) and `KnotMirrorError`'s computed
`lo + hi`. The viewer's `render_number` was a copy of the same rendering
and now calls it; `quantity`'s copy stays, since neither crate depends on
the other. The remainder of the class, in eight other crates, is filed as
`work/issues/refusal-floats-outside-the-spline-stack-render-through-f64-display`.
`stackup-report-joins-rendered-blockers-on-a-mark-they-may-contain`:
the row's second join never existed (it was `serialize`'s name list);
the one real join is replaced by one blocker per line under a count,
held by an `f6_variants!`-welded arm census and a split-back test, and
the three name-joins that rely on EDIT's in-flight name door are
recorded on that row. `D341`: closed as already fixed — the binding
census's member rule (Ev, 2026-09-09) requires every `Node` and `Datum`
arm to be spelled as a constructor or listed, and already lists
`Node::Sweep`. The row predated the rule and was never re-read; a
second Rust census this lane first wrote was reverted in review.

## `props-refusal-cannot-carry-measured-overshoot` closes name-only (2026-09-24)

The row asked for a ratification: may `props/curved.rs`'s refusals carry a
measured `f64`, or is the K-stream record the payload of record? Checked
the premise the name-only answer rests on — `k_stats::decide` records the
refusing margin under its predicate name — and it holds, so the answer is
name-only, written at `NotOneChartBranch` and `NotIsoRectangle`. Ev agreed
in chat that switching would need a surprisingly good reason, and let it
land without an `[ev]` PR.

## `port/wrap-a` merged; `min-clearance-refusal-stringly-twin` to review (2026-09-24)

PR #3178 merged, so its three rows — `refusal-messages-render-floats-through-f64-display`,
`stackup-report-joins-rendered-blockers-on-a-mark-they-may-contain` and `D341` —
are closed. `min-clearance-refusal-stringly-twin`: the gate the row waited on
is gone (`clearance` is an ungated module), so `MinClearanceRefusal`'s
`(class, String)` pair is deleted and the measure layer carries the engine's own
`ClearanceRefusal`: `MinClearanceLane::min_separation`'s error,
`NodeErrorKind::MeasureClearanceRefused`'s payload, and an exhaustive enum match
in `drive.rs`'s `box_independent_measure_class` (the reader the row called
`decide_assertion`). `pncad::document` curates the enum and its `Budget` /
`Selection` payloads; the rendered text is unchanged.

Review pass on #3188: the carrier's one producer, `clearance::min_separation`,
refuses with four arms (`EmptyScope`, `NoAdmittedPair`, `Unsupported`,
`PoisonEnclosure`), so the fixtures are rebuilt from those, and
`box_independent_measure_class` says which arms reach it. `Selection` moves to
the bisecting side and `ToleranceHasNoBand` to the terminal side, by what each
means; neither reaches the measure path. The PROPS duplicate
`min-clearance-refusal-carried-by-name-across-a-boundary-that-is-gone` is closed
against this row, and `measure-refused-reduces-the-typed-refusal-to-its-name`
is filed on PROPS's slate.
