# Review of PR #3900, "JOIN: the reflex corner's vertex-vertex sites", frozen head 7820c37e

**Verdict: APPROVE-WITH-FIXES.** The change does what it claims on every pose I ran, and nothing moved to a wrong body or a non-operand. But `strut_order` still decides, silently and wrongly, in a window where it should either order the germs or escalate (MAJOR-1). The fix is local to the new function.

How it was run: release builds. The "before" is the head with `strut_order` reverted to the cosine order, both in `mint_directed` and in the helper. That isolates the PR exactly, because the head's merged main predates JOIN-1's merge and current main is 107 commits further on. Review rows are on this branch, `join/reflex-corner-review`.

## Claims

1. **Falsified.** `strut_order` orders rightly past a half-turn and at exactly π, but not beside 0 or π.
   - Within a half-turn the comparand is `(g0 − g1)·e`, a cosine difference. Near 0 and near π the cosine is flat, so the difference is second order in the spacing.
   - Two germs 0.001° apart (1.7e-5 at the unit arm, about 17 000 bands) both decide their side, and then the cosine difference decides `Zero`. `Zero` maps to `Ok(false)`, "the second germ is nearer" (insert.rs:401), whichever germ is nearer.
   - Further, when the arrival edge lies along the normal, all three comparands decide `Zero` and the order still answers `Ok(false)` instead of escalating.
   - Predicate discipline: every decision goes through the `k_stats` funnel (`validate::decide`). But three different comparands share one name, and two `Zero` arms pick silently where the neighbours refuse (`direction_sense`, `chart_region_cross_order`). See MAJOR-1 and the Style section.
2. **Holds.**
   - **Widened reflex battery** `review_rc_wide_battery` (join_rc_probes.rs). It takes the 12 probe profiles, each also rotated about the corner by 0.003°, 7°, −20°, 33°, 100° and 190°. It runs 11 shears per axis (±0.75 … 0 included) and all four ops: 40 320 runs. Its oracle is fixed for caps that pass below `a`'s floor.
   - Each run's outcome has t2, t3′, cert, closed-form volume and `assert_legal_operand` columns.

     | before → after (revert → head) | runs |
     |---|---|
     | `SeamOrientation` → SOUND (t2, t3′, cert, volume < 1e-7, legal operand) | **8 481** |
     | any other line | 0 changed (31 839 identical) |
     | BAD before / after | 26 / 26 (the same lines) |
     | non-operand before / after | 0 / 0 |

   - **The PR's own battery** `join1_r1_reflex_battery` at the head, ∩ ∪ `a ∖ b`: 741 sound, 81 empty, 16 `FanStartMismatch`, 25 `JoinDesync` and 1 BAD. That is 822 sound or empty, as the PR says.
   - **A second JOIN-1 battery**, `join1_r1_declared_battery`: 27 000 lines, byte-identical between revert and head (24 356 sound, 2 638 empty, 6 `ShellWitnessExhausted`, 0 BAD). Its `outcome` has no cert or operand column (join1_r1_probes.rs:142).
3. **Holds.** Under the revert:
   - `the_strut_order_reads_angles_past_a_half_turn` panics at insert.rs:997;
   - `reflex_corner_struts_past_a_half_turn_build_sound` panics on its first pose, sqQ1 (0.5, −0.25) ∩, with `SeamOrientation`.

   Both rows are green at the head.
4. **Holds for the half-turn-cosine class as the PR scoped it, with one unlisted relative** (Style S8).
   - My sweep took a different shape from the PR's: every `decide*` whose name says order/near/first/next/turn/angle, every `levered(<dir>.dot(<dir>))`, and every `.cross(` in boolean/, splitting/, sector_shape and chord_join.
   - No other angular *ordering* turns up. The orders I found compare lengths (`bool_join_nearest`, `split_join_line_order`, `pm_census_span_order`, `chart_region_cross_order`) or decide membership (`sectors::within`, over convex sectors).
   - Blind spot: an order that a sort's closure computes in a helper with none of these names.
5. **Holds, replicated.** I applied the row's `run_order` fix exactly as written and ran `join1_r1_reflex_battery`, ∩ ∪ ∖ and `b ∖ a`, 1 152 lines:

   | before → after the fix | runs |
   |---|---|
   | `FanStartMismatch` → `SelfLoopEdge` | 12 |
   | `JoinDesync` → `SelfLoopEdge` | 7 |
   | `FanStartMismatch` → `JoinDesync` | 4 |
   | `FanStartMismatch` → `RestZipUnsupported` | 2 |
   | `FanStartMismatch` → **BAD** | **2** |

   The two new BAD poses are eBot ∪ (−0.5, −0.25), at v = 16 against 15.880, and (−0.25, −0.5), at 16 against 15.768. The row (`four-germ-vertex-pairs-run-b-in-a-order.md`, "The fix, and why it does not land alone") and the ZIP row's `## Traced` record exactly these, with their volumes. Holding the fix back is right.

## Findings

