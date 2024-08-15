use std::{
    fmt::{self, Display},
    path::PathBuf,
};

#[derive(Debug, Default, Clone)]
pub struct FileInfo{
    pub path: Option<PathBuf>,
}

impl FileInfo{
    pub fn from(file_name: &str) -> Self{
        Self{
            path: Some(PathBuf::from(file_name)),
        }
    }
}

//will either print out No Name or the file name
impl Display for FileInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self
            .path
            .as_ref()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("[No Name]");
        write!(formatter, "{name}")
    }
}
