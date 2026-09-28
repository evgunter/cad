unit: ATREST-3, PR #3191; frozen head 97b51065e; protocol c3129311b
class at spec: L / STRUCTURAL; triage: dual — a public return-type change across crates, and what every tier-3 door promises (broad, hard to reverse)
briefs: brief-R1.md / brief-R2.md (sha256 in sha256.txt), identical modulo lane paths; dispatched concurrently 2026-09-27
R1: APPROVE-WITH-FIXES 0 MAJ / 4 MIN / 4 NOTE + style; 226,055 tokens; ~42 min harness; probes executed (claims 1-3 at f64, D-C at f64, Interval measured to refuse); glimpse: with-build-slot printed other slot holders' pids, not inspected (benign)
R2: APPROVE-WITH-FIXES 0 MAJ / 4 MIN / 5 NOTE + style; 224,523 tokens; ~26 min harness; probes executed (claim 1 pairs, claims 2-3 at f64, D-C at Dual); no glimpse
fair: yes — no relaxation, neither interrupted
