use serde::{Deserialize, Serialize};
use std::fmt;

pub struct NetworkSystem {
    do_request_leaderboard: bool,
    submit_player_entry: Option<(String, f32)>,
    leaderboard_join_handle: Option<tokio::task::JoinHandle<Result<String, reqwest::Error>>>,
    submit_join_handle: Option<tokio::task::JoinHandle<()>>,
}

impl NetworkSystem {
    pub fn new() -> Self {
        NetworkSystem {
            do_request_leaderboard: false,
            submit_player_entry: None,
            leaderboard_join_handle: None,
            submit_join_handle: None,
        }
    }

    pub fn do_request_leaderboard(&mut self) {
        self.do_request_leaderboard = true;
    }

    pub fn do_submit_time_and_reqeust_leaderboard(&mut self, username: String, time: f32) {
        self.submit_player_entry = Some((username, time));
    }

    pub fn request_in_progress(&self) -> bool {
        self.do_request_leaderboard
    }

    pub async fn tick(&mut self) {
        if self.do_request_leaderboard {
            self.leaderboard_join_handle = Some(self.request_leaderboard().await);

            self.do_request_leaderboard = false;
        }

        if let Some((username, time)) = &self.submit_player_entry {
            let new_player_entry = PlayerEntry {
                username: username.to_string(),
                time: *time,
            };

            self.submit_join_handle = Some(self.submit_time(new_player_entry).await);
            self.submit_player_entry = None;
        }

        if let Some(submit_handle) = &self.submit_join_handle {
            if submit_handle.is_finished() {
                self.leaderboard_join_handle = Some(self.request_leaderboard().await);

                self.do_request_leaderboard = false;
                self.submit_join_handle = None;
            }
        }
    }

    async fn request_leaderboard(&self) -> tokio::task::JoinHandle<Result<String, reqwest::Error>> {
        let api_key = "bzkOp6qOAmopVFZhty69SSyB7OqTRDu1IqTs7TlLuBDeja7cDPGSaB0gL6c1IpBK";
        let client = reqwest::Client::new();

        tokio::spawn(async move {
            let response = client
                .get("https://data.mongodb-api.com/app/data-mjiob/endpoint/leaderboard")
                .header("api-key", api_key)
                .send()
                .await;

            match response {
                Ok(res) => {
                    let response_str = res.text().await.unwrap();

                    let leaderboard: Vec<PlayerEntry> =
                        serde_json::from_str(&response_str).unwrap();

                    let leaderboard_str: String = leaderboard
                        .iter()
                        .map(|entry| entry.to_string() + "\n\n")
                        .collect();

                    Ok(leaderboard_str)
                }
                Err(err) => Err(err),
            }
        })
    }

    async fn submit_time(&self, new_entry: PlayerEntry) -> tokio::task::JoinHandle<()> {
        let api_key = "bzkOp6qOAmopVFZhty69SSyB7OqTRDu1IqTs7TlLuBDeja7cDPGSaB0gL6c1IpBK";
        let client = reqwest::Client::new();

        tokio::spawn(async move {
            let response = client
                .put("https://data.mongodb-api.com/app/data-mjiob/endpoint/put")
                .header("api-key", api_key)
                .json(&new_entry)
                .send()
                .await;

            match response {
                Ok(_) => println!("Submit successful!"),
                Err(err) => println!("Error when posting submit: {:?}", err),
            }
        })
    }

    pub fn leaderboard_ready(&self) -> bool {
        if let Some(j_handle) = &self.leaderboard_join_handle {
            return j_handle.is_finished();
        }

        false
    }

    pub fn submit_finished(&self) -> bool {
        if let Some(j_handle) = &self.submit_join_handle {
            return j_handle.is_finished();
        }

        true
    }

    #[tokio::main]
    pub async fn get_response(self) -> String {
        if let Some(j_handle) = self.leaderboard_join_handle {
            return match j_handle.await {
                Ok(res) => match res {
                    Ok(str) => str,
                    Err(err) => format!("Error when getting leaderboard\n\n{}", err),
                },
                Err(err) => format!("Join error: {:?}", err),
            };
        }

        "Warning: thread has not completed yet...".to_owned()
    }
}

#[derive(Serialize, Deserialize)]
struct PlayerEntry {
    username: String,
    time: f32,
}

impl fmt::Display for PlayerEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.username, self.time)
    }
}
