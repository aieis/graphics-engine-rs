use std::collections::HashMap;

use anyhow::{anyhow, Result};

pub enum Value {
    Entry(Entry),
    EntryArray(Vec<Entry>),

    Integer(i64),
    IntegerArray(Vec<i64>),

    Float(f64),
    FloatArray(String),

    String(String),
    StringArray(Vec<String>),
}

impl core::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Entry(_entry) => write!(f, "Value <Entry>"),
			Value::EntryArray(items) => write!(f, "Value <[EntryArray; {}]", items.len()),
			Value::Integer(i) => write!(f, "Value <Integer {}>", i),
			Value::IntegerArray(items) => write!(f, "Value <IntegerArray {:?}>", items),
			Value::Float(i) => write!(f, "Value <Float {}>", i),
			Value::FloatArray(items) => write!(f, "Value <FloatArray {:?}>", items),
			Value::String(i) => write!(f, "Value <String {}>", i),
			Value::StringArray(items) => write!(f, "Value <StringArray {:?}>", items),
		}
    }
}

pub struct Entry {
    pub children: HashMap<String, Box<Value>>
}


pub struct DataSection {
    pub header: String,
    pub lines: (usize, usize),

	pub values: HashMap<String, Value>
}

impl core::fmt::Display for DataSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Section '{}' ({} => {}) :: {:?}", &self.header, self.lines.0, self.lines.1, self.values)
    }
}

pub struct DataFile {
    pub version: i64,
    pub entries: Vec<Entry>
}

impl DataFile {
    pub fn load_from_string(file_content: &str) -> Result<Self, String> {

        let lines: Vec<&str> = file_content.split('\n').collect::<Vec<_>>();

		let mut start = 0;
		let mut sections = Vec::new();

		while let Some(section) = Self::parse_next_section(&lines, start) {
			start = section.lines.1;
			sections.push(section);
		}


		for section in sections {
			println!("{}", section);
		}

        Err("TODO".to_string())
    }

    pub fn parse_next_section(lines: &[&str], start: usize) -> Option<DataSection> {

        let section_start = match Self::find_next_section_start(lines, start) {
            Some(l) => l,
            None => { return None; }
        };

		let name = Self::parse_section_name(lines[section_start]);

        let section_end = match Self::find_next_section_start(lines, section_start+1) {
            Some(l) => l,
            None => lines.len()
        };

		let mut values = HashMap::new();

		for line in lines[section_start..section_end].iter() {
			let line = line.split('#').next().unwrap().trim();

			if let Some(i) = line.find('=') {

				let key = &line[..i].trim();
				let v   = &line[i+1..].trim();

				if key.len() > 0 {
					values.insert(key.to_string(), Value::String(v.to_string()));
				}

			}
		}

		Some (DataSection {header: name, lines: (section_start, section_end), values})

    }

    pub fn parse_section_name(line: &str) -> String {
        let name: &str = line.split(']').next().unwrap();

		name[1..name.len()].to_string()

    }

    pub fn find_next_section_start(lines: &[&str], start: usize) -> Option<usize> {

        for i in start..lines.len() {
            if lines[i].chars().nth(0) == Some('[') {
                return Some(i);
            }
        }

        None
    }

	pub fn parse_value(value: &str) -> Result<Value> {

		if value.len() == 0 {
			return Err(anyhow!("Value is empty"));
		}

		let c = value.chars().nth(0).unwrap();

		if c == '{' {
			// match a "literal" array


			return Err(anyhow!("TODO: Match arrays"));

		}

		// check if value is a string
		if c == '"' {

			if value.len() > 1 && value.chars().last().unwrap() == '"' {

				return Ok(Value::String(value[1..value.len() - 1].to_string()));

			}

			return Err(anyhow!("Failed to match string.\n"));
		}

		if value.contains('.') {
			// match float

			let f: f64 = value.parse()?;

			return Ok(Value::Float(f));
		}

		// matching int

		let i: i64 = value.parse()?;

		Ok(Value::Integer(i))
	}
}


#[cfg(test)]
mod test {
	use super::*;

    #[test]
	pub fn test_parse_of_str() {

		let data = "

[general]
version = 1

[data]

[data.0]
name = \"Data Entry 0\"

[data.1]
name = \"Data Entry 1\"


";

		let _ = DataFile::load_from_string(data);

		assert!(true)

	}

    #[test]
    pub fn test_load_of_strings() {
        assert!(true, "{:?}", "[test_1] # ignored data ".split('#').next());
    }

    #[test]
    pub fn test_load_of_strings_2() {
		let s = "1";
		println!("{}", &s[1..]);
    }

}
