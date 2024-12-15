use crate::editor::filetype::FileType;
use std::{
    fmt::{self, Display},
    path::{Path, PathBuf},
};

#[derive(Debug, Default)]
pub struct FileInfo{
    path: Option<PathBuf>,
    file_type: FileType,
}

impl FileInfo{
    pub fn from(file_name: &str) -> Self{
        //checks the last letters for the filetype
        let path = PathBuf::from(file_name);
        let file_type = if path
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("rs"))
        {
            FileType::Rust
        } else {
            FileType::Text
        };

        Self{
            path: Some(PathBuf::from(file_name)),
            file_type,
        }
    }

    pub fn get_path(&self) -> Option<&Path>{
        self.path.as_deref()
    }

    pub const fn has_path(&self) -> bool{
        self.path.is_none()
    }

    pub const fn get_file_type(&self) -> FileType{
        self.file_type
    }
}

//will either print out No Name or the file name
impl Display for FileInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self
            .get_path()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("[No Name]");
        write!(formatter, "{name}")
    }
}
