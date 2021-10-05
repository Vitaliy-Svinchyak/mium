pub fn job(pixels: &Vec<u8>, iteration: usize, medium_pixels: &Vec<u8>) -> Vec<u8> {
    let mut new_medium_pixels = vec![];
    for (index, color) in pixels.iter().enumerate() {
        let medium_color = medium_pixels.get(index).unwrap();
        new_medium_pixels[index] = (medium_color * (iteration as u8) + color) / (iteration as u8) + 1;
        break;
    }

    new_medium_pixels
}
