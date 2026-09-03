---
id: pcurve-claim-ordinal-202
kind: issue
title: PCURVE claim - ordinal 202 (P-2 dual)
status: closed
opened: 2026-08-29
closed: 2026-09-03
github: 1209
refs: [1177, 1112, 1115, 1118, 1119, 1095]
---

## From GitHub issue 1209

opened 2026-08-29, 0 comments.

Claim-at-dispatch. Ordinal 202 = PCURVE P-2's v6 dual (#1177, frozen head `0ecd3f7e`). PCURVE band 200–299; 200 was Census G2, 201 was P-1b.

Arms are not named here and are not on main — recorded in the orchestrator's reviewer-fenced ledger per the blinding-leak rule (#1112, #1115) and Ev's ruling that verifiable precommitment is optional.

**This unit consumes block PCURVE-1's last slot.** Once these reviews conclude the block is closed and its record may merge to main.

**Contamination flag, same as #1140's**: the block draw stood on main until its redaction (#1118, #1119), so this slot's arm was derivable by arithmetic during that window. Reviewers were not dispatched in it, but git history retains the text. Recorded as a fact on the row rather than used as an exclusion.

Docs-only; merging immediately per #1095.

## Home

An A/B ordinal claim whose purpose is spent — the record of record is `docs/MODEL-AB-LOG.md`. The PCURVE program is closed, so the migrated bookkeeping file lands under `work/issues/`.
