pub mod downloader {
    use reqwest::blocking::Client;
    use serde::Deserialize;
    use regex::Regex;

    #[derive(Deserialize)]
    pub struct VideoInfo {
        pub title: String,
        pub url: String,
    }

    pub fn fetch_video_info(url: &str) -> VideoInfo {
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
}