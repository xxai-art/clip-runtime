use ort::{
  execution_providers::{CUDAExecutionProviderOptions, TensorRTExecutionProviderOptions},
  ExecutionProvider,
};

pub fn providers() -> [ExecutionProvider; 2] {
  // if let Ok(mem) = std::env::var("CUDA_MEM_LIMIT") {
  //   conf.gpu_mem_limit = mem;
  // }
  [
    ExecutionProvider::TensorRT(TensorRTExecutionProviderOptions::default()),
    ExecutionProvider::CUDA(CUDAExecutionProviderOptions::default()),
  ]
}
