use std::{str::Chars, collections::{VecDeque, HashMap}};
use crate::{value::{Expr, ValueDefine, FuncDefine, IfExpr, ElseIf, Define}, errors::ParseError};

const IF_NAME:&'static str = "ifdef";
const ENDIF_NAME:&'static str = "endif";
const DEFINE_NAME:&'static str = "define";
const ELIF_NAME:&'static str = "elif";
const ELSE_NAME:&'static str = "else";

#[derive(Debug)]
pub struct MacroFile {
    exprs:Vec<Expr>
}

impl MacroFile {
    pub fn load(string:&str) -> Result<MacroFile,ParseError> {
        MacroFileParser::new(string).parse()
    }

    pub fn scan_define(&mut self) -> Vec<Define> {
        let mut out_define_lst:Vec<Define> = vec![];
        let mut new_exprs:Vec<Expr> = vec![];
        for expr in self.exprs.drain(0..) {
            if let Some(e) = Self::scan_define_expr(expr,&mut out_define_lst) {
                new_exprs.push(e);
            }
        }
        self.exprs = new_exprs;
        out_define_lst
    }

    fn scan_define_expr(expr:Expr,out_defs:&mut Vec<Define>) -> Option<Expr> {
        match expr {
            Expr::Define(df) => {
                out_defs.push(df);
                None
            },
            Expr::ExprList(mut lst) => {
                let mut new_exprs:Vec<Expr> = vec![];
                for expr in lst.drain(0..) {
                    if let Some(e) = Self::scan_define_expr(expr,out_defs) {
                        new_exprs.push(e);
                    }
                }
                Some(Expr::ExprList(new_exprs)) 
            }
            _ => Some(expr)
        }
    }

    pub fn try_exp_if(&mut self,defs:&HashMap<String,ValueDefine>) {
        self.exprs = self.exprs.drain(0..).map(|e| Self::try_exp_if_expr(e, defs)).filter_map(|f| f).collect();
    }

    pub fn exp_if(&mut self,defs:&HashMap<String,ValueDefine>) {
        self.exprs = self.exprs.drain(0..).map(|e| Self::exp_if_expr(e, defs)).filter_map(|f| f).collect();
    }

    fn try_exp_if_expr(expr:Expr,defs:&HashMap<String,ValueDefine>) -> Option<Expr> {
        match expr {
            Expr::If(if_expr) => {
                if defs.contains_key(&if_expr.name) {
                    Self::_exp_if_expr(if_expr, defs)
                } else {
                    Some(Expr::If(if_expr))
                }
            },
            Expr::ExprList(mut lst) => {
               let new_lst:Vec<Expr> = lst.drain(0..).map(|e| Self::try_exp_if_expr(e, defs)).filter_map(|f| f).collect();
               if new_lst.len() > 0 { Some(Expr::ExprList(new_lst)) } else { None }
            },
            _ => Some(expr)
        }
    }

    fn exp_if_expr(expr:Expr,defs:&HashMap<String,ValueDefine>) -> Option<Expr> {
        match expr {
            Expr::If(if_expr) => Self::_exp_if_expr(if_expr, defs),
            Expr::ExprList(mut lst) => {
               let new_lst:Vec<Expr> = lst.drain(0..).map(|e| Self::exp_if_expr(e, defs)).filter_map(|f| f).collect();
               if new_lst.len() > 0 { Some(Expr::ExprList(new_lst)) } else { None }
            }
            _ => Some(expr)
        }
    }

    fn _exp_if_expr(expr:IfExpr,defs:&HashMap<String,ValueDefine>) -> Option<Expr>  {
        if defs.contains_key(&expr.name) {
            return Some(*expr.body);
        }
        
        for elif in expr.else_ifs {
            if defs.contains_key(&elif.name) {
                return Some(Expr::ExprList(elif.body));
            }
        }
        if expr.else_body.len() > 0 {
           return Some(Expr::ExprList(expr.else_body))
        }
        None
    }

    pub fn is_all_string(&self) -> bool {
        self._is_all_string(&self.exprs)
    }

    fn _is_all_string(&self,lst:&Vec<Expr>) -> bool {
        for expr in lst.iter() {
            match expr {
                Expr::String(_) => {},
                Expr::ExprList(elst) => {
                    if !self._is_all_string(elst) { return false };
                }
                _ => return  false
            }
        }
        true
    }

