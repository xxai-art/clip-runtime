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

    fn softmax(vec: &[f32]) -> Vec<f32> {
        let max = vec.iter().fold(f32::MIN, |a, &b| a.max(b));
        let mut exps: Vec<f32> = vec.iter().map(|x| (x - max).exp()).collect();
        let sum: f32 = exps.iter().sum();
        exps.iter_mut().for_each(|x| *x /= sum);
        exps
    }

    use crate::ort::ClipOrt;
    #[test]
    fn test() -> clip_txt::Result<()> {
        let dir = std::env::current_dir()?;
        let ort = ClipOrt::new()?;
        let model = ort.model(dir.join("model/AltCLIP-XLMR-L-m18").display().to_string());
        let clip_txt = model.txt("onnx/Txt", 77)?;

        let word_li = [
            "a photo of chinese woman",
            "a photo of dog",
            "a photo of cat",
            "a photo of rat",
            "a photo of man",
            "a photo of woman",
        ];

        let txt_feature = clip_txt.encode_batch(word_li.into_iter())?;

        let clip_img = model.img("onnx/Img", 224, clip_img::CropCenter())?;
        let fp = dir.join("cat.jpg");

        let fp = fp.display().to_string();
        let img_feature = clip_img.encode(&std::fs::read(fp)?)?;

        let text_probs = softmax(&img_feature.dot(&txt_feature.t()).into_raw_vec());
        for (p, word) in text_probs.iter().zip(word_li.iter()) {
            println!("{} {:.2}%", word, (*p) * 100.0)
        }
        Ok(())
    }
}
