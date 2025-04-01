//ALL SOURCES LISTED AT BOTTOM

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
  kind: TokenKind,
  lexeme: String,
  start_line: u32,
  end_line: u32,
  start_col: u32,
  end_col: u32,
}

impl Token {

  pub fn new() -> Token {
    Token{
      kind: TokenKind::Other, 
      lexeme: "".to_string(),
      start_line: 0,
      end_line: 0,
      start_col: 0,
      end_col: 0,
    }
  }

  pub fn get_kind(&self) -> TokenKind {
    self.kind
  }

  pub fn set_kind(&mut self, new_kind: TokenKind) {
    self.kind = new_kind;
  }

}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
  // Keywords
  True,
  False,
  Fn,
  Return,
  Let,
  While,
  //------
  Alpha,
  Digit,
  LeftParen, //open paren already implemented
  RightParen, //close paren already implemented
  LeftCurly, //open curly already implemented
  RightCurly, //close curly already implemented
  Equal,
  LessThan, //new✅
  GreaterThan,//new ✅
  Plus,
  PlusEqual,
  Divide, //new ✅
  DivideEqual,
  Multiply, //new ✅
  MultiplyEqual,
  Period, //new✅
  Minus, //minus already implemented
  MinusEqual,
  Quote,
  WhiteSpace,
  Semicolon,
  Comma,
  Identifier, //new✅
  Integer, //new ✅
  Float, //new ✅
  Comment, //new ✅
  MultiComment, //new✅
  FunctionArguments, //new✅
  NewLine, //new from HW 4 ✅
  Other, //other already implemented
  EOF,
}



