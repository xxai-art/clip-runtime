pub mod img;
pub mod ort;
pub mod providers;
pub mod session;
pub mod txt;
pub mod typedef;
pub mod util;

#[cfg(test)]
mod test {
  #[test]
  fn init() {
    tracing_subscriber::fmt::init();
  }

  // #[test]
  // fn test_image_encode() -> Result<()> {
  //   use crate::ort::ClipOrt;
  //   let mut dir = std::env::current_dir()?;
  //   dir.push("model/AltCLIP-XLMR-L-m18");
  //
  //   let ort = ClipOrt::new()?;
  //   let model = ort.model(dir.display().to_string());
  //   let clip_img = model.img("onnx/Img", 224, clip_img::CropCenter())?;
  //
  //   let mut fp: std::path::PathBuf = std::env::current_dir()?;
  //   fp.push("cat.jpg");
  //   let fp = fp.display().to_string();
  //   let img = std::fs::read(fp)?;
  //   let out = clip_img.encode(&img)?;
  //   println!("img {}", out);
  //   Ok(())
  // }
  //
  use crate::ort::ClipOrt;
  #[test]
  fn test() -> clip_txt::Result<()> {
    let mut dir = std::env::current_dir()?;
    dir.push("model/AltCLIP-XLMR-L-m18");

    let ort = ClipOrt::new()?;

    let model = ort.model(dir.display().to_string());

    let clip_txt = model.txt("onnx/Txt", 77)?;

    let word_li = [
      "a photo for chinese woman",
      "a photo of dog",
      "a photo for cat",
    ];

    let txt = clip_txt.encode_batch(word_li.into_iter())?;
    let clip_img = model.img("onnx/Img", 224, clip_img::CropCenter())?;
    let mut fp: std::path::PathBuf = std::env::current_dir()?;
    fp.push("cat.jpg");
    let fp = fp.display().to_string();
    let img = clip_img.encode(&std::fs::read(fp)?)?;

    dbg!(txt.shape());
    dbg!(img.shape());
    // dbg!(txt * img.transpose());
    Ok(())
  }
}
