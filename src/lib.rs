#![deny(clippy::all)]

use workspace_node_tools::monorepo::packages::{Monorepo, PackageInfo};

#[macro_use]
extern crate napi_derive;

#[napi(js_name = "getProjectRootPath")]
pub fn project_root_path() -> Option<String> {
  Monorepo::get_project_root_path()
}

#[napi(js_name = "getDefinedAgent")]
pub fn defined_agent() -> Option<String> {
  Some(Monorepo::get_agent().unwrap().to_string())
}

#[napi(js_name = "getMonorepoPackages")]
pub fn monorepo_packages() -> Vec<PackageInfo> {
    Monorepo::get_packages()
}