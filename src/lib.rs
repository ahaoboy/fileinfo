use std::str::FromStr;

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct FileInfo {
    permissions: String,
    hard_link_count: usize,
    owner: String,
    group: String,
    size: usize,
    last_modified: String,
    name: String,
    link: Option<String>,
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

fn take_n(s: &str, n: usize) -> (&str, &str) {
    (&s[n..], &s[..n])
}

fn take_space(s: &str) -> (&str, &str) {
    take_till(s, |c: char| !c.is_whitespace())
}

fn take_digit(s: &str) -> (&str, &str) {
    take_till(s, |c: char| !c.is_ascii_digit())
}

fn take_till<F: Fn(char) -> bool>(s: &str, check: F) -> (&str, &str) {
    let mut end = 0;
    for (pos, c) in s.char_indices() {
        if check(c) {
            end = pos;
            break;
        }
    }
    (&s[end..], &s[..end])
}

fn take_till_space(s: &str) -> (&str, &str) {
    take_till(s, |c| c.is_whitespace())
}

pub fn parse(input: &str) -> Option<FileInfo> {
    let input = input.trim();

    let (input, permissions) = take_n(input, 10);
    let (input, _) = take_space(input);

    let (input, hard_link_count) = take_digit(input);
    let hard_link_count = hard_link_count.parse::<usize>().ok()?;

    let (input, _) = take_space(input);

    let (input, owner) = take_till_space(input);
    let (input, _) = take_space(input);

    let (input, group) = take_till_space(input);
    let (input, _) = take_space(input);

    let (input, size) = take_digit(input);
    let size = size.parse::<usize>().ok()?;
    let (input, _) = take_space(input);

    let (input, month) = take_till_space(input);
    let (input, _) = take_space(input);

    let (input, day) = take_till_space(input);
    let (input, _) = take_space(input);

    let (input, time) = take_till_space(input);
    let (input, _) = take_space(input);
    let last_modified = format!("{month} {day} {time}");

    let (name, link) = if let Some(i) = input.find(" -> ") {
        (&input[..i], Some(input[i + 4..].to_string()))
    } else {
        (input, None)
    };

    Some(FileInfo {
        permissions: permissions.to_string(),
        hard_link_count,
        owner: owner.to_string(),
        group: group.to_string(),
        size,
        last_modified,
        name: name.to_string(),
        link,
    })
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
