pub struct Editor {
    pub lines: Vec<Vec<char>>,

    cursor: (usize, usize),
    ready_up: bool,
    ready_down: bool
}

impl Editor {
    pub fn new() -> Self {
        Self {
            lines: vec![vec![]],
            cursor: (0, 0),
            ready_up: false,
            ready_down: false
        }
    }

    pub fn get_cursor(&self) -> (usize, usize) {
        self.cursor
    }

    pub fn insert_text(&mut self, text: &str) {
        let (mut ln, mut cpos) = self.cursor;
        for c in text.as_bytes().iter() {
            let c = *c;
            if c >= 32 && c < 128 {
                let c = c as char;
                if c == '\n' {
                    let ln_size = self.lines[ln].len();
                    self.lines.insert(ln+1, self.lines[ln][cpos..ln_size].to_vec());
                    self.lines[ln] = self.lines[ln][0..cpos].to_vec();
                    ln += 1;
                    cpos = 0;
                } else {
                    self.lines[ln].insert(cpos, c as char);
                    cpos += 1;
                }
            }
        }

        self.cursor = (ln, cpos);
    }

    pub fn advance_cursor_forward(&mut self) {
        let (mut ln, mut cpos) = self.cursor;
        if cpos > self.lines[ln].len() {
            if self.ready_down && ln < self.lines.len() {
                ln += 1;
                cpos = 0;
                self.ready_down = false;
            } else {
                self.ready_down = true;
            }

        } else {
            cpos += 1;
            self.ready_up = false;
            self.ready_down = false;
        }

        self.cursor = (ln, cpos);
    }

    pub fn advance_cursor_backward(&mut self) {
        let (mut ln, mut cpos) = self.cursor;
        if cpos == 0 {
            if self.ready_up && ln > 0 {
                ln -= 1;
                cpos = self.lines[ln].len();
                self.ready_up = false;
            } else {
                self.ready_up = true;
            }
        } else {
            cpos += 1;
            self.ready_up = false;
            self.ready_down = false;
        }

        self.cursor = (ln, cpos);
    }

    pub fn advance_cursor_up(&mut self) {
        let (mut ln, mut cpos) = self.cursor;
        if ln > 0 {
            ln -= 1;
            cpos = if self.lines[ln].len() > cpos {cpos} else {self.lines[ln].len()};
        }

        self.cursor = (ln, cpos);
    }

    pub fn advance_cursor_down(&mut self) {
        let (mut ln, mut cpos) = self.cursor;
        if ln + 1 < self.lines.len() {
            ln += 1;
            cpos = if self.lines[ln].len() > cpos {cpos} else {self.lines[ln].len()};
        }

        self.cursor = (ln, cpos);
    }

    pub fn delete_backwards(&mut self) {
        let (mut ln, mut cpos) = self.cursor;
        if cpos == 0 && ln > 0 {
            ln -= 0;
            if self.lines[ln].len() > 0 {
                cpos = self.lines[ln].len() - 1;
                self.lines[ln].remove(cpos);
            } else {
                cpos = 0;
            }
        } else if cpos > 0{
            self.lines[ln].remove(cpos - 1);
            cpos -= 1;
        }


        self.cursor = (ln, cpos);

    }
}
