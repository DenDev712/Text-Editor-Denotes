#[derive(Debug, Default, PartialEq, Eq)]

pub struct DocumentStatus{
    pub total_lines: usize,
    pub current_line_index: usize,
    pub is_modified: bool,
    pub file_name: String,
}

impl DocumentStatus{
    pub fn modified_to_string(&self) -> String{
        if self.is_modified{
            String::from("{modified}")
        }else{
            String::new()
        }
    }

    pub fn line_count(&self) -> String{
        format!("{} lines", self.total_lines)
    }

    pub fn position_to_string(&self) -> String{
        format!(
            "{}/{}",
            self.current_line_index.saturating_add(1),
            self.total_lines
        )
    }
}