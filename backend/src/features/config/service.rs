use crate::common::error::AppError;
use uuid::Uuid;

use super::{
    model::{CreateConfigRequest, UpdateConfigRequest, ConfigResponse},
    repo::ConfigRepository,
};

pub struct ConfigService;

impl ConfigService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn list_configs(
        &self,
        db: &sqlx::PgPool,
        config_key: Option<&str>,
        config_type: Option<&str>,
    ) -> Result<Vec<ConfigResponse>, AppError> {
        let configs = ConfigRepository::list_configs(db, config_key, config_type).await?;
        let responses: Vec<ConfigResponse> = configs
            .into_iter()
            .map(|c| ConfigResponse {
                id: c.id,
                config_name: c.config_name,
                config_key: c.config_key,
                config_value: c.config_value,
                config_type: c.config_type,
                is_encrypt: c.is_encrypt,
                status: c.status,
                remark: c.remark,
                created_by: c.created_by,
                updated_by: c.updated_by,
                created_at: c.created_at,
                updated_at: c.updated_at,
            })
            .collect();
        Ok(responses)
    }

    pub async fn get_config_by_key(
        &self,
        db: &sqlx::PgPool,
        config_key: &str,
    ) -> Result<Option<ConfigResponse>, AppError> {
        let c = ConfigRepository::get_config_by_key(db, config_key).await?;
        Ok(c.map(|row| ConfigResponse {
            id: row.id,
            config_name: row.config_name,
            config_key: row.config_key,
            config_value: row.config_value,
            config_type: row.config_type,
            is_encrypt: row.is_encrypt,
            status: row.status,
            remark: row.remark,
            created_by: row.created_by,
            updated_by: row.updated_by,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }))
    }

    pub async fn create_config(
        &self,
        db: &sqlx::PgPool,
        req: CreateConfigRequest,
    ) -> Result<ConfigResponse, AppError> {
        let c = ConfigRepository::create_config(
            db,
            req.config_name,
            req.config_key,
            req.config_value,
            req.config_type,
            req.is_encrypt,
            req.status,
            req.remark,
        )
        .await?;
        Ok(ConfigResponse {
            id: c.id,
            config_name: c.config_name,
            config_key: c.config_key,
            config_value: c.config_value,
            config_type: c.config_type,
            is_encrypt: c.is_encrypt,
            status: c.status,
            remark: c.remark,
            created_by: c.created_by,
            updated_by: c.updated_by,
            created_at: c.created_at,
            updated_at: c.updated_at,
        })
    }

    pub async fn update_config(
        &self,
        db: &sqlx::PgPool,
        config_key: &str,
        req: UpdateConfigRequest,
    ) -> Result<ConfigResponse, AppError> {
        let c = ConfigRepository::update_config(
            db,
            config_key,
            req.config_name,
            req.config_key,
            req.config_value,
            req.config_type,
            req.is_encrypt,
            req.status,
            req.remark,
        )
        .await?;
        Ok(ConfigResponse {
            id: c.id,
            config_name: c.config_name,
            config_key: c.config_key,
            config_value: c.config_value,
            config_type: c.config_type,
            is_encrypt: c.is_encrypt,
            status: c.status,
            remark: c.remark,
            created_by: c.created_by,
            updated_by: c.updated_by,
            created_at: c.created_at,
            updated_at: c.updated_at,
        })
    }

    pub async fn delete_config(&self, db: &sqlx::PgPool, config_key: &str) -> Result<(), AppError> {
        ConfigRepository::delete_config(db, config_key).await
    }
}
