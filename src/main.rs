#[cfg(feature = "accelerate")]
extern crate accelerate_src;
#[cfg(feature = "mkl")]
extern crate intel_mkl_src;

use candle_core::{DType, Device, IndexOp, Tensor};
use clap::Parser;
use image::{DynamicImage, GenericImageView};
use std::path::PathBuf;

mod audio_io;

use image::ImageReader;
use std::io::Cursor;
use anyhow::anyhow;


#[derive(Parser)]
struct Args {
    #[arg(long)]
    image: String,

    #[arg(long)]
    model: String,

    #[arg(long, default_value = "output.png")]
    output: String,
}

fn load_image_to_tensor(path: &str) -> anyhow::Result<Tensor> {
    let img = image::open(path)?;
    let img = img.to_rgb8();
    let (width, height) = img.dimensions();

    let img_array: Vec<f32> = img
        .pixels()
        .flat_map(|pixel| pixel.0.iter().map(|c| *c as f32 / 255.0))
        .collect();

    let tensor = Tensor::from_vec(
        img_array,
        (height as usize, width as usize, 3),
        &candle_core::Device::Cpu,
    )?
    .permute((2, 0, 1))? // To shape [3, H, W]
    .unsqueeze(0)?// To shape [1, 3, H, W]
   // .to_dtype(DType::F16)?; 
   ;

    println!("image loaded tensor {:?}", tensor);
    Ok(tensor)
}

pub fn load_image<P: AsRef<std::path::Path>>(
    p: P,
    resize_longest: Option<usize>,
) -> anyhow::Result<(Tensor, usize, usize)> {
    let img = image::io::Reader::open(p)?
        .decode()
        .map_err(candle_core::Error::wrap)?;
    let (initial_h, initial_w) = (img.height() as usize, img.width() as usize);
    let img = match resize_longest {
        None => img,
        Some(resize_longest) => {
            let (height, width) = (img.height(), img.width());
            let resize_longest = resize_longest as u32;
            let (height, width) = if height < width {
                let h = (resize_longest * height) / width;
                (h, resize_longest)
            } else {
                let w = (resize_longest * width) / height;
                (resize_longest, w)
            };
            img.resize_exact(width, height, image::imageops::FilterType::CatmullRom)
        }
    };
    let (height, width) = (img.height() as usize, img.width() as usize);
    let img = img.to_rgb8();
    let data = img.into_raw();
    let data = Tensor::from_vec(data, (height, width, 3), &Device::Cpu)?.permute((2, 0, 1))?
    .to_dtype(DType::F32)?
    .unsqueeze(0)?;
    Ok((data, initial_h, initial_w))
}

fn save_tensor_to_image(tensor: Tensor, path: &str) -> anyhow::Result<()> {
    let tensor = tensor.squeeze(0)?.permute((1, 2, 0))?; // shape [H, W, 3]
    let shape = tensor.shape();
    println!("new shape of output image {:?}",shape);
    let (height, width, channels) = (shape.dims()[0], shape.dims()[1], shape.dims()[2]);

    let data = tensor.to_vec1::<f32>()?;
    let mut imgbuf = image::RgbImage::new(width as u32, height as u32);
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * channels;
            let pixel = image::Rgb([
                (data[idx] * 255.0).clamp(0.0, 255.0) as u8,
                (data[idx + 1] * 255.0).clamp(0.0, 255.0) as u8,
                (data[idx + 2] * 255.0).clamp(0.0, 255.0) as u8,
            ]);
            imgbuf.put_pixel(x as u32, y as u32, pixel);
        }
    }
    imgbuf.save(path)?;
    Ok(())
}

/// Saves an image to disk using the image crate, this expects an input with shape
/// (c, height, width).
pub fn save_image<P: AsRef<std::path::Path>>(img: &Tensor, p: P) -> anyhow::Result<()> {
    let p = p.as_ref();
    let (channel, height, width) = img.dims3()?;
    if channel != 3 {
        return Err(anyhow!("save_image expects an input of shape (3, height, width)"))
    }
    let img = img.permute((1, 2, 0))?.flatten_all()?.to_dtype(DType::U8)?;
    let pixels = img.to_vec1::<u8>()?;
    let image: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        match image::ImageBuffer::from_raw(width as u32, height as u32, pixels) {
            Some(image) => image,
            None =>  return Err(anyhow!("error saving image {p:?}")),
        };
    image.save(p).map_err(candle_core::Error::wrap)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Load and prepare image
    //let input_tensor = load_image_to_tensor(&args.image)?;
    let (input_tensor,h,w) = load_image(&args.image,None)?;
    //let input_tensor =load_image_with_std_mean(p, res, mean, std)
    println!("Input shape: {:?}", input_tensor.shape());

   // let res = save_tensor_to_image(input_tensor.i(0)?, "output.png");
   let res = save_image(&input_tensor.i(0)?, "output.png");
    println!("output {:?}",res);
    // Load ONNX model
    let model_path = PathBuf::from(&args.model);
    //4xNomos8kSCHAT-L.onnx
    //4xNomosWebPhoto_esrgan_fp32_opset17.onnx
    let model = candle_onnx::read_file(model_path)?;
    let graph = model.graph.as_ref().unwrap();
    println!("graph {:?}", graph.name);
    println!("graph.input[0] {:?}",graph.input[0]);

    // Prepare input map
    let mut inputs = std::collections::HashMap::new();
    inputs.insert(graph.input[0].name.to_string(), input_tensor);

    println!("inputs {:?}", inputs);
    // Run the model
    let mut outputs = candle_onnx::simple_eval(&model, inputs)?;

    let output = outputs.remove(&graph.output[0].name).unwrap();

    println!("Output shape: {:?}", output.shape());

    // Save output image
    save_tensor_to_image(output, &args.output)?;

    println!("Upscaled image saved to {}", args.output);
    Ok(())
}
