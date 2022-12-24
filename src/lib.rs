pub fn get_args() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    args[1..].to_vec()
}

pub fn get_argument_at(index: usize) -> Option<String> {
    match get_args().get(index) {
        Some(arg) => {
            Some(arg.to_owned())},
        None => None,
    }
}
