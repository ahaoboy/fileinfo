mod parse;
pub use parse::parse;
use std::str::FromStr;

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct FileInfo {
    pub permissions: String,
    pub hard_link_count: usize,
    pub owner: String,
    pub group: String,
    pub size: usize,
    pub last_modified: String,
    pub name: String,
    pub link: Option<String>,
}

impl FileInfo {
    pub fn is_dir(&self) -> bool {
        self.permissions.starts_with('d')
    }

    pub fn is_file(&self) -> bool {
        !self.is_dir()
    }

    pub fn is_link(&self) -> bool {
        self.permissions.starts_with('l')
    }
}

impl<'a> TryFrom<&'a str> for FileInfo {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        parse(value).ok_or(())
    }
}

impl FromStr for FileInfo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s).ok_or(())
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{parse, FileInfo};
    #[test]
    fn test() {
        let s = "drwxr-xr-x 1 owner group         3452 Jun 11 10:15 device";
        let info = parse(s);
        assert_eq!(
            info,
            Some(FileInfo {
                permissions: "drwxr-xr-x".to_string(),
                hard_link_count: 1,
                owner: "owner".to_string(),
                group: "group".to_string(),
                size: 3452,
                last_modified: "Jun 11 10:15".to_string(),
                name: "device".to_string(),
                link: None
            })
        )
    }

    #[test]
    fn test_link() {
        let s = "lrwxrwxrwx 1 owner group           10 Jun 11 10:15 link.txt -> file1.txt";
        let info = parse(s);
        assert_eq!(
            info,
            Some(FileInfo {
                permissions: "lrwxrwxrwx".to_string(),
                hard_link_count: 1,
                owner: "owner".to_string(),
                group: "group".to_string(),
                size: 10,
                last_modified: "Jun 11 10:15".to_string(),
                name: "link.txt".to_string(),
                link: Some("file1.txt".to_string())
            })
        );
    }

    #[test]
    fn test_from_str() {
        let s = "lrwxrwxrwx 1 owner group           10 Jun 11 10:15 link.txt -> file1.txt";
        let info = FileInfo::try_from(s);
        assert!(info.is_ok());

        let info = FileInfo::from_str(s);
        assert!(info.is_ok());
    }
}
