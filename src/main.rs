
mod wavConcatenation;
mod strToFonem;
mod wavMultConcatenation;
fn main() {
    // use std::io::{stdin,stdout,Write};
    // let mut s=String::new();
    // print!("Please enter some text: ");
    // let _=stdout().flush();
    // stdin().read_line(&mut s).expect("Did not enter a correct string");
    // if let Some('\n')=s.chars().next_back() {
    //     s.pop();
    // }
    // if let Some('\r')=s.chars().next_back() {
    //     s.pop();
    // }

    let mut base_str = String::from("hej jag anger torsdagens gemensam barock");
    let mut return_str = String::from("");
    let mut fonemfiles: Vec<String> = vec![];

    for word in base_str.split_whitespace() {
        return_str.push_str(strToFonem::run(word));
        return_str.push(' ');
    }
    // println!("{:?}",return_str);
    // for s in return_str.split('+') {
    //     println!("{}", s)
    // }
    for word in return_str.split_whitespace() {
        for fonem in word.split('+') {
            let mut filenam = String::from("fonem/");
            filenam.push_str(fonem);
            filenam.push_str(".wav");
            fonemfiles.push(filenam);
        }
        fonemfiles.push(String::from("fonem/silence500ms.wav"));
    }
    wavMultConcatenation::run(fonemfiles, "output/testis.wav");
}
// println!("{}",strToFonem::run("andas"));
 // concatenate_wavs("sine.wav", "sine1.wav", "test2.wav" );