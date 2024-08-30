use crate::model;

pub async fn fetch_github_activities(username: &str) -> Result<(), String> {
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();

    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("RUST-CLI-APP"),
    );

    let url: String = format!("https://api.github.com/users/{}/events", username);
    let response = client.get(url).headers(headers).send().await;

    match response {
        Ok(response) => {
            let response_body = response.text().await;

            let events = serde_json::from_str::<Vec<model::Event>>(&response_body.unwrap());

            match events {
                Ok(events) => {
                    if events.is_empty() {
                        println!("\nNo events found for the user {} recently", username);
                        println!("-----------------------------------");
                        return Ok(());
                    }

                    for event in events {
                        println!("{}", normalize_event(event));
                    }
                    println!("-----------------------------------");
                    print!("All activities fetched successfully!");
                }
                Err(_) => {
                    println!("Oops, user {} may not exists, please give another try", username);
                }
            }

            Ok(())
        }
        Err(_) => Err("Failed to fetch data".to_string()),
    }
}

// Output:
// Actor pushed <commit_length> commits to <repo> at <created_at>
// Actor starred <repo> at <created_at>
// Actor forked <repo> at <created_at>
// Actor created a repository <repo> at <created_at>
fn normalize_event(event: model::Event) -> String {
    match event.event_type.as_str() {
        "PushEvent" => {
            let commit_length = event.payload["size"].as_u64().unwrap();
            format!(
                "{} pushed {} commits to {} at {}",
                event.actor.display_login, commit_length, event.repo.name, event.created_at
            )
        }
        "WatchEvent" => {
            format!(
                "{} starred {} at {}",
                event.actor.display_login, event.repo.name, event.created_at
            )
        }
        "ForkEvent" => {
            format!(
                "{} forked {} at {}",
                event.actor.display_login, event.repo.name, event.created_at
            )
        }
        "CreateEvent" => {
            format!(
                "{} created a repository {} at {}",
                event.actor.display_login, event.repo.name, event.created_at
            )
        }
        "PullRequestEvent" => {
            format!(
                "{} opened a pull request at {}",
                event.actor.display_login, event.created_at
            )
        }
        "PublicEvent" => {
            format!(
                "{} open sourced a repository at {}",
                event.actor.display_login, event.created_at
            )
        }
        _ => format!(
            "{} did {} something at {}",
            event.actor.display_login, event.event_type, event.created_at
        ),
    }
}
