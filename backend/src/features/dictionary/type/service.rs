use crate::common::cache::redis::RedisCache;
use crate::common::error::AppError;
use uuid::Uuid;

use super::{
    model::{CreateTypeRequest, UpdateTypeRequest, TypeResponse},
    repo::TypeRepository,
};

#[derive(Default)]
pub struct TypeService;

impl TypeService {
    pub fn new() -> Self {
        Self {}
    }

    /// List dictionary types with caching (TTL: 1 hour)
    pub async fn list_types_cached(
        &self,
        db: &sqlx::PgPool,
        cache: &RedisCache,
    ) -> Result<Vec<TypeResponse>, AppError> {
        // Try to get from cache first
        if let Some(cached) = cache.get::<Vec<TypeResponse>>("dict:type:list").await? {
            tracing::debug!("Dictionary type list cache hit");
            return Ok(cached);
        }

        // Cache miss - query database
        tracing::debug!("Dictionary type list cache miss, querying database");
        let types = self.list_types(db).await?;

        // Store in cache with 1 hour TTL
        cache.set("dict:type:list", &types).await?;

        Ok(types)
    }

    pub async fn list_types(&self, db: &sqlx::PgPool) -> Result<Vec<TypeResponse>, AppError> {
        let types = TypeRepository::list_types(db).await?;
        let responses: Vec<TypeResponse> = types
            .into_iter()
            .map(|t| TypeResponse {
                id: t.id,
                dict_name: t.dict_name,
                dict_type: t.dict_type,
                status: t.status,
                remark: t.remark,
                created_by: t.created_by,
                updated_by: t.updated_by,
                created_at: t.created_at,
                updated_at: t.updated_at,
            })
            .collect();
        Ok(responses)
    }

    pub async fn get_type_by_id(
        &self,
        db: &sqlx::PgPool,
        type_id: Uuid,
    ) -> Result<Option<TypeResponse>, AppError> {
        let t = TypeRepository::get_type_by_id(db, type_id).await?;
        Ok(t.map(|row| TypeResponse {
            id: row.id,
            dict_name: row.dict_name,
            dict_type: row.dict_type,
            status: row.status,
            remark: row.remark,
            created_by: row.created_by,
            updated_by: row.updated_by,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }))
    }

    pub async fn create_type(
        &self,
        db: &sqlx::PgPool,
        cache: &RedisCache,
        req: CreateTypeRequest,
    ) -> Result<TypeResponse, AppError> {
        let t = TypeRepository::create_type(
            db,
            req.dict_name,
            req.dict_type,
            req.status,
            req.remark,
        )
        .await?;

        // Invalidate dictionary type list cache after successful creation
        cache.invalidate("dict:type:list").await;

        Ok(TypeResponse {
            id: t.id,
            dict_name: t.dict_name,
            dict_type: t.dict_type,
            status: t.status,
            remark: t.remark,
            created_by: t.created_by,
            updated_by: t.updated_by,
            created_at: t.created_at,
            updated_at: t.updated_at,
        })
    }

    pub async fn update_type(
        &self,
        db: &sqlx::PgPool,
        cache: &RedisCache,
        type_id: Uuid,
        req: UpdateTypeRequest,
    ) -> Result<TypeResponse, AppError> {
        let t = TypeRepository::update_type(
            db,
            type_id,
            req.dict_name,
            req.dict_type,
            req.status,
            req.remark,
        )
        .await?;

        // Invalidate dictionary type list cache after successful update
        cache.invalidate("dict:type:list").await;

        Ok(TypeResponse {
            id: t.id,
            dict_name: t.dict_name,
            dict_type: t.dict_type,
            status: t.status,
            remark: t.remark,
            created_by: t.created_by,
            updated_by: t.updated_by,
            created_at: t.created_at,
            updated_at: t.updated_at,
        })
    }

    pub async fn delete_type(
        &self,
        db: &sqlx::PgPool,
        cache: &RedisCache,
        type_id: Uuid,
    ) -> Result<(), AppError> {
        // Delete from database first - only invalidate cache on success
        TypeRepository::delete_type(db, type_id).await?;

        // Invalidate dictionary type list cache after successful deletion
        cache.invalidate("dict:type:list").await;

        Ok(())
    }
}
