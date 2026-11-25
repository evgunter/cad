---
id: the-pcurve-door-is-two-functions-and-a-dual-holds-no-proposal
kind: unit
title: The pcurve door is two functions (derive, certify); a dual derives determined images, holds no certificate and no proposal, and never reaches the incidence read
status: spec
opened: 2026-11-25
priority: P1
cost: H
parent: from-computed-claims-a-zero-tangent-at-a-dual
---


The rule is `crates/geom-core/README.md` DL5.3 as ruled by Ev on `[ev]` PR 7719 (2026-11-25), with `docs/DUAL-DESIGN.md` DL3's pcurve clause and `crates/geom-brep/README.md`'s edge-door paragraph, which that PR re-worded. It supersedes `spline-incidence-is-reached-from-incidence-readers-open-to-a-dual`, whose implementer stopped at the question that PR answers; that file's "Measured, 2026-11-24" section is this unit's starting measurement (who calls the incidence read, at which scalars, what consumes the verdict, the five dual rows that pin today's behaviour). It is the same work as `work/certread/the-certification-files-hold-body-data-beside-certification` items (a) and (c), seen from the dual's side: do them together and say on that row what is left of it.

## What must be true

1. **A pcurve door is two functions, as the edge door is (PR 5689).** Derive (choose the image's class, build the image) is construction, generic over every deciding scalar. Certify (checks 1–4, the envelope, the certificate object, and the rule that ends a door's refusal by the incidence read) is declared over `Certifies`. `PcurveCache::certify` builds a `PcurveCertificate<Dual64>` on the closed-form arms today; afterwards no dual holds a pcurve certificate, as none holds an edge certificate (`Real::Carries<…> = Uncertified`).
2. **The incidence read is `Certifies` end to end**, on spline and analytic charts: `incident_as`, `point_incident`, `read_edge_incidence`, `read_incidence`, `incidence_ended*`, the certify doors, `validate_pcurves`. No dual path reaches it, on its value view either: its verdict only refuses or certifies, and the base evaluation has made that finding (DL3). `spline_incidence.rs` and the `incidence` module are listed in `scripts/gates/certification-interiors.sh` as they are (the interior's `Decide<Value = T>` bound becomes `Certifies` by name). The analytic arms stop forming a rounding allowance in dual arithmetic.
3. **The `Pcurve` variants that hold a proposal have no values at a dual** (`General`, `Fitted`; a proposal is `f64` by type until the door of its check, DL5.2), and the door that admits one is `Certifies`. A face whose edge has only a proposal image holds no row for it at a dual; nothing is refused, excused or flagged at run time.
4. **The run-time route goes:** `FittedLaneUnsupported` at a dual, `AtRestPolicy::fitted_lane() -> Option` (and `offset_fit_lane`, `derive_chart_foot`'s `Ok(None)`: DL5.3's third bullet, "never an `Option` a dual answers `None` to"), and `not_owed`.
5. **Kept through the change:** a dual face keeps a determined row it already held when the row cannot be re-derived (`carry_rows`, reached today only through `not_owed`: a sub-edge left by a split of an iso edge is the case). Derive's return has to say "not derived here, keep what is held" before `not_owed` retires.

## Known consequences

- Five dual rows pin today's behaviour (named in the superseded file's measurement): the two `at_a_dual_an_off_carrier_*` rows cease to be expressible or become rows that the base build refuses; `spline_incidence::rows::at_a_dual_a_point_reads_…` becomes a compile-fail pin; the diagonal row's "no row stored" stays; `not_owed_rows` goes.
- `carry_rows` and the edit-time transport (`euler*.rs`, `mirror.rs`, `null.rs`) see fewer rows at a dual than at the base; they tolerate a half-edge with no row today (by their docs; not checked per file).
- A dual can certify the conic and line arms of a projected image today (`run_projected_checks<T: Decide>` reads no lane there; by inspection); the split closes that.

## Not this unit

`a-certificates-verdict-selects-what-is-built-at-three-sites` (the arc-rim fallback, `row_certifies`, the measured step): until it lands, a dual and its base can differ in one row at those sites; say in the PR where. `a-value-view-door-may-return-a-number-and-spline-walls-feet-do`. Moving the fit itself behind its door (`the-fitted-chart-image-is-c6s-fit-lift-and-a-pcurve-holds-it-at-t`): this unit makes the proposal variants uninhabited at a dual and must leave that one a clean seam. The dual trim reader (`work/germ/a-duals-trim-reader-on-a-spline-face-with-a-proposal-edge`).

## Unmeasured

Whether the dual corpus (`editor-core` `m10_di_dual_corpus`, `loft_prism` at `Dual64` and `DualInterval`) reaches the incidence read at run time (two probes timed out in the build, at 2 h and 4 h). Whether any dual measurement reads a pcurve certificate field (`samples`, `max_residual`, `envelope`) as data. Whether any verb that runs at a dual (the `verbs` crate, face merges, chord joins were not swept) leaves an edge on a spline wall with only a proposal image.

## Dispatch, 2026-11-25

Letter **H**; tier **DUAL**: a door every pcurve row is built and certified through is split, a set of types loses its values at one scalar, and a run-time route four crates read is retired. Branch `scalar/the-pcurve-door-is-two-functions`, lane `scalar-certifies`.

**Held, 2026-11-25:** not dispatched. Ev asked in chat, as the implementer draw was taken, that nothing new be kicked off. The branch exists with no work on it; the letter, tier and draw stand for when it goes.
