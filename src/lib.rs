mod models;
use html_escape;
use models::*;
use reqwest::Client;

pub async fn get_transcption(video_url: String, languages: &[String]) -> Vec<Transcription> {
    let client = Client::new();

    todo!()
}

async fn create_consent_cookie(client: &mut Client, html: String) {}

async fn fetch_html(client: &Client, url: String) -> String {
    html_escape::decode_html_entities(
        &client
            .get(url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:107.0) Gecko/20100101 Firefox/107.0",
            )
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap(),
    )
    .to_string()
}

async fn fetch_video_html(client: &mut Client, url: String) -> String {
    let html = fetch_html(client, url).await;
    if html.contains("action=\"https://consent.youtube.com/s\"") {
        todo!()
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work() {}
}
