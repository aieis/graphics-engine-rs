use std::iter::Map;


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

pub struct Entry {
    pub children: Map<String, Box<Value>>
}


pub struct DataSection {
    pub header: String,
    pub lines: (usize, usize)
}

pub struct DataFile {
    pub version: i64,
    pub entries: Vec<Entry>
}

impl DataFile {
    pub fn load_from_string(file_content: &str) -> Result<Self, String> {

        let lines: Vec<&str> = file_content.split('\n').collect::<Vec<_>>();


        Err("TODO".to_string())


    }

    pub fn parse_next_section(lines: &[&str], start: usize) -> Option<DataSection> {

        let next_section_start = match Self::find_next_section_start(lines, start) {
            Some(l) => l,
            None => { return None; }
        };

        None
        
    }

    pub fn parse_section_name(line: &str) -> String {
        let lines: Vec<&str> = line.rsplit('\n').collect::<Vec<_>>();

    }

    pub fn find_next_section_start(lines: &[&str], start: usize) -> Option<usize> {

        for i in start..lines.len() {
            if lines[i].chars().nth(0) == Some('[') {
                return Some(i);
            }
        }

        None
    }
}



mod test {
    #[test]
    pub fn test_load_of_strings() {
        assert!(false, "{:?}", "[test_1] # ignored data ".split('#').next());
    }
    
}
