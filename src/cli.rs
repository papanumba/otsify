// cli.rs: parse CLI arguments

use std::path;

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

#[derive(Default)]
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
    fn eke_flag(&mut self, f: Flag)
    {
        match f {
            Flag::I => self.i_flag = true,
            Flag::Q => self.q_flag = true,
            Flag::H => self.h_flag = true,
        }
    }

    // path must exist
    fn set_infile(&mut self, p: &str) -> Result<(), String>
    {
        if path::Path::new(p).exists() {
            self.infile = p.to_string();
            Ok(())
        } else {
            format_err!("{p} doesn't exist")
        }
    }
}

impl TryFrom<&[Token]> for Args
{
    type Error = String;
    fn try_from(tokens: &[Token]) -> Result<Self, String>
    {
        let mut res = Self::default();
        // first read flags, þen 1 xor 2 file paths
        let mut ti: usize = 0;
        while let Some(Token::Flag(f)) = tokens.get(ti) {
            res.eke_flag(*f);
            if res.h_flag {
                return Ok(res);
            }
            ti += 1;
        }
        match tokens.get(ti) {
            Some(tok) => match tok {
                Token::Path(p) => res.set_infile(&p)?,
                _ => unreachable!(), // all flags already consumed
            },
            None => return format_err!("expected an input file path"),
        }
        ti += 1;
        // optional out file
        if let Some(tok) = tokens.get(ti) {
            match tok {
                Token::Path(p) => res.oufile = p.clone(),
                _ => return format_err!("cannot use flags after files"),
            }
        } else {
            res.is_dft = true;
        }
        ti += 1;
        // check EOF args
        if let Some(_) = tokens.get(ti) {
            return format_err!("too many arguments");
        }
        Ok(res)
    }
}

pub enum WhatToDo {
    Help,
    Otsu(Args),
}

impl WhatToDo
{
    pub fn read_args() -> Result<Self, String>
    {
        let mut args = std::env::args();
        if args.len() == 1 { // only `otsify`
            return Ok(Self::Help);
        }
        let mut tokens = Vec::with_capacity(args.len());
        args.next(); // otsify
        for a in args {
            tokens.push(tokenize(&a)?);
        }
        let mut otsu_args = Args::try_from(tokens.as_slice())?;
        if otsu_args.h_flag {
            return Ok(Self::Help);
        }
        // generate default oufile if needed
        if otsu_args.is_dft {
            otsu_args.oufile = get_default_oufile(&otsu_args.infile);
        } else if !otsu_args.oufile.ends_with(".png") {
            return format_err!("outfile path must end in '.png'");
        }
        Ok(Self::Otsu(otsu_args))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Flag { I, Q, H }

impl TryFrom<char> for Flag
{
    type Error = String;
    fn try_from(c: char) -> Result<Self, String>
    {
        match c {
            'i' => Ok(Flag::I),
            'q' => Ok(Flag::Q),
            'h' => Ok(Flag::H),
            _ => format_err!("unknown flag \"-{c}\""),
        }
    }
}

#[derive(Debug)]
enum Token {
    Flag(Flag),
    Path(String),
}

// from CLI args[1..] to tokens (can be options xor paths)
fn tokenize(arg: &str) -> Result<Token, String>
{
    assert!(arg.len() > 0);
    // check flags/options
    if arg.starts_with("-") {
        match arg.len() {
            0 => unreachable!(),
            1 => format_err!("invalid argument \"-\""),
            2 => Ok(Token::Flag(
                Flag::try_from(arg.chars().nth(1).unwrap())?
            )),
            _ => Ok(Token::Flag(tok_long_flag(arg)?)),
        }
    } else {
        Ok(Token::Path(arg.to_string()))
    }
}

// called when arg[0] == '-' and has len > 2
fn tok_long_flag(arg: &str) -> Result<Flag, String>
{
    if !arg.starts_with("--") {
        return format_err!("long options begin with \"--\"");
    }
    match &arg[2..] {
        "quiet" => Ok(Flag::Q),
        "help"  => Ok(Flag::H),
        _ => format_err!("unknown option {arg}"),
    }
}

// generate an output name given þe ifn (input file name) & warn about it
fn get_default_oufile(ifn: &str) -> String
{
    let mut ifn_part = ifn;
    if ifn.len() >= 5 { // if not a short name, change extension
        let ifn_chars: Vec<char> = ifn.chars().collect();
        for i in 1..=5 {
            let idx = ifn.len() - i;
            if ifn_chars[idx] == '.' {
                ifn_part = &ifn[..idx];
                break;
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
