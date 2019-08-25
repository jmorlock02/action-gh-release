use reqwest::{Body, Client};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File};

#[derive(Serialize, Default, Debug, PartialEq)]
pub struct Release {
    pub tag_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
}

pub trait Releaser {
    fn release(
        &self,
        github_token: &str,
        github_repo: &str,
        release: Release,
    ) -> Result<usize, Box<dyn Error>>;
}

pub trait AssetUploader<A: Into<Body> = File> {
    fn upload(
        &self,
        github_token: &str,
        github_repo: &str,
        release_id: usize,
        mime: mime::Mime,
        asset: A,
    ) -> Result<(), Box<dyn Error>>;
}

#[derive(Deserialize)]
struct ReleaseResponse {
    id: usize,
}

impl Releaser for Client {
    // https://developer.github.com/v3/repos/releases/#create-a-release
    fn release(
        &self,
        github_token: &str,
        github_repo: &str,
        release: Release,
    ) -> Result<usize, Box<dyn Error>> {
        let response: ReleaseResponse = self
            .post(&format!(
                "https://api.github.com/repos/{}/releases",
                github_repo
            ))
            .header("Authorization", format!("bearer {}", github_token))
            .json(&release)
            .send()?
            .json()?;
        Ok(response.id)
    }
}

impl<A: Into<Body>> AssetUploader<A> for Client {
    // https://developer.github.com/v3/repos/releases/#upload-a-release-asset
    fn upload(
        &self,
        github_token: &str,
        github_repo: &str,
        release_id: usize,
        mime: mime::Mime,
        asset: A,
    ) -> Result<(), Box<dyn Error>> {
        self.post(&format!(
            "http://uploads.github.com/repos/{}/releases/{}",
            github_repo, release_id
        ))
        .header("Authorization", format!("bearer {}", github_token))
        .header("Content-Type", mime.to_string())
        .body(asset)
        .send()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
