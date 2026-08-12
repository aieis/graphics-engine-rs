use std::collections::HashMap;

use anyhow::{anyhow, Result};

pub enum Value {
    Entry(Entry),
    // EntryArray(Vec<Entry>),

    Integer(i64),
    // IntegerArray(Vec<i64>),

    Float(f64),
    // FloatArray(String),

    String(String),
    // StringArray(Vec<String>),

    List(Vec<Value>)
}

impl core::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Entry(_entry) => write!(f, "Value <Entry>"),
			// Value::EntryArray(items) => write!(f, "Value <[EntryArray; {}]", items.len()),
			Value::Integer(i) => write!(f, "Value <Integer {}>", i),
			// Value::IntegerArray(items) => write!(f, "Value <IntegerArray {:?}>", items),
			Value::Float(i) => write!(f, "Value <Float {}>", i),
			// Value::FloatArray(items) => write!(f, "Value <FloatArray {:?}>", items),
			Value::String(i) => write!(f, "Value <String \"{}\">", i),
			// Value::StringArray(items) => write!(f, "Value <StringArray {:?}>", items),

            Value::List(i) => write!(f, "Value <List {:?}>", i)
		}
    }
}

pub struct Entry {
    pub children: HashMap<String, Box<Value>>
}


pub struct DataSection {
    pub header: String,
    pub name: String,
    pub lines: (usize, usize),

	pub values: HashMap<String, Value>,
    pub children: HashMap<String, DataSection>,
    pub list_children: HashMap<String, Vec<DataSection>>

}

impl core::fmt::Debug for DataSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("DataSection");

        s.field("name", &self.name);

        for val in self.values.iter() {
            s.field(&val.0, val.1);
        }

        for child in self.children.iter() {
            s.field(&child.0, child.1);
        }

        for child in self.list_children.iter() {
            s.field(&child.0, child.1);
            
        }

        s.finish()
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
			println!("{:#?}", section);
		}

        Err("TODO".to_string())
    }

    pub fn parse_next_section(lines: &[&str], start: usize) -> Option<DataSection> {

        let section_start = match Self::find_next_section_start(lines, start) {
            Some(l) => l,
            None => { return None; }
        };

		let header = Self::parse_section_name(lines[section_start]);
        let name   = header.split('.').last().unwrap().to_string();

        let mut next_section_start = Self::find_next_section_start(lines, section_start+1);
        let mut section_end = match next_section_start {
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
                    if let Ok(v) = Self::parse_value(v) {
					    values.insert(key.to_string(), v);
                    }
				}
			}
		}

        let mut children = HashMap::new();
        let mut list_children: HashMap<String, Vec<DataSection>> = HashMap::new();

        let header_as_parent = format!("{}.", &header);

        while let Some(l) = next_section_start {

            let next_section_name = Self::parse_section_name(lines[l]);

            if !next_section_name.starts_with(&header_as_parent) {
                break;
            }

            let data_section = Self::parse_next_section(lines, l);

            if data_section.is_none() {
                break;
            }

            let data_section = data_section.unwrap();
            let header_parts = data_section.header.split('.').collect::<Vec<_>>();

            section_end  = data_section.lines.1;

            if let Ok(_) = data_section.name.parse::<i64>() {
                // TODO: This is an array. Should check if I have any values

                if header_parts.len() < 2 {
                    return None;
                }

                let key = header_parts[header_parts.len() - 2].to_string();

                if list_children.contains_key(&key) {
                    for v in list_children.iter_mut() {
                        if key.eq(v.0) {
                            v.1.push(data_section);
                            break;
                        }
                    }
                } else {
                    list_children.insert(key, vec![data_section]);
                }

            } else {
                // Normal data section
                children.insert(data_section.name.clone(), data_section);
            }


            next_section_start = Self::find_next_section_start(lines, section_end);
        }



		Some (DataSection {header, lines: (section_start, section_end), values, children, list_children, name })

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
			if value.len() > 1 && value.chars().last().unwrap() == '}' {

                let array_values: Vec<&str> = value[1..value.len() - 1].split(',').collect::<Vec<_>>();
                let array_values: Vec<&str> = array_values.iter().map(|a| { a.trim() }).collect::<Vec<_>>();
                let array_values = array_values.iter().map(|a| { Self::parse_value(a) }).flatten().collect::<Vec<_>>();
			    return Ok(Value::List(array_values));
            }

			return Err(anyhow!("Failed to read array.\n"));
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

[numbers]
data = {0, 1, 2, 3, 4, 5}


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
