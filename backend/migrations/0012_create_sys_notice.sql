-- System Notice Table
CREATE TABLE sys_notice (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    notice_title VARCHAR(200) NOT NULL,
    notice_type CHAR(1) NOT NULL,
    notice_content TEXT,
    notice_status CHAR(1) DEFAULT '0',
    is_top CHAR(1) DEFAULT '0',
    priority INTEGER DEFAULT 0,
    publisher_id UUID,
    publisher_name VARCHAR(50),
    publish_time TIMESTAMP WITH TIME ZONE,
    view_count INTEGER DEFAULT 0,
    created_by UUID,
    updated_by UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_notice_type ON sys_notice(notice_type);
CREATE INDEX idx_notice_status ON sys_notice(notice_status);
CREATE INDEX idx_notice_publish_time ON sys_notice(publish_time);
