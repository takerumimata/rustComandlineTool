use std::env;

fn main() {
    // コマンドラインから受け取った引数を動的配列にstringとして保存していく
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
