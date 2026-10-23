# Review A — PR 5613 (D′2), frozen head ca29c81468

**Verdict: APPROVE-WITH-FIXES.** Two MAJORs need fixing before merge; both are small and local. One is a written value silently lost when a constraint is removed, demonstrated by a red probe. The other is the retired frame helpers' behaviour surviving in the tour, found by inspection. The rest of the PR holds. CI `gate ok` green on ca29c814 (run 39925572244: demos tour + wild, python suite, viewer feature build, test, lint). Probes pushed to a review branch (`r?_dprime2_probe.rs`); `…removing_a_chains_middle_link` is red on purpose to demonstrate MAJOR-1. No glimpse.

## MAJOR
**MAJOR-1 (claim 4): removing a link from a written spin chain silently drops a later value the user wrote.** Shown by running it; sure.
- Where: `edit.rs` `rebundle` → `mate/pin.rs` `kept`.
- Setup: a copy with no mate, spins written `Z(0.1), X(0.2), Z(0.3)`; `RemoveConstraint` on the middle `Spin(X)`.
- What happens: the leftover links are `Z, Z`; `kept` treats the second `Z` as a repeat and drops it, then `unnamed` appends `Spin(X)=0, Spin(Y)=0`.
- Measured: bundle afterwards `[Spin(Z)=0.1, slides, Spin(X)=0, Spin(Y)=0]`; the copy turns 0.1 rad, not the 0.4 that setting the removed link to 0 would give. Nothing reports the drop: `EditRecord` lists only what was minted.
- Coverage: `intent_s3_d_placements::the_door_admits_a_bundle_with_no_mate_only_whole` removes only the first link of `Z,X,Z`, which is harmless.
- Related, by design but silent: an `AddConstraint` whose mate fixes a direction keeps the first of two `Spin(Z)` links and drops the other; in `X(0.2), Z(0.3)` the kept `Spin(Z)` changes meaning (second chain link → spin about the mated direction). Probe `…_rebundle_keeps_what`.

**MAJOR-2 (claim 1): the retired helpers survive in `demos/tour`, undisclosed and unfiled.** By reading; sure.
- `demos/tour/src/world.rs` `frame_off` is `frame_at`/`picked_frame` almost line for line: calls `evaluate`, picks the first line-or-conic edge and a vertex off it, writes moves as literals through `moved_frame`. Live in `bracket.rs` (split plane via `plane_on`) and `impeller.rs` (pattern axis via `axis_on`). New in this PR.
- `against` keeps `place_against`'s read-back: when the target is a placed copy, it composes `solve_document(..).relative(node)` into literal `atan2` values (reachable, no current caller).
- `moves` is `written_moves`' `atan2` rule.
- The module doc of `world.rs` says "nothing read back off an evaluation" — false.
- The PR's sweep covered `crates/` only; the gate on `mate/pin.rs` cannot see `demos/`; the filed viewer item names only the viewer.
- Deviation 2 (viewer `frame_off`/`moves_of`) itself is acceptable: a viewer design question, filed P2.

## MINOR
**MINOR-1 (claim 2): the space chart jumps near the antipode, with no refusal.** Shown by running it; sure that it happens; acceptability is Ev's call.
- Boss hung under plate copy `B`'s bottom face (normal −z), `Slide(X)=10`. B tilt 0 → x=+10; 1e-9..1e-2 about x → +10; 1e-9 about y → −10. A nanoradian edit moves the boss 20 units; `mate::reference_about`'s chord band does not fire at 1e-9.
- New: main used the body's own frame. Not in the spec, not filed; needs an [ev] item.

**MINOR-2 (claim 6): msolve14's `chain` scene no longer means what it says.** Re-derived by hand; likely. Third bolt: head plane z = 4, slides at 0 from the world origin give (0,0,4) vs main's (4,1,4) — agrees with the PR table; test 24's formula confirmed by execution. But the bolt now floats in bolt 2's head plane, 4.1 units from bolt 2; the fixture comment is false; same in `pinned-chain`. The spec says generators are rewritten to write the motion they meant; here the digest was re-pinned and `at(0,0,0)` kept.

**MINOR-3: correction to the brief's premise.** `VERSION` 19 is the content key's format version, never persisted. Claim 7 otherwise holds by execution (`…_mains_documents_load`): main's `golden.cad` and `gallery_ring.pncad` refuse loudly ("unknown variant `spin_about` … regenerate the file"); `Value` is `deny_unknown_fields`; `die_tool.pncad`, `plate_param.pncad` refuse earlier on an id-format change.

**MINOR-4: `ENVELOPE` 35 → 34** (`persist/nesting.rs`); the pose-chain depth hazard is filed with a done-when. Likely.

## NOTE
- Claim 3 holds (reading + test 20 row): `written` counts turns by axis ≤3, no two in a row about one axis; `unnamed` completes with distinct axes; order first written first made. Test 20's departure from the spec ("three slides" beside a ball is incoherent) is an improvement, but spec row 20 still says "three slides".
- Claim 5 holds by reading: `inline` mints a Space, carries nodes verbatim, refuses only `UnplaceablePart` (deviation 1, interim, acceptable); `flatten` checks constant-0 identity, a `Node::Space` source, refuses a shared space, typed.
- Claim 6 other re-pins not re-derived.

## Style
Exercised Q1–Q7; Q8 partial (`mate/value.rs`, `mate/pin.rs` end to end).
- Q1 sure: frame → atan2 Euler moves in three copies (`test_support::moves_at`, viewer `moves_of`, tour `moves`), near-identical incl. `cos_beta > f64::EPSILON.sqrt()`; fix-mints-the-defect.
- Q2/Q5 sure: `Composed::head` doc stale.
- Q4 sure: `placement.rs` `Frame::compose` still calls itself inline's rule.
- Q5 sure: tour `world.rs` module doc (MAJOR-2).
- Q3 likely: test 21 slides along z under an instance turned about z (commutes).
- Q3 likely: test 20's "turns about the copy's origin rather than the mated point" cannot go red.
- Q3 sure: the rebundle row removes only the first link.
- Q4 likely: spec row 20 / §5′.2 still say three slides.
- Q6 sure: tour `frame_off`/`against` not disclosed or scheduled.
- Q7 unsure: viewer `in_one_space` keeps world position only where the two spaces stand alike.
- Q7 likely: `kept` and `written` restate the chain rule separately; a third in `unnamed`.
- Q2 unsure: `unnamed` puts spins after slides, relying on unstated ordering.
