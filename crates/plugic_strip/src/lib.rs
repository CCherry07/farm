use farmfe_core::{
  config::Config,
  error::CompilationError,
  module::ModuleType,
  plugin::{Plugin, PluginLoadHookResult},
  serde_json,
};
use farmfe_toolkit::{anyhow::Ok, fs};

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

pub struct FarmPluginJson {}

impl FarmPluginJson {
  pub fn new(_: &Config) -> Self {
    Self {}
  }
}

impl Plugin for FarmPluginJson {
  fn name(&self) -> &str {
    "FarmPluginStrip"
  }

  fn load(
    &self,
    _param: &farmfe_core::plugin::PluginLoadHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    Ok(None)
  }

  fn transform(
    &self,
    param: &farmfe_core::plugin::PluginTransformHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginTransformHookResult>> {
    Ok(None)
  }
}
