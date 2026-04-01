use crate::api::types::{ApiResponse, Community, CommunityList, Tweet};
use crate::api::XClient;
use crate::error::AgentXError;

const COMMUNITY_FIELDS: &str = "id,name,description,created_at,member_count,access,join_policy";

impl XClient {
    pub async fn search_communities(
        &self,
        query_str: &str,
        max_results: u32,
        next_token: &Option<String>,
    ) -> Result<CommunityList, AgentXError> {
        let mut query = vec![
            ("query".to_string(), query_str.to_string()),
            ("community.fields".to_string(), COMMUNITY_FIELDS.to_string()),
        ];
        crate::api::pagination::apply_pagination_params(&mut query, max_results, next_token);

        let resp = self.get("/communities/search", &query).await?;
        let api: ApiResponse<Vec<Community>> = resp.json().await?;

        Ok(CommunityList {
            communities: api.data.unwrap_or_default(),
            next_token: api.meta.as_ref().and_then(|m| m.next_token.clone()),
            result_count: api.meta.as_ref().and_then(|m| m.result_count),
        })
    }

    pub async fn get_community(&self, id: &str) -> Result<Community, AgentXError> {
        let query = vec![("community.fields".to_string(), COMMUNITY_FIELDS.to_string())];

        let resp = self.get(&format!("/communities/{id}"), &query).await?;
        let api: ApiResponse<Community> = resp.json().await?;

        api.data
            .ok_or_else(|| AgentXError::NotFound(format!("Community {id} not found")))
    }

    pub async fn post_to_community(
        &self,
        community_id: &str,
        text: &str,
    ) -> Result<Tweet, AgentXError> {
        let body = serde_json::json!({
            "text": text,
            "community_id": community_id
        });
        let resp = self.post("/tweets", body).await?;
        let api: ApiResponse<Tweet> = resp.json().await?;
        api.data.ok_or_else(|| AgentXError::Api {
            status: 0,
            message: "No data in community post response".to_string(),
        })
    }
}
