use crate::{ast::AST, Value};
use crate::ast::ASTKind;
use crate::value::Dict;
use indexmap::IndexMap;

impl From<AST> for Value {
    fn from(ast: AST) -> Self {
        Value::from(ast.kind)
    }
}


impl From<ASTKind> for Value {
    fn from(ast: ASTKind) -> Self {
        let mut builder = Scope::default();
        builder.build(ast)
    }
}


pub struct Scope {
    top: Value,
    pin_path: Vec<Vec<Value>>,
    key_path: Vec<Vec<Value>>,
}

impl<'a> Default for Scope {
    fn default() -> Self {
        Self {
            top: Value::from(Dict::default()),
            pin_path: vec![],
            key_path: vec![]
        }
    }
}


impl Scope {
    pub fn build(&mut self, ast: ASTKind ) -> Value {
        match ast {
            ASTKind::Program(v)|ASTKind::Dict(v) => {
                v.into_iter().for_each(|item|self.visit_ast(item.kind))
            }
            ASTKind::String(v) => {
                self.top = Value::from(*v)
            }
            _ => unimplemented!("ASTKind::{:?}", ast)
        }
        self.top.to_owned()
    }

    pub fn visit_ast(&mut self, ast: ASTKind) {
        match ast {
            ASTKind::Dict(v) => {
                for item in v {
                    self.visit_ast(item.kind);
                }
            }
                ASTKind::Pair(key, value) => {
                    self.push_key(key.kind);
                    self.visit_ast(value.kind);
                    self.pop_key();
                }
            ASTKind::String(v) => {
                *self.get_pointer() = Value::from(*v)
            }

            _ => unimplemented!("ASTKind::{:?}", ast)
        }
    }

    fn get_pointer(&mut self) -> &mut Value {
        let mut pointer = &mut self.top;
        for path in self.pin_path.iter().flatten().chain(self.key_path.iter().flatten()) {
            match path {
                Value::String(key) => {
                    pointer = pointer.ensure_key(key.as_str().to_string())
                }
                Value::Number(index) => {
                    println!("{:?}", index);
                    unimplemented!()
                }
                _ => unreachable!()
            }
        }
        return pointer
    }

    fn new_pin(&mut self, namespace: ASTKind) {
        let namespace = self.extract_namespace(namespace);
        self.key_path.push(namespace)
    }

    fn push_key(&mut self, namespace: ASTKind) {
        let namespace = self.extract_namespace(namespace);
        self.key_path.push(namespace)
    }

    fn pop_key(&mut self) -> Option<Vec<Value>> {
        self.key_path.pop()
    }

    fn extract_namespace(&self, namespace: ASTKind )->Vec<Value> {
        let mut out = vec![];
        match namespace {
            ASTKind::Namespace(ns) => {
                for item in ns {
                    match item.kind {
                        ASTKind::String(v) => {
                            out.push(Value::from(*v))
                        }
                        ASTKind::Number(v) => {  out.push(Value::from(*v))},
                        _ => unreachable!()
                    }

                }
            },
            _ => unreachable!()
        };
        return out
    }
}

impl Value {
    pub fn ensure_key(&mut self, key: String) -> &'_ mut Value {
        match self {
            Value::Null => {
                *self = Dict::empty();
                self
            }

            Value::Dict(dict) => {dict.ensure_key(key)}

            _ => unimplemented!("{:?}",self)
        }
    }
}

impl Dict  {
    pub fn ensure_key(&mut self, key: String) -> &'_ mut Value {
        self.entry(key).or_default()
    }
}