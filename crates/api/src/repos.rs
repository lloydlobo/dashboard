//! `repos`
//!
//! # `GH_TOKEN`
//!
//! The `GH_TOKEN` is a personal access token that can be generated from the Github website.
//!
//! <YOUR_PERSONAL_ACCESS_TOKEN>: minimum requirements — `admin:org`, `admin:public_key`,
//! `repo`.
//!
//! To generate it, follow these steps:
//!
//! * Log in to your Github account and go to your settings.
//! * In the left-side menu, click on Developer Settings.
//! * Click on Personal Access Tokens.
//! * Click on Generate new token.
//! * Provide a name for your token and select the scopes that you need.
//! * Click on Generate token.
//!
//! You will only be able to see the token once. Make sure to save it in a safe place.
//!
//! [See also](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/about-authentication-to-github)
//! Here are a few more suggestions to keep in mind:
//!
//! * If you're using the octocrab crate to make many API requests, it's a good idea to reuse the
//!   Client instance across multiple requests to reduce the overhead of creating a new client for
//!   each request.
//! * If you need to paginate through a large result set, such as a list of repositories, you can
//!   use the Page struct from the octocrab crate to make multiple API requests and retrieve the
//!   next page of results.
//! * You may want to handle error cases, such as rate limiting or network errors, by wrapping the
//!   API calls in a Result type and checking the result for errors.
//! * To ensure your code works as expected in a CI environment, you'll need to make sure that all
//!   required environment variables, including the GitHub access token, are set in the CI
//!   environment.
//! * It's recommended to store the access token in an encrypted format in the CI environment, such
//!   as using a secure environment variable, to keep the token secure.
//! * Consider using a dependency management tool such as cargo-vendor or cargo-lockfile to manage
//!   the dependencies of your Rust application, to ensure that the same dependencies are used
//!   across development, CI, and production environments.

use std::collections::HashMap;

use octocrab::{models::Repository, Octocrab, Page};
use serde::{Deserialize, Serialize};

/// `list_user_repos` prints the names of repositories for the authenticated user.
///
/// # Prerequisites
///
/// * Create a personal access token in your GitHub account settings.
/// * Store the access token as an environment variable, for example GH_TOKEN.
/// * In your Rust application, retrieve the access token from the environment variable using the
///   dotenv crate.
/// * Use the octocrab crate to make API requests to the GitHub API, passing the access token as an
///   argument when creating a client.
/// * Use the client to fetch information about repositories, such as a list of repositories for a
///   specific user.
///
/// [See also](https://github.com/XAMPPRocky/octocrab/blob/master/examples/list_repos_for_authenticated_user.rs)
pub async fn list_user_repos() -> octocrab::Result<Page<Repository>> {
    let token = std::env::var("GH_TOKEN").expect("GH_TOKEN env variable is required");
    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let repos = octocrab
        .current()
        .list_repos_for_authenticated_user()
        .type_("owner")
        .sort("updated")
        .per_page(100)
        .send()
        .await?;

    Ok(repos)
}

pub fn to_hashmap(repo: Repository) -> HashMap<String, String> {
    let repo = serde_json::to_value(repo).unwrap();
    repo.as_object()
        .unwrap()
        .iter()
        .map(|(key, value)| {
            let value_str = match value {
                serde_json::Value::String(s) => String::from(s),
                serde_json::Value::Null => String::new(),
                _ => value.to_string(),
            };
            (key.to_string(), value_str)
        })
        .collect()
}

pub fn try_repos(repos: Page<Repository>) -> octocrab::Result<Vec<List>> {
    let lists = get_list(repos, |r| {
        List::new()
            .with_name(r.name)
            .with_url(r.url.to_string())
            .with_description(r.description.unwrap_or_default())
    })?;

    Ok(lists)
}

// // let fields = vec!["name", "url", "description"];
// // let hash = repos.into_iter().map(|repo| to_hashmap(repo, &fields)).collect::<Vec<_>>();
// fn to_hashmap_with_check(repo: Repository, fields: &[&str]) -> HashMap<String, String> {
//     let mut res = HashMap::new();
//     for field in fields {
//         check_field!(field, str);
//         let value = repo.*field;
//         res.insert(field.to_string(), value.to_string());
//     }
//     res
// }

