use std::env;
use std::fs;
use std::io;
use fontdue;
use std::collections::HashMap;
use std::ops::Add;


fn main() {
    let args: Vec<String> = env::args().collect();
    //let path = &args[1];
    //let path2 = &String::from("/Users/beppe/Git/asciiGenerator/fonts/Lucida Console.ttf");
    let data = include_bytes!("/Users/beppe/Git/asciiGenerator/fonts/Lucida Console.ttf") as &[u8];
    let font = fontdue::Font::from_bytes(data, fontdue::FontSettings::default()).expect("yessir");
    let mut output: HashMap<char,   [[u8]]> = HashMap::new(); //todo Second value should be bitmap type???
    let mut char_index:u32 = 0;
    //NOTE this starts at 0
    const RASTER_FACTOR: f32 = 15.0;
    while char_index < 1000000 {
        let char = std::char::from_u32(char_index).expect("yessiiiiiir");
        char_index += 1;
        let (metrics, mut unsorted_raster) = font.rasterize(char, RASTER_FACTOR);
        if (unsorted_raster.len() > 0) {


            (chunks, remainder) = unsorted_raster.as_chunks(RASTER_FACTOR as usize);
            output.insert(char, chunks.clone());
        }
    }
    //println!("{}", convertData(data));
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