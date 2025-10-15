-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255),
    display_name VARCHAR(255) NOT NULL,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    avatar_url TEXT,
    reputation_score DECIMAL(10, 2) DEFAULT 0,
    total_ratings_given INTEGER DEFAULT 0,
    total_ratings_received INTEGER DEFAULT 0,
    email_verified_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create accounts table
CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    provider VARCHAR(255) NOT NULL,
    provider_account_id VARCHAR(255) NOT NULL,
    refresh_token TEXT,
    expires_at TIMESTAMP,
    token_type VARCHAR(100),
    scope TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create problems_or_tasks table
CREATE TABLE problems_or_tasks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    title VARCHAR(500) NOT NULL,
    content TEXT NOT NULL,
    file_url TEXT,
    tags JSONB,
    difficulty VARCHAR(20) CHECK (difficulty IN ('easy', 'medium', 'hard')),
    average_rating DECIMAL(3, 2) DEFAULT 0,
    total_ratings INTEGER DEFAULT 0,
    total_submissions INTEGER DEFAULT 0,
    view_count INTEGER DEFAULT 0,
    deadline TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create task_ratings table
CREATE TABLE task_ratings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    task_id UUID NOT NULL,
    rater_id UUID NOT NULL,
    rating_value INTEGER NOT NULL CHECK (rating_value BETWEEN 1 AND 4),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (task_id) REFERENCES problems_or_tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (rater_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(task_id, rater_id)
);

-- Create submissions table
CREATE TABLE submissions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    task_id UUID NOT NULL,
    content TEXT NOT NULL,
    file_url TEXT,
    status VARCHAR(50) CHECK (status IN ('draft', 'submitted', 'under_review', 'accepted', 'rejected')),
    average_rating DECIMAL(3, 2) DEFAULT 0,
    total_ratings INTEGER DEFAULT 0,
    is_featured BOOLEAN DEFAULT false,
    submitted_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (task_id) REFERENCES problems_or_tasks(id) ON DELETE CASCADE
);

-- Create submission_ratings table
CREATE TABLE submission_ratings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    submission_id UUID NOT NULL,
    rater_id UUID NOT NULL,
    rating_value INTEGER NOT NULL CHECK (rating_value BETWEEN 1 AND 4),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (submission_id) REFERENCES submissions(id) ON DELETE CASCADE,
    FOREIGN KEY (rater_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE(submission_id, rater_id)
);

-- Create task_comments table
CREATE TABLE task_comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    task_id UUID NOT NULL,
    user_id UUID NOT NULL,
    comment TEXT NOT NULL,
    is_edited BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    FOREIGN KEY (task_id) REFERENCES problems_or_tasks(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create submission_comments table
CREATE TABLE submission_comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    submission_id UUID NOT NULL,
    user_id UUID NOT NULL,
    comment TEXT NOT NULL,
    is_edited BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    FOREIGN KEY (submission_id) REFERENCES submissions(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create task_comment_replies table
CREATE TABLE task_comment_replies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    task_comment_id UUID NOT NULL,
    user_id UUID NOT NULL,
    reply TEXT NOT NULL,
    is_edited BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    FOREIGN KEY (task_comment_id) REFERENCES task_comments(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create submission_comment_replies table
CREATE TABLE submission_comment_replies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    submission_comment_id UUID NOT NULL,
    user_id UUID NOT NULL,
    reply TEXT NOT NULL,
    is_edited BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    FOREIGN KEY (submission_comment_id) REFERENCES submission_comments(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create indexes for better query performance
CREATE INDEX idx_accounts_user_id ON accounts(user_id);
CREATE INDEX idx_accounts_provider ON accounts(provider, provider_account_id);

CREATE INDEX idx_problems_or_tasks_user_id ON problems_or_tasks(user_id);
CREATE INDEX idx_problems_or_tasks_difficulty ON problems_or_tasks(difficulty);
CREATE INDEX idx_problems_or_tasks_deleted_at ON problems_or_tasks(deleted_at);
CREATE INDEX idx_problems_or_tasks_tags ON problems_or_tasks USING gin(tags);

CREATE INDEX idx_task_ratings_task_id ON task_ratings(task_id);
CREATE INDEX idx_task_ratings_rater_id ON task_ratings(rater_id);

CREATE INDEX idx_submissions_user_id ON submissions(user_id);
CREATE INDEX idx_submissions_task_id ON submissions(task_id);
CREATE INDEX idx_submissions_status ON submissions(status);
CREATE INDEX idx_submissions_deleted_at ON submissions(deleted_at);

CREATE INDEX idx_submission_ratings_submission_id ON submission_ratings(submission_id);
CREATE INDEX idx_submission_ratings_rater_id ON submission_ratings(rater_id);

CREATE INDEX idx_task_comments_task_id ON task_comments(task_id);
CREATE INDEX idx_task_comments_user_id ON task_comments(user_id);
CREATE INDEX idx_task_comments_deleted_at ON task_comments(deleted_at);

CREATE INDEX idx_submission_comments_submission_id ON submission_comments(submission_id);
CREATE INDEX idx_submission_comments_user_id ON submission_comments(user_id);
CREATE INDEX idx_submission_comments_deleted_at ON submission_comments(deleted_at);

CREATE INDEX idx_task_comment_replies_task_comment_id ON task_comment_replies(task_comment_id);
CREATE INDEX idx_task_comment_replies_user_id ON task_comment_replies(user_id);
CREATE INDEX idx_task_comment_replies_deleted_at ON task_comment_replies(deleted_at);

CREATE INDEX idx_submission_comment_replies_submission_comment_id ON submission_comment_replies(submission_comment_id);
CREATE INDEX idx_submission_comment_replies_user_id ON submission_comment_replies(user_id);
CREATE INDEX idx_submission_comment_replies_deleted_at ON submission_comment_replies(deleted_at);

-- Create function to automatically update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create triggers for updated_at columns
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_accounts_updated_at BEFORE UPDATE ON accounts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_problems_or_tasks_updated_at BEFORE UPDATE ON problems_or_tasks
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_task_ratings_updated_at BEFORE UPDATE ON task_ratings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_submissions_updated_at BEFORE UPDATE ON submissions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_submission_ratings_updated_at BEFORE UPDATE ON submission_ratings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_task_comments_updated_at BEFORE UPDATE ON task_comments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_submission_comments_updated_at BEFORE UPDATE ON submission_comments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_task_comment_replies_updated_at BEFORE UPDATE ON task_comment_replies
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_submission_comment_replies_updated_at BEFORE UPDATE ON submission_comment_replies
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- update is_edited to true for task_comment_replies and submission_comment_replies
CREATE OR REPLACE FUNCTION update_is_edited_column()
RETURNS TRIGGER AS $$
    BEGIN
        NEW.is_edited = true;
        RETURN NEW;
    END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_task_comments_is_edited BEFORE UPDATE ON task_comments
    FOR EACH ROW EXECUTE FUNCTION update_is_edited_column();

CREATE TRIGGER update_submission_comments_is_edited BEFORE UPDATE ON submission_comments
    FOR EACH ROW EXECUTE FUNCTION update_is_edited_column();
    
CREATE TRIGGER update_task_comment_replies_is_edited BEFORE UPDATE ON task_comment_replies
    FOR EACH ROW EXECUTE FUNCTION update_is_edited_column();

CREATE TRIGGER update_submission_comment_replies_is_edited BEFORE UPDATE ON submission_comment_replies
    FOR EACH ROW EXECUTE FUNCTION update_is_edited_column();
