pub struct Scanner<I: Iterator>
where
    I::Item: Clone,
{
    src: I,
    buf: Vec<I::Item>,
    pos: isize,
}

// Scanners are Iterators
impl<I> Iterator for Scanner<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.pos += 1;
        // Check if we need to fill the buffer
        let lacking = self.pos - (self.buf.len() as isize) + 1;
        if lacking > 0 {
            self.buf.extend(self.src.by_ref().take(lacking as usize));
        }
        // limit the buffer position to the buffer length at most
        self.pos = std::cmp::min(self.pos, self.buf.len() as isize);
        self.current()
    }
}

impl<I> Scanner<I>
where
    I: Iterator,
    I::Item: Clone,
{
    pub fn new(source: I) -> Scanner<I> {
        Scanner {
            src: source,
            buf: Vec::new(),
            pos: -1,
        }
    }

    // Allows getting current buffer position to backtrack
    pub fn buffer_pos(&self) -> isize {
        self.pos
    }

    // Reset buffer position, normally used for backtracking
    // If position is out of bounds set_buffer_pos returns false
    pub fn set_buffer_pos(&mut self, pos: isize) -> bool {
        if pos < -1 || pos > (self.buf.len() as isize) {
            return false;
        }
        self.pos = pos;
        true
    }

    // Returns the current token on which the scanner is positioned
    pub fn current(&self) -> Option<I::Item> {
        let pos = self.pos as usize;
        if self.pos < 0 || pos >= self.buf.len() {
            return None;
        }
        Some(self.buf[pos].clone())
    }

    // Steps the scanner back and returns the token at that position
    pub fn prev(&mut self) -> Option<I::Item> {
        if self.pos >= 0 {
            self.pos -= 1;
        }
        self.current()
    }

    // Returns the token ahead without actually advancing the scanner
    pub fn peek(&mut self) -> Option<I::Item> {
        let backtrack = self.pos;
        let peeked = self.next();
        self.pos = backtrack;
        peeked
    }

    // Returns the previous token without actually backtracking the scanner
    pub fn peek_prev(&mut self) -> Option<I::Item> {
        let backtrack = self.pos;
        let peeked = self.prev();
        self.pos = backtrack;
        peeked
    }

    // Returns a view of the current underlying buffer
    pub fn view(&self) -> &[I::Item] {
        let n = (self.pos + 1) as usize;
        &self.buf[..n]
    }

    // Consumes the buffer into a new token (which can be ignored)
    pub fn extract(&mut self) -> Vec<I::Item> {
        // Check where to shift buffer
        let split_point = std::cmp::min(self.pos + 1, self.buf.len() as isize);
        assert!(split_point >= 0);
        // Reset buffer cursor
        self.pos = -1;
        // Split buffer and keep the remainder
        let mut remaining = self.buf.split_off(split_point as usize);
        std::mem::swap(&mut self.buf, &mut remaining);
        remaining
    }
}

impl<I> Scanner<I>
where
    I: Iterator,
    I::Item: Clone + PartialEq,
{
    // Advance the scanner only if the next char is the expected one
    // self.current() will return the matched char if accept matched
    pub fn accept(&mut self, what: &I::Item) -> Option<I::Item> {
        let backtrack = self.buffer_pos();
        if let Some(next) = self.next() {
            if &next == what {
                return Some(next);
            }
        }
        self.set_buffer_pos(backtrack);
        None
    }

    // Advance the scanner only if the next char is in the 'any' set,
    // self.current() will return the matched char if accept matched any
    pub fn accept_any(&mut self, any: &[I::Item]) -> Option<I::Item> {
        let backtrack = self.buffer_pos();
        if let Some(next) = self.next() {
            if any.contains(&next) {
                return Some(next);
            }
        }
        self.set_buffer_pos(backtrack);
        None
    }

    // Advance the scanner only if a full match for items form 'what'.
    // self.current() will return the last item from 'what'
    pub fn accept_all(&mut self, what: impl Iterator<Item = I::Item>) -> bool {
        let backtrack = self.buffer_pos();
        for item in what {
            if self.accept(&item).is_none() {
                self.set_buffer_pos(backtrack);
                return false;
            }
        }
        true
    }

    // Skip over the 'over' set, result is if the scanner was advanced,
    // self.current() will return the last matching char
    pub fn skip_all(&mut self, over: &[I::Item]) -> bool {
        let mut advanced = false;
        while self.accept_any(over).is_some() {
            advanced = true;
        }
        advanced
    }

    // Find an element in the 'any' set or EOF, return if the scanner advanced,
    // self.current() returns the last non-matching char
    pub fn until_any(&mut self, any: &[I::Item]) -> bool {
        let mut advanced = false;
        while let Some(next) = self.peek() {
            if any.contains(&next) {
                break;
            }
            self.next();
            advanced = true;
        }
        advanced
    }
}

