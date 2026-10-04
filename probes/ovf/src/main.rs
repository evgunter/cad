use geom_core::linalg::lsq;
fn main() {
    let r = std::panic::catch_unwind(|| lsq::factor_banded(&[0, usize::MAX], &[vec![1.0], vec![1.0]]).map(|_| ()));
    println!("first=[0, usize::MAX], width 1: {r:?}");
}
