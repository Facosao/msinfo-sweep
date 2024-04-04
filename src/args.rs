pub fn parse_args() -> Vec<String> {
    let args = std::env::args().into_iter();
    let arg = args.last().expect("No argument passed to the program!");
    let mut hosts: Vec<String> = Vec::new();

    let file = std::fs::read_to_string(&arg);
    match file {
        Ok(file_string) => {
            // Try to read and parse text file with multiple hostnames.

            for line in file_string.lines() {
                hosts.push(line.to_string());
            }
        }

        Err(_) => {
            // Assume arg is a single hostname.
            // #TODO: Validate hostname?
            hosts.push(arg.to_string());
        }
    }

    hosts
}