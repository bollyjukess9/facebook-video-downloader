use std::io;

pub fn get_video_url() -> String {
    println!("Enter the Facebook video URL:");
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Failed to read line");
    url.trim().to_string()
}