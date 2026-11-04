# Review: PR 6509 (`spiricarm/rotational-lane-sphere`), the rotational-chart Green lane, stage 2: the sphere chart

You are one of two independent Opus reviewers of evgunter/cad PR 6509. The review is FULL: claims to falsify, plus the style lane. **The frozen head is `fe92371df87eda4077cb36883cbc8b52ae7aac38`.** Review exactly that commit and name it in your report. You run in your own cloud container, with the repo checked out at that commit.

## Read first
- `CLAUDE.md`.
- `docs/prompts/reviewer-style-lane.md`, in full. It is binding: name the questions you exercised, and give every finding a `sure`/`likely`/`unsure` confidence.
- `docs/prompts/implementer-discipline.md`.
- The unit's spec: `work/spiricarm/the-rotational-lane-reads-the-sphere-chart.md`.
- Stage 1's review row: `docs/dual-review-log/rows/PR-6478.md`. Its classes are likely to recur here: an uncontained chart term, circular swap rows, unwitnessed pads, pins that assert a face kind only in prose.
- `work/spiricarm/plan.md`'s lessons. A narrower bound is not evidence; containment is.
- The PR description, via the GitHub tools (PR 6509's body). Do NOT read its comments or reviews.

## Isolation (binding)
- Until your report is delivered, do not fetch, check out or read any `spiricarm-rev6509-*` branch but your own, and do not read the PR's comments or reviews.
- Disclose any accidental glimpse.
- Post nothing to GitHub.

## Method
- Probes that demonstrate a finding by execution are worth far more than inspection.
- Use a `CARGO_TARGET_DIR` outside the checkout. Keep runs targeted: the frame rows are slow, about 3000 s in debug.
- **The foreground rule has two halves, and both bind.** Never arm background waiters for your own builds. Launch any job longer than about 9 minutes `setsid`-detached with its PID recorded, and poll it in the foreground. Never end your turn with background work running.

## Claims to falsify
1. **The pole rule.** `s_f·(1_N − 1_S) = W`, pole term `2πr³·s_f·(1_N + 1_S)`, and `|Ω| = s_f·A₀ + 2πr²·q` with `q ≡ W (mod 2)` fixed by the range `(0, 4πr²]`. Check that:
   - it is right for both senses, every `W`, one or two loops, and poles on the loop;
   - the "at a pole junction the closure's whole-turn choice is arbitrary, so the connector goes into the region term" argument is sound;
   - `require_sphere_sides` cannot pass a face whose area is wrong by `4πr²`.
   Try faces that hold one pole, both, or neither, a hemisphere exactly, and a face near the full sphere.
2. **The sense bit** (`RotationalChartQ::Sphere { outward }`, read by `quad_lane::face_sense`) is the face's true orientation on every route that reaches the lane, and a flipped bit cannot be certified.
3. **The sphere integrand.** `F = r³ sin v`, `G = r² sin v`, `sin v = p_z/|p|` on a projected chord, enclosed soundly by the Taylor rule. That includes near a pole, where `|p|` is fine but the azimuth rate blows up. `entry_turn`'s refinement to 1024 pieces must refuse rather than guess.
4. **The connector terms.** `connector_terms` (`F(v)·(Δu − mτ)` plus the corner pad `2g(e_i + e_j + g)`) closes the P2 junction gap on BOTH charts. The PR says the corner pad's bound is argued, not derived. Derive it, or find a case that breaks it.
5. **Stage 1's chart-term pad** (`chart_term_pad`) still covers the sphere's chart term with the sphere's lever.
6. **The oracles are independent of the code under test.** Cap, zone and lune areas and Simpson `A⃗`. The 8 sphere swap rows run under both the own and the twin name. Try mutants beyond the 8 the PR lists.
7. **The routing.** `cut_face_lane`'s sphere arm. `reads_rings` is opened for every sphere face. Circle-only faces still take Gauss–Bonnet, and nothing now reaches both routes or neither.
8. **The rows that moved.**
   - `cylinder_sphere_frame`'s 52 runs build at the oracle's volume, pinned `at_its_volume`. Do they assert containment of the oracle, or a narrower thing?
   - Their tolerance is `resolution_pad` at up to 7.6e-7. Is that justified?
   - The re-pins (`Door::Pieces`, the seam-region door's 21/3, `germ_torus_components`, `germ_sphere_torus_placements`) read the refused face's kind, and moved only where the change moved them.
   - Teapot wall 2 now stops at the torus face.
9. **The tracker changes made in the PR.**
   - It closes `a-projected-boundary-on-an-analytic-chart-has-no-volume-lane` (P0) and `the-rotational-lane-omits-the-image-chains-junction-gaps-from-its-region-term` (P2), and unparks SPLITDOOR's sphere cut-in row.
   - Is each closure true against its row's own "done when"?
   - Are the two filed rows right and correctly banded? They are `inside/point-in-solid-refuses-a-sphere-face-bounded-by-a-spline-chord` (P1) and `spiricarm/the-spiric-wall-lane-omits-its-junctions-connectors-from-its-region-term` (P2). In particular, is a cylinder × sphere boolean whose result cannot be a later operand still a P0 under `work/README.md`'s Priority?

## Deliver
Your FINAL message is your report, at most 150 lines. Also commit it as `review-6509.md` at the repo root on a new branch `spiricarm-rev6509-<your label>/report`, and push it. Your label is in your first message. Push probes to `spiricarm-rev6509-<your label>/probes`.

The report must contain:
- a verdict from the ladder: APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT;
- findings as MAJOR, MINOR or NOTE, each with `file:function`, what, evidence and confidence;
- a `## Style` section;
- the claims you upheld or falsified;
- your disclosures.
