use image::{DynamicImage, FilterType, GenericImageView, ImageFormat};
use rustler::{Atom, Binary, Encoder, Env, NifResult, NifStruct, OwnedBinary, ResourceArc, Term};
use std::io::Write as _;

use crate::atoms::{gif, jpg, ok, png, unsupported_image_format};

#[derive(NifStruct)]
#[module = "Mirage"]
pub struct Mirage {
    byte_size: usize,
    extension: Atom,
    height: u32,
    width: u32,
    resource: ResourceArc<Image>,
}

pub struct Image {
    image: DynamicImage,
    format: ImageFormat,
}

/// from_bytes(path: String) -> Result<Mirage>
#[rustler::nif(schedule = "DirtyCpu")]
pub fn from_bytes(binary: Binary) -> NifResult<Mirage> {
    match image::load_from_memory(binary.as_slice()) {
        Ok(image) => {
            if let Ok(format) = image::guess_format(&binary.as_slice()) {
                let mirage = Mirage {
                    byte_size: binary.len(),
                    extension: extension(format),
                    width: image.width(),
                    height: image.height(),
                    resource: ResourceArc::new(Image { image, format }),
                };

                return Ok(mirage);
            }
            return Err(rustler::Error::Atom("unsupported_image_format"));
        }
        Err(_) => Err(rustler::Error::BadArg),
    }
}

/// resize(resource: ResourceArc<Image>, width: u32, height: u32) -> Result<Vec<u8>>
#[rustler::nif(schedule = "DirtyCpu")]
pub fn resize(
    env: Env,
    resource: ResourceArc<Image>,
    width: u32,
    height: u32,
) -> NifResult<(Atom, Binary, Mirage)> {
    let resized = resource
        .image
        .resize_to_fill(width, height, FilterType::Triangle);
    let mut output = Vec::new();
    let mut binary = OwnedBinary::new(resized.raw_pixels().len()).unwrap();

    match resized.write_to(&mut output, resource.format) {
        Ok(_) => {
            binary
                .as_mut_slice()
                .write_all(&output)
                .map_err(|_| rustler::Error::Atom("io_error"))?;
            let extension = extension(resource.format);
            let bytes = binary.release(env);
            let byte_size = bytes.as_slice().len();

            let mirage = Mirage {
                byte_size,
                extension,
                height,
                width,
                resource,
            };

            Ok((ok(), bytes, mirage))
        }
        Err(_) => Err(rustler::Error::BadArg),
    }
}

fn extension(format: ImageFormat) -> Atom {
    match format {
        ImageFormat::PNG => png(),
        ImageFormat::JPEG => jpg(),
        ImageFormat::GIF => gif(),
        _ => unsupported_image_format(),
    }
}

pub fn load<'a>(env: Env, _info: Term<'a>) -> bool {
    rustler::resource!(Image, env);
    true
}
