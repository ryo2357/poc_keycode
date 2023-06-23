use std::fs::File;
use std::io::Write;
fn main() -> anyhow::Result<()> {
    let output_path = "output/keycode_2.rs";
    let mut file = File::create(output_path).unwrap();
    // それぞれのレコード上をループする
    for i in 4..29 {
        let line = format!("pub const :KeyMapping = K({});", i);
        writeln!(file, "{}", line).unwrap();
    }
    Ok(())
}