    pub fn to_string(&self) -> String {
        let mut out_string = String::default();
        for expr in self.exprs.iter() {
            Self::_take_to_string(expr, &mut out_string)
        }
        out_string
    }

    fn _take_to_string(expr:&Expr,out_string:&mut String) {
        match expr {
            Expr::String(s) => {
                out_string.push_str(s.as_str());
            },
            Expr::ExprList(lst) => {
                lst.iter().for_each(|e| Self::_take_to_string(e, out_string));
            }
            _ => {}
        }
    }
}



pub struct MacroFileParser<'a> {
    chars:Chars<'a>,
    cache_list:VecDeque<char>,
}

impl<'a> MacroFileParser<'a> {
    pub fn new(code:&'a str) -> Self {
        MacroFileParser { 
            chars: code.chars(),
            cache_list:VecDeque::default() 
        }
    }

    pub fn parse(&mut self) -> Result<MacroFile,ParseError> {
        let mut exprs:Vec<Expr> = vec![];
        while let Some(expr) = self.parse_expr()? {
            exprs.push(expr);
        }
        Ok(MacroFile {exprs})
    }

    pub fn parse_expr(&mut self) -> Result<Option<Expr>,ParseError> {
        let chr = self.lookahead(1);
        match chr {
            Some('#') => Ok(Some(self.parse_macro()?)),
            Some(_) => Ok(Some(self.parse_string()?)),
            None => { Ok(None) }
        }
    }

    fn parse_macro(&mut self) -> Result<Expr,ParseError> {
        let macro_name = self.take_macro_name();
        match macro_name {
            Some("define") => self.parse_define(),
            Some("ifdef") => self.parse_if(),
            s => {
                let mut err_macro = self.take_count(10);
                err_macro.insert(0, '#');
                Err(ParseError::ErrMacro(err_macro))
            }
        }
    }

