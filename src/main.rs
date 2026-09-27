fn main() {
    let mut text = String::from("Пик Балмера");
    reverse_words_at_odd_positions(&mut text);
    println!("{text}");
}

///Returnes length of the UTF-8 Character, in case of continuation byte returns None
const fn utf8_len(b: u8) -> Option<usize> {
    match b {
        0x00..=0x7F => Some(1),
        0xC0..=0xDF => Some(2),
        0xE0..=0xEF => Some(3),
        0xF0..=0xFF => Some(4),
        _ => None,
    }
}

fn reverse_word(word: &mut str) {
    let bytes = unsafe { word.as_bytes_mut() };

    bytes.reverse();

    for idx in 0..bytes.len() {
        let byte = bytes[idx];
        match utf8_len(byte) {
            Some(utf_len) => {
                bytes[(idx + 1 - utf_len)..(idx + 1)].reverse();
            }
            None => {
                continue;
            }
        }
    }
}

struct OddWordIterMut<'a> {
    text: Option<&'a mut str>,
    word_idx: usize,
}

impl<'a> OddWordIterMut<'a> {
    pub fn new(text: &'a mut str) -> Self {
        Self {
            text: Some(text),
            word_idx: 0,
        }
    }
}

impl<'a> Iterator for OddWordIterMut<'a> {
    type Item = &'a mut str;
    fn next(&mut self) -> Option<Self::Item> {
        self.word_idx += 1;

        let mut text = self.text.take()?;
        let first_letter_idx = text.find(|c: char| !c.is_whitespace())?;

        let (_, rest) = text.split_at_mut(first_letter_idx);
        text = rest;

        let first_space_idx = text
            .find(|c: char| c.is_whitespace())
            .unwrap_or_else(|| text.len());
        let (word, rest) = text.split_at_mut(first_space_idx);
        self.text = Some(rest);

        if self.word_idx % 2 == 1 {
            Some(word)
        } else {
            self.next()
        }
    }
}

fn reverse_words_at_odd_positions(text: &mut String) {
    for word in OddWordIterMut::new(text) {
        reverse_word(word);
    }
}

#[cfg(test)]
#[path = "../tests/unit/monologue.rs"]
mod monologue_tests;
