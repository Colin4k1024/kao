use crate::common::error::AppError;

use super::{
    model::{CreatePostRequest, PostResponse, UpdatePostRequest},
    repo::{
        create_post, delete_post, get_post_by_code, get_post_by_id, list_posts, update_post,
    },
};

#[derive(Default)]
pub struct PostService;

impl PostService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn list_posts(&self, db: &sqlx::PgPool) -> Result<Vec<PostResponse>, AppError> {
        let posts = list_posts(db).await?;
        Ok(posts
            .into_iter()
            .map(|p| PostResponse {
                id: p.id,
                post_name: p.post_name,
                post_code: p.post_code,
                post_group: p.post_group,
                sort: p.sort,
                status: p.status,
                created_by: p.created_by,
                created_at: p.created_at,
                updated_at: p.updated_at,
            })
            .collect())
    }

    pub async fn get_post_by_id(
        &self,
        db: &sqlx::PgPool,
        post_id: i64,
    ) -> Result<Option<PostResponse>, AppError> {
        let post = get_post_by_id(db, post_id).await?;
        Ok(post.map(|p| PostResponse {
            id: p.id,
            post_name: p.post_name,
            post_code: p.post_code,
            post_group: p.post_group,
            sort: p.sort,
            status: p.status,
            created_by: p.created_by,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }))
    }

    pub async fn create_post(
        &self,
        db: &sqlx::PgPool,
        req: CreatePostRequest,
        created_by: Option<String>,
    ) -> Result<PostResponse, AppError> {
        // Check if post code already exists
        if get_post_by_code(db, &req.post_code).await?.is_some() {
            return Err(AppError::Validation {
                field: "post_code".to_string(),
                message: "Post code already exists".to_string(),
            });
        }

        let sort = req.sort.unwrap_or(0);
        let status = req.status.unwrap_or(1);

        let post = create_post(
            db,
            req.post_name,
            req.post_code,
            req.post_group,
            sort,
            status,
            created_by,
        )
        .await?;

        Ok(PostResponse {
            id: post.id,
            post_name: post.post_name,
            post_code: post.post_code,
            post_group: post.post_group,
            sort: post.sort,
            status: post.status,
            created_by: post.created_by,
            created_at: post.created_at,
            updated_at: post.updated_at,
        })
    }

    pub async fn update_post(
        &self,
        db: &sqlx::PgPool,
        post_id: i64,
        req: UpdatePostRequest,
    ) -> Result<PostResponse, AppError> {
        let post = update_post(
            db,
            post_id,
            req.post_name,
            req.post_code,
            req.post_group,
            req.sort,
            req.status,
        )
        .await?;

        Ok(PostResponse {
            id: post.id,
            post_name: post.post_name,
            post_code: post.post_code,
            post_group: post.post_group,
            sort: post.sort,
            status: post.status,
            created_by: post.created_by,
            created_at: post.created_at,
            updated_at: post.updated_at,
        })
    }

    pub async fn delete_post(&self, db: &sqlx::PgPool, post_id: i64) -> Result<(), AppError> {
        delete_post(db, post_id).await
    }
}
