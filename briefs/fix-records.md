# embed/records (PR 7124) — fix pass off the union of two reviews

Both reviews of head `18c956f37a` are NOT-MERGEABLE-AS-IS. They are beside
this file verbatim, `briefs/records-review-A.md` and `briefs/records-review-B.md`;
read both in full before starting, since the items below name findings by
their ids there (A's `M1`, B's `m5`, …) and do not repeat the evidence.
Each reviewer's probes and mutants are described in its report; rebuild
them from the description (a red row first, then the fix). Both reviewers
re-read this pass, so answer every item in your report by its letter:
done (with the row that pins it), or not done and why.

The standing discipline is `docs/prompts/implementer-discipline.md`, by
path, as before. Merge main first.

## The property, and what failed it

The unit exists for one sentence: **a finished body holds no record its
gate did not confirm, and no public call can make one that does.** Three
public paths break it, each demonstrated by both reviewers.

**A. `finish_assembled` / `Assembled::new` (A M1, B M2).** A record
enters a `⊔`'s gate from two sources only: a finished arrival's own held
records (confirmed at that arrival's gate, carried by lineage, moved with
it), and what the gate's own census between the arrivals records. No
public constructor takes a loose list. Prefer closing this by type (the
gate takes the arrivals as finished bodies and reads their records
itself) over re-confirming every carried record; if the code's shape
does not allow that, the gate certifies each carried record exactly as
`finish` does and refuses the same way. Either way: A's sub-point that
the product gather discards a contradiction inside a `censused` arrival
(`product.rs:1568-1579`) is fixed with it; `Bar::ArrivalsDecided`'s doc
becomes true or goes (item E); the `cfg(test)` fixture
`validate::unconfirmed` keeps its premise or is removed (B M2(b)); the
two dropped split rows come back; the blend's own gate is pinned from
sweep.

**B. `records_certified` (A M2, B M1).** "Confirmed" is not "not Stale
and not Contradicted". A record is held only where the census confirms
it; every other reading of a recorded pair (undecidable, thin in band,
unsupported, touching boundary, escalated, and any variant added later)
refuses at the door that handed it over, typed. Ask the census which
findings name a record rather than filtering by variant, so a new
variant cannot be dropped silently. B's class note is an obligation:
sweep `record_at_rest`'s `recordable` filter and `gate3_findings` for
the same shape (a refusal list narrowed by variant to mean "fine") and
report what you found at each, fixed or not.

**C. `FinishedBody::try_recite` (A M3, B m1).** On `Err` the body is
unchanged: its records, its bar. And a public method that rewrites a
finished body's citations to anything is a second way round the gate: it
either goes from the public surface or what it writes is certified. Say
which you chose and who calls it.

## The gate's price

**F. Confirm-only gate (A m1, B m3).** The gate confirms the records it
holds; it does not run the whole census and discard what names no
record. A measured this: the fast set identical, and
`six_crossing_corners_build_every_op` back near its uncertified time.
Build it (it is also the natural home of item B: there is no finding
list left to filter), and move that test back to the fast set; the move
to the slow set was the cost of the discarded census, not of the unit.
Re-measure the price table in the PR body on the merged tree, with the
meter committed to a measure branch and named in the body.

## Pins the suite lacks

**D. Three hand-offs with no row (A M4; mutants survive).** n-ary carved
members and tool material finish with no records, 0 red; a box scalar's
taker finishes a replay leaf without its records, 0 red; a move that
drops records is caught only by the `unreachable!` at `wire.rs:4445`.
One row each that goes red under A's mutant, and the move's row asserts
the records, not a panic. A's style note on `props.rs:7973` (`take`
drops rows) is read with the box-scalar item: say whether it is the same
hole.

**I. Product gather against the `⊔` graft (A claim 7, B m6).** A measured
558 gathers equal and found no committed check. Commit one: an assert at
the gather or a row over the population, whichever is cheaper to keep
true.

**L. The deleted cusp-edge test and the dropped curved split row (A
claim 8, B m8).** B shows the stated reason for the deletion is false
once item A is fixed. Re-pin the property the test held (not necessarily
the test), and the split row (ii).

**K. `_with` callers (A claim 5, B m4).** `boolean_op_with` /
`boolean_placed`, `checks/at_rest.rs:1351`, and the n-ary pairwise
judgement can hand fewer records than the operands hold; A's kiss with
default inputs ends with 0 records and passes. For each caller: either
it hands the operands' records, or its site says why it may not and a
row pins what the result then holds. The PR body's claim is corrected to
what is true.

