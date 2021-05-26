extern crate hound;
use hound::{WavWriter};
pub fn run(files_input: Vec<String>, file_name: &str){
    // println!("{:?}",files_input[0].as_str());
    let header =hound::WavReader::open(files_input[0].as_str()).unwrap().spec();
    let mut writer = WavWriter::create(file_name, header).expect("Failed to created WAV writer for output file");
    for file in files_input.iter() {
        let mut reader = hound::WavReader::open(file.as_str()).unwrap();
        let signal = reader.samples::<i16>();
        for x in signal {
            writer.write_sample(x.unwrap()).unwrap();
        }
    }
}
