//! M4 PR 4 spec D7: the golden-digest family gains RESOLUTION — same
//! recipe + same verdicts ⇒ byte-identical `ResolveError`/`Diagnosis`
//! output over the diagnosis corpus (deliberately-broken documents).
//! The pinned digest must hold at every CI ε row (the corpus is
//! margin-fat by construction; nothing in it is band-edge at 1e-6 …
//! 1e-12), and the corpus derives every probe from evaluation output
//! and ambient `Tol::witness().get()` only — no hard-coded ε anywhere.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use fixture::pr4::diagnosis_corpus;

/// FNV-1a 64 over the labeled Debug encodings (the same digest idiom
/// as the PR 3 name-table golden — arena keys inside tombstones are
/// included DELIBERATELY: same recipe + verdicts ⇒ same kill history
/// ⇒ same keys; a drift here is a replay-identity break).
fn digest(rows: &[(&'static str, editor_core::Resolution)]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |s: &str| {
        for b in s.bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x1000_0000_01b3);
        }
    };
    for (label, res) in rows {
        feed(label);
        feed(&format!("={res:?};"));
    }
    h
}

/// The pinned diagnosis-corpus digest (update ONLY on a ratified
/// resolution-semantics change — this is the replay-identity family's
/// resolution member).
// What the pin holds: flip-vanish → GroupResized { was: 2, now: 1 }
// naming B's cap vertex as the one cutter gone; cascade → Cascade;
// structural-param → StructuralParam; node-gone; ambiguous. Re-pinned
// when profile pieces became named by minted step ids: the diagnosed
// names spell `{ step, role }`, and every row keeps its shape.
const DIAGNOSIS_DIGEST: u64 = 0xd8f6_5129_6654_53d0;

#[test]
fn diagnosis_corpus_is_golden() {
    let rows = diagnosis_corpus::<f64>();
    assert_eq!(
        digest(&rows),
        DIAGNOSIS_DIGEST,
        "diagnosis digest drifted: got {:#018x}\nrows: {rows:#?}",
        digest(&rows)
    );
}

#[test]
fn diagnosis_output_is_deterministic_across_runs() {
    // Byte-identical Debug output, run to run, in one process (the
    // cross-process pin is the golden above; the cross-ε pin is the
    // CI matrix running this whole file at every row).
    let a = format!("{:?}", diagnosis_corpus::<f64>());
    let b = format!("{:?}", diagnosis_corpus::<f64>());
    assert_eq!(a, b);
}
