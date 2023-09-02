use reqwest::header;
use serde_json::Value;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "";
    let url = "https://api.github.com/repos/Simperby-Ethcon/dev-chain/commits";
    
    let client = reqwest::Client::new();

    loop {
        let res = client
            .get(url)
            .header(header::AUTHORIZATION, format!("token {}", token))
            .header(header::USER_AGENT, "my-app")
            .send()
            .await?;

        println!("{:#?}", res.status());

        let commits: Vec<Value> = res.json().await?;
        if let Some(commit) = commits.first() {
            if let Some(commit_obj) = commit.get("commit") {
                if let Some(message) = commit_obj.get("message") {
                    let msg = message.as_str().unwrap();
                    println!("Latest commit message: {}", msg);
                }
            }
            if let Some(sha) = commit.get("sha") {
                println!("Latest commit sha: {}", sha);

                let commit_url = format!("https://api.github.com/repos/OWNER/REPO/git/commits/{}", sha.as_str().unwrap());
                let commit_res = client
                    .get(&commit_url)
                    .header(header::AUTHORIZATION, format!("token {}", token))
                    .header(header::USER_AGENT, "my-app")
                    .send()
                    .await?;

                println!("{:#?}", commit_res.status());

                let commit_data: Value = commit_res.json().await?;
                if let Some(files) = commit_data.get("files") {
                    println!("Files changed in latest commit: {:#?}", files);
                }
            }
        }

        sleep(Duration::from_secs(60)).await;
    }

    Ok(())
}
