use level2::Builder;

#[allow(dead_code)]
#[derive(Debug, Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

fn main() {
    let command = Command::builder()
        .executable("find")
        .args(vec!["-c".into(), "-vvv".into()])
        .env(vec![])
        .current_dir("/home/changfeng/workspace/rs/rust-training/proc_macros/level2")
        .finish()
        .unwrap();
    println!("{:#?}", command);
}
