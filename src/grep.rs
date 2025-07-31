use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write, stdin, stdout};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct Grep;

impl Grep {
    fn highlight(&self, line: &str, regex: &Regex, stdout: &mut StandardStream) {
        let mut last_end = 0;

        for mat in regex.find_iter(line) {
            let (start, end) = (mat.start(), mat.end());

            write!(stdout, "{}", &line[last_end..start]).unwrap();

            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true)).unwrap();
            write!(stdout, "{}", &line[start..end]).unwrap();
            stdout.reset().unwrap();

            last_end = end;
        }

        writeln!(stdout, "{}", &line[last_end..]).unwrap();
    }

    fn process(&self, regex: Regex, path: &str) {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Could not open file '{}': {}", path, e);
                return;
            }
        };
        let reader = BufReader::new(file);
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => continue,
            };

            if regex.is_match(&line) {
                self.highlight(&line, &regex, &mut stdout);
            }
        }
    }

    pub fn run(&self) {
        let stdin = stdin();
        let mut stdout = stdout();
        let mut input = String::new();

        loop {
            print!("~ > ");
            stdout.flush().unwrap();
            input.clear();
            stdin.read_line(&mut input).unwrap();

            let args: Vec<&str> = input.trim().split_ascii_whitespace().collect();

            if args.is_empty() {
                continue;
            }

            match args[0] {
                "exit" => break,
                "mygrep" => {
                    if args.len() < 3 {
                        eprintln!("Usage: mygrep <pattern> <file>");
                        continue;
                    }

                    let pattern = args[1];
                    let path = args[2];

                    let regex = match Regex::new(pattern) {
                        Ok(re) => re,
                        Err(e) => {
                            eprintln!("Invalid regex pattern: {}", e);
                            continue;
                        }
                    };

                    self.process(regex, path);
                }
                _ => {
                    eprintln!("Unknown command: {}", args[0]);
                    continue;
                }
            }
        }
    }
}
