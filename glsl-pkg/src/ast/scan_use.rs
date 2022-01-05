use std::{str::Chars, collections::VecDeque};

use super::{lex_string::LexString, UseInfo, SymbolName};

pub struct ScanUse<'a> {
    string:LexString<'a>,
    cache_list:VecDeque<char>,
}

impl<'a> ScanUse<'a> {
    pub fn new(code:&'a str) -> Self {
        ScanUse { string:LexString::new(code),cache_list:VecDeque::default() }
    }

    pub fn scan(&mut self) -> (Vec<UseInfo>,String) {
        let mut uses:Vec<UseInfo> = vec![];
        while let Some(use_info) = self.try_scan_use() {
            uses.push(use_info);
        }
        (uses,self.string.remain_string())
    }

    fn try_scan_use(&mut self) -> Option<UseInfo> {
        self.string.skip_whitespace();
        if !self.string.look_string("use") {
            return None;
        }
        self.string.skip_whitespace();
        let use_path = self.string.take_while(|chr| chr.is_alphanumeric() || chr == '.')?;
        let name = SymbolName::parse(use_path);

        self.string.skip_whitespace();
        if self.string.lookahead(1) == Some(';') {
            self.string.next();
        }
        Some(UseInfo { name })
    }

    

    
}