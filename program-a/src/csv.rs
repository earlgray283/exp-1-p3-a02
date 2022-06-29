use anyhow::{anyhow, Result};
use std::marker;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

pub trait FromCsvLine: marker::Sized {
    fn from_str(s: &str) -> Result<Self>;
}

const TAG_CAPACITY: usize = 22_810_397;

pub async fn load_csv<T: FromCsvLine>(path: &str) -> Result<Vec<T>> {
    let csv_file = File::open(path).await?;
    let mut csv_reader = BufReader::new(csv_file);
    let mut list = Vec::with_capacity(TAG_CAPACITY);
    loop {
        let mut buf = String::with_capacity(300);
        let n = csv_reader.read_line(&mut buf).await?;
        if n == 0 {
            break;
        }
        list.push(T::from_str(buf.as_str()).map_err(|_| anyhow!("failed to parse {}", &buf))?);
    }
    println!("{}", list.len());
    Ok(list)
}
