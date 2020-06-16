use crate::{
    boolean::{output_expected_boolean, output_false, output_true},
    get_output,
    integers::{fail_integer, fail_synthesis, IntegerTester},
    parse_program,
    EdwardsConstrainedValue,
    EdwardsTestCompiler,
};
use leo_compiler::ConstrainedValue;
use leo_inputs::types::{IntegerType, U64Type};
use leo_types::{InputValue, Integer};

use snarkos_curves::edwards_bls12::Fq;
use snarkos_models::gadgets::{
    r1cs::TestConstraintSystem,
    utilities::{alloc::AllocGadget, uint::UInt64},
};

fn output_expected_allocated(program: EdwardsTestCompiler, expected: UInt64) {
    let output = get_output(program);

    match output {
        EdwardsConstrainedValue::Return(vec) => match vec.as_slice() {
            [ConstrainedValue::Integer(Integer::U64(actual))] => assert_eq!(*actual, expected),
            _ => panic!("program output unknown return value"),
        },
        _ => panic!("program output unknown return value"),
    }
}

#[test]
#[ignore] //temporarily ignore memory expensive tests for travis
fn test_u64() {
    test_uint!(Testu64, u64, IntegerType::U64Type(U64Type {}), UInt64);

    Testu64::test_min(std::u64::MIN);
    Testu64::test_max(std::u64::MAX);

    Testu64::test_input();

    Testu64::test_add();
    // Testu64::test_sub(); //Todo: Catch subtraction overflow error in gadget
    Testu64::test_mul();
    Testu64::test_div();
    Testu64::test_pow(); // takes ~2mins

    Testu64::test_eq();
    Testu64::test_ge();
    Testu64::test_gt();
    Testu64::test_le();
    Testu64::test_gt();

    Testu64::test_assert_eq();
    Testu64::test_ternary();
}