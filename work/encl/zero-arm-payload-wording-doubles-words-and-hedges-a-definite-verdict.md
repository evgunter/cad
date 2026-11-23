---
id: zero-arm-payload-wording-doubles-words-and-hedges-a-definite-verdict
kind: issue
title: geom-brep: the Zero-arm payload idiom repeats a word within a sentence, hedges a definite Negative verdict, and prints '0e0 m'
status: closed
pr: 5157
opened: 2026-10-13
priority: P4
cost: E
branch: encl/zero-arm-wording
closed: 2026-10-19
---

(Filed by the ENCL orchestrator from PR 4976's delta review, style findings S1–S5.)

## What

PR 4976 gave every ENCL Zero-arm payload one idiom: "is not shown … (margin {:e} m)". Read in place, it has these rough edges (`crates/geom-brep/src/certify.rs`, `CertifyError`'s Display; `offset_meters.rs`, `MeterError`'s Display):

- **S1, "cross" twice:** NotTransverse reads "the faces are not shown to cross (…), where the edge's description says they cross".
- **S2, "not shown" twice:** NotSecondOrderSeparated reads "not shown to curve apart … so they are not shown to fix where the edge runs".
- **S3, hedging a definite verdict:** NotTransverse{Negative} at rest says "not shown to cross (margin -5e-9 m)", while its ending says "There is no way through: this is a kernel defect or a damaged file". The hedge is true, but weaker than the verdict it sits beside. D4 ¶1 (iv) asks a definite arm to tell its sibling's story, and an exact payload word may serve that better.
- **S4, "margin" is kernel vocabulary:** on IntervalNotForward the number is the span, and the door's next sentence calls it "This length".
- **S5, exponent form for round numbers:** "speed 2e0 m" and "(margin 0e0 m)" read poorly.

Each is a reading of a rendered string. Settle them together, with whole-string pins, so the idiom stays one idiom.

## Ruled (ENCL orchestrator, 2026-10-14)

Fix S1, S2, S4 and S5 as plain wording. Keep one idiom across every Zero-arm payload:
- **S1, S2:** no word repeats within a sentence.
- **S4:** IntervalNotForward's number is called what the door calls it, "length", not "margin".
- **S5:** round numbers print readably. Keep exponent form for small or large magnitudes, but not "2e0 m" or "0e0 m".

S3 (a definite Negative payload hedged as "not shown"): the payload prints what was read. A Negative verdict was decided, so its payload says so, for example "the faces cross the wrong way (margin -5e-9 m)", while Zero arms keep "not shown". Payloads never claim no size.

Pin every touched payload whole. Keep every render within the editor-core `refusal_concision*` budgets.

Out of scope: the endings, and the open `[ev]` PR 5134.
