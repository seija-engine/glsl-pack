use std::{collections::HashMap, path::{Path, PathBuf}};

use crate::{file::MacroFile, value::{Define, FuncDefine, ValueDefine}};

#[derive(Default,Debug)]
pub struct MacroContext {
    pub files:HashMap<PathBuf,MacroFile>,
    pub fn_defines:HashMap<String,FuncDefine>,
    pub defines:HashMap<String,ValueDefine>
}

impl MacroContext {
    pub fn new() -> Self {
        MacroContext::default()
    }

    pub fn add_define(&mut self,name:&str) {
        self.defines.insert(name.to_string(), ValueDefine::new(name.to_string(), None));
    }

    pub fn load_file<P:AsRef<Path>>(&mut self,path:P) {
        let read_string = std::fs::read_to_string(path.as_ref());
        match read_string {
            Ok(s) => {
                let c = path.as_ref().display().to_string();
                self.load_string(c, &s);
            },
            Err(err) => {
                log::error!("load file err:{:?}",err);
            }
        }
    }

    pub fn load_string(&mut self,name:String,code_string:&str) {
        match MacroFile::load(code_string) {
            Ok(file) => {
                self.files.insert(name.into(),file);
            },
            Err(err) => {
                log::error!("load file err name:{} err:{:?}",name,err);
            }
        }  
    }

    pub fn exp_all(&mut self) {
        loop {
            self.exp();
            if self.is_all_string() {
                return;
            }
        }
    }


    pub fn exp(&mut self) {
        let def_count = self.scan_define();
        if def_count > 0 {
            self.try_exp_if();
        } else {
            self.exp_if();
        }
    }

    fn scan_define(&mut self) -> usize {
        let mut vd_count = 0;
        for def in self.files.values_mut().map(|f| f.scan_define()).flatten() {
            match def {
                Define::ValueDefine(vd) => {
                    vd_count += 1;
                    self.defines.insert(vd.name.clone(), vd);
                },
                Define::FuncDefine(fd) => {
                    self.fn_defines.insert(fd.fn_name.clone(), fd);
                }
            }
       }
       vd_count
    }

    fn try_exp_if(&mut self) {
        for file in self.files.values_mut() {
            file.try_exp_if(&self.defines)
        }
    }

    fn exp_if(&mut self) {
        for file in self.files.values_mut() {
            file.exp_if(&self.defines)
        }
    }

    fn is_all_string(&self) -> bool {
        for expr in self.files.values() {
            if !expr.is_all_string() {
                return false;
            }
        }
        return true;
    }
}



#[test]
fn test_fn() {
    let mut context = MacroContext::new();
    context.load_file("tests/a.c");
    context.load_file("tests/b.c");
    context.exp_all();

    for (name,file) in context.files.iter() {
       let mut out_name:String = name.to_str().unwrap().to_string();
       out_name.push_str(".out.c");
       
       std::fs::write(out_name, file.to_string()).unwrap();
    }
}