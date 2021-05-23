
mod wavConcatenation;
mod strToFonem;
mod wavMultConcatenation;
fn main() {
    let mut base_str = String::from("a b c b d e f g h i j k l m n o p");
    let mut return_str = String::from("");
    let mut fonemfiles: Vec<String> = vec![];
    // println!("{}",strToFonem::run("andas"));
    for word in base_str.split_whitespace() {
        return_str.push_str(strToFonem::run(word));
        return_str.push(' ');
    }
    println!("{:?}",return_str);
    for s in return_str.split('+') {
        println!("{}", s)
    }
    for word in return_str.split_whitespace() {
        for fonem in word.split('+') {
            let mut filenam = String::from("fonem/");
            filenam.push_str(fonem);
            filenam.push_str(".wav");
            fonemfiles.push(filenam);
        }
        fonemfiles.push(String::from("fonem/silence500ms.wav"));
    }
    
    // concatenate_wavs("sine.wav", "sine1.wav", "test2.wav" );
    wavMultConcatenation::run(fonemfiles, "testis.wav");

}