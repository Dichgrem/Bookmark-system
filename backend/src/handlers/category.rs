use crate::auth::AuthUser;
use crate::error::AppError;
use crate::models::Category;
use crate::result::ApiResult;
use crate::AppState;
use axum::extract::State;
use axum::Json;
use rusqlite::params_from_iter;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub id: i64,
}

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<ApiResult<Vec<Category>>, AppError> {
    let conn = state.db.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, name, user_id, parent_id, sort_order
         FROM category WHERE user_id = ?1 ORDER BY sort_order ASC",
    )?;
    let categories: Vec<Category> = stmt
        .query_map(rusqlite::params![auth.0], |row| {
            Ok(Category {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                user_id: row.get(2)?,
                parent_id: row.get(3)?,
                sort_order: row.get(4)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(ApiResult::success(categories))
}

pub async fn add(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(mut category): Json<Category>,
) -> Result<ApiResult<Category>, AppError> {
    category.name = category.name.trim().to_string();
    if category.name.is_empty() {
        return Ok(ApiResult::error("分类名称不能为空"));
    }
    if category.name.len() > 200 {
        return Ok(ApiResult::error("分类名称过长"));
    }
    let conn = state.db.get()?;

    if let Some(id) = category.id {
        let owner: Option<i64> = conn
            .query_row(
                "SELECT user_id FROM category WHERE id = ?1",
                rusqlite::params![id],
                |row| row.get(0),
            )
            .ok();
        if owner != Some(auth.0) {
            return Ok(ApiResult::error("无权操作此分类"));
        }
        conn.execute(
            "UPDATE category SET name=?1, user_id=?2, parent_id=?3, sort_order=?4 WHERE id=?5",
            rusqlite::params![
                category.name,
                auth.0,
                category.parent_id,
                category.sort_order,
                id
            ],
        )?;
        category.user_id = auth.0;
        Ok(ApiResult::success(category))
    } else {
        conn.execute(
            "INSERT INTO category (name, user_id, parent_id, sort_order) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                category.name,
                auth.0,
                category.parent_id,
                category.sort_order
            ],
        )?;
        let id = conn.last_insert_rowid();
        Ok(ApiResult::success(Category {
            id: Some(id),
            name: category.name,
            user_id: auth.0,
            parent_id: category.parent_id,
            sort_order: category.sort_order,
        }))
    }
}

pub async fn batch_update(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(categories): Json<Vec<Category>>,
) -> Result<ApiResult<()>, AppError> {
    if categories.len() > 1000 {
        return Ok(ApiResult::error("单次更新数量不能超过 1000"));
    }
    let ids: Vec<i64> = categories.iter().filter_map(|c| c.id).collect();
    if ids.is_empty() {
        return Ok(ApiResult::success_empty());
    }
    let mut conn = state.db.get()?;
    let tx = conn.transaction()?;

    let placeholders: Vec<String> = ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("?{}", i + 1))
        .collect();
    let sql = format!(
        "SELECT id, user_id FROM category WHERE id IN ({})",
        placeholders.join(",")
    );
    let owners: HashMap<i64, i64> = {
        let mut stmt = tx.prepare(&sql)?;
        let pairs: Vec<(i64, i64)> = stmt
            .query_map(params_from_iter(ids), |row| Ok((row.get(0)?, row.get(1)?)))?
            .filter_map(|r| r.ok())
            .collect();
        pairs.into_iter().collect()
    };

    for cat in &categories {
        if let Some(id) = cat.id {
            if owners.get(&id) == Some(&auth.0) {
                tx.execute(
                    "UPDATE category SET name=?1, user_id=?2, parent_id=?3, sort_order=?4 WHERE id=?5",
                    rusqlite::params![cat.name, auth.0, cat.parent_id, cat.sort_order, id],
                )?;
            }
        }
    }
    tx.commit()?;
    Ok(ApiResult::success_empty())
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<DeleteRequest>,
) -> Result<ApiResult<()>, AppError> {
    let mut conn = state.db.get()?;
    let owner: Option<i64> = conn
        .query_row(
            "SELECT user_id FROM category WHERE id = ?1",
            rusqlite::params![req.id],
            |row| row.get(0),
        )
        .ok();
    if owner != Some(auth.0) {
        return Ok(ApiResult::error("无权操作此分类"));
    }
    let mut ids = Vec::new();
    collect_category_ids(&conn, req.id, &mut ids)?;
    let tx = conn.transaction()?;
    for id in &ids {
        tx.execute(
            "DELETE FROM bookmark WHERE category_id = ?1",
            rusqlite::params![id],
        )?;
        tx.execute("DELETE FROM category WHERE id = ?1", rusqlite::params![id])?;
    }
    tx.commit()?;
    Ok(ApiResult::success_empty())
}

fn collect_category_ids(
    conn: &rusqlite::Connection,
    parent_id: i64,
    ids: &mut Vec<i64>,
) -> Result<(), rusqlite::Error> {
    ids.push(parent_id);
    let mut stmt = conn.prepare("SELECT id FROM category WHERE parent_id = ?1")?;
    let children: Vec<i64> = stmt
        .query_map(rusqlite::params![parent_id], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();
    for child_id in children {
        collect_category_ids(conn, child_id, ids)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_db() -> rusqlite::Connection {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE category (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                user_id INTEGER NOT NULL,
                parent_id INTEGER,
                sort_order INTEGER DEFAULT 0
            );",
        )
        .unwrap();
        conn
    }

    #[test]
    fn collect_single_category() {
        let conn = setup_db();
        let mut ids = Vec::new();
        collect_category_ids(&conn, 1, &mut ids).unwrap();
        assert_eq!(ids, vec![1]);
    }

    #[test]
    fn collect_nested_categories() {
        let conn = setup_db();
        conn.execute(
            "INSERT INTO category (id, name, user_id) VALUES (1, 'root', 1)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO category (id, name, user_id, parent_id) VALUES (2, 'child', 1, 1)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO category (id, name, user_id, parent_id) VALUES (3, 'grandchild', 1, 2)",
            [],
        )
        .unwrap();

        let mut ids = Vec::new();
        collect_category_ids(&conn, 1, &mut ids).unwrap();
        assert_eq!(ids.len(), 3);
        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
        assert!(ids.contains(&3));
    }
}
