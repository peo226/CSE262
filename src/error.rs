
// You are free to add more error variants if you need them.

#[derive(Debug,PartialEq)]
pub enum AsaErrorKind {
  UndefinedFunction,
  InvalidMathChildren,
  IncompatibleComparison,
  InvalidComparisonOperator,
  VariableNotDefined(String),
  DivisionByZero,
  NumberOverflow,
  NumberUnderflow,
  Generic(String),  
}