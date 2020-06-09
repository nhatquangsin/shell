pub fn split_command(command: &str) -> Vec<String> {
    let mut splits = vec![];
    let mut tmp = "".to_string();

    for char in command.chars() {
        // if char == '\'' {
        //     println!("{}", char);
        // }
        // TODO: process case quote and double quote
        if char == ' ' {
            splits.push(tmp.trim().to_string());
            tmp = "".to_string();
        } else {
            tmp.push(char);
        }
    }

    splits.push(tmp.trim().to_string());

    splits
}
