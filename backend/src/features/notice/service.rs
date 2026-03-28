use crate::common::error::AppError;
use uuid::Uuid;

use super::{
    model::{CreateNoticeRequest, UpdateNoticeRequest, NoticeResponse},
    repo::NoticeRepository,
};

#[derive(Default)]
pub struct NoticeService;

impl NoticeService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn list_notices(
        &self,
        db: &sqlx::PgPool,
        notice_type: Option<&str>,
        notice_status: Option<&str>,
    ) -> Result<Vec<NoticeResponse>, AppError> {
        let notices = NoticeRepository::list_notices(db, notice_type, notice_status).await?;
        let responses: Vec<NoticeResponse> = notices
            .into_iter()
            .map(|n| NoticeResponse {
                id: n.id,
                notice_title: n.notice_title,
                notice_type: n.notice_type,
                notice_content: n.notice_content,
                notice_status: n.notice_status,
                is_top: n.is_top,
                priority: n.priority,
                publish_time: n.publish_time.map(|t| t.to_rfc3339()),
                view_count: n.view_count,
                publisher_id: n.publisher_id,
                publisher_name: n.publisher_name,
                created_by: n.created_by,
                updated_by: n.updated_by,
                created_at: n.created_at,
                updated_at: n.updated_at,
            })
            .collect();
        Ok(responses)
    }

    pub async fn get_notice_by_id(
        &self,
        db: &sqlx::PgPool,
        notice_id: Uuid,
    ) -> Result<Option<NoticeResponse>, AppError> {
        let n = NoticeRepository::get_notice_by_id(db, notice_id).await?;
        Ok(n.map(|row| NoticeResponse {
            id: row.id,
            notice_title: row.notice_title,
            notice_type: row.notice_type,
            notice_content: row.notice_content,
            notice_status: row.notice_status,
            is_top: row.is_top,
            priority: row.priority,
            publish_time: row.publish_time.map(|t| t.to_rfc3339()),
            view_count: row.view_count,
            publisher_id: row.publisher_id,
            publisher_name: row.publisher_name,
            created_by: row.created_by,
            updated_by: row.updated_by,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }))
    }

    pub async fn create_notice(
        &self,
        db: &sqlx::PgPool,
        req: CreateNoticeRequest,
    ) -> Result<NoticeResponse, AppError> {
        let n = NoticeRepository::create_notice(
            db,
            req.notice_title,
            req.notice_type,
            req.notice_content,
            req.notice_status,
            req.is_top,
            req.priority,
            req.publish_time,
        )
        .await?;
        Ok(NoticeResponse {
            id: n.id,
            notice_title: n.notice_title,
            notice_type: n.notice_type,
            notice_content: n.notice_content,
            notice_status: n.notice_status,
            is_top: n.is_top,
            priority: n.priority,
            publish_time: n.publish_time.map(|t| t.to_rfc3339()),
            view_count: n.view_count,
            publisher_id: n.publisher_id,
            publisher_name: n.publisher_name,
            created_by: n.created_by,
            updated_by: n.updated_by,
            created_at: n.created_at,
            updated_at: n.updated_at,
        })
    }

    pub async fn update_notice(
        &self,
        db: &sqlx::PgPool,
        notice_id: Uuid,
        req: UpdateNoticeRequest,
    ) -> Result<NoticeResponse, AppError> {
        let n = NoticeRepository::update_notice(
            db,
            notice_id,
            req.notice_title,
            req.notice_type,
            req.notice_content,
            req.notice_status,
            req.is_top,
            req.priority,
            req.publish_time,
        )
        .await?;
        Ok(NoticeResponse {
            id: n.id,
            notice_title: n.notice_title,
            notice_type: n.notice_type,
            notice_content: n.notice_content,
            notice_status: n.notice_status,
            is_top: n.is_top,
            priority: n.priority,
            publish_time: n.publish_time.map(|t| t.to_rfc3339()),
            view_count: n.view_count,
            publisher_id: n.publisher_id,
            publisher_name: n.publisher_name,
            created_by: n.created_by,
            updated_by: n.updated_by,
            created_at: n.created_at,
            updated_at: n.updated_at,
        })
    }

    pub async fn delete_notice(&self, db: &sqlx::PgPool, notice_id: Uuid) -> Result<(), AppError> {
        NoticeRepository::delete_notice(db, notice_id).await
    }

    pub async fn increment_view_count(
        &self,
        db: &sqlx::PgPool,
        notice_id: Uuid,
    ) -> Result<i32, AppError> {
        NoticeRepository::increment_view_count(db, notice_id).await
    }
}
