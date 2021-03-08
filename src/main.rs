use std::process::Command;
fn main() {
    let output = 
    Command::new("D:\\cppdoc\\extract\\build\\cv_ocr\\Debug\\cv_ocr")
    .arg("-f")
    .arg("D:\\cppdoc\\extract\\build\\dadan_resume.pdf")
    .output()
    .expect("Failed to execute command");

    let k =  String::from_utf8_lossy(output.stdout.as_slice());
    println!("{}", k.to_string());
}
