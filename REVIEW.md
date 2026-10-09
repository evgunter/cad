IN PROGRESS

# Review of PR 4415 (frozen head 24aa90e7)

Interim notes (containers restart):
- claim 7: ported files byte-identical to 4417's merge 14f0cdf1; trial merge of origin/main clean, ported files then diff 0 vs main.
- claim 6 census (eps 1e-9, tilts ±2e-8,±5e-9, 11 520 runs): 49 moved, all ResultInvalid{ShellRoleUndecided} -> Escalated{ShellRole}; 0 other.
- claim 3: exact-rational re-derivation of all 49 matches the certified enclosures to <=6e-10 rel; witness d=1e-8 V/A=3.28871625245e-9 inside the enclosure.
