
#[derive(Clone)]
pub struct Options {
    pub length: usize,
    pub max_attempts: usize,
    pub number_range: std::ops::Range<i32>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            length: 4,
            max_attempts: 10,
            number_range: 0..9,
        }
    }
}

impl Options {
    pub fn from_args() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut options = Self::default();
        
        for (i, arg) in args.iter().enumerate() {
            match arg.as_str() {
                "--length" | "-l" => {
                    if i + 1 < args.len() {
                        if let Ok(length) = args[i + 1].parse::<usize>() {
                            options.length = length;
                        }
                    }
                }
                "--attempts" | "-a" => {
                    if i + 1 < args.len() {
                        if let Ok(attempts) = args[i + 1].parse::<usize>() {
                            options.max_attempts = attempts;
                        }
                    }
                }
                "--min" => {
                    if i + 1 < args.len() {
                        if let Ok(min) = args[i + 1].parse::<i32>() {
                            options.number_range = min..options.number_range.end;
                        }
                    }
                }
                "--max" => {
                    if i + 1 < args.len() {
                        if let Ok(max) = args[i + 1].parse::<i32>() {
                            options.number_range = options.number_range.start..max;
                        }
                    }
                }
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                _ => {}
            }
        }
        
        options
    }
}

fn print_help() {
    println!("Mastermind Game Options:");
    println!("  -l, --length <NUMBER>     Set code length (default: 4)");
    println!("  -a, --attempts <NUMBER>   Set max attempts (default: 10)");
    println!("  --min <NUMBER>            Set minimum number (default: 0)");
    println!("  --max <NUMBER>            Set maximum number (default: 9)");
    println!("  -h, --help                Show this help message");
}
