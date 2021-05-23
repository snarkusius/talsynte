
mod wavConcatenation;
mod strToFonem;
mod wavMultConcatenation;
fn main() {
    let mut base_str = String::from("barock andas av bet");
    let mut return_str = String::from("");
    let mut numbers: Vec<&str> = vec![];
    // println!("{}",strToFonem::run("andas"));
    for word in base_str.split_whitespace() {
        return_str.push_str(strToFonem::run(word));
        return_str.push(' ');
    }
    println!("{:?}",return_str);
    for s in return_str.split('+') {
        println!("{}", s)
    }
    for s in return_str.split('+') {
        println!("{}", s)
    }
    // return_str.split_whitespace()
    // concatenate_wavs("sine.wav", "sine1.wav", "test2.wav" );
    wavMultConcatenation::run(vec!["fonem/aaa.wav", "fonem/p.wav","fonem/a_.wav"], "apa.wav");

}