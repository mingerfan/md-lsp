use ropey::Rope;

#[derive(Default)]
pub struct Text {
    content: Rope,
}

pub enum TextChange {
    Insert { start_line: usize, start_character: usize, text: String },
    Delete { start_line: usize, start_character: usize, end_line: usize, end_character: usize },
    Replace { start_line: usize, start_character: usize, end_line: usize, end_character: usize, text: String },
    FullReplace { text: String },
}

impl std::fmt::Display for TextChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextChange::Insert { start_line, start_character, text } => {
                write!(f, "Insert at line {}, character {}: '{}'", start_line, start_character, text)
            }
            TextChange::Delete { start_line, start_character, end_line, end_character } => {
                write!(f, "Delete from line {}, character {} to line {}, character {}", start_line, start_character, end_line, end_character)
            }
            TextChange::Replace { start_line, start_character, end_line, end_character, text } => {
                write!(f, "Replace from line {}, character {} to line {}, character {} with '{}'", start_line, start_character, end_line, end_character, text)
            }
            TextChange::FullReplace { text } => {
                write!(f, "Full replace with '{}'", text)
            }
        }
    }
}

impl Text {
    pub fn new() -> Self {
        Self {
            content: Rope::new(),
        }
    }

    pub fn get_content(&self) -> String {
        self.content.to_string()
    }

    pub fn refresh(&mut self, new_content: String) {
        self.content = Rope::from_str(&new_content);
    }

    fn utf8_pos_to_char_idx(&self, line: usize, character: usize) -> anyhow::Result<usize> {
        let line_start_byte = self.content.try_line_to_byte(line)?;
        let byte_idx = line_start_byte
            .checked_add(character)
            .ok_or_else(|| anyhow::anyhow!("byte index overflow"))?;
        let char_idx = self.content.try_byte_to_char(byte_idx)?;
        Ok(char_idx)
    }

    pub fn update(&mut self, change: TextChange) -> anyhow::Result<()>{
        match change {
            TextChange::Insert { start_line, start_character, text } => {
                let idx = self.utf8_pos_to_char_idx(start_line, start_character)?;
                self.content.try_insert(idx, &text)?;
            }
            TextChange::Delete { start_line, start_character, end_line, end_character } => {
                let start_idx = self.utf8_pos_to_char_idx(start_line, start_character)?;
                let end_idx = self.utf8_pos_to_char_idx(end_line, end_character)?;
                self.content.try_remove(start_idx..end_idx)?;
            }
            TextChange::Replace { start_line, start_character, end_line, end_character, text } => {
                let start_idx = self.utf8_pos_to_char_idx(start_line, start_character)?;
                let end_idx = self.utf8_pos_to_char_idx(end_line, end_character)?;
                self.content.try_remove(start_idx..end_idx)?;
                self.content.try_insert(start_idx, &text)?;
            }
            TextChange::FullReplace { text } => {
                self.refresh(text);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Text, TextChange};

    #[test]
    fn insert_chinese_with_utf8_offsets() {
        let mut text = Text::default();
        text.refresh(String::new());

        text
            .update(TextChange::Insert {
                start_line: 0,
                start_character: 0,
                text: "科".to_string(),
            })
            .expect("first insert should succeed");

        text
            .update(TextChange::Insert {
                start_line: 0,
                start_character: 3,
                text: "技".to_string(),
            })
            .expect("second insert should succeed when offset is utf8 bytes");

        assert_eq!(text.get_content(), "科技");
    }
}