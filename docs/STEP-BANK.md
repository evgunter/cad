# STEP bank — licensed wild-corpus candidates

**Purpose.** A staging bank of suitably-licensed wild STEP files, gathered ahead
of demand so that future units can adopt fixtures without a fresh hunt. The
files live **outside this repo**, at `~/.local/share/cad-work/step-bank/<vein>/`.
This register is the index: what was found, where it came from, what the license
says **about the data**, and which frontier class each file exercises.

**This is not legal advice.** It is a conservative engineering sanity audit by an
agent, not a lawyer. Where a judgment call was needed the audit takes the
restrictive side; anything reading UNCLEAR is flagged rather than resolved.

**Hunt date:** 2026-08-09. Every license claim below was fetched from upstream on
that date. The standard applied is `docs/WILD-CORPUS-LICENSES.md`, in particular
its **D2 lesson: a repository's source license does not automatically cover the
files in its `data/` directory.** Verdicts below therefore say what covers the
*geometry*, not just what covers the repo.

**Bank files are NOT repo fixtures.** Nothing here is committed to
`crates/step-import/tests/fixtures/wild/` until a unit adopts it, and adoption
carries its own license re-verification at that time (upstream can change, and
the fixture posture — committed, redistributed — is stricter than the bank's).

**Frontier classes.** C1 assembly instancing · C2 genuinely-freeform NURBS /
trimmed splines · C3 cone/sphere/torus (future recognition kinds) · C4
fillet-heavy (TangentIntersection pressure) · C5 dialect breadth · C6 scale ·
C7 AP203/AP242 vs the current AP214 lean.

**Census note.** Counts are `grep` entity tallies, not parses. `FACE_SURF` counts
`FACE_SURFACE` (the pre-`ADVANCED_FACE` AP203 dialect); `BSPL_S` counts
`B_SPLINE_SURFACE*` with the rational subcount in parentheses; `NAUO/PD` is
`NEXT_ASSEMBLY_USAGE_OCCURRENCE` over `PRODUCT_DEFINITION` — **NAUO ≫ PD is the
instancing signal** (many placements of few breps).

## Bank register — 29 candidates, 7 veins, 203 MB

*(The `nist-pmi/` vein directory also retains the other 22 STEP files from the same
public-domain archive, plus `NIST-PMI-STEP-Files.zip` itself — 32 STEP files on disk, 10
registered below. They are all under one PD grant; the table lists only the picks.)*

| file (under bank root) | vein / upstream | license — data-specific verdict | sha256[0:12] | schema · originating system | census highlights | class | pri |
|---|---|---|---|---|---|---|---|
| `adafruit/4242_PyGamer.step` | adafruit · `github.com/adafruit/Adafruit_CAD_Parts` → `4242 Adafruit PyGamer/` | **MIT.** Repo-wide MIT, GitHub metadata agrees, no hardware/CC split in README; the repo *is* a CAD-parts repo, so the STEP files are its subject matter, not incidental `data/`. Attribution required. Precedent: 5 Adafruit files already adopted (M7-4). | `f0caf940c27a` | AP214 (`AUTOMOTIVE_DESIGN`) · Autodesk Translation Framework v12 / ST-DEVELOPER v18 | 4622 AF · **NAUO 86 / PD 36** · 102 MSB · 883 cyl · 21 sph · 8 bspl-surf | C1 C6 | **H** — cleanest instancing case at moderate size (86 placements of 36 parts); MIT, precedent-set vein |
| `adafruit/5420_MEMENTO.step` | ↑ `5420 MEMENTO/` | ↑ MIT | `e7e1d1c1ec18` | AP214 · ATF v12.14 | 5712 AF · **NAUO 115 / PD 32** · 153 MSB · 18 bspl-surf (6 rat) · 11 sph · 13 tor | C1 C6 | **H** — highest instancing ratio in the bank (3.6 placements per product) |
| `adafruit/6425_Fruit_Jam_Case.step` | ↑ `6425 Fruit Jam Case/` | ↑ MIT | `37bb62e1ec8f` | AP214 · ATF v14.10 (2025 export) | 8451 AF · **NAUO 136 / PD 42** · 237 MSB · **134 bspl-surf (21 rat)** · 1762 cyl · 91 cone · 30 tor | C1 C2 C6 | **H** — instancing *and* a moulded enclosure's splines; newest exporter version in the bank |
| `adafruit/2277_RGB_Matrix.step` | ↑ `2277 RGB Matrix 5mm/` | ↑ MIT | `99d2f14e90a7` | AP214 · ATF v12.20 | **57 MB** · **36 704 AF** · 34 306 plane · **2048 tor** · 56 MSB · NAUO 8 / PD 8 | C6 C4 | **H** — 216× the largest current fixture; the latency benchmark. Analytic-only, so it should import today |
| `framework/FW13_full_laptop.stp` | framework · `github.com/FrameworkComputer/Mainboard` → `Framework Laptop 13 CAD.stp` | **CC BY 4.0.** README: "Framework Laptop 13 © 2026 by Framework Computer Inc is licensed under CC BY 4.0", and the 3D CAD section says "you are free to modify, remix, and redistribute them". Rights-holder publishing its own product CAD — the license is *about* the data. Attribution required. Caveat: sub-assemblies were drawn by ODM partners (author fields read `asus`, `Alfie`, `Sakura`), so Framework licenses what Framework holds. | `19778e9a7c8f` | AP203 (`CONFIG_CONTROL_DESIGN`) · **CREO PARAMETRIC BY PTC INC 2022484** | **21 MB** · 6652 AF · **3133 bspl-surf (934 rational)** · 5880 bspl-curve · **NAUO 116 / PD 117** · 3015 cyl · 129 tor · 75 extrusion · 1 BREP_WITH_VOIDS | C1 C2 C5 C6 C7 | **H** — the single richest file found: instancing + heavy rational NURBS + Creo dialect + AP203 + scale |
| `framework/FW13_hinge_R_assy.stp` | ↑ `Hinges/13_5_hinge_R_assy.stp` | ↑ CC BY 4.0 | `be1fbe624389` | AP203 · Creo 2022014 | 503 KB · 269 AF · **NAUO 6 / PD 6** · 6 MSB · 13 tor · 12 bspl-surf (4 rat) | C1 C4 C5 | **H** — small enough to be a unit-test fixture while still a real multi-body assembly |
| `framework/FW_expansioncard_threaded.stp` | framework · `github.com/FrameworkComputer/ExpansionCards` → `Mechanical/Printable/3D/` | **CC BY 4.0** (same explicit README grant, own repo) | `6128186260b2` | AP203 · Creo 2018020 | 526 KB · 259 AF · **NAUO 13 / PD 12** · **54 bspl-surf (18 rat)** · 12 sph · 12 tor · 10 cone | C1 C2 C3 C5 | **H** — best size-to-coverage ratio in the bank: instancing, rational splines and all three analytic frontier kinds in half a megabyte |
| `framework/FW13_camera_module.stp` | framework · `Mainboard/Webcam/` | ↑ CC BY 4.0 | `c8be514e8f31` | AP203 (+`SHAPE_APPEARANCE_LAYERS_GROUPS`) · Creo 2024103 | 2.98 MB · 1445 AF · **4 SURFACE_OF_REVOLUTION** · 163 bspl-curve · 35 cone · 10 tor | C3 C5 | M — only bank file with `SURFACE_OF_REVOLUTION`; also exercises the two-item FILE_SCHEMA list |
| `framework/FW_printable_case_full.stp` | framework · `Mainboard/Mainboard/Printable Case/` | ↑ CC BY 4.0 | `5b2a62f7a61d` | AP203 · Creo 2022014 | 2.5 MB · 1641 AF · **115 cone** · NAUO 18 / PD 10 · 678 cyl | C1 C3 C5 | M — cone-dense analytic part; useful as a stage-1 recognition stress before splines land |
| `nist-pmi/files/nist_ctc_02_asme1_rc.stp` | nist-pmi · NIST MBE PMI V&C project, `NIST-PMI-STEP-Files.zip` (nist.gov/document/nist-pmi-step-files) | **US Government work, public-domain-equivalent.** Project page, verbatim: "The test cases, CAD models, and STEP files can be used without any restrictions." Explicitly names the STEP files. Acknowledgement requested, NIST logo/endorsement forbidden. Same disposition as the two NIST fixtures already adopted. | `e6df6d0ed68f` | AP203 · scrubbed (README: "information that shows which CAD software generated the files has been removed") | **664 AF** · **94 bspl-surf (30 rational)** · 318 cyl · 158 cone · 22 sph · 132 bspl-curve | C2 C3 | **H** — densest analytic+spline mix at only 1.15 MB; PD, so zero adoption friction |
| `nist-pmi/files/nist_ctc_02_asme1_ap242-e2.stp` | ↑ | ↑ PD | `99a0a2079dde` | **AP242 ed.2** (`…MIM_LF {…442 3 1 4}`) · scrubbed | 637 AF · 34 bspl-surf · 314 cyl · **158 cone** · 24 sph · semantic PMI | C3 C7 | **H** — same part as the row above in AP242 ed.2: a controlled AP203-vs-AP242 A/B on identical geometry |
| `nist-pmi/files/nist_ctc_02_asme1_ap203.stp` | ↑ | ↑ PD | `ae0049980cb1` | AP203 (`…403 2 1 2`) · scrubbed | 3.36 MB · 487 AF · **80 bspl-surf (26 rational)** · 224 cyl · 82 cone · graphical PMI | C2 C7 | M — third rendition of CTC-02; the graphical-PMI bulk is what makes it 3 MB |
| `nist-pmi/files/nist_ctc_04_asme1_ap242-e1.stp` | ↑ | ↑ PD | `20b43b54ce25` | **AP242 ed.1** · scrubbed | 484 AF · **116 cone · 22 sph · 29 tor** · 232 cyl · 226 bspl-curve | C3 C7 | **H** — the best single file for cone/sphere/torus recognition work |
| `nist-pmi/files/nist_ctc_05_asme1_rd.stp` | ↑ | ↑ PD | `5140b8f41fa1` | AP203 · scrubbed | 327 KB · 209 AF · **27 bspl-surf (9 rational)** · 100 cyl · 26 cone · 4 sph · 12 tor | C2 C3 | **H** — smallest file in the bank carrying rational surfaces; ideal first spline fixture |
| `nist-pmi/files/nist_ftc_07_asme1_rd.stp` | ↑ | ↑ PD | `a0bc55165678` | **`CONFIG_CONTROL_DESIGN`** (bare AP203 schema string, no version tag) · scrubbed | 403 KB · 269 AF · 20 bspl-surf · 18 sph · **24 tor** · 28 cone | C3 C4 C7 | **H** — exercises the short AP203 schema identifier the current corpus never sees |
| `nist-pmi/files/nist_ftc_10_asme1_ap242-e2.stp` | ↑ | ↑ PD | `9d7711f48e81` | AP242 ed.2 · scrubbed | 1.88 MB · 282 AF · **39 tor** · 425 bspl-curve · 165 cyl · 16 cone · 10 sph | C4 C7 | M — fillet-runout torus density; pairs with `nist_ftc_10_asme1_rb` semantics |
| `nist-pmi/files/nist_stc_10_asme1_ap242-e2.stp` | ↑ | ↑ PD | `fba4f46cab1c` | AP242 ed.2 · scrubbed | **4.88 MB** · 256 AF · **43 tor** · 223 plane · 117 cyl · 3 bspl-surf (1 rat) | C4 C6 C7 | M — most tori in the NIST set, at a size that stresses the parser |
| `nist-pmi/files/nist_stc_09_asme1_ap242-e3.stp` | ↑ | ↑ PD | `737423afcb22` | **AP242 ed.3** (`…442 4 1 4`) · scrubbed | 5.29 MB · 125 AF · 178 plane · 59 cyl · 20 bspl-curve | C6 C7 | **H** — the only AP242 **edition 3** files anywhere in reach (the `stc_*` group); schema-string coverage matters more here than the geometry |
| `nist-pmi/files/nist_ftc_09_asme1_ap242-e1.stp` | ↑ | ↑ PD | `f1215fe15a78` | AP242 ed.1 · scrubbed | **6.11 MB** (largest NIST file) · 163 AF · 92 cyl · 8 cone | C6 C7 | **H** — AP242 twin of `nist/nist_ftc_09_asme1_rd.stp`, an *already-adopted* fixture: same part, 23× the bytes. Direct latency A/B |
| `nist-edm/weldment_asm_solid.stp` | nist-edm · `github.com/usnistgov/engineering-design-models` → `models/STEP/STI/weldment_asm_solid/` | **Public domain as asserted by NIST for the collection.** Repo description: "a public-domain collection of industry-relevant CAD models"; `LICENSE.md` is the NIST terms-of-use (17 U.S.C. §105, no domestic copyright). Caveat, stated plainly: the models were *donated by industrial collaborators* in the 1990s, so the PD status rests on NIST's assertion as host, not on a per-file grant. Stronger than STEPcode's `data/` (an explicit PD claim about the models exists) but weaker than the MBE-PMI set (NIST-authored). Per `STI/README`, this file's design source is the NIST DPPA repository — **not** the STEPNet / STEP-Tools-donated subset. | `a99e0cac838b` | AP203 · Pro/ENGINEER via ST-DEVELOPER | 1.74 MB · 1201 AF · **NAUO 108 / PD 34** · 33 MSB · **206 cone** · 622 cyl · 16 tor | C1 C3 C5 | **H** — a genuine industrial weldment assembly: 108 placements of 34 parts, PD, and PTC-lineage dialect |
| `nist-edm/vaccase_asm_solid.stp` | ↑ `models/STEP/STI/vaccase_asm_solid/` | ↑ PD-as-asserted; DPPA-sourced | `a2127a07f6bb` | AP203 · Pro/ENGINEER | 435 KB · 285 AF · **NAUO 14 / PD 9** · 8 MSB · 210 cyl | C1 C5 | M — compact PD assembly; the fallback if the CC-BY / MIT assemblies hit an attribution objection |
| `nist-edm/part02.step` | ↑ `models/Allied-Signal/part02/` | ↑ PD-as-asserted (Allied-Signal donation) | `59ad75dc5549` | AP203 · none declared · ST-DEVELOPER v1.4, 1995 | 772 KB · **AF 0 / FACE_SURFACE 427** · **83 tor** · 206 cyl · 22 cone · 115 plane | C4 C5 | **H** — the *only* file found that uses `FACE_SURFACE` instead of `ADVANCED_FACE`. A dialect the importer almost certainly rejects today, and a real aerospace part besides |
| `nist-edm/piston.stp` | ↑ `models/STEP/STI/piston/` | ↑ PD-as-asserted; DPPA-sourced, translated to STEP by STEP Tools' ST-ACIS | `53e51989a305` | AP203 · ST-ACIS / ST-DEVELOPER 1.6 | 99 KB · 46 AF · 7 tor · 69 bspl-curve · 10 cyl · 3 cone | C4 | L — tiny fillet-and-groove part; cheap regression fixture, little else |
| `nist-edm/tmountop20.step` | ↑ `models/STEP/tmountop20/` | ↑ PD-as-asserted; authored at NIST (`('Bill Regli'), ('NIST')`) — the strongest PD case in this vein | `21f58e0434fc` | AP203 · **EDS — UNIGRAPHICS 10.5** | 241 KB · 141 AF · 80 plane · 61 cyl | C5 | M — the bank's only NX/Unigraphics-lineage file. Ancient (1995) and geometrically dull, but it is the NX dialect sample |
| `occt/linkrods.step` | occt · `github.com/Open-Cascade-SAS/OCCT` → `data/step/linkrods.step` | **LGPL-2.1 + OCCT exception, as redistributed — with a caveat.** Same structural concern as STEPcode D2 (a code license over a `data/` dir), but materially better: the file header names **MATRA-DATAVISION**, i.e. OCCT's own originating author, so the redistributor plausibly *is* the rights holder rather than a conduit. Not PD, and LGPL notice obligations would attach on adoption. Adopt only with a NOTICE entry. | `3674e4b01ee0` | `AUTOMOTIVE_DESIGN_CC1` (AP214 CC1 variant) · **EUCLID / OL-2.0B**, 1998 | 1.79 MB · 37 AF but **50 bspl-surf (16 rational)** and only **6 planes** · 268 bspl-curve · 9 tor | C2 C5 | **H** — the most spline-dominated part in the bank (surfaces outnumber faces); also an exotic schema string and an exporter nobody tests |
| `occt/screw.step` | ↑ `data/step/screw.step` | ↑ same LGPL caveat | `4b3649a4f5c4` | `AUTOMOTIVE_DESIGN_CC1` · EUCLID | 89 KB · 10 AF · 63 bspl-curve · 3 tor · 2 cone | C5 | L — small companion; useful only to pin the CC1 schema string cheaply |
| `freecad/Schenkel.stp` | freecad · `github.com/FreeCAD/FreeCAD` → `data/examples/Schenkel.stp` | **CC BY-SA 4.0 — declared inside the file itself.** The STEP header carries `Copyright (C) 2011, Juergen Riegel … licensed under the Creative Commons CC-BY-SA 4.0 License`. This is the D2 gold standard: a per-file grant, so FreeCAD's LGPL is irrelevant. Share-alike: attribution + license notice required, and adaptations must match. | `69df4cb83831` | AP203 (`CONFIG_CONTROL_DESIGN`) · **CATIA Version 5 Release 14** | 590 KB · 409 AF · **12 sph · 11 tor · 6 cone · 4 SURFACE_OF_REVOLUTION · 1 SURFACE_OF_LINEAR_EXTRUSION** · 219 plane | C3 C4 C5 | **H** — the bank's CATIA sample *and* its widest analytic-surface spread, with an unimpeachable per-file license |
| `freecad/as1-ac-214.stp` | freecad · ↑ `data/tests/Step/as1-ac-214.stp` | **UNCLEAR — do not adopt without a decision.** No per-file grant (unlike `Schenkel.stp`), and `AS1` is the canonical PDES/STEPNet interoperability assembly that FreeCAD redistributes rather than authored — the exact TAIL_TURBINE shape. FreeCAD's LGPL is a code license over a `data/tests/` dir. | `1bb1a0e55dc4` | AP214 · **AutoCAD 2000 / AutoCAD STEP 2000** | 83 KB · 53 AF · **NAUO 13 / PD 9** · 5 MSB · 28 cyl | C1 C5 | L — geometrically the textbook instancing fixture and an AutoCAD dialect sample, but the license blocks it. Recorded so the next hunt does not rediscover it |
| `ploopy/ploopy_classic_revD_top.STEP` | ploopy · `github.com/ploopyco/trackball` → `hardware/Mechanicals/STEPs/Revision D/Top.STEP` | **CERN OHL v1.2**, and specifically over the data: the README says "the hardware is released under OHL CERN v1.2", and `hardware/LICENSE` (the directory containing the STEP files) is the CERN OHL v1.2 text. Reciprocal: redistribution must carry the licence and keep modifications under it. Workable for a fixture with a NOTICE entry; mixed-licence, so flag before adopting. | `024b2e497968` | AP203 · not declared — header fields blank; formatting (`'STEP AP203'` description, `'NONE'` labels) is the **SolidWorks** writer signature, *inferred not stated* | 9.77 MB · 610 AF · **305 bspl-surf** · 985 bspl-curve · 132 plane · 155 cyl · 20 tor · 6 sph | C2 C5 C6 | **H** — a freeform ergonomic shell: half its faces are splines with heavily trimmed boundaries. The best trimmed-spline stress in the bank |

## Frontier-class coverage

| class | H-priority candidates | state |
|---|---|---|
| C1 assembly instancing | PyGamer, MEMENTO, Fruit Jam Case, FW13 full laptop, FW hinge, FW expansion card, NIST weldment | **7 H — well covered.** NAUO/PD ratios span 1.0 to 3.6, sizes 503 KB to 21 MB |
| C2 freeform NURBS / trimmed splines | OCCT linkrods, Ploopy top, FW13 full laptop, NIST ctc_02_rc, ctc_05_rd, Fruit Jam Case | **6 H — well covered**, from 327 KB starters to a 934-rational-surface monster |
| C3 cone/sphere/torus | NIST ctc_04, ctc_02_e2, ftc_07_rd, Schenkel, FW expansion card | **5 H — covered** |
| C4 fillet-heavy | NIST part02 (83 tori), stc_10 (43), ftc_10 (39), RGB Matrix (2048), Schenkel | **covered** |
| C5 dialect breadth | Creo (Framework ×5), CATIA V5 (Schenkel), EUCLID (OCCT ×2), Autodesk Fusion/ATF (Adafruit ×4), Pro/E (NIST EDM ×2), Unigraphics 10.5 (tmountop20), AutoCAD (as1, blocked), SolidWorks (Ploopy, *inferred*) | **covered — with one soft spot:** no file whose header *states* SolidWorks or modern NX/Siemens |
| C6 scale | RGB Matrix 57 MB, FW13 laptop 21 MB, Fruit Jam 13.7 MB, Ploopy 9.8 MB, MEMENTO 9.1 MB, NIST ftc_09 6.1 MB | **covered** — top of bank is 216× the largest existing fixture |
| C7 AP203/AP242 | AP242 ed.1/ed.2/**ed.3** across the NIST set; bare `CONFIG_CONTROL_DESIGN`; `AUTOMOTIVE_DESIGN_CC1`; matched AP203/AP242 pairs of one part | **covered** |

## Considered and rejected

- **CAx-IF / MBx-IF interoperability rounds** — not pursued. `WILD-CORPUS-LICENSES.md` D2
  already traced `TAIL_TURBINE.stp` back to a PDES/CAx-IF round file whose terms
  nobody can produce. Any file whose lineage is a test round is presumed unclear.
- **`stepcode/stepcode` `data/`** — already EXCLUDED by D2; no new evidence found. Its
  `data/ap214e3/` tree is the same CAx-IF content under another roof.
- **`tpaviot/pythonocc-demos` `assets/models/`** — no LICENSE file in the repo at all, and
  the assets are plainly third-party (`KR600_R2830-4.stp` is a KUKA robot, i.e. a vendor
  model; `as1-oc-214.stp` is the CAx assembly again). Rejected on both license and provenance.
- **`Ultimaker/Ultimaker2` (and `…2ExtendedPlus`)** — `LICENSE` reads **CC BY-NC 3.0**.
  Non-commercial restriction; incompatible with an MIT-or-Apache repo. Rejected on license,
  despite excellent 55–60 MB assemblies.
- **`nasa-jpl/open-source-rover`** — repo is Apache-2.0 but every STEP file is under
  `electrical/pcb/…/3d_models/` and is a **vendor-supplied component model** (Molex, JST,
  Pololu, Raspberry Pi). No mechanical STEP of JPL's own design exists in the repo.
  Rejected on provenance — the exact translation-of-someone-else's-model failure mode.
- **`KiCad/kicad-packages3D`** — repo license reports NOASSERTION; the models are
  CC-BY-SA-with-exception *and* very largely derived from manufacturer datasheets/models.
  Provenance too diffuse to clear per-file, and the parts are geometrically trivial.
- **`FreeCAD/FreeCAD` `data/tests/Step/as1-ac-214*.stp`** — banked but flagged UNCLEAR; see
  the table row. Do not adopt without an explicit decision from Evan.
- **NIST EDM `models/STEP/STI/` files whose `README` design source is STEPNet, STEP Tools,
  Inc. or TEAM** (`moon_buggy_asm.stp`, `as1_pe.stp`, `mbb.stp`, `iso14649-demo.stp`,
  `ph4m3-st.stp`, `teampart.stp`) — third-party donations redistributed by NIST. Only the
  `NIST DPPA Repository`-sourced files in that directory were taken.
- **`CadQuery/cadquery`** — GitHub reports the repo license as **NOASSERTION**, and the
  flag is a classifier artifact: the LICENSE file states Apache-2.0 in prose behind a custom
  preamble, so the Apache-2.0 verdict for the existing `cq_red_cube_blue_cylinder.step`
  fixture stands (`WILD-CORPUS-LICENSES.md`'s post-audit note is the record). No new files
  taken.
- **GrabCAD / Thingiverse / TraceParts and manufacturer part portals** — not searched, per
  the standing rule.

## Standing subscription worth keeping

`github.com/usnistgov/engineering-design-models` — a public-domain-asserted archive of
1990s industrial CAD in a dozen native formats, only lightly mined here. It holds the
`FACE_SURFACE` dialect, Pro/E and Unigraphics assemblies, and roughly 30 more STEP parts
that were not censused. Re-visit it whenever a new frontier class opens.
