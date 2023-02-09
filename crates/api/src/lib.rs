//! # fetchcrab
//!
//! `fetchcrab` is a api wrapper around `octocrab` is a Rust library for accessing the GitHub API.
//! With `octocrab`, you can fetch information about repositories, users, etc. from the GitHub API.
//!
//! ## Prerequisites
//!
//! To use fetchcrab, you will need to have the following:
//!
//! * A personal access token (PAT) from your GitHub account.
//! * The dotenv crate to load environment variables in your Rust application.
//! * The octocrab crate to make API requests to the GitHub API.
//!
//! ## Usage
//!
//! An example of how to use octocrab to fetch a list of repositories for a specific user:
//!
//! ```rust
//! use api::repos::list_user_repos;
//! use octocrab::Result;
//!
//! #[tokio::main] // Requires `tokio` `full` features
//! async fn main() -> Result<()> {
//!     dotenv::dotenv().ok(); // Load environment variables from the .env file.
//!     list_user_repos().await?; // Fetch the list of repositories for the authenticated user.
//!     Ok(())
//! }
//! ```
//!
//! To handle error cases, such as rate limiting or network errors, you can wrap the API calls in a
//! Result type and check the result for errors.
//!
//! ## Setting up a Personal Access Token (PAT)
//!
//! A personal access token (PAT) is a secure way to access the GitHub API. To create a PAT, follow
//! these steps:
//!
//! * Log in to your GitHub account and go to your settings.
//! * In the left-side menu, click on Developer Settings.
//! * Click on Personal Access Tokens.
//! * Click on Generate new token.
//! * Provide a name for your token and select the scopes that you need (e.g. repo,
//!   admin:public_key, admin:org).
//! * Click on Generate token.
//!
//! You will only be able to see the token once, so make sure to save it in a safe place.
//!
//! For more information on personal access tokens and security,
//! see the [GitHub documentation](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/about-authentication-to-github).!

// #[macro_use]
// extern crate derive_builder;

pub mod repos;
// pub mod notifications;
// pub mod derive;

#[cfg(test)]
mod tests {
    use octocrab::Result;

    use crate::repos::list_user_repos;

    #[tokio::test]
    async fn test_list_user_repos() -> Result<()> {
        dotenv::dotenv().ok();
        assert!(list_user_repos().await.is_ok());
        Ok(())
    }
}
