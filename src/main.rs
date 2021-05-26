
// mod wav_concatenation;
mod str_to_fonem;
mod wav_mult_concatenation;
fn main() {

    let base_str = String::from("hej jag anger torsdagens gemensam barock");
    let output_fil_name: &str = "output/test_file_output.wav";
    let mut return_str = String::from("");
    let mut fonem_files: Vec<String> = vec![];

    for word in base_str.split_whitespace() {
        return_str.push_str(str_to_fonem::run(word));
        return_str.push(' ');
    }
    // println!("{:?}",return_str);
    // for s in return_str.split('+') {
    //     println!("{}", s)
    // }
    for word in return_str.split_whitespace() {
        for fonem in word.split('+') {
            let mut file_name = String::from("fonem/");
            file_name.push_str(fonem);
            file_name.push_str(".wav");
            fonem_files.push(file_name);
        }
        fonem_files.push(String::from("fonem/silence500ms.wav"));
    }
    wav_mult_concatenation::run(fonem_files, output_fil_name);
}
