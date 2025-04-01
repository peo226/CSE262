use crate::parser::Node;
use std::collections::HashMap;
use crate::error::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  String(String),
  Number(i32),
  Bool(bool),
}

type Frame = HashMap<String, Value>;

#[derive(Debug)]
pub struct Interpreter {
  // Stack:
  // Each element in the stack is a function stack frame.
  // Crate a new stack frame on function entry.
  // Pop stack frame on function return.
  // Key - Variable name
  // Value - Variable value
  stack: Vec<Frame>,
}


impl Interpreter {

  pub fn new() -> Interpreter {
    Interpreter {
      stack: vec![Frame::new()],
    }
  }

  pub fn exec(&mut self, node: &Node) -> Result<Value,AsaErrorKind> {
    match node {
      Node::Program{children} => {
        let mut return_val = Value::Bool(true);
        for n in children {
          match n {
            Node::Expression{..} |
            Node::VariableDefine{..} |
            Node::String{..} |
            Node::Number{..} |
            Node::Bool{..} => {
              let exec_val = self.exec(n);
              match exec_val {
                Ok(val) => {return_val = val;},
                Err(e) => {return Err(e);},
                _ => {
                  let err_msg = format!("Error executing child node: {:?}", children[0]);
                  return Err(AsaErrorKind::UndefinedFunction);
                }
              }
            }
            _ => {
              let err_msg = format!("Unable to find node type for {:?}", n);
              return Err(AsaErrorKind::Generic(err_msg));
            }
          }
        }
        Ok(return_val)
      },
      // Evaluates a mathematical expression based on the elements in the children argument. If the expression is valid, the code evaluates it and returns a new Value object with the resulting value. If the expression is not valid, the code returns an error message.
      Node::MathExpression{name, children} => {
        let val1 = self.exec(&children[0]);
        let val2 = self.exec(&children[1]);
        match (val1, val2) {
          (Ok(val_obj1), Ok(val_obj2)) => {
            //extract number from value
            match (val_obj1, val_obj2){
              (Value::Number(num1), Value::Number(num2)) => {
                 //compute
                match name.as_slice() {
                  b"add" => {
                    Ok(Value::Number(num1 + num2))
                  },
                  b"sub" => {
                    Ok(Value::Number(num1 - num2))
                  },
                  _ => {
                    Err(AsaErrorKind::InvalidMathChildren) 
                  }
                }
              },
              _ => {
                Err(AsaErrorKind::InvalidMathChildren) 
              }
            }
          },
          _ => Err(AsaErrorKind::Generic("One of the children failed to execute".to_string())) 
        }
      },

      // Evaluates a comparison expression based on the name (comparison type) and the children (two identifiers)
      Node::Comparison{name, children} => {   
        let val1 = self.exec(&children[0]);
        let val2 = self.exec(&children[1]);
        match (val1, val2) {
          (Ok(val_obj1), Ok(val_obj2)) => {
            //extract number from value
            match (val_obj1, val_obj2){
              (Value::Bool(num1), Value::Bool(num2)) => {
                match name.as_slice() {
                  b"==" => {
                    Ok(Value::Bool(num1 == num2))
                  },
                  b"!=" => {
                   Ok(Value::Bool(num1 != num2))
                  },
                  
                  _ => {
                    Err(AsaErrorKind::InvalidComparisonOperator) 
                  }
                }
              },
              (Value::String(num1), Value::String(num2)) => {
                match name.as_slice() {
                  b"==" => {
                    Ok(Value::Bool(num1 == num2))
                  },
                  b"!=" => {
                   Ok(Value::Bool(num1 != num2))
                  },
                  
                  _ => {
                    Err(AsaErrorKind::InvalidComparisonOperator) 
                  }
                }
              },
              (Value::Number(num1), Value::Number(num2)) => {
                match name.as_slice() {
                  b">" => {
                    Ok(Value::Bool(num1 > num2))
                  },
                  b"<" => {
                    Ok(Value::Bool(num1 < num2))
                  },
                  b">=" => {
                    Ok(Value::Bool(num1 >= num2))
                  },
                  b"<=" => {
                    Ok(Value::Bool(num1 <= num2))
                  },
                  b"==" => {
                    Ok(Value::Bool(num1 == num2))
                  },
                  b"!=" => {
                   Ok(Value::Bool(num1 != num2))
                  },
                  
                  _ => {
                    Err(AsaErrorKind::InvalidComparisonOperator) 
                  }
                }
              },
              _ => {
                Err(AsaErrorKind::IncompatibleComparison) 
              }
            }
          },
          (Err(e),_) => Err(e),
          (_, Err(e)) => Err(e),
          _ => Err(AsaErrorKind::Generic("One of the children failed to execute".to_string())) 
        }
      },

      // Retrieves the value of the identifier from the current frame on the stack. If the variable is defined in the current frame, the code returns its value. If the variable is not defined in the current frame, the code returns an error message.
      Node::Identifier{value} => {
        let stack = &self.stack;
        let res  = match String::from_utf8(value.to_vec()) {
          Ok(string) => string,
          Err(e) => "".to_string(),
        };
        
        if stack.len() > 0 && stack[0].contains_key(&res){
          let def_value =  stack[0].get(&res);
          match def_value {
            Some(def_value) => Ok(def_value.clone()),
            _ =>  Err(AsaErrorKind::VariableNotDefined(res))
          }
        }else{
          Err(AsaErrorKind::Generic("Stack error (0 size or key not found)".to_string()))
        }

      },
      // Checks the type of the first element in the children argument and deciding what to do based on that type. If the type is a VariableDefine or FunctionReturn node, the code runs the run method on that node and returns the result.
      Node::Statement{children} => {
        match children[0] {
          Node::VariableDefine{..} | Node::FunctionReturn{..} => {
            let res = self.exec(&children[0]);
            match res {
              Ok(res) => Ok(res),
              _ => Err(AsaErrorKind::Generic("Error executing child node".to_string()))
            }
          },
          _ => {
            let err_msg = format!("Unspecified node type: {:?}", children[0]);
            return Err(AsaErrorKind::Generic(err_msg));
          }
        }
      },
      // Defines a new variable by assigning a name and a value to it. The name is retrieved from the first element of the children argument, and the value is retrieved by running the run method on the second element of the children argument. The key-value pair is then inserted into the last frame on the stack field of the current runtime object.
      Node::VariableDefine{children} => {
        let name = match &children[0]{
          Node::Identifier{value} => {
            match String::from_utf8(value.to_vec()) {
              Ok(name) => name, 
              Err(_) => {return  Err(AsaErrorKind::VariableNotDefined("error converting identifier to string".to_string()));}
          }
          },
          _=> return  Err(AsaErrorKind::VariableNotDefined("invalid variable identifier".to_string()))
        };
        
    
        let result = self.exec(&children[1]);
        let stack = &mut self.stack[0];
        match (name,result) {
          (name, Ok(val_to_add)) => {
           stack.insert(name.clone(), val_to_add.clone());
            Ok(val_to_add)
          },
          _ =>{Err(AsaErrorKind::VariableNotDefined("unable to define variable".to_string()))}
        }
      }
      // Evaluate the child node using the exec() method.
      Node::Expression{children} => {
        self.exec(&children[0])
      }
      Node::Number{value} => {
        Ok(Value::Number(*value))
      }
      Node::String{value} => {
        Ok(Value::String(value.clone()))
      }
      Node::Bool{value} => {
        Ok(Value::Bool(*value))
      }
      // Return an error message.
      x => {
        unimplemented!();
      },
    }
  }

}