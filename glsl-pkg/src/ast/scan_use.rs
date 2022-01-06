use super::{lex_string::LexString, ast_package::PkgPath};

pub struct ScanUse<'a> {
    string:LexString<'a>
}

impl<'a> ScanUse<'a> {
    pub fn new(code:&'a str) -> Self {
        ScanUse { string:LexString::new(code) }
    }

    pub fn scan(&mut self) -> (Vec<PkgPath>,String) {
        let mut uses:Vec<PkgPath> = vec![];
        while let Some(use_info) = self.try_scan_use() {
            uses.push(use_info);
        }
        (uses,self.string.remain_string())
    }

    fn try_scan_use(&mut self) -> Option<PkgPath> {
        self.string.skip_whitespace();
        if !self.string.look_string("use") {
            return None;
        }
        self.string.skip_whitespace();
        let use_path = self.string.take_while(|chr| chr.is_alphanumeric() || chr == '.')?;
        let pkg_path = PkgPath::from_string(use_path);

        self.string.skip_whitespace();
        if self.string.lookahead(1) == Some(';') {
            self.string.next();
        }
        
        Some(pkg_path)
    }

    

    
}