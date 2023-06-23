use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
fn main() -> anyhow::Result<()> {
    let dict = make_jis_dict()?;

    let file_path = "data/code_csv.csv";
    let csv_file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(csv_file);
    let output_path = "output/keycode_3.rs";
    let mut file = File::create(output_path).unwrap();
    // それぞれのレコード上をループする
    for result in rdr.records().skip(2) {
        let record: csv::StringRecord = result?;
        let record_struct = Record_struct::parse(record)?;

        match dict.get(&record_struct.usage_id) {
            Some(jis_key) => {
                let line = format!(
                    "// code {}: {}, JIS109: {}",
                    record_struct.usage_id, record_struct.usage_name, jis_key
                );
                writeln!(file, "{}", line).unwrap();
            }
            None => {
                let line = format!(
                    "// code {}: {}",
                    record_struct.usage_id, record_struct.usage_name
                );
                writeln!(file, "{}", line).unwrap();
            }
        }

        let line = format!(
            "// pub const : KeyMapping = KeyMapping::K({});",
            record_struct.usage_id
        );
        writeln!(file, "{}", line).unwrap();
    }
    Ok(())
}

fn make_jis_dict() -> anyhow::Result<HashMap<u8, String>> {
    let mut map = HashMap::new();
    let file_path = "data/jis.csv";
    let csv_file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(csv_file);
    for result in rdr.records() {
        let record: csv::StringRecord = result?;
        let hex = u8::from_str_radix(&record[5], 16)?;
        let jis_key = String::from(&record[1]);
        map.insert(hex, jis_key);
    }
    Ok(map)
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
}
