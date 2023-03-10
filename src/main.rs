use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::blocking::{multipart, Client};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//Theese are here so they're only compiled once
static OUT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"OUTPUT FILE<br /><[^>]*>[^<]*").expect("hardcoded"));
static ERR_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"STDERR<[^>]*><[^>]*>[^<]*").expect("hardcoded"));
static BODY_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^>]*$").expect("hardcoded"));

//Make the call to the oracle
fn forsee(filename: &String, oracle: i32) -> Result<(String, String), String> {
    let form = match multipart::Form::new().file("input", filename) {
        Ok(v) => v,
        Err(_) => return Err(format!("File `{filename}` not found")),
    };

    let client = Client::new();
    let res = match client
        .post(format!("https://compilers.cool/oracles/o{oracle}/"))
        .multipart(form)
        .send()
    {
        Ok(v) => match v.text() {
            Ok(v2) => v2,
            Err(_) => {
                return Err(String::from(
                    "Error parsing response from oracle (tell derrick)",
                ));
            }
        },
        Err(_) => {
            return Err(String::from(
                "Error sending request to oracle (tell derrick (also check connection))",
            ));
        }
    };

    let out_html = match OUT_RE.find(&res) {
        Some(v) => v.as_str(),
        None => "",
    };
    let err_html = match ERR_RE.find(&res) {
        Some(v) => v.as_str(),
        None => "",
    };

    let out_body = String::from(
        BODY_RE
            .find(out_html)
            .expect("It certainly ends with something")
            .as_str(),
    );
    let err_body = String::from(BODY_RE.find(err_html).expect("See above").as_str());
    Ok((sanitize(out_body), sanitize(err_body)))
}

fn sanitize(code: String) -> String {
    code.replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("&quot;", "\"")
        .replace("&amp;", "&")
}



fn main() -> Result<(), String> {
    //Assemble list of files
    let mut oracle: Option<i32> = None;
    let oracle_re = Regex::new(r"^--o[0-9]+$").expect("hardcoded");
    let mut files = vec![];


    //Read in files from the cli args
    for arg in env::args().skip(1) {
        match oracle_re.find(&arg) {
            Some(_) => {
                oracle = match &arg[3..].parse::<i32>() {
                    Ok(s) => Some(*s),
                    Err(_) => {
                        println!("Unrecognized argument {arg}");
                        oracle
                    }
                };
            }
            None => {
                let path = Path::new(&arg);
                match path.extension() {
                    Some(v) => {
                        if let Some("jeff") = v.to_str() {
                            files.push(arg);
                        }
                    }
                    None => {
                        println!("Unrecognized file `{arg}`");
                    }
                }
            }
        }
    }


    //After getting list of filenames, we send each to the oracle and print outputs
    match oracle {
        //Oracle is valid
        Some(oracle_ver) => {
            for filename in files {
                match forsee(&filename, oracle_ver) {
                    Ok((out_body, err_body)) => {
                        let stem = Path::new(&filename)
                            .file_stem()
                            .expect("It is there, else no out/err")
                            .to_str()
                            .expect("should be convertible...");
                        let out_filename = format!("{stem}.out.expected");
                        let err_filename = format!("{stem}.err.expected");

                        match File::create(&out_filename) {
                            Ok(mut file) => match file.write_all(out_body.as_bytes()) {
                                Ok(_) => println!("✅ Wrote {out_filename}"),
                                Err(_) => println!("❌ Failed to write `{out_filename}`"),
                            },
                            Err(_) => println!("❌ Failed to create `{out_filename}`"),
                        }

                        match File::create(&err_filename) {
                            Ok(mut file) => match file.write_all(err_body.as_bytes()) {
                                Ok(_) => println!("✅ Wrote {err_filename}"),
                                Err(_) => println!("❌ Failed to write `{err_filename}`"),
                            },
                            Err(_) => println!("❌ Failed to create `{err_filename}`"),
                        }
                    }
                    Err(e) => {
                        println!("❌ {e}")
                    }
                }
            }
        }
        //Oracle is not
        None => return Err(String::from("No oracle version entered")),
    }
    Ok(())
}
