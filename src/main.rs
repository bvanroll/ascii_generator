use std::env;
use std::fs;
use std::io;
use fontdue;
use std::collections::HashMap;
use std::ops::Add;
use std::fs::File;
use serde_json::to_string;


fn main() {
    let args: Vec<String> = env::args().collect();
    //let mut currentPath = env::current_dir().expect("what").as_path();
    let data = fs::read("./font/Lucida Console.ttf").expect("it didnt work");
    let font = fontdue::Font::from_bytes(data, fontdue::FontSettings::default()).expect("yessir");
    let mut font_data: HashMap<char, Vec<Vec<u8>>> = HashMap::new(); //todo Second value should be bitmap type???
    let mut char_index:u32 = 0;
    //NOTE this starts at 0
    const RASTER_FACTOR: f32 = 15.0;
    while char_index < 10000 {
        let char = std::char::from_u32(char_index).expect("error converting u32 to char");
        let (metrics, mut unsorted_raster) = font.rasterize(char, RASTER_FACTOR);
        if (unsorted_raster.len() > 0) {
            let sorted = rasterize(unsorted_raster, RASTER_FACTOR as usize);
            font_data.insert(char, sorted);

            //(chunks, remainder) = unsorted_raster.as_chunks(RASTER_FACTOR as usize);
            //output.insert(char, chunks.clone());
        }
        char_index += 1;
    }

    print_json(&mut font_data);
    //INFO the data in fontdata is a grid as an array with each value being a number from 1 to 100, not a float
    //TODO load in image and convert to greyscale
    //TODO create image string
    //TODO loop through chunks of image adding a new line at the end of each line
    //TODO compare each chunk to the font_data to find its most representative character and add to the image string

}

fn print_json(output: &mut HashMap<char, Vec<Vec<u8>>>) {
    let json = serde_json::to_string(&output).unwrap();

    print!("{}", json);
}


fn rasterize(mut unsorted: Vec<u8>, chunk_size: usize) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    while unsorted.len() > chunk_size {
        let mut temp = unsorted.clone();
        unsorted = temp.split_off(chunk_size);
        result.push(temp.clone());
    }
    return result;
}

//returns remainder, chunk
fn rasterize_chunk_recursive(mut source: Vec<u8>, mut chunk_size: usize) -> Vec<u8> {
    if (source.len() < chunk_size) {
        let mut chunk: Vec<u8> = Vec::with_capacity(chunk_size);
        chunk = source.split_off(chunk_size);
        return chunk;
    } else {
        let remainder: Vec<u8> = Vec::with_capacity(0);
        let chunk = source.clone();
        return chunk;

    }
}