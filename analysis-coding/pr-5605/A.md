# Review A — PR 5605 (census: planar face pair, flush pair, absorption confined by location), frozen head ddb44f28

**Verdict: NOT-MERGEABLE-AS-IS** (one MAJOR; the rest small fixes). CI green on the head. No glimpse. Probes on a review branch (6 rows; one red on purpose).

## MAJOR
**M1 (claim 4): the flush door reports faces that share an interior area as flush.** `census.rs` `flush_at_rest` writes a `SameOriented` flush row when Door 2 answers `Err(TouchingBoundary)` — the region walk's refusal, left standing because the interior-witness rung declined; not a certificate. Instrumented heat_sink: all 20 new flush rows pair a united fin wall (z 0.25–1.0) with the coincident instance fin wall (z 0.1875–1.0): same plane, same facing, 0.75×0.75 shared interior. Reproduced in topo: probe (base ∪ sunk fin, then the same fin grafted as an instance) red with 4 rows; Door 2 answers TouchingBoundary in both frame orders for the four wall pairs. The same walls as two plain prisms read correctly (0 rows), so the rung is incomplete once one face comes out of a boolean. Goldens bless 20 wrong rows in heat_sink, @10, @40, @160 at all three ε. Bodies still refused elsewhere, but the flush door states something false. Execution · sure.

## MINOR
- **m1 (claim 1):** the "refuses rather than absorbs" arm has no row: mutant `None => errors.extend(read)` → `None => {}` in `Declared::confine` survives 3190 topo + 503 related rows; no test asserts the CensusUndecidable text. Execution · sure.
- **m2 (claim 1):** the tangent-line arm tested one direction only: mutant `in_overlap_closure` → `Some(true)` for any Line survives; `Some(false)` is killed by sweep's rod rows. Execution · sure.
- **m3 (claim 6):** topo README "Confirmed pairs" paragraph still states the by-pair rule and lists retired `vv_face_backed`/`ve_face_backed`/`sides_face_backed`; another line names `Declared::ve_face_backed`. Inspection · sure.
- **m4 (claim 6):** tracker items left behind: contacthold `census-face-rungs-for-a-collinear-overlap-are-unconfined` still parked asking for retired symbols; `declared-only-meetings-clear-at-the-census-gate-unread` builds on retired rungs. Inspection · sure.
- **m5:** stale prose: `validate.rs` doc (by-pair), census module doc ("on two carriers … answers for nothing"), two test comments citing retired names. Inspection · sure.

## NOTE
- n1: non-convex holds (L-shaped base; collinear run outside overlap and spike tip each own rows; touch along overlap boundary through re-entrant corner stays the pair's); by-pair probe discriminates; corners-meet mutant killed by 8 rows. Execution · sure.
- n2: the crossing rung's new location check changes nothing reachable (mutant survives; geometry: a transversal crossing of two coplanar opposed faces' boundary edges always shares a quadrant). Likely.
- n3: incidence lanes sound for area pairs but loose for line/point pairs within band (an in-band vertex ~√(2Rε) off the tangent line absorbed by incidence). Unsure.
- n4: line and point arms go beyond `[assertions]`' literal "meeting over an area". Likely.
- n5: goldens moved as the body says, but heat_sink gained 20 rows in each of four documents, not one; those are M1's. Execution · sure.
- n6: claim 3 reads as described. Likely.

## Style (Q1–Q7; Q8 partial)
- Q3 sure: m1/m2 rows cannot go red.
- Q4 sure: the README paragraph this PR wrote states the retired rule (fix mints the defect); same in validate.rs.
- Q1 likely: line arm inlines its own wedge loop next to `corners_meet`; `Declared::answers` (by-pair) beside `holds` (by-location).
- Q2 likely: `pair_edge_face`'s doc defends incidence beyond its argument.
- Q6 sure: chart_region's "decline is the empty overlap" is an unguarded measurement-shaped claim M1 falsifies.
- Q7 unsure: `confine` judges every finding at `q`.
