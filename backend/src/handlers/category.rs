use crate::auth::AuthUser;
use crate::error::AppError;
use crate::models::Category;
use crate::result::ApiResult;
use crate::AppState;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub id: i64,
}

pub async fn list(
    State(state): State<AppState>,
    _auth: AuthUser,
) -> Result<ApiResult<Vec<Category>>, AppError> {
    let conn = state.db.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, name, parent_id, sort_order
         FROM category ORDER BY sort_order ASC",
    )?;
    let categories: Vec<Category> = stmt
        .query_map([], |row| {
            Ok(Category {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                parent_id: row.get(2)?,
                sort_order: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ApiResult::success(categories))
}

pub async fn add(
    State(state): State<AppState>,
    _auth: AuthUser,
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
        conn.execute(
            "UPDATE category SET name=?1, parent_id=?2, sort_order=?3 WHERE id=?4",
            rusqlite::params![category.name, category.parent_id, category.sort_order, id],
        )?;
        Ok(ApiResult::success(category))
    } else {
        conn.execute(
            "INSERT INTO category (name, parent_id, sort_order) VALUES (?1, ?2, ?3)",
            rusqlite::params![category.name, category.parent_id, category.sort_order],
        )?;
        let id = conn.last_insert_rowid();
        Ok(ApiResult::success(Category {
            id: Some(id),
            name: category.name,
            parent_id: category.parent_id,
            sort_order: category.sort_order,
        }))
    }
}

pub async fn batch_update(
    State(state): State<AppState>,
    _auth: AuthUser,
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

    for cat in &categories {
        if let Some(id) = cat.id {
            tx.execute(
                "UPDATE category SET name=?1, parent_id=?2, sort_order=?3 WHERE id=?4",
                rusqlite::params![cat.name, cat.parent_id, cat.sort_order, id],
            )?;
        }
    }
    tx.commit()?;
    Ok(ApiResult::success_empty())
}

pub async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Json(req): Json<DeleteRequest>,
) -> Result<ApiResult<()>, AppError> {
    let mut conn = state.db.get()?;
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
        .collect::<Result<Vec<_>, _>>()?;
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
        conn.execute("INSERT INTO category (id, name) VALUES (1, 'root')", [])
            .unwrap();
        conn.execute(
            "INSERT INTO category (id, name, parent_id) VALUES (2, 'child', 1)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO category (id, name, parent_id) VALUES (3, 'grandchild', 2)",
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
