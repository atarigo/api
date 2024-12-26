use crate::user::model;
use anyhow::Result;
use sqlx::SqlitePool;

pub async fn debug(pool: &SqlitePool) -> Result<()> {
    let user = model::CreateUser {
        id: "foo".to_string(),
        email: "bar@example.com".to_string(),
        username: "bar".to_string(),
    };
    model::create_user(pool, &user).await?;

    let users = model::read_user(pool, None).await?;
    println!("users: [{}]", users.len());

    let user = model::UpdateUser {
        email: Some("bar2@example.com".to_string()),
        username: Some("bat2".to_string()),
    };
    model::update_user(pool, "foo", &user).await?;

    model::delete_user(pool, "foo").await?;

    Ok(())
}

/*
async fn register_user(pool: &SqlitePool, session: String) -> Result<()> {
    println!("__session in cookies: {}", session);
    // decord JTW `session`
    // get user information

    // fixme: typing issue
    let user = model::CreateUser {
        id: "foo".to_string(),
        email: "bar@example.com".to_string(),
        username: "bar".to_string(),
    };
    model::create_user(pool, &user).await?;
    Ok(())
}
*/

// fn login_user(pool: &SqlitePool, user) {}

// fn logout_user() {}
