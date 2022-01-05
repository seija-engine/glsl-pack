#[derive(Debug)]
pub enum Expr {
    Define(Define),
   
    If(IfExpr),
    String(String),
    ExprList(Vec<Expr>)
}

#[derive(Debug)]
pub enum Define {
    ValueDefine(ValueDefine),
    FuncDefine(FuncDefine),
}

#[derive(Debug)]
pub struct ValueDefine {
    pub name:String,
    pub value:Option<String>
}

impl ValueDefine {
    pub fn new(name:String,value:Option<String>) -> Self {
        ValueDefine {name,value}
    }
}

#[derive(Debug)]
pub struct FuncDefine {
    pub fn_name:String,
    pub args:Vec<String>,
    pub body:String
}

impl FuncDefine {
    pub fn new(name:String,args:Vec<String>,body:String) -> Self {
        FuncDefine {fn_name:name,args,body }
    }
}

#[derive(Debug)]
pub struct IfExpr {
    pub name:String,
    pub body:Box<Expr>,
    pub else_ifs:Vec<ElseIf>,
    pub else_body:Vec<Expr>
}


impl IfExpr {
    pub fn new(name:String,body:Box<Expr>,else_ifs:Vec<ElseIf>,else_body:Vec<Expr>) -> IfExpr {
        IfExpr { name: name, body, else_body,else_ifs }
    }
}

#[derive(Debug)]
pub struct ElseIf {
    pub name:String,
    pub body:Vec<Expr>
}

impl ElseIf {
    pub fn new(name:String,body:Vec<Expr>) -> Self {
        ElseIf { name, body}
    }

}