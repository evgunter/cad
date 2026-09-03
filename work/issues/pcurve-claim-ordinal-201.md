---
id: pcurve-claim-ordinal-201
kind: issue
title: PCURVE claim - ordinal 201 (P-1b dual)
status: closed
opened: 2026-08-28
closed: 2026-09-03
github: 1140
refs: [1107, 1112, 1115, 1118, 1119, 1095]
---

## From GitHub issue 1140

Opened 2026-08-28; 0 comments.

Claim-at-dispatch. Ordinal 201 = PCURVE P-1b's v6 dual (#1107, frozen head `0422043a`). PCURVE band 200–299; 200 was Census G2.

Arms are not named here and are not on main — recorded in the orchestrator's reviewer-fenced ledger per the blinding-leak rule (#1112, #1115) and Ev's ruling that verifiable precommitment is optional. The block record merges to main only once PCURVE-1's last slot's reviews conclude.

**This pair carries a contamination flag.** PCURVE-1's block draw stood on main in a form that determined this unit's implementer arm by arithmetic, from P-1a's spec until its redaction on 2026-08-28 (#1118, #1119). P-1b's reviewers were not dispatched during that window, but git history retains the redacted text. Ev ruled the pair COUNTS; the exposure is recorded on the row as a fact rather than used as an exclusion.

Docs-only; merging immediately per #1095.

## Home

An A/B ordinal claim whose purpose is spent — the record of record is `docs/MODEL-AB-LOG.md`, which owns the row and its contamination note. The PCURVE program is closed, so the migrated bookkeeping file lands under `work/issues/`.