    fn take_macro_name(&mut self) -> Option<&'static str> {
        if self.lookahead(1) == Some('#') {
            self.next();
        } else {
            return None;
        }
        if self.look_string(DEFINE_NAME) {
            return Some(DEFINE_NAME)
         } else if self.look_string(IF_NAME) {
            return Some(IF_NAME);
         } else if self.look_string(ENDIF_NAME) {
            return Some(ENDIF_NAME);
         } else if self.look_string(ELIF_NAME) {
            return Some(ELIF_NAME);
         } else if self.look_string(ELSE_NAME) {
             return Some(ELSE_NAME);
         }
        None
    }
    
    fn parse_define(&mut self) -> Result<Expr,ParseError> {        
        self.skip_white();
        let (name,args) = self.take_define_name();
        self.skip_while(|chr| chr == ' ' || chr == '\r');

        if let Some(chr) = self.lookahead(1) {
            if chr == '\n' {
                if args.is_none() {
                    let value_def = ValueDefine::new(name,None);
                    return Ok(Expr::Define(Define::ValueDefine(value_def)));
                } else {
                    return Err(ParseError::MissFuncBody(name));
                }
            }
        }

        let mut value = self.take_while(|c| c != '\n');
        value = value.trim().to_string();
        if let Some(args) = args {
            Ok(
                Expr::Define(Define::FuncDefine(FuncDefine::new(name, args, value))) 
            ) 
        } else {
            let v = if value.is_empty() { None } else { Some(value) };
            Ok(
                Expr::Define(Define::ValueDefine(ValueDefine::new(name,v)))
            )
        }
    }

    fn take_define_name(&mut self) -> (String,Option<Vec<String>>) {
        let name = self.take_while(is_macro_name_chr);
        let mut args:Option<Vec<String>> = None;
        self.skip_while(|c| c == ' ');
        if let Some(chr) = self.lookahead(1) {
            if chr == '(' {
                let take_args = self.take_args();
                if take_args.len() > 0 {
                    args = Some(take_args);
                }
            }    
        }
        (name,args)
    }

    fn take_args(&mut self) -> Vec<String> {
        let mut args:Vec<String> = vec![];
        self.next();
        self.skip_white();
        while let Some(chr) = self.lookahead(1) {
            self.skip_white();
            if chr == ')' { self.next(); break; }
            if chr == ',' {self.next(); continue; }
            let arg = self.take_while(is_macro_name_chr);
            args.push(arg);
        }
        return args;
    }

    fn parse_string(&mut self) -> Result<Expr,ParseError> {
        let mut string = String::default();
        while let Some(chr) = self.lookahead(1) {
            if chr == '#' {
                break;
            } else {
                string.push(chr);
                self.next();
            }
            
        }
        Ok(Expr::String(string))
    }

    fn take_count(&mut self,count:usize) -> String {
        let mut ret = String::default();
        for idx in 1..count {
          if let Some(chr) = self.lookahead(idx) {
              ret.push(chr);
          } else {
              return ret;
          }
        }
        ret
    }

    fn look_string(&mut self,str:&str) -> bool {
        let mut index = 1;
        for chr in str.chars() {
            if let Some(lchr) = self.lookahead(index) {
                if lchr != chr {
                    return false;
                }
                index += 1;
            } else {
                return false;
            }
        }
        str.chars().for_each(|_| { self.next();} );
        true
    }

    fn next(&mut self) -> Option<char> {
        if self.cache_list.len() > 0 {
            return self.cache_list.pop_front()
        }
        self.chars.next()
    }

    fn skip_while(&mut self,f:fn(char) -> bool) {
        while let Some(chr) = self.lookahead(1) {
            if f(chr) {
                self.next();
            } else {
                break;
            }
        }
    }

    fn take_while(&mut self,f:fn(char) -> bool) -> String {
        let mut ret = String::default();
        while let Some(chr) = self.lookahead(1) {
            if f(chr) {
                ret.push(chr);
                self.next();
            } else {
                break;
            }
        }
        ret
    }

    fn skip_white(&mut self) {
        self.skip_while(|c| c.is_whitespace())
    }

    fn lookahead(&mut self,count:usize) -> Option<char> {
        let sub_count:i32 = count as i32 - self.cache_list.len() as i32;
        if sub_count > 0 {
            for _ in 0..sub_count {
                if let Some(chr) = self.chars.next() {
                    self.cache_list.push_back(chr);
                } else {
                    return None;
                }
            }   
        }
        return Some(self.cache_list[count - 1]);
    }

    fn push_back_str(&mut self,str:&str) {
        for c in str.chars() {
            self.cache_list.push_back(c);
        }
    }
    
    fn push_back(&mut self,chr:char) {
        self.cache_list.push_back(chr);
    }

    fn parse_if(&mut self) -> Result<Expr,ParseError> {
        self.skip_while(|c| c == ' ');
        let name = self.take_while(is_macro_name_chr);
        let mut body_exprs:Vec<Expr> = vec![];
        let mut else_ifs:Vec<ElseIf> = vec![];
        let mut else_exprs:Vec<Expr> = vec![];
        let mut push_list = &mut body_exprs;
        loop {
            let chr = self.lookahead(1).ok_or(ParseError::ErrIf)?;
            if chr == '#' {
                let macro_name = self.take_macro_name().ok_or(ParseError::ErrIf)?;
                match macro_name {
                    "endif" => { break; }
                    "elif" => {
                        self.skip_while(|c| c == ' ');
                        let name = self.take_while(is_macro_name_chr);
                        let else_if = ElseIf::new(name, vec![]);
                        else_ifs.push(else_if);
                        push_list = &mut else_ifs.last_mut().unwrap().body;
                    }
                    "else" => {
                        push_list = &mut else_exprs;   
                    }
                    _ => { self.push_back('#'); self.push_back_str(macro_name); }
                }
            }
            let expr = self.parse_expr()?.ok_or(ParseError::ErrIf)?;
            push_list.push(expr);
        }

       
        let body_expr = if body_exprs.len() > 1 {
            Expr::ExprList(body_exprs)
        } else {
            body_exprs.remove(0)
        };
        let if_expr = IfExpr::new(name, Box::new(body_expr),else_ifs, else_exprs);
        Ok(Expr::If(if_expr))
    }
}

fn is_macro_name_chr(chr:char) -> bool {
    chr.is_alphanumeric() || chr == '_'
 }

#[test]
fn test_string() {
    let string = r#"
        #ifdef FUCK
          #define RNM 123
        #elif NMB
          IN NMB
          #ifdef HH
            QQQQ
            #define PI 3.1415926
          #endif 
        #else
          FUCK_ELSE
          #define fkinline(a,b) (a + b);
        #endif
    "#;

    let file = MacroFile::load(string);
    dbg!(file.unwrap());
    
}