pub fn lex(input: &str) -> Vec<Token> {
  let mut tokens = Vec::new();
  let list = input.as_bytes();
  let mut i = 0; //index iterator
  let mut line = 1; //line of the input
  let mut col = 1; //column where the token begins
  let mut diff = 0; //length of the token - 1 
  while i < list.len() {
    let start_sub = i;
    let c = list[i];
    let mut kind = match c {
        48..=57 => TokenKind::Digit,
        65..=90 | 97..=122 => TokenKind::Alpha,
        32 | 9=> TokenKind::WhiteSpace,
        60 => TokenKind::LessThan,
        61 => TokenKind::Equal,
        62 => TokenKind::GreaterThan,
        59 => TokenKind::Semicolon,
        123 => TokenKind::LeftCurly,
        125 => TokenKind::RightCurly,
        10 => TokenKind::NewLine,
        40 => TokenKind::LeftParen,
        41 => TokenKind::RightParen,
        42 => TokenKind::Multiply,
        43 => TokenKind::Plus,
        44 => TokenKind::Comma,
        45 => TokenKind::Minus,
        46 => TokenKind::Period,
        47 => TokenKind::Divide,
        34 => TokenKind::Quote,
        _x => TokenKind::Other,
    };
   
    //check if fn
    if c == b'f' {
        if i + 1 < list.len(){
          if list[i + 1] == b'n' {
            kind = TokenKind::Fn;
            i += 1;
            diff = 1;
          }
        }
    }
    //check if true
    if c == b't' {
      if i + 3 < list.len(){
        if list[i + 1] == b'r' && list[i + 2] == b'u' && list[i + 3] == b'e' {
          kind = TokenKind::True;
          i += 3;
          diff = 3;
        }
      }
    }
    //check if false
    if c == b'f' {
      if i + 4 < list.len(){
        if list[i + 1] == b'a' && list[i + 2] == b'l' && list[i + 3] == b's' && list[i + 4] == b'e' {
          kind = TokenKind::False;
          i += 4;
          diff = 4;
        }
      }
    }
    //check if let
    if c == b'l' { 
      if i + 2 < list.len(){
        if list[i + 1] == b'e' && list[i + 2] == b't' {
          kind = TokenKind::Let;
          i += 2;
          diff = 2;
        }
      }
    }
    //check if return
    if c == b'r' {
      if i + 5 < list.len(){
        if list[i + 1] == b'e' && list[i + 2] == b't' && list[i + 3] == b'u' && list[i + 4] == b'r' && list[i + 5] == b'n' {
          kind = TokenKind::Return;
          i += 5;
          diff = 5;
        }
      }
    }

     //check if while
     if c == b'w' {
      if i + 4 < list.len(){
        if list[i + 1] == b'h' && list[i + 2] == b'i' && list[i + 3] == b'l' && list[i + 4] == b'e'  {
          kind = TokenKind::While;
          i += 4;
          diff = 4;
        }
      }
    }
    
    //check if comment
    if c == b'/' {
      if i + 1 < list.len(){
        if list[i + 1] == b'/' {
          kind = TokenKind::Comment;
          i += 1;
          diff = 1;

          //add everything up until a newline
           if i + 1 < list.len() {
            while i+1  < list.len() {
              let next_c = list[i+1];
              let next_kind = match next_c {
                10 => TokenKind::NewLine,
                _x => TokenKind::Other,
              };
              
              if next_kind != TokenKind::NewLine  { //anything except for a newline
                i += 1;
                diff += 1;
              } else { 
                break;//stop comment at a newline
              }
            }
          }
        }
      }
    }


    //check if multi line comment
    if c == b'/' {
      if i + 1 < list.len(){
        if list[i + 1] == b'*' {
          kind = TokenKind::MultiComment;
          let mut num_endings = 0;
          let mut num_starts  = 1;
          i += 1;
          diff = 1;
          if i + 1 < list.len() {
            while i+1  < list.len() {
              let next_c = list[i+1];
              let next_kind = match next_c {
                47 => TokenKind::Divide,
                42 => TokenKind::Multiply,
                _x => TokenKind::Other,
              };
              if next_kind == TokenKind::Divide && list[i] == b'*'  {//if we run into a */
                i += 1;
                diff += 1;
                num_endings += 1;
                if num_endings == num_starts {break;}//checking that the amount of /* == amount of */
              } else if next_kind == TokenKind::Multiply && list[i] == b'/'  {//if we run into a /*
                i += 1;
                diff += 1;
                num_starts += 1;
              }
              else{
                i += 1;
                diff += 1;
              }
            }
          }
        }
      }
    }

    //checking for integers, floats, and identifiers
    //this converts all digits to integers, or floats if possible
    if kind == TokenKind::Digit {
      kind = TokenKind::Integer; //convert to integer
      if i + 1 < list.len() { //need to check if this is more than just one digit
        while i+1  < list.len() {
          let next_c = list[i+1];
          let next_kind = match next_c {
            48..=57 => TokenKind::Digit,
            46 => TokenKind::Period,
            65..=90 | 97..=122 => TokenKind::Alpha,
            _x => TokenKind::Other,
          };
          
          if next_kind == TokenKind::Digit { //integers
            i += 1;
            diff += 1;
          } else if next_kind == TokenKind::Period { //floats
            //need to check that we haven't already detected a period
            if kind == TokenKind::Integer {
              kind = TokenKind::Float; 
              i += 1;
              diff += 1;
            } else {
              break; //stop analyzing this sequence of tokens as one component
            }
          } else if next_kind == TokenKind::Alpha {
              if kind != TokenKind::Float {
                kind = TokenKind::Identifier;
                i += 1;
                diff += 1;
              } else {
                break;
              }
          } else {
            break;
          }
        }
      }
    }
    
    //checking for identifiers (sequence of alphanumerical)
    if kind == TokenKind::Alpha {
      kind = TokenKind::Identifier; //convert to identifier
      if i + 1 < list.len() {
        while i+1  < list.len() {
          let next_c = list[i+1];
          let next_kind = match next_c {
            65..=90 | 97..=122 => TokenKind::Alpha,
            48..=57 => TokenKind::Digit,
            _x => TokenKind::Other,
          };
          
          if next_kind == TokenKind::Alpha ||  next_kind == TokenKind::Digit { 
            i += 1;
            diff += 1;
          } else { 
            break;//something else besides alphanum
          }
        }
      }
    }

    //checking for function arguments . . . identifier "(" {identifier} ")"  
    if tokens.len() > 0 { 
      let prev_kind = match tokens[tokens.len()-1] {
        Token {
          kind: k,
          lexeme: _,
          start_col: _,
          end_col: _ ,
          start_line: _,
          end_line: _,
        } =>  k
      };

      if prev_kind == TokenKind::Identifier && kind == TokenKind::LeftParen {
        if i + 1 < list.len() {
          //creating checkpoints in case we dont find a closing parenthesis
          let checkpoint_i = i; 
          let checkpoint_d = diff;
          let mut args = vec![];
          let mut num_commas = 0;

          while i+1  < list.len() {
            let next_c = list[i+1];
            let next_kind = match next_c {
              65..=90 | 97..=122 => TokenKind::Alpha,
              48..=57 => TokenKind::Digit,
              44 => TokenKind::Comma,
              41 => TokenKind::RightParen,
              _x => TokenKind::Other,
            };

            if next_kind == TokenKind::Alpha || next_kind == TokenKind::Digit {
              i += 1;
              diff += 1;
              args.push(next_c);
            } else if next_kind == TokenKind::Comma {
              i += 1;
              diff += 1;
              num_commas += 1;
              args.push(next_c);
            } 
            else if next_kind == TokenKind::RightParen {
              let separated: Vec<_> = args
              .split(|&e| e == 44)
              .filter(|v| !v.is_empty())
              .collect(); //LINK #2

              // println!("{:?} {:?}, {:?}", separated.len(), num_commas, separated);
              if separated.len() == num_commas + 1 {
                i += 1;
                diff += 1;
                kind = TokenKind::FunctionArguments;
                break;
              } else {
                //revert back to checkpoints if we break the pattern
                i = checkpoint_i;
                diff = checkpoint_d;
                break;
              }
            } else { 
              //revert back to checkpoints if we break the pattern
              i = checkpoint_i;
              diff = checkpoint_d;
              break;
            }
          }
        }
      }
    }
    
    //checking for math (+=, -=, *=, /=)
    if i + 1 < list.len()  && list[i+1] == 61 {
      match kind{
        TokenKind::Plus => {kind = TokenKind::PlusEqual; i+= 1;},
        TokenKind::Minus => {kind = TokenKind::MinusEqual; i+= 1;},
        TokenKind::Multiply => {kind = TokenKind::MultiplyEqual; i+= 1;},
        TokenKind::Divide => {kind = TokenKind::DivideEqual; i+= 1;},
        _ => {}
      }
    }
    

    let lexeme_vec = list[start_sub..i+1].to_vec(); 
    let lexeme_str =  match String::from_utf8(lexeme_vec)//LINK #1 
    {
      Ok(s) => s,
      Err(_e) => "".to_string(),
    }; 
    if kind != TokenKind::WhiteSpace { //ignoring whitespace
      println!("{:?}",lexeme_str.clone());
    }
    //create token struct
    let token = Token {
        kind,
        lexeme: lexeme_str,
        start_col: col,
        end_col: col + diff,
        start_line: line,
        end_line: line,
    };
    diff = 0;
    i +=1;
    col +=1;
    if kind != TokenKind::WhiteSpace { //ignoring whitespace
      tokens.push(token.clone());
    }
    if kind == TokenKind::NewLine{ //changed c==10 to a kind==TokenKind::NewLine to utilize the new TokenKind 
      line +=1;
      col = 1;
    }
  }

  let token = Token {
    kind: TokenKind::EOF,
    lexeme: "".to_string(),
    start_col: col,
    end_col: col ,
    start_line: line,
    end_line: line,
    };
  tokens.push(token);
 //return tokens
 tokens
}
   
  /*
  LINK #1: 
  I used this link to figure out how to handle the from_utf8 error 
  https://stackoverflow.com/questions/19076719/how-do-i-convert-a-vector-of-bytes-u8-to-a-string



  LINK #2:
  I used this link to figure out how to split a vector by a value
  https://stackoverflow.com/questions/60266626/how-can-i-split-a-vector-into-a-vector-of-slices-based-on-a-separator

  */