
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    TypeVariable(TypeVariable),
    BoolType,
    IntType,
    FuncType(Box<FuncType>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncType {
    pub param_type: Box<Type>,
    pub return_type: Box<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeVariable {
    pub name: String
}

impl TypeVariable {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

pub struct TypeVariableGenerate {
    index: usize,
}

impl TypeVariableGenerate {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn next(&mut self) -> TypeVariable {
        let seq: Vec<char> = ('a'..='z').collect();
        let mut result = Vec::new();
        let mut n = self.index;

        loop {
            result.push(seq[n % seq.len()]);
            n /= seq.len();
            if n == 0 { break; }
        }

        self.index += 1;
        TypeVariable::new(format!("'{}", String::from_iter(result.into_iter())))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FinalType {
    BoolType,
    IntType,
    FuncType(Box<FinalFuncType>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct FinalFuncType {
    pub param_type: Box<FinalType>,
    pub return_type: Box<FinalType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeInference {
    pub left: Box<Type>,
    pub right: Box<Type>
}
impl TypeInference {
    pub fn unpack(self) -> (Box<Type>, Box<Type>) {
        (self.left, self.right)
    }
}