#[derive(Debug, clap::Subcommand)]
pub enum CommunityAction {
    /// Search for communities
    Search {
        /// Search query
        query: String,
        /// Maximum number of results (10-100)
        #[arg(long, default_value = "10")]
        max_results: u32,
        /// Pagination token
        #[arg(long)]
        next_token: Option<String>,
    },
    /// Get a community by ID
    Get {
        /// Community ID
        id: String,
    },
    /// Post to a community
    Post {
        /// Community ID
        id: String,
        /// Post text content
        text: String,
    },
}
