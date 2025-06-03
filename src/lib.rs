mod config;
pub mod generic_names;
mod injector;
pub mod loader_utils;

pub use config::Config;
pub use injector::Injector;
use swc_core::ecma::{ast::Pass, visit::visit_mut_pass};

pub fn transform(cwd: &str, filepath: &str, config: Config) -> impl Pass {
    visit_mut_pass(Injector::new(cwd, filepath, config))
}
