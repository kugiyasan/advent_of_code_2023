use chrono::{Datelike, Local};
use reqwest::{header, Client};
use scraper::{Html, Selector};
use std::io;
use tokio::runtime::Runtime;

pub fn submit(answer: &str, part2: bool) {
    let now = Local::now();
    let rt = Runtime::new().unwrap();

    let part = if part2 { "part2" } else { "part1" };
    println!("Press 'y' to submit {answer} for {part}?: ");

    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation).unwrap();

    if confirmation != "y\n" {
        println!("Skipping");
        return;
    }

    let year = now.year();
    let day = now.day();
    println!("Submitting AoC {year} day {day} {part}: {answer:?}");
    rt.block_on(async { _submit(year, day, part2, answer).await })
}

async fn _submit(year: i32, day: u32, part2: bool, answer: &str) {
    let client = Client::new();
    let url = format!("https://adventofcode.com/{year}/day/{day}/answer");

    let cookies = include_str!("../../cookies.txt").trim();

    let level = if part2 { "2" } else { "1" };
    let form = [("level", level), ("answer", &answer)];

    let response = client
        .post(url)
        .header(header::COOKIE, cookies)
        .form(&form)
        .send()
        .await;

    match response {
        Ok(res) => {
            let html = res.text().await.unwrap();
            print_message(&html);
        }
        Err(err) => println!("FAILED: {}", err),
    }
}

fn print_message(html: &str) {
    let document = Html::parse_document(html);
    let selector = Selector::parse("main > article > p").unwrap();
    let text = document.select(&selector).next().unwrap();
    println!("{}", text.inner_html());
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_submit() {
//         submit("1", false);
//     }
// }
