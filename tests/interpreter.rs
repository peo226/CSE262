extern crate asalang;
extern crate nom;
use std::io::Write;

use asalang::*;
use nom::IResult;

macro_rules! test_fragment {
  ($func:ident, $test:tt, $expected:expr) => (
    #[test]
    fn $func() -> Result<(),AsaErrorKind> {
      let tokens = lex($test);
      match program(tokens) {
        Ok((tokens, tree)) => {
          assert_eq!(tokens.is_done(), true); // Check that input token stream is fully parsed
          let mut interpreter = Interpreter::new();
          let result = interpreter.exec(&tree);
          std::io::stdout().flush();
          assert_eq!(result, $expected);
          Ok(())
        },
        Err(e) => Err(AsaErrorKind::Generic(format!("{:?}",e))),
      }
    }
  )
}

// Test interpreter fragments (no main function)
test_fragment!(interpreter_numeric, r#"123"#, Ok(Value::Number(123)));
test_fragment!(interpreter_string, r#""hello""#, Ok(Value::String("hello".to_string())));
test_fragment!(interpreter_bool_true, r#"true"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_bool_false, r#"false"#, Ok(Value::Bool(false)));
test_fragment!(interpreter_identifier, r#"x"#, Err(AsaErrorKind::Generic("Stack error (0 size or key not found)".to_string())));
test_fragment!(interpreter_variable_define, r#"let x = 123;"#, Ok(Value::Number(123)));
test_fragment!(interpreter_variable_init, r#"let x = 1;"#, Ok(Value::Number(1)));
test_fragment!(interpreter_variable_bool, r#"let bool = true;"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_variable_string, r#"let string = "Hello";"#, Ok(Value::String("Hello".to_string())));
test_fragment!(interpreter_variable_init_no_space, r#"let x=1;"#, Ok(Value::Number(1)));
test_fragment!(interpreter_math, r#"1 + 1"#, Ok(Value::Number(2)));
test_fragment!(interpreter_math_no_space, r#"1-1"#, Ok(Value::Number(0)));
test_fragment!(interpreter_math_multiply, r#"2 + 4"#, Ok(Value::Number(6)));
test_fragment!(interpreter_assign_math, r#"let x = 1 + 1;"#, Ok(Value::Number(2)));
test_fragment!(interpreter_define_full_program, r#"let x = 1 + 1; let y = 5 - 2; let z = x + y;"#, Ok(Value::Number(5)));


//additional tests
test_fragment!(interpreter_compare_greater, r#"5 > 10"#, Ok(Value::Bool(false)));
test_fragment!(interpreter_compare_multi_ident, r#"let a = 10; let b = 20; let c = a+b; let d = 0; d > c"#, Ok(Value::Bool(false)));
test_fragment!(interpreter_compare_greater_equal, r#"let abc = 10; abc >= 0"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_compare_invalid_compare, r#"hi == 10"#, Err(AsaErrorKind::Generic("Stack error (0 size or key not found)".to_string())));
test_fragment!(interpreter_compare_less_equal, r#"5 <= 10"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_compare_var_bool, r#"let x = 10 < 20; x == false"#, Ok(Value::Bool(false)));
test_fragment!(interpreter_compare_bool_identify_true, r#"let x = true; x == true"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_compare_bool_identify_false, r#"let x = true; x != true"#, Ok(Value::Bool(false)));
test_fragment!(interpreter_compare_bool_identifiers, r#"let x = true; let y = true; x == y"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_compare_bool_identifiers_num, r#"let x = 10; let y = 5; let z = x + 2000; z < y"#, Ok(Value::Bool(false)));

test_fragment!(interpreter_compare_str, r#" "hello" == "bye" "#, Ok(Value::Bool(false)));
test_fragment!(interpreter_compare_string_number, r#" 0 > "2""#, Err(AsaErrorKind::IncompatibleComparison));