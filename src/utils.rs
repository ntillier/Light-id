pub fn parse_id(id: &str, chars: &Vec<char>) -> usize {
    let mut status = 0;

    for (index, char) in id.chars().rev().enumerate() {
        status += chars
            .iter()
            .position(|i| i == &char)
            .expect("Invalid character")
            * chars.len().pow(index.try_into().unwrap());
    }

    status
}

pub fn format_id(id: &usize, min: &usize, chars: &Vec<char>) -> String {
    let mut current = String::new();

    let mut remaining: usize = *id;

    loop {
        current.push(chars[remaining % chars.len()]);

        remaining = remaining / chars.len();

        if remaining == 0 {
            break;
        }
    }

    while &current.len() < min {
        current.push(chars[0]);
    }

    current.chars().rev().collect()
}
