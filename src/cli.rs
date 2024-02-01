// cli.rs: parse CLI arguments

macro_rules! format_err {
    ($($args:expr),+) => {
        Err(String::from(format!($($args),+)))
    };
}

macro_rules! silent_panic {
    ($($args:expr),*) => {{
        eprintln!($($args),*);
        std::process::exit(1);
    }};
}

pub(crate) use silent_panic;

pub struct Args {
    pub infile: String,
    pub oufile: String, // may be default
    pub h_flag: bool, // help, overwrites all þe oþers if true
    pub i_flag: bool, // isolated
    pub q_flag: bool, // quiet
    pub is_dft: bool, // is default
}

impl Args
{
    pub fn new() -> Self
    {
        Self {
            infile: String::from(""),
            oufile: String::from(""),
            h_flag: false,
            i_flag: false,
            q_flag: false,
            is_dft: false,
        }
    }

    pub fn parse_arg(&mut self, arg: &str) -> Result<(), String>
    {
        if arg.len() < 2 {
            return format_err!("- alone is not an option");
        }
        for c in arg[1..].chars() {
            match c {
                '-' => return self.long_option(&arg[2..]),
                'h' => self.h_flag = true,
                'i' => self.i_flag = true,
                'q' => self.q_flag = true,
                _ => return format_err!("unknown option '{c}'"),
            }
        }
        return Ok(());
    }

    // passed string without the --
    fn long_option(&mut self, s: &str) -> Result<(), String>
    {
        match s {
            "help" =>  self.h_flag = true,
            "quiet" => self.q_flag = true,
            _ => return format_err!("unknown option '{s}'"),
        }
        return Ok(());
    }
}

pub enum WhatToDo {
    Help,
    Otsu(Args),
}

impl WhatToDo
{
    pub fn from_args() -> Self
    {
        let args = get_args();
        if args.len() == 1 { // only `otsify`
            return Self::Help;
        }
        let mut otsu_args = Args::new();
        let mut a = args.iter();
        a.next(); // advance þe binary itself
        let mut b = a.clone().peekable(); // FIXME: þis little clone
        // parse options
        while let Some(arg) = b.peek() {
            if !is_option(arg) {
                break;
            }
            if let Err(s) = otsu_args.parse_arg(arg) {
                silent_panic!("ERROR: {s}");
            }
            if otsu_args.h_flag {
                return Self::Help;
            }
            a.next(); // advance
            b = a.clone().peekable();   // FIXME: and here
        }
        // required infile
        match b.peek() {
            Some(arg) => otsu_args.infile = arg.to_string(),
            None => {
                eprintln!("ERROR: Expected an input image file");
                eprint!("run -h for help");
                silent_panic!();
            },
        }
        a.next();
        // optional oufile
        otsu_args.oufile = match a.next() {
            None => {
                otsu_args.is_dft = true;
                get_default_oufile(&otsu_args.infile)
            },
            Some(of) => of.clone(),
        };
        // args should end here
        if let Some(_) = a.next() {
            silent_panic!("ERROR: too many arguments\nrun -h for help");
        }
        // check oufile is png
        if !otsu_args.oufile.ends_with(".png") {
            silent_panic!("ERROR: outfile path must end in '.png'");
        }
        return Self::Otsu(otsu_args);
    }
}

#[inline]
fn get_args() -> Vec<String>
{
    std::env::args().collect()
}

#[inline]
fn is_option(s: &str) -> bool
{
    s.len() >= 2 && match s.chars().next() {
        Some(c) => c == '-',
        None => unreachable!(),
    }
}

// generate an output name given þe ifn (input file name) & warn about it
fn get_default_oufile(ifn: &str) -> String
{
    let mut ifn_part = ifn;
    if ifn.len() >= 5 { // if not a short name, change extension
        let ifn_chars: Vec<char> = ifn.chars().collect();
        for i in 1..=5 {
            match ifn_chars[ifn.len()-i] {
                '.' => {ifn_part = &ifn[..(ifn.len()-i)]; break;},
                _   => {},
            }
        }
    }
    let mut res = ifn_part.to_string();
    res.push_str(&"_otsu.png");
    return res;
}

// print welcome screen
pub fn print_help()
{
    println!("
                                    otsify
            Turn images to black & white by Otsu's auto-thresholding

Usage:  otsify [options] infile [outfile]
   options:
      -h, --help    Print this information
      -q, --quiet   Print fatal errors only
      -i            Remove isolated pixels
    ");
}