- **MAJOR-1 — `strut_order` silently misorders germs beside 0 and π, and answers when nothing orders them.** (insert.rs:397-401, 383-389.)
  - Demonstrated by `review_the_strut_order_never_misorders_germs_beside_a_half_turn_bound`, red at the head. Four pairs come back wrong: 0.001°/0.002°, 179.998°/179.999°, 180°/180.001° and
    180.001°/180.002°, each `got false`.
  - Also by `review_the_strut_order_escalates_when_nothing_orders_the_germs`, red at the head: an arrival edge along the normal gives `Ok(false)`.
  - The window is about √(2ε/arm) wide (≈ 4.5e-5 rad at a 1 m arm), and it is reachable: the probe's own `−x` germ lies exactly at π.
  - It is not a regression; the cosine rule had the same flat spots. But the new function and the join.rs:81 doc claim a correct order over the full turn. Within one half-turn the cross product of the two germs, `(g0 × g1)·n`, is linear in their spacing, and a decided zero should refuse, as `direction_sense` does.
- **NOTE-1 — the commit sequence shows an escalation relaxed to keep a synthetic fixture green.**
  - 5515da8d read an on-line germ with `sectors::direction_sense`, which refuses a decided zero.
  - 5347c89d inlined a bare cosine whose `Zero` reads "near" (insert.rs:383-389). Its stated trigger was `f12_four_survivor_pairing`, whose synthetic arrival edge is normal to its sector.
  - That fixture now passes because every comparand decides `Zero`; nothing orders its germs.
- **NOTE-2 — a pre-existing wrong-body class, wider than its row** (identical before and after; not this PR's).
  - sqQ1 ∪ at rotation 0 gives v = 16 at 10 widened poses: sx ∈ {−0.75, −0.5, −0.3, −0.25} with sy > 0. ZIP's `a-flush-declared-reflex-union-ships-the-wrong-volume` names one.
  - Separately, 16 builds pass the certificate with the right volume and are legal operands, yet fail tier 3′: sqQ1 ∪ at rotation 0.003° with sx = 0, sy ≠ 0 (10 runs) or (−0.25, −0.75) (1 run), and dRight `a ∖ b` at rotation 0.003° with sx = 0, sy < 0 (5 runs).
  - I found no row that records the tier-3′ failures. They are worth a row for the orchestrator.
- **NOTE-3 — the PR's "sound" counts read a weaker oracle.** `join1_r1_probes::outcome` leaves out the certificate and the operand gate, while `join_rc_probes::outcome` includes the certificate. The widened battery checks both and agrees with the PR's counts, so this changes no number.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8. For Q8 I read `boolean/insert.rs` end to end, apart from the test module's older fixtures.

- **S1 (sure, Q2/Q4).** insert.rs:245 still calls the strut order a "convex-sector dot comparison".
- **S2 (sure, Q4/Q6).** `docs/predicate-dimension-audit.md:418` ledgers `bool_strut_order` as the single cosine-difference comparand at insert.rs:197. The new side reading and along reading have no ledger row, and the cited line is stale.
- **S3 (likely, Q1).** One predicate name, `bool_strut_order`, covers three dimensionally different comparands: a sine side (insert.rs:381), an on-line cosine (387) and a cosine difference (398). `chord_join::run_corner_opens` (chord_join.rs:1613-1623), the same sine-then-cosine shape, names its two separately. K telemetry cannot tell them apart.
- **S4 (sure on the facts, likely on the judgement; Q1 and the fresh-instance trap).** The along reading is a near-copy of `sectors::direction_sense` (sectors.rs:932) without its decided-zero refusal. It is the fix minting a second spelling of a rule that already has a home.
- **S5 (likely, Q1/Q4).** `splitting/containment.rs:834-845` already states the doctrine that an angular window "compresses" near its ends, so membership is decided "as DISTANCES, never as an angle". `strut_order`'s cosine within a half-turn is exactly that compression. The rule has a home that this site does not follow.
- **S6 (sure, Q3).** The unit row's germ spacings are all ≥ 45°, and the eight integration poses have axis-aligned germs. Neither row can go red when the order degrades beside a bound; MAJOR-1's row can.
- **S7 (sure, Q1).** `join_rc_probes.rs` is a copy of `join1_r1_probes.rs`'s `area`, `ccw`, `moments`, `clip_convex` and `outcome`, and the reflex pose builder. The two `outcome`s already differ: one checks the certificate, the other does not.
- **S8 (unsure, Q4, sweep).** `recl.rs:762` decides edge-edge membership by a convex-wedge test, which holds only within a half-turn. Its own prose says reflex dihedral wedges are "not yet discriminated", and that only the A/B symmetry check refuses. The PR's sweep dispositions do not list it.
- **S9 (likely, Q5).** The new join.rs:81-83 text says germs past a half-turn are "ordered as a convex one's". That is true off the bounds, and false in MAJOR-1's window.
- **S10 (unsure, Q5).** The ops.rs "Known limitations" rewrite dropped the sentence that the vertex-on-face form of the corner answers exactly. That may be intended.
- **S11 (unsure, Q6).** The PR body's "92,388 lines unchanged" claim rests on a measurement with no guard and no register. The batteries are `#[ignore]` and print-only.
- **S12 (unsure, Q7).** The half-turn classification, then a cosine, then three separate `Zero` branches is more control flow than I would want for "which germ comes first clockwise". Each `Zero` arm is a place a silent pick can hide, as two of them do.

REVIEW COMPLETE
