use std::fs;

fn split_hands(content: &str) -> Vec<String> {
    // on recupere un fichier entier et il faut le decouper en liste de hands.
    let mut acc: String = String::new();
    let mut res: Vec<String> = Vec::new();
    for line in content.lines() {
        if line.starts_with("Winamax Poker - ") {
            if !acc.is_empty() {
                res.push(acc);
                acc = String::new();
            }
        }
        acc.push_str(line);
        acc.push('\n');
    }
    if !acc.is_empty() {
        res.push(acc);
    }
    res
}

pub fn load_data() -> Result<Vec<String>, std::io::Error> {
    let list_dir = fs::read_dir(
        "/Users/clement/Library/Application Support/winamax/documents/accounts/MrAlambic/history",
    )?;
    let mut list_hands: Vec<String> = Vec::new();
    for entry in list_dir {
        let entry = match entry {
            Ok(f) => f,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        let path = entry.path();
        if !path.extension().is_some_and(|ext| ext == "txt") {
            continue;
        }

        match fs::read_to_string(path) {
            Ok(content) => list_hands.extend(split_hands(&content)),
            Err(e) => println!("Error: {:?}", e),
        }
    }
    println!("{:?}", list_hands.len());
    Ok(list_hands)
}