static WHITE: &[char] = &[' ', '\n', '\r', '\t'];
static DIGITS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
static HEXDIGITS: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B', 'C',
    'D', 'E', 'F',
];
static ALPHA: &[char] = &[
    '_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
    'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
static ALNUM: &[char] = &[
    '_', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
    'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A',
    'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl<I: Iterator<Item = char>> Scanner<I> {
    pub fn extract_string(&mut self) -> String {
        self.extract().into_iter().collect()
    }

    pub fn scan_whitespace(&mut self) -> Option<String> {
        self.skip_all(WHITE);
        Some(self.extract_string())
    }

    // scan numbers like -?[0-9]+(\.[0-9]+)?([eE][+-][0-9]+)?
    pub fn scan_number(&mut self) -> Option<String> {
        let backtrack = self.buffer_pos();
        // optional sign
        self.accept_any(&['+', '-']);
        // require integer part
        if !self.skip_all(DIGITS) {
            self.set_buffer_pos(backtrack);
            return None;
        }
        // check for fractional part, else it's just an integer
        let backtrack = self.buffer_pos();
        if self.accept(&'.').is_some() && !self.skip_all(DIGITS) {
            self.set_buffer_pos(backtrack);
            return Some(self.extract_string()); // integer
        }
        // check for exponent part
        let backtrack = self.buffer_pos();
        if self.accept_any(&['e', 'E']).is_some() {
            self.accept_any(&['+', '-']); // exponent sign is optional
            if !self.skip_all(DIGITS) {
                self.set_buffer_pos(backtrack);
                return Some(self.extract_string()); //float
            }
        }
        // // accept imaginary numbers
        // it doesnt
        //self.accept(&'i');
        Some(self.extract_string())
    }

    pub fn scan_math_op(&mut self) -> Option<String> {
        const OPS: &[char] = &['+', '-', '*', '/', '^', '(', ')', ','];
        if self.accept_any(&['>', '=', '<']).is_some() {
            // accept '<', '>', '=', '<=', '>=', '=='
            self.accept(&'=');
            Some(self.extract_string())
        } else if self.accept(&'*').is_some() {
            // accept '*', '**'
            self.accept(&'*');
            Some(self.extract_string())
        } else if self.accept_any(OPS).is_some() {
            Some(self.extract_string())
        } else {
            None
        }
    }

    // scan integers like 0x34 0b10101 0o657
    pub fn scan_integer(&mut self) -> Option<String> {
        let backtrack = self.buffer_pos();
        if self.accept(&'0').is_some()
            && match self.accept_any(&['x', 'o', 'b']) {
                Some('x') => self.skip_all(HEXDIGITS),
                Some('o') => self.skip_all(&HEXDIGITS[..8]),
                Some('b') => self.skip_all(&HEXDIGITS[..2]),
                _ => false,
            }
        {
            return Some(self.extract_string());
        }
        self.set_buffer_pos(backtrack);
        None
    }

    // scan a quoted string like "this is \"an\" example"
    pub fn scan_quoted_string(&mut self, q: char) -> Option<String> {
        let backtrack = self.buffer_pos();
        self.accept(&q)?;
        while let Some(n) = self.next() {
            if n == '\\' {
                self.next();
                continue;
            }
            if n == q {
                return Some(self.extract_string());
            }
        }
        self.set_buffer_pos(backtrack);
        None
    }

    // scan [a-zA-Z_][a-zA-Z0-9_]+
    pub fn scan_identifier(&mut self) -> Option<String> {
        self.accept_any(ALPHA)?;
        self.skip_all(ALNUM);
        Some(self.extract_string())
    }

    // scan an optional prefix (unit multiplier) and unit
    pub fn scan_unit(&mut self) -> bool {
        static IMAGINARY_UNITS: &[&str] = &["i"];
        for unit in IMAGINARY_UNITS {
            if self.accept_all(unit.chars()) {
                self.extract_string(); // ignore
                return true;
            }
        }
        false
    }
}
