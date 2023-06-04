load-dynamic：这不会链接到任何动态库，而是在运行时使用加载库 dlopen()。这可用于控制 ONNX 运行时二进制文件的路径（这意味着它们不必总是紧挨着您的可执行文件），并避免共享库地狱。

要使用它，请启用 load-dynamic 功能，并将 ORT_DYLIB_PATH 环境变量设置为您的 onnxruntime.dll/ libonnxruntime.so/ 的路径 libonnxruntime.dylib 您也可以使用相对路径，例如 ORT_DYLIB_PATH=./libonnxruntime.so（它将相对于可执行文件）。为方便起见，您应该下载或编译 ONNX Runtime 二进制文件，将它们放在永久位置，并永久设置环境变量。