// let fields = vec!["name", "url"];
// let hash = repos.into_iter().map(|repo| to_hashmap(repo, fields)).collect::<Vec<_>>();
fn to_hashmap_with(repo: Repository, fields: Vec<&str>) -> HashMap<String, String> {
    let mut res = HashMap::new();

    let repo = serde_json::to_value(repo).unwrap();
    let obj = repo.as_object().unwrap();
    for (key, value) in obj {
        if fields.contains(&key.as_str()) {
            let value_str = match value {
                serde_json::Value::String(s) => String::from(s),
                serde_json::Value::Null => String::new(),
                _ => value.to_string(),
            };
            res.insert(key.to_string(), value_str.to_string());
        }
    }
    res
}

pub fn get_list(
    p: Page<Repository>,
    builder: impl Fn(Repository) -> List,
) -> octocrab::Result<Vec<List>> {
    Ok(p.into_iter().map(builder).collect())
}

// #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct List {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// impl Iterator for List {}
impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! check_field {
    ($field:ident, $ty:ty) => {
        match stringify!($field) {
            #[allow(unused_mut)]
            #[allow(unreachable_code)]
            #[allow(unused_variables)]
            stringify!(name) | stringify!(url) | stringify!(description) => {
                let mut _temp: &$ty = &Repository {};
                let _field = &_temp.$field;
            }
            _ => panic!(
                "The field `{}` does not exist on the `Repository` struct.",
                stringify!($field)
            ),
        }
    };
}

// In this example, the macro unwrap_struct generates the unwrap method for the List struct.
// This method iterates over all fields of the struct, unwraps them and inserts them into a new
// struct, which is then returned. This approach allows you to add more fields to the struct in the
// future, without having to update the unwrap method.
#[macro_export]
macro_rules! unwrap_struct_iter {
    ($struct:ident) => {
        impl $struct {
            pub fn unwrap(self) -> $struct {
                let new_struct = Self::default();
                // for field in self.into_iter() { match field { Some(value) =>
                // new_struct.insert(value), None => {} } }
                new_struct
            }
        }
    };
}
unwrap_struct_iter!(List);

impl List {
    pub fn new() -> Self {
        Self { name: None, url: None, description: None }
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

#[cfg(test)]
mod tests {
    use octocrab::{Octocrab, Result};

    #[tokio::test]
    async fn test_list_user_repos_not_empty() -> Result<()> {
        dotenv::dotenv().ok();
        let token = std::env::var("GH_TOKEN").expect("GH_TOKEN env variable is required");
        let octocrab = Octocrab::builder().personal_token(token).build()?;

        let my_repos = octocrab
            .current()
            .list_repos_for_authenticated_user()
            .type_("owner")
            .sort("updated")
            .per_page(100)
            .send()
            .await?;

        // Assert that the response is not empty
        assert!(!my_repos.items.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_list_user_repos_check_links() -> Result<()> {
        // Limit the time the test can take
        let time_limit = std::time::Duration::from_secs(5);
        // Start timer to limit the time the test takes
        let start_time = std::time::Instant::now();

        // Load environment variables from .env file
        dotenv::dotenv().ok();

        // Get the token from environment variable
        let token = std::env::var("GH_TOKEN").expect("GH_TOKEN env variable is required");
        // Build the Octocrab client with the token
        let octocrab = Octocrab::builder().personal_token(token).build()?;
        // Get the list of repos for the authenticated user
        let repos = octocrab
            .current()
            .list_repos_for_authenticated_user()
            .type_("owner")
            .sort("updated")
            .per_page(100)
            .send()
            .await?;

        for repo in repos {
            // If the time elapsed is greater than the limit, break out of the loop
            if start_time.elapsed() > time_limit {
                break;
            }
            // Call the `check` function from the `lychee_lib` library and get the response
            let response = lychee_lib::check(repo.url.to_string()).await;
            assert!(response.is_ok());
        }

        Ok(())
    }
}
