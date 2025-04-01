use midterm::*;

fn test_lex(input: &str) -> Vec<TokenKind> {
  /*let mut token_kind_vector: Vec<TokenKind> = Vec::new();
  let tokens = lex(input);
  for t in tokens {
    token_kind_vector.push(t.kind);
  }
  token_kind_vector*/

  lex(input).iter().map(|t| t.get_kind()).collect::<Vec<TokenKind>>()

}

#[test]
fn test_01() {
  assert_eq!(test_lex("123 abc"),vec![
    TokenKind::Integer, 
    TokenKind::Identifier,
    TokenKind::EOF]);
}

#[test]
fn test_02() {
  assert_eq!(test_lex("123.456 5.5.. 0.0 abc"),vec![
    TokenKind::Float,
    TokenKind::Float,
    TokenKind::Period,
    TokenKind::Period,
    TokenKind::Float,
    TokenKind::Identifier,
    TokenKind::EOF]);
}

#[test]
fn test_03() {
  assert_eq!(test_lex("//comment test"),vec![
    TokenKind::Comment,
    TokenKind::EOF]);
}


#[test]
fn test_04() {
  let test_str = "fn foo(a,b,c) {
    let x = a + 1; 
    // This is a comment
    let y = bar(c - b);
    return x * y; // Add the results
  }

  fn bar(a) {
      return a - 3;
  }

  fn main() {
    return foo(123,56,7.89);  
  }";

  assert_eq!(test_lex(test_str),vec![
    //fn foo(a,b,c) {
    TokenKind::Fn, 
    TokenKind::Identifier, 
    TokenKind::FunctionArguments,
    TokenKind::LeftCurly, 
    TokenKind::NewLine,

    //let x = a + 1;
    TokenKind::Let, 
    TokenKind::Identifier, 
    TokenKind::Equal, 
    TokenKind::Identifier, 
    TokenKind::Plus, 
    TokenKind::Integer,
    TokenKind::Semicolon,
    TokenKind::NewLine,

    // This is a comment
    TokenKind::Comment,
    TokenKind::NewLine,

    // let y = bar(c - b);
    TokenKind::Let, 
    TokenKind::Identifier, 
    TokenKind::Equal, 
    TokenKind::Identifier, 
    TokenKind::LeftParen, 
    TokenKind::Identifier,
    TokenKind::Minus,
    TokenKind::Identifier,
    TokenKind::RightParen,
    TokenKind::Semicolon,
    TokenKind::NewLine,
   
    //return x * y; // Add the results
    TokenKind::Return,
    TokenKind::Identifier,
    TokenKind::Multiply,
    TokenKind::Identifier,
    TokenKind::Semicolon,
    TokenKind::Comment,
    TokenKind::NewLine,
    //}
    TokenKind::RightCurly,
    TokenKind::NewLine,
    TokenKind::NewLine,

    //fn bar(a) {
    TokenKind::Fn, 
    TokenKind::Identifier, 
    TokenKind::FunctionArguments,
    TokenKind::LeftCurly,
    TokenKind::NewLine,

    //return a - 3;
    TokenKind::Return,
    TokenKind::Identifier,
    TokenKind::Minus,
    TokenKind::Integer,
    TokenKind::Semicolon,
    TokenKind::NewLine,

    //}
    TokenKind::RightCurly,
    TokenKind::NewLine,
    TokenKind::NewLine,

    //fn main() {
    TokenKind::Fn, 
    TokenKind::Identifier, 
    TokenKind::LeftParen, 
    TokenKind::RightParen,
    TokenKind::LeftCurly,
    TokenKind::NewLine,

	  //return foo(123,56,7.89);  
    TokenKind::Return,
    TokenKind::Identifier,
    TokenKind::LeftParen,
    TokenKind::Integer,
    TokenKind::Comma,
    TokenKind::Integer,
    TokenKind::Comma,
    TokenKind::Float,
    TokenKind::RightParen,
    TokenKind::Semicolon,
    TokenKind::NewLine,

    //}
    TokenKind::RightCurly,


    TokenKind::EOF]);
}

#[test]
fn test_05() {
  assert_eq!(test_lex("123 /*multi comment test*/ a string 
  56.123"),vec![
    TokenKind::Integer,  
    TokenKind::MultiComment,
    TokenKind::Identifier,
    TokenKind::Identifier,
    TokenKind::NewLine,
    TokenKind::Float,
    TokenKind::EOF]);
}

#[test]
fn test_06() {
  assert_eq!(test_lex("123 /*multi comment /*inside comment*/ test*/ a string 
  56.123"),vec![
    TokenKind::Integer,  
    TokenKind::MultiComment,
    TokenKind::Identifier,
    TokenKind::Identifier,
    TokenKind::NewLine,
    TokenKind::Float,
    TokenKind::EOF]);
}

#[test]
fn test_07() {
  assert_eq!(test_lex("while (a < b) {a += 1}"),vec![
    TokenKind::While,  
    TokenKind::LeftParen,
    TokenKind::Identifier,
    TokenKind::LessThan,
    TokenKind::Identifier,
    TokenKind::RightParen,
    TokenKind::LeftCurly,
    TokenKind::Identifier,
    TokenKind::PlusEqual,
    TokenKind::Integer,
    TokenKind::RightCurly,
    TokenKind::EOF]);
}

#[test]
fn test_08() {
  assert_eq!(test_lex("
  
  
  "),vec![
    TokenKind::NewLine,
    TokenKind::NewLine,
    TokenKind::NewLine,
    TokenKind::EOF]);
}

#[test]
fn test_09() {
  assert_eq!(test_lex("testing123 123o abc(1,2,a,b,c)"),vec![
    TokenKind::Identifier, 
    TokenKind::Identifier, 
    TokenKind::Identifier,
    TokenKind::FunctionArguments,
    TokenKind::EOF]);
}

#[test]
fn test_10() {
  assert_eq!(test_lex("1 += 2.352; a -= (bar(x,y,z) * 5);"),vec![
    TokenKind::Integer, 
    TokenKind::PlusEqual, 
    TokenKind::Float,
    TokenKind::Semicolon,
    TokenKind::Identifier, 
    TokenKind::MinusEqual, 
    TokenKind::LeftParen,
    TokenKind::Identifier, 
    TokenKind::FunctionArguments,
    TokenKind::Multiply,  
    TokenKind::Integer, 
    TokenKind::RightParen,
    TokenKind::Semicolon,
    TokenKind::EOF]);
}