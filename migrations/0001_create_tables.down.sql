-- Add down migration script here
-- Drop triggers
DROP TRIGGER IF EXISTS update_submission_comment_replies_updated_at ON submission_comment_replies;
DROP TRIGGER IF EXISTS update_task_comment_replies_updated_at ON task_comment_replies;
DROP TRIGGER IF EXISTS update_submission_comments_updated_at ON submission_comments;
DROP TRIGGER IF EXISTS update_submission_comments_updated_at ON task_comments;
DROP TRIGGER IF EXISTS update_submission_ratings_updated_at ON submission_ratings;
DROP TRIGGER IF EXISTS update_submissions_updated_at ON submissions;
DROP TRIGGER IF EXISTS update_task_ratings_updated_at ON task_ratings;
DROP TRIGGER IF EXISTS update_problems_or_tasks_updated_at ON problems_or_tasks;
DROP TRIGGER IF EXISTS update_accounts_updated_at ON accounts;
DROP TRIGGER IF EXISTS update_users_updated_at ON users;
DROP TRIGGER IF EXISTS update_task_comments_is_edited ON taks_comments;
DROP TRIGGER IF EXISTS update_submission_comments_is_edited ON submission_comments;
DROP TRIGGER IF EXISTS update_task_comment_replies_is_edited ON task_comment_replies;
DROP TRIGGER IF EXISTS update_submission_comment_replies_is_edited ON submission_comment_replies;

-- Drop function
DROP FUNCTION IF EXISTS update_updated_at_column();
DROP FUNCTION IF EXISTS update_is_edited_column();

-- Drop tables in reverse order (respecting foreign key dependencies)
DROP TABLE IF EXISTS submission_comment_replies;
DROP TABLE IF EXISTS task_comment_replies;
DROP TABLE IF EXISTS submission_comments;
DROP TABLE IF EXISTS submission_comments;
DROP TABLE IF EXISTS submission_ratings;
DROP TABLE IF EXISTS task_ratings;
DROP TABLE IF EXISTS submissions;
DROP TABLE IF EXISTS problems_or_tasks;
DROP TABLE IF EXISTS accounts;
DROP TABLE IF EXISTS users;

-- drop the UUID extension
DROP EXTENSION IF EXISTS "uuid-ossp";
