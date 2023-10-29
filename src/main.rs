use reqwest::{blocking::Client, Url};
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

mod expect_pretty;
use expect_pretty::*;

#[derive(PartialEq, Serialize, Deserialize)]
struct Target {
    url: String,
    id: String,
    check: String,
}

// Struct to save to json
impl Target {
    fn new(url: Url, id: String, check: String) -> Self {
        Self {
            url: String::from(url.as_str()),
            id,
            check,
        }
    }

    fn get_url(&self) -> Url {
        Url::parse(&self.url).expect_p(&format!("Error when parsing url: {}", self.url))
    }

    fn set_check(&mut self, check: String) {
        self.check = check
    }
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {}", self.url, self.id)
    }
}

// Path of targets.json
#[inline]
fn get_json_path() -> PathBuf {
    let mut exe_path = std::env::current_exe().expect_p("Couldn't retrieve current directory");
    exe_path.pop();
    exe_path.join(Path::new("targets.json"))
}

// Parse command line flags
fn parse_flags(flag: String) {
    match flag.as_str() {
        "-a" => add(),
        "-d" => delete(),
        "-h" => help(),
        "-help" => help(),
        "-l" => list(),
        _ => eprintln!("Invalid Flag"),
    }
}

// Print help text
fn help() {
    println!("Usage:\nRun without flags to check targets\nAdd a search target: stock_checker.exe -a [url] [element-id]\nRemove a search target: stock_checker.exe -d [url]\nList all current search targets: stock_checker.exe -l")
}

fn list() {
    println!();
    let targets = get_targets();
    for target in targets {
        println!("{}", target)
    }
}

fn get_targets() -> Vec<Target> {
    let path = get_json_path();

    if let Ok(f) = File::open(path) {
        if let Ok(v) = serde_json::from_reader(f) {
            return v;
        }
    }

    // If invalid data, initialise file
    save_targets(&Vec::new());
    Vec::new()
}

// Save all targets to targets.json
fn save_targets(targets: &Vec<Target>) {
    let path = get_json_path();

    let file = File::create(path).expect_p("Couldn't create file");
    serde_json::to_writer_pretty(file, &targets).expect_p("Could not write to file");
}

// Retrieves html of an element with id on the webpage
fn scrape(url: &Url, id: &String) -> String {
    let client = Client::new();
    let response = client
        .get(url.clone())
        .send()
        .expect_p("Could not send")
        .text()
        .unwrap();

    let document = Html::parse_document(&response);

    let selector = scraper::Selector::parse(&format!("{}", id)).unwrap();
    document
        .select(&selector)
        .next()
        .expect_p("No elements with id or unique class in page\nDid you specify the id or class with '#' or '.'?")
        .html()
}

// Adds target to targets.json
fn add() {
    let url_str = env::args().nth(2).expect_p("No URL argument");
    let url =
        Url::parse(&url_str).expect_p("Invalid URL, perhaps you forgot to specify a protocol?");
    let id = env::args().nth(3).expect_p(
        "No id argument\nIf you specified an id with #, you may need to preface it with \\",
    );
    let check = scrape(&url, &id);

    let mut targets = get_targets();

    if let Some(_) = targets.iter().find(|x| x.get_url() == url) {
        eprintln!("URL already in targets.json");
        return;
    }

    targets.push(Target::new(url, id, check));

    save_targets(&targets)
}

// Removes target from targets.json
fn delete() {
    let url_str = env::args().nth(2).expect_p("No URL argument");
    let url = Url::parse(&url_str).expect_p("Invalid URL");

    let mut targets = get_targets();
    let i = targets
        .iter()
        .position(|x| x.get_url() == url)
        .expect_p("URL not in targets");
    targets.remove(i);
    save_targets(&targets)
}

fn update_check(target: &Target, new_check: String) {
    let mut targets = get_targets();
    let old_target: &mut Target = targets.iter_mut().find(|x| *x == target).unwrap();
    old_target.set_check(new_check);
    save_targets(&targets)
}

// If webpage changed, send notif
fn check() {
    let targets = get_targets();
    for target in &targets {
        let scrape = scrape(&target.get_url(), &target.id);
        if scrape != target.check {
            send_notif(target);
            update_check(target, scrape);
        }
    }
}

// Send notification over http POST
fn send_notif(target: &Target) {
    let client = Client::new();
    client
        .post(include_str!("url.txt"))
        .body(format!(
            "Stock Checker: tag {} has been updated at {}",
            target.id, target.url
        ))
        .send()
        .expect_p("Could not send notif");
}

fn main() {
    let flag = env::args().nth(1);

    if let Some(v) = flag {
        parse_flags(v);
    } else {
        check()
    }
}
