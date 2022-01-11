use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::{path::Path};
use anyhow::Result;
use anyhow::{bail};
use serde_json::Value;
use crate::MacroGroup;
use crate::errors::PackageLoadError;
use crate::pkg_inst::PackageInstance;
use crate::shader::Shader;

#[derive(Debug)]
pub struct Package {
    pub info:Arc<PackageInfo>,
    insts:HashMap<MacroGroup,Arc<PackageInstance>>
}

impl Package {
    pub fn load<P:AsRef<Path>>(path:P) -> Result<Package> {
        let info = PackageInfo::load(path)?;
        Ok(Package {
            info:Arc::new(info),
            insts:HashMap::default()
        })
    }

    pub fn get_inst(&mut self,group:&MacroGroup) -> Arc<PackageInstance> {
        if !self.insts.contains_key(group) {
            let inst = PackageInstance::create(group.clone(),self.info.clone());
            self.insts.insert(group.clone(), Arc::new(inst));
        }
        self.insts.get(group).unwrap().clone()
    }
}

#[derive(Debug,Default)]
pub struct PackageInfo {
    pub path:PathBuf,
    pub name:String,
    pub shaders:Vec<Arc<Shader>>
}

impl PackageInfo {
    pub fn load<P:AsRef<Path>>(path:P) -> Result<PackageInfo> {
        let package_json_path = path.as_ref().join("package.json");
        if !package_json_path.exists() {
            bail!(PackageLoadError::NotFoundPackageJson)
        }

        let json_string = std::fs::read_to_string(package_json_path)?;
        let json:Value = serde_json::from_str(&json_string)?;

        let pkg_name = json.get("name").and_then(Value::as_str).ok_or(PackageLoadError::JsonError("name"))?;
        let json_shaders = json.get("shaders").and_then(Value::as_array).ok_or(PackageLoadError::JsonError("shaders"))?;
        let mut shaders:Vec<Arc<Shader>> = vec![];
        for v in json_shaders {
            shaders.push(Arc::new(Shader::from_json(v)?));
        }
        Ok(PackageInfo {
            path:path.as_ref().to_path_buf(),
            name:pkg_name.to_string(),
            shaders
        })
    }
}



