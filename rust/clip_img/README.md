# clip_img

[transformers.CLIPImageProcessor](https://huggingface.co/docs/transformers/model_doc/clip#transformers.CLIPImageProcessor) 的 rust 实现。用于将图片转为向量（一般是 224x224），然后传给 clip。

[cat.jpg.json](./cat.jpg.json) 是 python 导出的 CLIPImageProcessor 生成的图片数组 ([导出代码](https://github.com/xxaier/FlagAI/blob/6044594b40dfbaf6c8957f3ef04d751b6883de2b/onnx/test/onnx/onnx_img.py#L25))。

`cargo test -p clip_img` 会生成数值，并导出到 `cat.jpg.png`

最后，我看 2 张图效果一样，如下图。

![](https://pub-b8db533c86124200a9d799bf3ba88099.r2.dev/2023/06/5uwpKe_.webp)

本来打算照抄 [anansi 的 embedds/src/embedd/image_processor.rs](https://github.com/infrawhispers/anansi/blob/883fd5d1d8c2aa16289d028e6bead2ac37bc39af/embedds/src/embedd/image_processor.rs#L69)。

但是发现它的代码又啰嗦又烂（居然中间还把图片压了一遍 jpg），而且还引入了 libvips 这个 C 库的依赖造成编译很麻烦。

于是果断决定全部自己重写，重写的过程中参考了 [clip-as-service](https://github.com/jina-ai/clip-as-service/blob/679de4e3c9cb02b712f58540f6a3dd2e32d8e5e9/server/clip_server/model/clip.py#L29) 的逻辑。

这里吐槽一下搞机器学习的人们，居然都是 CenterCrop，居中剪裁图片为正方形。

这样的话，更加我以前在互联网公司工作的经验，按照手机拍的照片高宽比，居中剪裁，很容易就把人脸裁掉一半。

基于这种 clip 去做搜索，肯定会效果不如意。

所以我定义的 processor 函数接口如下，剪裁函数是可以传入的，我先按居中剪裁来实现，这样方便评测 rust 的实现是否和 python 版的一致。

```
pub fn processor(
  crop: impl Fn(&RgbImage, (u32, u32), u32) -> RgbImage,
  dim: u32,
  img: &RgbImage,
) -> Array3<f32>
```

我个人建议加个人脸识别再去剪裁。

以后再搞人脸 / 主体识别+剪裁吧。

不多写了，空谈误国，实干兴邦 (talk is cheap， show you the code)。

代码见 : [./src/lib.rs](./src/lib.rs)。

最后，感谢 chatgpt，它写了[主要代码](https://chat.openai.com/share/26a66855-18f1-436a-a6c4-b75db7858a23) 和 [测试代码](https://chat.openai.com/share/300c0212-4b15-474a-aa7e-3590b10db04c)，我只是负责调试、改 bug 和 code review。
