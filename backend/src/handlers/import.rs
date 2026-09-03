use crate::error::AppError;
use crate::models::{Bookmark, Category};
use rusqlite::Connection;

pub fn extract_attr_value(line: &str) -> String {
    if let Some(close_start) = line.rfind("</") {
        let before_close = &line[..close_start];
        if let Some(pos) = before_close.rfind('>') {
            return before_close[pos + 1..].to_string();
        }
    }
    String::new()
}

pub fn insert_bookmark_if_new(
    conn: &Connection,
    bookmark: &Bookmark,
    user_id: i64,
) -> Result<Option<i64>, AppError> {
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM bookmark WHERE url = ?1 AND user_id = ?2",
            rusqlite::params![bookmark.url, user_id],
            |row| row.get::<_, i64>(0),
        )
        .map(|c| c > 0)
        .unwrap_or(false);

    if !exists {
        conn.execute(
            "INSERT INTO bookmark (title, url, icon, category_id, user_id, sort_order) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                bookmark.title,
                bookmark.url,
                bookmark.icon,
                bookmark.category_id,
                user_id,
                bookmark.sort_order,
            ],
        )?;
        let id = conn.last_insert_rowid();
        conn.execute(
            "UPDATE bookmark SET sort_order = ?1 WHERE id = ?1 AND sort_order = 0",
            rusqlite::params![id],
        )?;
        Ok(Some(1))
    } else {
        Ok(None)
    }
}

pub fn get_category_by_name(
    conn: &Connection,
    name: &str,
    user_id: i64,
    parent_id: Option<i64>,
) -> Option<Category> {
    conn.query_row(
        "SELECT id, name, user_id, parent_id, sort_order FROM category WHERE name = ?1 AND user_id = ?2 AND ((?3 IS NULL AND parent_id IS NULL) OR parent_id = ?3)",
        rusqlite::params![name, user_id, parent_id],
        |row| {
            Ok(Category {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                user_id: row.get(2)?,
                parent_id: row.get(3)?,
                sort_order: row.get(4)?,
            })
        },
    )
    .ok()
}

pub fn import_from_html(conn: &Connection, html: &str, user_id: i64) -> Result<i64, AppError> {
    let mut stack: Vec<i64> = Vec::new();
    let mut count = 0i64;

    for line in html.lines() {
        let trimmed = line.trim();
        if trimmed.contains("<DT><H3") {
            let name = extract_attr_value(trimmed);
            let name = name.trim().to_string();
            let parent_id = stack.last().copied();
            let cat = get_category_by_name(conn, &name, user_id, parent_id);
            let cat_id = if let Some(c) = cat {
                c.id.unwrap()
            } else {
                conn.execute(
                    "INSERT INTO category (name, user_id, parent_id, sort_order) VALUES (?1, ?2, ?3, 0)",
                    rusqlite::params![name, user_id, parent_id],
                )?;
                conn.last_insert_rowid()
            };
            stack.push(cat_id);
        } else if trimmed.contains("</DL>") {
            stack.pop();
        } else if trimmed.contains("<DT><A") {
            let url = trimmed
                .split("HREF=\"")
                .nth(1)
                .and_then(|s| s.split('"').next())
                .unwrap_or("")
                .to_string();
            let icon = trimmed
                .split("ICON=\"")
                .nth(1)
                .and_then(|s| s.split('"').next())
                .map(|s| s.to_string());
            let title = extract_attr_value(trimmed);
            let title = title.trim().to_string();

            if !url.is_empty() {
                let category_id = stack.last().copied();
                let bookmark = Bookmark {
                    id: None,
                    title,
                    url,
                    icon,
                    category_id,
                    user_id,
                    sort_order: 0,
                };
                if let Some(c) = insert_bookmark_if_new(conn, &bookmark, user_id)? {
                    count += c;
                }
            }
        }
    }

    Ok(count)
}
