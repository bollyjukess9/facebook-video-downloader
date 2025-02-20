use std::fs;
use std::io::{self, Write};
use std::path::Path;
use reqwest::blocking::Client;
use serde::Deserialize;
use regex::Regex;

#[derive(Deserialize)]
struct VideoInfo {
    title: String,
    url: String,
}

fn main() {
    let video_url = get_video_url();
    let video_info = fetch_video_info(&video_url);
    download_video(&video_info);
}

fn get_video_url() -> String {
    println!("Enter the Facebook video URL:");
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Failed to read line");
    url.trim().to_string()
}

fn fetch_video_info(url: &str) -> VideoInfo {
    let client = Client::new();
    let response = client.get(url).send().expect("Failed to send request");
    let body = response.text().expect("Failed to read response text");
    parse_video_info(&body)
}

fn parse_video_info(body: &str) -> VideoInfo {
    let title_regex = Regex::new(r#"<title>(.*?)</title>"#).unwrap();
    let url_regex = Regex::new(r#"(https://video\.fbcdn\.net/.*?\.mp4)"#).unwrap();
    
    let title = title_regex.captures(body).unwrap()[1].to_string();
    let url = url_regex.captures(body).unwrap()[0].to_string();
    
    VideoInfo { title, url }
}

fn download_video(video_info: &VideoInfo) {
    let response = reqwest::blocking::get(&video_info.url).expect("Failed to download video");
    let mut file = fs::File::create(&video_info.title).expect("Failed to create file");
    let content = response.bytes().expect("Failed to read video bytes");
    file.write_all(&content).expect("Failed to write to file");
    println!("Downloaded: {}", video_info.title);
}