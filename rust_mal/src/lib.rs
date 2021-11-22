pub mod mal {
    use reqwest::header::{HeaderMap, HeaderValue};
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Manga {
        data: Vec<Data>,
        paging: Paging,
    }

    #[derive(Debug, Deserialize)]
    struct Data {
        node: Node,
        ranking: Ranking,
    }

    #[derive(Debug, Deserialize)]
    struct Node {
        id: u32,
        title: String,
        main_picture: Picture,
    }

    #[derive(Debug, Deserialize)]
    struct Ranking {
        rank: u32,
    } 

    #[derive(Debug, Deserialize)]
    struct Paging {
        next: String,
    }

    #[derive(Debug, Deserialize)]
    struct Picture {
        medium: String,
        large: String,
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
        pub async fn get_light_novels(&self) -> Result<(), Box<dyn std::error::Error>> {
            let client = reqwest::Client::new();
            let mut headers = HeaderMap::new();
            headers.insert(
                "X-MAL-CLIENT-ID",
                format!("{}", &self._token)
                    .parse::<HeaderValue>()
                    .unwrap(),
            );
            let resp = client
                .get(format!("{}/manga/ranking?ranking_type=novels&limit=500'", &self._base_url))
                .headers(headers)
                .send()
                .await?;

            let json = resp.json::<Manga>().await?;
            println!("{:#?}", json);
            Ok(())
        }
    }

}
