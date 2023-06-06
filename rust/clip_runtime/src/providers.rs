use ort::ExecutionProvider;

pub fn providers() -> [ExecutionProvider; 1] {
  [ExecutionProvider::coreml()]
}
