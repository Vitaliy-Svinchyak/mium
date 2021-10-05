use jpeg::ImageInfo;

pub async fn job(bytes: Vec<u8>) -> (Vec<u8>, ImageInfo) {
    let reader = bytes.as_slice();
    let mut decoder = jpeg::Decoder::new(reader);
    let pixels = decoder.decode().expect("failed to decode image");
    let metadata = decoder.info().unwrap();

    (pixels, metadata)
}
