use ort::{execution_providers, ExecutionProvider};

pub fn providers() -> [ExecutionProvider; 1] {
  let openvino = {
    let mut openvino = execution_providers::OpenVINOExecutionProviderOptions::default();
    if let Ok(dir) = std::env::var("OPENVINO_CACHE") {
      openvino.cache_dir = Some(dir);
    }
    openvino
  };
  [ExecutionProvider::OpenVINO(openvino)]
}
