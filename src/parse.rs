use crate::FileInfo;

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
