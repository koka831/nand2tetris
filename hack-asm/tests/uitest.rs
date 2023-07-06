use insta::assert_debug_snapshot;

use hack_asm::{compile, read_and_format};

macro_rules! assert_snapshot {
    ($file:literal) => {
        let program = read_and_format($file).unwrap();
        let binary = compile(&program).unwrap();
        assert_debug_snapshot!(binary);
    };
}

#[test]
fn compile_add() {
    assert_snapshot!("./tests/fixtures/add.asm");
}

#[test]
fn compile_pong() {
    assert_snapshot!("./tests/fixtures/pong.asm");
}

#[test]
fn compile_rect() {
    assert_snapshot!("./tests/fixtures/rect.asm");
}
