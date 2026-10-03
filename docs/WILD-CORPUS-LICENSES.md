# Wild corpus — license audit and montage clearance

**Purpose.** The 13 STEP files under `crates/step-import/tests/fixtures/wild/`
were license-verified when imported (M7-4, PR #193; dispositions recorded in
`crates/step-import/NOTICE` and in each file's provenance comment). This
document (a) consolidates that record, (b) re-verifies every upstream by
retrieval, and (c) answers a question M7-4 did not ask: **may the kernel's own
tessellated render of these bodies be committed to this repo as a montage PNG?**
A render is a derived work redistributed under this repo's MIT OR Apache-2.0 —
a materially different posture from a test fixture, which is functional input.

**This is not legal advice.** It is a conservative engineering sanity audit by an
agent, not a lawyer. Anything reading NO or UNCLEAR is flagged rather than
resolved. Where a judgment call was needed, the audit takes the restrictive side.

**Audit date:** 2026-08-09. Upstream retrievals performed 2026-08-08/09; every
license claim below was re-fetched from upstream on those dates, not taken from
the M7-4 record. Findings where the two disagree are in **Deltas**, at the end.

## Per-file table

| file | vein | upstream (retrieval-verified) | license | attribution required | render-in-repo OK? | notes |
|---|---|---|---|---|---|---|
| `adafruit/1982_MPR121.step` | adafruit | `adafruit/Adafruit_CAD_Parts` → `1982 MPR121/1982 MPR121.step` | MIT | **yes** | **YES** | imports; renamed locally (D3) |
| `adafruit/328_2500mAh_battery.step` | adafruit | ↑ `328 2500mAh battery/328 2500mAh battery.step` | MIT | **yes** | **YES** | imports |
| `adafruit/64_Halfsize_Breadboard.step` | adafruit | ↑ `64 Halfsize Breadboard/…step` | MIT | **yes** | **YES** | imports |
| `adafruit/805_slide_switch.step` | adafruit | ↑ `805 slide switch/805 slide switch.step` | MIT | **yes** | **YES** | imports |
| `adafruit/931_OLED_128x32_I2C.step` | adafruit | ↑ `931 OLED 128x32 I2C/…step` | MIT | **yes** | **YES** | imports |
| `nist/nist_ftc_09_asme1_rd.stp` | nist | NIST MBE PMI Validation & Conformance Testing test cases | US Gov work, **PD-equivalent** | courtesy only | **YES** | imports; no-endorsement line required |
| `nist/nist_ftc_11_asme1_rb.stp` | nist | ↑ same project | US Gov work, **PD-equivalent** | courtesy only | **YES** | imports since the M7-5 band-seam re-mint (#252); no-endorsement line required |
| `occ-oss/cq_red_cube_blue_cylinder.step` | occ-oss | `CadQuery/cadquery` → `tests/testdata/red_cube_blue_cylinder.step` | Apache-2.0 | **yes** | **YES** | imports since #252; Onshape header (D4) |
| `occ-oss/b123d_nema17_bracket.step` | occ-oss | `gumyr/build123d` → `docs/topology_selection/examples/nema-17-bracket.step` | Apache-2.0 | **yes, incl. upstream NOTICE** | **YES** (moot) | refuses today; **see D1** |
| `stepcode/sg1-c5-214.stp` | stepcode | `stepcode/stepcode` → `data/ap214e3/sg1-c5-214.stp` | BSD-3-Clause *as redistributed* — origin unclear | **yes** | **EXCLUDED — UNCLEAR (D2)** | imports; the one import-class exclusion |
| `stepcode/dm1-id-214.stp` | stepcode | ↑ `data/ap214e3/dm1-id-214.stp` | ↑ same | **yes** | **EXCLUDED — UNCLEAR (D2)** | refuses (`B_SPLINE_SURFACE`) |
| `stepcode/io1-cm-214.stp` | stepcode | ↑ `data/ap214e3/io1-cm-214.stp` | ↑ same | **yes** | **EXCLUDED — UNCLEAR (D2)** | refuses (`\X2\`) |
| `stepcode/TAIL_TURBINE.stp` | stepcode | ↑ `data/ap214e3/`**`s1-c5-214/`**`TAIL_TURBINE.stp` | ↑ same | **yes** | **EXCLUDED — UNCLEAR (D2)** | refuses; path misrecorded (D5) |

Source URLs: `github.com/adafruit/Adafruit_CAD_Parts` · `github.com/CadQuery/cadquery`
· `github.com/gumyr/build123d` · `github.com/stepcode/stepcode` · NIST:
`nist.gov/ctl/smart-connected-systems-division/smart-connected-manufacturing-systems-group/mbe-pmi-0`

**Verdict distribution: 9 render-OK · 7 of those require attribution · 4
EXCLUDED (all stepcode).** Of the 9 cleared, 8 import today (the 5 Adafruit
files, both NIST files and the CadQuery cube — the last two joined at the M7-5
band-seam re-mint, #252); only `b123d_nema17_bracket` still refuses, so **the
montage is exactly those 8** — `demos/wild`'s pinned cell set — and it clears
cleanly. `sg1-c5-214.stp` is the only file this audit removes from a montage the
kernel could otherwise produce.

## What each license says about a committed render

- **MIT (Adafruit).** Grants use/modify/distribute without restriction, on the
  single condition that the copyright and permission notice ride "all copies or
  substantial portions". A tessellated PNG is a derived work, so the notice must
  appear beside the montage. Verified today at
  `raw.githubusercontent.com/adafruit/Adafruit_CAD_Parts/main/LICENSE`: "MIT
  License / Copyright (c) 2016 Adafruit Industries". No hardware/CC-BY-SA split
  in the README, and GitHub metadata reports `MIT` for the whole repo.
- **NIST (public-domain-equivalent).** The project page states verbatim: "The
  test cases, CAD models, and STEP files can be used without any restrictions."
  A US Government work is not subject to domestic copyright, so no permission is
  needed and no attribution is *required*; NIST *requests* acknowledgement and
  forbids use of its name/logo to imply endorsement. Render permitted; carry the
  acknowledgement and the no-endorsement sentence.
- **Apache-2.0 (CadQuery, build123d).** §4 permits redistribution of Derivative
  Works provided the recipient gets the License, modified files are marked,
  attribution notices are retained, and — §4(d) — **any NOTICE file's contents
  are reproduced**. A PNG carries none of this itself, so the montage README
  must. build123d ships a NOTICE (see D1); CadQuery does not (root listing
  confirmed today — no NOTICE file). Compatible with this repo's dual license.
- **BSD-3-Clause (STEPcode).** Clause 2 requires redistribution "in binary form"
  to reproduce the copyright notice, conditions and disclaimer in the
  documentation or other materials — a PNG is exactly that case, so the full
  notice would have to ride the montage. Clause 3 forbids endorsement claims.
  None of that is the blocker; **D2 is.**

## Attribution block — paste into the montage README

> **Third-party source geometry.** The bodies in this montage were imported from
> STEP files authored by others and tessellated by this project's own kernel; the
> rendered images are our derived work, the underlying models are not.
>
> **Adafruit parts** (`1982 MPR121`, `328 2500mAh battery`, `64 Halfsize
> Breadboard`, `805 slide switch`, `931 OLED 128x32 I2C`) from
> <https://github.com/adafruit/Adafruit_CAD_Parts>, used under the MIT License:
> *Copyright (c) 2016 Adafruit Industries. Permission is hereby granted, free of
> charge, to any person obtaining a copy of this software and associated
> documentation files (the "Software"), to deal in the Software without
> restriction… THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND.*
> Full text: `crates/step-import/tests/fixtures/wild/adafruit/LICENSE-adafruit.txt`.
>
> **NIST model** (`nist_ftc_09_asme1_rd`) from the National Institute of
> Standards and Technology's MBE PMI Validation and Conformance Testing project.
> Produced by an agency of the U.S. Government and not subject to copyright in
> the United States. Acknowledgement is given at NIST's request. *Neither NIST
> nor the U.S. Government endorses, recommends, or has any connection with this
> software; no NIST name or logo is used to imply endorsement.*

*(The montage has since added `nist_ftc_11` and the CadQuery cube (#252), and the
shipped block in `demos/README.md` carries both — the second NIST file under the
NIST terms above, and the CadQuery entry with its source URL, "used under the
Apache License, Version 2.0" linking the committed license text, and the
statement that the geometry was modified only by our own tessellation. If
`b123d_nema17_bracket` ever imports, extend it the same way, plus the NOTICE text
from D1.)*

## Deltas — where this audit differs from the M7-4 record

**D1 — FINDING (DISCHARGED): build123d ships a NOTICE file, and the M7-4 record
did not reproduce it.** That record had `crates/step-import/NOTICE` calling
build123d plain Apache-2.0 and stopping there. Upstream `gumyr/build123d` has a
root `NOTICE` (625 bytes, confirmed present today), and Apache-2.0 §4(d) makes
reproducing it a *condition* of redistributing the work or a derivative. Its
text: *"Copyright (c) 2022–2025 The build123d Contributors. Licensed under the
Apache License, Version 2.0…"* plus an acknowledgement that build123d originated
from CadQuery code. The obligation is the committed fixture's — it does not wait
for the montage — and it is met: `crates/step-import/NOTICE`'s `occ-oss/` section
reproduces that text. It was the one live compliance gap the audit found.

**D2 — FINDING (upholds and sharpens M7-4's own flag): the STEPcode data files'
license is UNCLEAR, and the audit excludes all four from any montage.** M7-4
flagged this for Ev and proceeded on the reading that "the repository license
governs its committed assets". Independent checking makes that reading thinner,
not thicker: STEPcode's `COPYING` is BSD-3 for STEPcode, and its `INTENT.md` —
which the license itself points at for scope — addresses only *source code and
documentation*, saying nothing about `data/` or test models. Meanwhile
`TAIL_TURBINE.stp`'s own header names CATIA V5R19 and the path
`E:\Public\Archive_PDES\TR22\NativeFiles\s1` — a PDES/CAx-IF interoperability
round file, third-party content STEPcode redistributes rather than authors. A
redistributor's license does not necessarily convey rights it never held. As a
*test fixture* this is defensible (functional input, upstream's long-standing
redistribution). As *committed repo artwork* it is a distribution the audit will
not clear. **EXCLUDED-FROM-MONTAGE: `sg1-c5-214.stp`, `dm1-id-214.stp`,
`io1-cm-214.stp`, `TAIL_TURBINE.stp`.** Only `sg1-c5-214.stp` costs anything —
the other three refuse import regardless. Ev can overrule this; it should be
an explicit decision, not a default.

**D3 — the Adafruit fixtures were renamed and the provenance comments record the
local name, not the upstream one.** Upstream uses spaces and per-part folders
(`1982 MPR121/1982 MPR121.step`); the fixtures use underscores. All five files
were located and confirmed present upstream today. No license consequence, but
the `file:` line in each provenance comment is not a retrieval key — the table
above supplies the real upstream paths. Note that `1982 MPR121 Stemma
Breakout/1982 MPR121Q QT.step` is a *different* part; the fixture is `1982 MPR121/`.

**D4 — CadQuery's Onshape lineage is confirmed but unresolved (as M7-4 said).**
`tests/testdata/red_cube_blue_cylinder.step` is present upstream today and
CadQuery's Apache-2.0 covers the repo with no NOTICE file to propagate. The
file's Onshape translation-service header remains the same thin spot M7-4
recorded — output of a translation service, committed by maintainers as test
data. The audit does not upgrade this to UNCLEAR: unlike D2, the content is a
trivial red cube and blue cylinder authored for the test, not a third-party
production model. Render cleared, with Apache-2.0 attribution.

**D5 — two upstream paths in the provenance comments are imprecise.**
`TAIL_TURBINE.stp` is at `data/ap214e3/s1-c5-214/TAIL_TURBINE.stp`, not
`data/ap214e3/` (it is absent from that directory's listing, so the recorded URL
does not resolve to it). `b123d_nema17_bracket.step` is at
`docs/topology_selection/examples/nema-17-bracket.step`, not `docs/`. Both
verified present today. Citation precision only — no license consequence.

**No disagreement found on:** the Adafruit MIT claim, the NIST public-domain
claim (the "without any restrictions" sentence is quoted verbatim on the live
page), the CadQuery Apache-2.0 claim, the build123d Apache-2.0 claim, or the
STEPcode BSD-3 claim *as a description of STEPcode's own license*. No upstream
has vanished; all 13 files' sources resolve today.

**Post-audit note (2026-08-09, the STEP-bank hunt's flag, resolved):**
GitHub's API reports CadQuery's repo license as NOASSERTION. Verified
against the LICENSE file directly: it states Apache-2.0 in prose with
a custom preamble ("free software … under the terms of the Apache
Public License, v 2.0"), which GitHub's automated classifier cannot
parse but which is an unambiguous grant. The audit's Apache-2.0
verdict for `cq_red_cube_blue_cylinder.step` stands. Recorded so the
next audit does not re-flag the classifier artifact.
