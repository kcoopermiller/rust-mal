pub mod mal {
    use reqwest::header::{HeaderMap, HeaderValue};
    use serde::{Deserialize, Serialize};
    use csv;
    use std::fs::OpenOptions;
    #[derive(Debug, Deserialize, Serialize)]
    struct Manga {
        title: String,
        alternative_titles: AltTitles,
        media_type: String,
    }
    #[derive(Debug, Deserialize, Serialize)]
    struct AltTitles {
        synonyms: Vec<String>,
        en: String,
        ja: String,
    }

    #[derive(Debug, Clone)]
    pub struct Client {
        _token: String,
        _base_url: String,
    }

    impl Client {
        pub fn new(token: String) -> Self {
            Client {
                _token: token,
                _base_url: String::from("https://api.myanimelist.net/v2"),
            }
        }

        // maybe create one big get functions with a lot of parameters
        pub async fn get_light_novels(&self, id: i32) -> Result<(), Box<dyn std::error::Error>> {
            let client = reqwest::Client::new();
            let mut headers = HeaderMap::new();
            headers.insert(
                "X-MAL-CLIENT-ID",
                format!("{}", &self._token)
                    .parse::<HeaderValue>()
                    .unwrap(),
            );
            let resp = client
                .get(format!("{}/manga/{}?fields=title,alternative_titles,media_type", &self._base_url, &id))
                .headers(headers)
                .send()
                .await?;

            
            match resp.status().as_str() {
                "200" => {
                    let json = resp.json::<Manga>().await?;
                    if json.media_type == "light_novel" {
                        println!("{:#?}", json.title);
                        let file = OpenOptions::new().write(true).append(true).open("light_novels.csv").unwrap();
                        let mut writer = csv::Writer::from_writer(file);

                        writer.write_record(&[json.title, json.alternative_titles.en, json.alternative_titles.ja])?;

                        writer.flush()?;
                    }
                    
                }
                _ => ()
            }
            Ok(())
        }
    }

}
