use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::io::SeekFrom;
use std::path::Path;
mod tests;

#[derive(Debug, Clone, Default)]
pub struct Index {
    name: String,
    offset: u32,
    size: u32,
    dict_name: String,
}
pub fn format_oxford_gb(abc: String) -> String {
    let abc = abc.replace("*", "\n  *");
    let abc = abc.replace(" 1 ", "\n1. ");
    let abc = abc.replace(" 2 ", "\n2. ");
    let abc = abc.replace(" 3 ", "\n3. ");
    return abc;
}

impl Index {
    pub fn get_data(&self) -> String {
        let mut dict_file = File::open(&self.dict_name).unwrap();
        dict_file.seek(SeekFrom::Start(self.offset.into())).unwrap();
        let mut data = vec![0; self.size as usize];
        dict_file.read_exact(&mut data).unwrap();
        let res = String::from_utf8_lossy(&data);
        res.to_string()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Dict {
    dict: HashMap<String, Index>,
}
/// idx file
///     word_str: UTF-8 string , terminate by '\0'
///     word_data_offset: u32, word data offset in .dict file
///     word_data_size:   u32, word data size in .dict file
impl Dict {
    pub fn add_dict_file(&mut self, filepath: &str) {
        let mut content: Vec<u8> = vec![];
        let idx_path = format!("{}.idx", filepath);
        let dict_path = format!("{}.dict", filepath);
        let dz_path = format!("{}.dict.dz", filepath);
        if !Path::new(&dict_path).exists() {
            println!(".dict not exist: {}", dict_path);
            let dz_file = File::open(&dz_path).unwrap();
            let mut gz = GzDecoder::new(dz_file);
            let mut dict_file = File::create(&dict_path).unwrap();
            let _ = io::copy(&mut gz, &mut dict_file);
        }
        let mut f = File::open(&idx_path).unwrap();
        let _ = f.read_to_end(&mut content);
        let mut idx = 0;
        let mut it = content.iter();
        loop {
            let mut index = Index::default();
            index.dict_name = dict_path.clone();
            let ff = it.position(|&x| x == 0);
            if ff.is_none() {
                break;
            }
            let name = String::from_utf8_lossy(&content[idx..idx + ff.unwrap()]);
            idx += ff.unwrap() + 1;
            let data_offset = u32::from_be_bytes((&content[idx..idx + 4]).try_into().unwrap());
            let data_size = u32::from_be_bytes((&content[idx + 4..idx + 8]).try_into().unwrap());
            index.name = name.to_string();
            index.offset = data_offset;
            index.size = data_size;
            self.dict.insert(index.name.clone(), index.clone());
            for _ in 0..8 {
                it.next();
            }
            idx += 8;
        }

        println!("Total words in dict: {}", self.dict.len());
    }
    pub fn enter_lookup_loop(&self) {
        loop {
            let mut cmd = String::new();
            print!(">> ");
            let _ = io::stdout().flush();
            let _ = io::stdin().read_line(&mut cmd);
            cmd = cmd.trim().to_lowercase().to_string();
            let res = self.find_word(&cmd);
            println!("{}", res.unwrap_or("".to_string()));
            println!("==========");
        }
    }
    pub fn find_word(&self, input: &str) -> Option<String> {
        if let Some(index) = self.dict.get(input) {
            return Some(index.get_data());
        }
        None
    }
}
