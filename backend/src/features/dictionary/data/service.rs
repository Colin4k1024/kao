use crate::common::cache::redis::RedisCache;
use crate::common::error::AppError;
use uuid::Uuid;

use super::{
    model::{CreateDataRequest, UpdateDataRequest, DataResponse},
    repo::DataRepository,
};

#[derive(Default)]
pub struct DataService;

impl DataService {
    pub fn new() -> Self {
        Self {}
    }

    /// List dictionary data by type with caching (TTL: 1 hour)
    pub async fn list_data_by_type_cached(
        &self,
        db: &sqlx::PgPool,
        cache: &RedisCache,
        dict_type: &str,
    ) -> Result<Vec<DataResponse>, AppError> {
        let cache_key = format!("dict:data:{}", dict_type);

        // Try to get from cache first
        if let Some(cached) = cache.get::<Vec<DataResponse>>(&cache_key).await? {
            tracing::debug!("Dictionary data cache hit for type: {}", dict_type);
            return Ok(cached);
        }

        // Cache miss - query database
        tracing::debug!("Dictionary data cache miss for type: {}, querying database", dict_type);
        let data = self.list_data_by_type(db, dict_type).await?;

        // Store in cache with 1 hour TTL
        cache.set(&cache_key, &data).await?;

        Ok(data)
    }

    pub async fn list_data_by_type(
        &self,
        db: &sqlx::PgPool,
        dict_type: &str,
    ) -> Result<Vec<DataResponse>, AppError> {
        let data = DataRepository::list_data_by_type(db, dict_type).await?;
        let responses: Vec<DataResponse> = data
            .into_iter()
            .map(|d| DataResponse {
                id: d.id,
                dict_sort: d.dict_sort,
                dict_label: d.dict_label,
                dict_value: d.dict_value,
                dict_type: d.dict_type,
                css_class: d.css_class,
                list_class: d.list_class,
                is_default: d.is_default,
                status: d.status,
                remark: d.remark,
                created_by: d.created_by,
                updated_by: d.updated_by,
                created_at: d.created_at,
                updated_at: d.updated_at,
            })
            .collect();
        Ok(responses)
    }

    pub async fn get_data_by_id(
        &self,
        db: &sqlx::PgPool,
        data_id: Uuid,
    ) -> Result<Option<DataResponse>, AppError> {
        let d = DataRepository::get_data_by_id(db, data_id).await?;
        Ok(d.map(|row| DataResponse {
            id: row.id,
            dict_sort: row.dict_sort,
            dict_label: row.dict_label,
            dict_value: row.dict_value,
            dict_type: row.dict_type,
            css_class: row.css_class,
            list_class: row.list_class,
            is_default: row.is_default,
            status: row.status,
            remark: row.remark,
            created_by: row.created_by,
            updated_by: row.updated_by,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }))
    }

    pub async fn create_data(
        &self,
        db: &sqlx::PgPool,
        cache: &RedisCache,
        req: CreateDataRequest,
    ) -> Result<DataResponse, AppError> {
        let d = DataRepository::create_data(
            db,
            req.dict_sort,
            req.dict_label,
            req.dict_value,
            req.dict_type.clone(),
            req.css_class,
            req.list_class,
            req.is_default,
            req.status,
            req.remark,
        )
        .await?;

        // Invalidate dictionary data cache for this type after successful creation
        let cache_key = format!("dict:data:{}", req.dict_type);
        cache.invalidate(&cache_key).await;

        Ok(DataResponse {
            id: d.id,
            dict_sort: d.dict_sort,
            dict_label: d.dict_label,
            dict_value: d.dict_value,
            dict_type: d.dict_type,
            css_class: d.css_class,
            list_class: d.list_class,
            is_default: d.is_default,
            status: d.status,
            remark: d.remark,
            created_by: d.created_by,
            updated_by: d.updated_by,
            created_at: d.created_at,
            updated_at: d.updated_at,
        })
    }

    pub async fn update_data(
        &self,
        db: &sqlx::PgPool,
        cache: &RedisCache,
        data_id: Uuid,
        req: UpdateDataRequest,
    ) -> Result<DataResponse, AppError> {
        let d = DataRepository::update_data(
            db,
            data_id,
            req.dict_sort,
            req.dict_label,
            req.dict_value,
            req.dict_type.clone(),
            req.css_class,
            req.list_class,
            req.is_default,
            req.status,
            req.remark,
        )
        .await?;

        // Invalidate dictionary data cache for this type after successful update
        let cache_key = format!("dict:data:{}", d.dict_type);
        cache.invalidate(&cache_key).await;

        Ok(DataResponse {
            id: d.id,
            dict_sort: d.dict_sort,
            dict_label: d.dict_label,
            dict_value: d.dict_value,
            dict_type: d.dict_type,
            css_class: d.css_class,
            list_class: d.list_class,
            is_default: d.is_default,
            status: d.status,
            remark: d.remark,
            created_by: d.created_by,
            updated_by: d.updated_by,
            created_at: d.created_at,
            updated_at: d.updated_at,
        })
    }

    pub async fn delete_data(
        &self,
        db: &sqlx::PgPool,
        cache: &RedisCache,
        data_id: Uuid,
    ) -> Result<(), AppError> {
        // First get the data to know its type for cache invalidation
        let data = DataRepository::get_data_by_id(db, data_id).await?;
        let dict_type = data.map(|d| d.dict_type);

        // Delete from database - only invalidate cache on success
        DataRepository::delete_data(db, data_id).await?;

        // Invalidate dictionary data cache for this type after successful deletion
        if let Some(t) = dict_type {
            let cache_key = format!("dict:data:{}", t);
            cache.invalidate(&cache_key).await;
        }

        Ok(())
    }
}
