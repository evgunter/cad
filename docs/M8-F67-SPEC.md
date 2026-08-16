# M8-F67 — the #214 F6/F7 typed-margin fold-in

Orchestrator work order for the M8 slate's last pre-walk item (the
"#214 F6/F7 riders"). Substrate: fresh exploration 2026-08-15; the
audit is docs/predicate-dimension-audit.md, the standing rules are
issue #214's body and geom-core/src/k_stats.rs:105–131. M8-2's PR
#306 deferred F7 explicitly ("F7 note"); this unit takes it.

## The defect, verified

F7 and the two F6 `pcurve_interval_forward` sites are the SAME
quantity handled three inconsistent ways:

- geom-brep/src/certify.rs:1154 (`nurbs_span_meter`): gates the
  BARE RATE `speed_lower_bound()` (m/param) through
  `decide_flagged(..., "F7")` against the metre band.
- geom-brep/src/pcurve_cache.rs:1851 (`param_rate`): returns
  `T::one()` for `Curve3::Nurbs` — silently declines to use the
  bound at all; feeds `pcurve_interval_forward` at :2331 (fitted)
  and :2817 (iso), the census's F6 sites.
- topo/src/split.rs:193 does it right (`Margin::metered`) — the
  honest precedent, audited OK.

Plus pcurve_cache.rs:2350 (`pcurve_azimuth_period`): rad × arm
where the arm is 1 for exactly one surface kind (CONE — torus got
its real arm in M6-3; plane/nurbs never reach the gate).

## Required shape (binding)

1. **F7**: certify.rs:1154's gate becomes a LENGTH —
   `Margin::metered(domain_len, meter)` (carrier knot-domain
   length × the certified speed lower bound), collapsed-arm idiom
   per enters.rs:84–93/:141–150: non-positive or poison meter →
   `MarginDiag::Invalid` (escalate), backwards span stays
   `IntervalNotForward` — the two failure modes stay distinct.
   The downstream `Margin::metered(span, meter)` at :1167 is
   already correct; leave it.
2. **F6 (both interval_forward sites)**: `param_rate` answers
   `speed_lower_bound()` for `Curve3::Nurbs`. The helper is
   currently always-finite and gains a poison arm — the fitted
   and iso lanes get the collapsed-arm gate too, not a bare
   substitution. Templates in-tree: pcurve_cache.rs:2498
   (ARC-RIM) and :1919 (harmonic).
3. **F6 (azimuth_period, cone)**: `(τ − Δu) · azimuth_lever
   (surface, v_sup)` with `v_sup` from `boxed` + `window`
   (templates :1926–1953, chart_arms_at:2114–2121). VERIFY the
   boxed-derived `v_sup` dominates per chart_arms_at's stated safe
   direction; if it does not, STOP and report — do not invent a
   different bound.
4. **Ledger + census move together**: geom-core/tests/
   flagged_census.rs `LEDGER_FLAGGED_SITES` 12 → 8; the audit
   doc's census paragraph and the four retired rows updated. Truth
   pass on the audit rows this unit reads: the
   `pcurve_trim_containment | mixed | FLAG F6` row is FALSE today
   (the site is a metered `decide` since M6-3) — correct it; fix
   the stale line numbers of touched rows only.
5. **k-lint / K stream**: this is the FIRST F-retirement that is
   NOT K-stream-neutral — `nurbs_span_meter`'s recorded margin
   changes from a rate to a metre (today it is linted against the
   4.0e-5 metre floor in the wrong units; `EPS_COUPLED_PREDICATES`
   does not list it). Expect a K baseline delta: if the k-lint
   gate fires, do NOT change geometry — re-derive per the K-REPORT
   runbook, with the dimension change named as the cause; the PR
   body states the delta.

## Excluded, with named follow-ups (file at PR time, signed)

- topo/src/pcurves.rs `azimuth_arm`'s `_ => T::one()` (:644) and
  the `v_meter` `T::one()` fallbacks (:1027, :1273): doc-flagged
  only, cross-crate to fix honestly (needs geom-brep's
  `nurbs_stretch_bounds` from topo), and the region is under
  active edit by M8-4. File the follow-up; narrow the audit row
  so the Plane case (u/v already metres — 1 is exactly right) is
  recorded OK rather than flagged.
- Everything in M8-4's active regions (nurbs_iso_derive and the
  IsoUnsupported doc block). M8-4 is IN FLIGHT on
  kernel/m8-4-iso-intersection touching pcurve_cache.rs (~:632)
  and pcurves.rs (~:407, :589+): merge origin/main and BUILD THE
  UNION every time main moves; the F6 sites are ~1700 lines from
  its hunks, so conflicts should be textual-trivial — if a real
  semantic collision appears, STOP and report.

## Acceptance

1. **Scale twins per the rim_dim_scale_twins.rs model** (probe
   feature): the margin IS the named length; mm-vs-metre twin
   ratio exactly 1000 (a wrong-dimension comparand answers 1e6);
   plus a REPARAMETRIZED twin (same locus, t → 2t) answered
   identically by the fixed comparand and differently by the bare
   rate — that pair is what proves the fold-in, not just a rename.
2. **Pre-assigned adversarial attack** (the F3+F4 review's MAJOR
   was a weakening direction): construct a body whose fine feature
   the new metering brings IN-BAND and demonstrate the gate still
   refuses / decides it correctly; the attack row ships red-then-
   green against a deliberately-weakened mutant.
3. **Tightening honesty**: `param_rate` 1 → a bound typically ≪ 1
   may newly refuse short spans. Run the corpus + tour: EVERY
   newly-refusing row is reported three-outcome ε-honest (its
   posture asserted, never widened away); if
   `freecad.rs::CORPUS_EPS_CEILING` must move, re-derive it
   honestly per the F5 precedent and say so.
4. Positive-statement pins stay green or their movement is
   explained row-by-row (sweep/tests/m7_skin_integral.rs,
   m8_14_long_turn_sweep.rs, topo/tests/m5_pr7_split_meter.rs,
   the fitted/iso at-rest suites).
5. Census suite green at 8; hosted CI fully green (the only
   gate); any re-blessing through the hosted pipeline only.

## Process

Full unit protocol: implementer = block M8-15 slot 0 (drawn
2026-08-15: byte 186, fable position 2 → slot 0 OPUS); difficulty
M and task-class NUMERIC logged pre-draw (this spec + M8-LOG); one
blinded adversarial reviewer + fix pass; A/B row at merge with
per-phase tokens/wall-clock; review ordinal claimed from the
ledger ON MAIN at review-dispatch time and pushed to main
immediately. No Co-Authored-By in lane commits; comments state
invariants, not history.
