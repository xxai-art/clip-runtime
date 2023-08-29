pub use avif_img::image::RgbImage;
use avif_img::{
  image::{self, imageops::crop_imm},
  load_image,
};
use ndarray::Array3;

pub fn resize(img: &RgbImage, wh: (u32, u32)) -> RgbImage {
  let (w, h) = wh;
  image::imageops::resize(img, w, h, image::imageops::FilterType::CatmullRom)
}

pub trait Croper {
  fn crop(&self, img: &RgbImage, wh: (u32, u32), dim: u32) -> RgbImage;
}

pub struct CropCenter();

impl Croper for CropCenter {
  fn crop(&self, img: &RgbImage, wh: (u32, u32), dim: u32) -> RgbImage {
    let left = (wh.0 - dim) / 2;
    let top = (wh.1 - dim) / 2;
    crop_imm(img, left, top, dim, dim).to_image()
  }
}

pub struct CropTop();

impl Croper for CropTop {
  fn crop(&self, img: &RgbImage, wh: (u32, u32), dim: u32) -> RgbImage {
    let left = (wh.0 - dim) / 2;
    let top = 0;
    crop_imm(img, left, top, dim, dim).to_image()
  }
}

pub fn processor(
  ext: Option<&str>,
  img: &[u8],
  dim: u32,
  croper: &impl Croper,
) -> anyhow::Result<(Array3<f32>, u32, u32)> {
  let img = load_image(ext, img)?.to_rgb8();
  let (w, h) = img.dimensions();
  let w_f64 = w as f64;
  let h_f64 = h as f64;
  let dim_f64 = dim as f64;

  #[allow(clippy::comparison_chain)]
  let wh = if w < h {
    (dim, (dim_f64 * h_f64 / w_f64) as u32)
  } else if w > h {
    ((dim_f64 * w_f64 / h_f64) as u32, dim)
  } else {
    (dim, dim)
  };

  let img = if wh.0 == dim && wh.1 == dim {
    img
  } else {
    resize(&img, wh)
  };

  let img = if wh.0 != wh.1 {
    croper.crop(&img, wh, dim)
  } else {
    img
  };

  let dim = dim as usize;
  let mut a = Array3::zeros((3, dim, dim));

  // Normalize the pixels.
  for i in 0..dim {
    for j in 0..dim {
      let p = img.get_pixel(j as u32, i as u32);
      a[[0, i, j]] = (p[0] as f32 / 255.0 - 0.48145466) / 0.26862954;
      a[[1, i, j]] = (p[1] as f32 / 255.0 - 0.4578275) / 0.261_302_6;
      a[[2, i, j]] = (p[2] as f32 / 255.0 - 0.40821073) / 0.275_777_1;
    }
  }

  Ok((a, w, h))
}

/*
Compose(
[
_blob2image,
Resize(n_px, interpolation=BICUBIC),
CenterCroper(n_px),
_convert_image_to_rgb,
ToTensor(),
Normalize(
(0.48145466, 0.4578275, 0.40821073),
(0.26862954, 0.26130258, 0.27577711),
),
]
)
*/
#[cfg(test)]
mod tests {
  use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
  };

  use anyhow::Result;
  use avif_img::image::{codecs::png::PngEncoder, ImageBuffer, ImageEncoder, Rgb};
  use ndarray::Array3;

  use crate::image::ColorType;

  fn to_png(mut img: Array3<f32>, path: &str) -> Result<()> {
    let (_, height, width) = img.dim();
    for i in 0..height {
      for j in 0..width {
        img[[0, i, j]] = (img[[0, i, j]] * 0.26862954 + 0.48145466) * 255.0;
        img[[1, i, j]] = (img[[1, i, j]] * 0.261_302_6 + 0.4578275) * 255.0;
        img[[2, i, j]] = (img[[2, i, j]] * 0.275_777_1 + 0.40821073) * 255.0;
      }
    }

    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width as u32, height as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
      let x = x as usize;
      let y = y as usize;
      let r = (img[[0, y, x]]) as u8;
      let g = (img[[1, y, x]]) as u8;
      let b = (img[[2, y, x]]) as u8;

      *pixel = Rgb([r, g, b]);
    }

    let fout = File::create(Path::new(path))?;
    let w = BufWriter::new(fout);

    let encoder = PngEncoder::new(w);

    let w = imgbuf.width();
    let h = imgbuf.height();
    encoder.write_image(imgbuf.into_raw().as_slice(), w, h, ColorType::Rgb8)?;

    Ok(())
  }

  fn json_to_narray(fp: &str) -> Result<Array3<f32>> {
    let file = File::open(fp)?;
    let reader = BufReader::new(file);

    // 读取 JSON 文件为一个 3D 的 Vec<f32>
    let v: Vec<Vec<Vec<f32>>> = serde_json::from_reader(reader)?;

    // 将 Vec<f32> 转为 Array3<f32>
    Ok(Array3::from_shape_vec(
      (v.len(), v[0].len(), v[0][0].len()),
      v.into_iter().flatten().flatten().collect(),
    )?)
  }

  #[test]
  fn test_image_processor() -> Result<()> {
    let fp = std::env::current_dir()?
      .join("cat.jpg")
      .display()
      .to_string();
    let img = std::fs::read(&fp)?;
    let dim = 224;
    let (img, ..) = crate::processor(Some("jpg"), &img, dim, &crate::CropCenter())?;
    to_png(img, &(fp.clone() + ".png"))?;
    let py_img = json_to_narray(&(fp.clone() + ".json"))?;
    to_png(py_img, &(fp + ".py.png"))?;
    Ok(())
  }
}
