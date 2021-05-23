extern crate hound;
use hound::{SampleFormat, WavSpec, WavWriter};
pub fn run(filesInput: Vec<String>, filename: &str){
    println!("{:?}",filesInput[0].as_str());
    let header =hound::WavReader::open(filesInput[0].as_str()).unwrap().spec();
    let mut writer = WavWriter::create(filename, header).expect("Failed to created WAV writer for output file");
    for file in filesInput.iter() {
        let mut reader = hound::WavReader::open(file.as_str()).unwrap();
        let signal = reader.samples::<i16>();
        for x in signal {
            writer.write_sample(x.unwrap()).unwrap();
        }
    }
}
    // let mut reader1 = hound::WavReader::open(filesInput[0]).unwrap();
    // let mut reader2 = hound::WavReader::open(filesInput[1]).unwrap();

    // // gör om reders till pekare på vektorer/wav filen
    // let signal1 = reader1.samples::<i16>();
    // let signal2 = reader2.samples::<i16>();
    

    // for x in signal1.chain(signal2) {
    //     writer.write_sample(x.unwrap()).unwrap();
    // }