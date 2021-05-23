extern crate hound;
use hound::{SampleFormat, WavSpec, WavWriter};
pub fn run(filename1: &str, filename2: &str, filename: &str){
   

    let mut reader1 = hound::WavReader::open(filename1).unwrap();
    let mut reader2 = hound::WavReader::open(filename2).unwrap();
    let header =reader1.spec();

    // gör om reders till pekare på vektorer/wav filen
    let signal1 = reader1.samples::<i16>();
    let signal2 = reader2.samples::<i16>();
    
    let mut writer = WavWriter::create(filename, header).expect("Failed to created WAV writer for output file");
    for x in signal1.chain(signal2) {
        writer.write_sample(x.unwrap()).unwrap();
    }


    
}
// let header = WavSpec {
    //     channels: 1,
    //     sample_rate: 44100,
    //     bits_per_sample: 16,
    //     sample_format: SampleFormat::Int,
    // };
    // let header  = spec(filename1);

    // let num_samples1 = reader1.len() as usize;
    // let num_samples2 = reader2.len() as usize;

    // for n in 0..num_samples1 {
    //     writer.write_sample(signal1.next().unwrap().unwrap()).unwrap();
    // }
    // for n in 0..num_samples2 {
    //     writer.write_sample(signal2.next().unwrap().unwrap()).unwrap();
    // }


    // let mut reader = hound::WavReader::open("sine.wav").unwrap();
    // let mut signal: Vec<i16> = reader.samples().collect();
    //    let mut signal = reader.samples::<i16>().collect::<Vec<_>>();
    //    let mut signal2 = reader.samples::<i16>().collect::<Vec<_>>();
    //    signal.append(&mut signal2);
    
    
    // let mut writer = WavWriter::create("longBoy.wav", header).expect("Failed to created WAV writer");
    // for n in 0..num_samples {
    //     let x = signal[n];
    //     writer.write_sample(x).unwrap();
    // }

    // println!("{:?}",signal);