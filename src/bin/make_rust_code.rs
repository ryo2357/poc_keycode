use std::fs::File;
use std::io::Write;
fn main() -> anyhow::Result<()> {
    let file_path = "data/code_csv.csv";
    let csv_file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(csv_file);
    let output_path = "output/keycode.rs";
    let mut file = File::create(output_path).unwrap();
    // それぞれのレコード上をループする
    for result in rdr.records().skip(2) {
        let record: csv::StringRecord = result?;
        let record_struct = Record_struct::parse(record)?;
        // println!("{:?}", record);
        // println!("{:?}", record_struct.make_line());
        writeln!(file, "{}", record_struct.make_line()).unwrap();
    }
    Ok(())
}

#[derive(Debug)]
struct Record_struct {
    usage_id: u8,
    usage_name: String,
    key_name: String,
}
impl Record_struct {
    fn parse(record: csv::StringRecord) -> anyhow::Result<Self> {
        let usage_id = u8::from_str_radix(&record[0], 10)?;
        let usage_name = String::from(&record[2]);
        let key_name = String::from(&record[6]);
        Ok(Self {
            usage_id,
            usage_name,
            key_name,
        })
    }

    fn make_line(&self) -> String {
        let line = format!(
            "pub const KEY_{}: u8 = {}; // {}",
            self.key_name, self.usage_id, self.usage_name
        );
        line
    }
}
