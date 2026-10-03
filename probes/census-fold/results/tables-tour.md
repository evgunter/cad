
## eps = 1e-09  (99 distinct f64 bodies passing tier 3)

| stratum | bodies | pass 3' (empty) | fail | of which gated/minted WITH contacts elsewhere | UndeclaredContact | CensusUndecidable | InstanceInterference | CensusEscalated | CensusUnsupported | CensusLaneUnsupported |
|---|---|---|---|---|---|---|---|---|---|---|
| op/other, 1 solid | 91 | 91 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| op/other, multi-solid | 8 | 1 | 7 | 6 | 7 | 0 | 1 | 0 | 0 | 0 |
| contact/assembly-subject fixtures | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

Timing (census alone / tier 3; 3' whole / tier 3), median of 3 runs per gate:

| stratum | n | census/t3 median | p90 | max | 3'/t3 median | p90 | max | t3 median s | census median s | census max s |
|---|---|---|---|---|---|---|---|---|---|---|
| all | 99 | 0.10 | 1.04 | 19.3 | 1.16 | 2.15 | 13.2 | 2.2e-04 | 2.1e-05 | 5.0e-03 |
| 1 solid, planar | 20 | 0.46 | 1.04 | 1.2 | 1.57 | 2.15 | 2.3 | 1.5e-04 | 6.3e-05 | 7.2e-04 |
| 1 solid, curved | 71 | 0.08 | 0.24 | 0.6 | 1.10 | 1.36 | 1.9 | 2.9e-04 | 1.3e-05 | 4.1e-04 |
| multi-solid | 8 | 7.99 | 19.33 | 19.3 | 9.08 | 13.23 | 13.2 | 2.1e-04 | 1.8e-03 | 5.0e-03 |
| non-fixture | 99 | 0.10 | 1.04 | 19.3 | 1.16 | 2.15 | 13.2 | 2.2e-04 | 2.1e-05 | 5.0e-03 |

## eps = 1e-06  (101 distinct f64 bodies passing tier 3)

| stratum | bodies | pass 3' (empty) | fail | of which gated/minted WITH contacts elsewhere | UndeclaredContact | CensusUndecidable | InstanceInterference | CensusEscalated | CensusUnsupported | CensusLaneUnsupported |
|---|---|---|---|---|---|---|---|---|---|---|
| op/other, 1 solid | 93 | 93 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| op/other, multi-solid | 8 | 1 | 7 | 6 | 7 | 0 | 1 | 0 | 0 | 0 |
| contact/assembly-subject fixtures | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

Timing (census alone / tier 3; 3' whole / tier 3), median of 3 runs per gate:

| stratum | n | census/t3 median | p90 | max | 3'/t3 median | p90 | max | t3 median s | census median s | census max s |
|---|---|---|---|---|---|---|---|---|---|---|
| all | 101 | 0.10 | 0.82 | 7.7 | 1.13 | 2.03 | 12.5 | 2.5e-04 | 2.1e-05 | 2.1e-03 |
| 1 solid, planar | 20 | 0.29 | 0.86 | 1.1 | 1.43 | 2.03 | 2.2 | 2.3e-04 | 6.0e-05 | 7.2e-04 |
| 1 solid, curved | 73 | 0.05 | 0.21 | 0.6 | 1.06 | 1.40 | 1.8 | 2.9e-04 | 1.3e-05 | 3.2e-04 |
| multi-solid | 8 | 5.68 | 7.69 | 7.7 | 7.31 | 12.46 | 12.5 | 2.4e-04 | 1.5e-03 | 2.1e-03 |
| non-fixture | 101 | 0.10 | 0.82 | 7.7 | 1.13 | 2.03 | 12.5 | 2.5e-04 | 2.1e-05 | 2.1e-03 |

## eps = 1e-12  (99 distinct f64 bodies passing tier 3)

| stratum | bodies | pass 3' (empty) | fail | of which gated/minted WITH contacts elsewhere | UndeclaredContact | CensusUndecidable | InstanceInterference | CensusEscalated | CensusUnsupported | CensusLaneUnsupported |
|---|---|---|---|---|---|---|---|---|---|---|
| op/other, 1 solid | 91 | 91 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| op/other, multi-solid | 8 | 1 | 7 | 6 | 7 | 0 | 1 | 0 | 0 | 0 |
| contact/assembly-subject fixtures | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

Timing (census alone / tier 3; 3' whole / tier 3), median of 3 runs per gate:

| stratum | n | census/t3 median | p90 | max | 3'/t3 median | p90 | max | t3 median s | census median s | census max s |
|---|---|---|---|---|---|---|---|---|---|---|
| all | 99 | 0.10 | 1.01 | 16.2 | 1.22 | 2.10 | 18.7 | 2.4e-04 | 2.1e-05 | 4.0e-03 |
| 1 solid, planar | 20 | 0.47 | 1.01 | 1.2 | 1.63 | 2.10 | 2.6 | 1.8e-04 | 5.7e-05 | 7.0e-04 |
| 1 solid, curved | 71 | 0.06 | 0.22 | 0.6 | 1.11 | 1.54 | 2.0 | 2.6e-04 | 1.5e-05 | 3.8e-04 |
| multi-solid | 8 | 12.99 | 16.16 | 16.2 | 13.85 | 18.73 | 18.7 | 2.3e-04 | 3.4e-03 | 4.0e-03 |
| non-fixture | 99 | 0.10 | 1.01 | 16.2 | 1.22 | 2.10 | 18.7 | 2.4e-04 | 2.1e-05 | 4.0e-03 |

## Verdict changes across eps (same fingerprint)

bodies present at every eps: 99
pass/fail flips: 0; variant-set changes: 0
