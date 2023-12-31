use std::fs::File;
fn main() {
    parse_sample().unwrap();
}

fn read_sample() -> anyhow::Result<()> {
    let file_path = "data/test.csv";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    // それぞれのレコード上をループする
    for result in rdr.records().skip(2) {
        let record = result?;
        println!("{:?}", record);
        println!("{:?}", &record[2]);
    }
    Ok(())
}

fn parse_sample() -> anyhow::Result<()> {
    let file_path = "data/test.csv";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    // それぞれのレコード上をループする
    for result in rdr.records().skip(2) {
        let record: csv::StringRecord = result?;
        let record_struct = Record_struct::parse(record)?;
        // println!("{:?}", record);
        println!("{:?}", record_struct);
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
}