**Mutants.** Re-run both of yours and the reviewers' (A's three in M4,
B's mutant C "plain doors carry only operand a's records", which went
red on only 2 rows; B n3: none of mutant A's reds is in editor-core, so
add a document-level row that pins certification). Report reds per
mutant over the fast set.

**O.** The slow rows this diff adds or edits: the gate's step ran 0 of
them on both shards. Run them by name and report.

## Smaller, all owed

**E. `Bar` (A m2, B m2).** Nothing reads it. If `origin/embed/gate`
holds a reader, name it, keep `Bar`, make a leaf move keep it (the doc
says an isometry keeps it; both probes show `ArrivalsDecided` turning
into `RecordsConfirmed`), stop setting `ArrivalsDecided` for an all-None
closure, and pin it. If there is no reader, remove it; a field with no
reader and a false doc is not a seam.

**G. Recourse (A m3, B m7).** A stale record at a door is the kernel's
carry, not a declaration the user can remove: one recourse text per
family, the door's ending where a door wrote the record;
`BlendError::Carry` agrees with its `Recourse` row. One spelling of the
refusal sentence, not three (both style sections).

**H. Citations (A m5, B m5).** Minted `Decided(k)` starts at 0 because
`numbering_from` has no caller; at distribute it names the pieces' rows,
and B adds that a piece's `Carried{arrival,k}` cites the read's record
k. Make a citation name what it cites after distribute and after a
merge, pin it, and fix `distribute.rs:191-195`'s doc. Drop
`numbering_from`/`first_row` if nothing needs the knob.

**J. `From<Body> for Built`, `Built::of`, `Built::recording` (A style,
B n6).** `finish(Body::clone(&finished_with_records))` compiles at any
door and drops the records in silence; the PR body calls that
impossible. Remove the `From` so every finish spells what it hands
(`Built::of(body)` then reads as "this body touches nothing" at its
site, and a grep finds each), make `recording` add or rename it to say
it replaces, and make `Built`'s doc say what the gate does on a
record-free body (item M).

**M. Text.** (1) `crates/topo/README.md`'s tier-3 paragraph: the
sentence your diff replaced came through `[ev]` PR 4951 (B n5, commit
`156cf5c0cf`); the PR body's "no ratification beyond PR 5967" is wrong,
so correct it there and say what you changed and where you looked. The
replacement is a description of state the change moved, and that is why
it may ride this PR, but as written it reads as a rule ("a meeting no
record backs is not raised by a deciding door's gate") eight lines under
the ratified sentence it falls short of ("none, for a body that touches
nothing, where the census must find no coincidence at all"). Word it as
what it is: what the gate does today, that it falls short of the
sentence above, and the rows that owe the rest. Do not touch the
ratified sentence. (2) The flip row names `deciding_door_census`, which
does not exist (A m4). (3) `ShellError::Carry` has no row raising it (A
m6): add one or show it unreachable and remove it. (4) Owners are told
in their own logs, not only in the PR body (A m7, B n7): a line on each
owning program's `log.md` for each seam you leave them.

**N. Style, both sections.** Stale `carry.rs` docs; `Carry::member` back
to its narrowest visibility; `Citing::Each`'s positional list beside the
body with its `unreachable!`; `records_certified`'s doc; the product
gather's deep clone of a graft it throws away; `SplitContacts`' pub
fields; `AtRestBody`'s impls in two blocks; the rest as each section
lists them. Take what is sure or likely; for anything you leave, one
line why.

**Q.** `Census::verdict(doc, tol)` ignores `tol` (A claim 7): fix it or
file it on its owner with the witness.

**R.** File one row: a stale carried record meets three different ends
(A style); and A claim 6's three working copies
(`product::Source.contacts`, `acc_records`, `SceneBody`) as a row on
ONECOPY's side if they are not already there.

## Before you report

Merged with main, on that tree: the fast set, the slow rows by name
(item O), the endpoint census, the tour walk
(`cd demos/tour; cargo run --release --example embed_walk | grep '^WALK'`;
say the count, and say which bodies changed against main), the mutants.
The report leads with the three paths of the property (A, B, C), each
with the row that was red and is green, then the rest by letter. No
model identifiers in commits or the PR body.
