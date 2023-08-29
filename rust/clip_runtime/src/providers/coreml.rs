use ort::{execution_providers, ExecutionProvider};

pub fn providers() -> [ExecutionProvider; 1] {
  [ExecutionProvider::CoreML(
    execution_providers::CoreMLExecutionProviderOptions {
      enable_on_subgraph: true,
      use_cpu_only: false,
      only_enable_device_with_ane: true,
    },
  )]
}
