use ort::ExecutionProvider;

pub fn providers() -> [ExecutionProvider; 2] {
  [ExecutionProvider::coreml(), ExecutionProvider::cpu()]
}
