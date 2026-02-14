
// use anyhow::{Context, Ok};
use axum::{extract::{Path, Query, State}, response::IntoResponse, routing::{get, post}, Json, Router};
use axum_valid::Valid;
use rusqlite_from_row::FromRow;
use rusqlite::{ named_params, params_from_iter, types::Value};
use serde_json::json;
use tracing::info;


use crate::{biz::{PaginatorWith, Table, UserPayload}, svr::error::HttpResError, AppState};

pub fn build_user_routes() -> Router<AppState>{
    Router::new().nest("/user",Router::new()
        // query & create & for biz

        // query
        .route("/q", get(q))
        // create
        .route("/c", post(c))
        // .route("/cBulk", post(cBulk))
        // // update
        // .route("/u", post(u))
        // .route("/uBulk", post(uBulk))
        // // delete
        // .route("/d/{:id}", get(d))
        // .route("/dBulk", post(dBulk))

        // // real_delete
        // .route("/dReal/{:id}", get(dReal))
        // .route("/dRealBulk", post(dRealBulk))


    )
}
// info!("{:?}",  &new_user); // 异步高效输出

// ==================================================


async fn q(State(state): State<AppState>,Query(payload): Query<PaginatorWith<UserPayload>>) -> Result<impl IntoResponse, HttpResError> {
    let users = state.dmail_db.call(move |conn| {
        let mut sql_params_list: Vec<Value> = Vec::new();
        let mut sql = String::from("SELECT * FROM user WHERE deleted_at IS NULL");
        if payload.custom.age.is_some() {
            sql.push_str(" AND age = :age");
            sql_params_list.push(payload.custom.age.into());
        }
        if payload.custom.name.is_some() {
            sql.push_str(" AND name = :name");
            sql_params_list.push(payload.custom.name.into());
        }
        if payload.id.is_some() {
            sql.push_str(" AND id > :id LIMIT :ps");
            sql_params_list.push(payload.id.into());
            sql_params_list.push(payload.ps.into());
        } else {
            sql.push_str(" LIMIT :ps OFFSET :pn * :ps");
            sql_params_list.push(payload.ps.into());
            sql_params_list.push(payload.pn.into());
        };
        Ok(conn.prepare_cached(&sql)?.query_map(params_from_iter(sql_params_list),Table::<UserPayload>::try_from_row)?.collect::<Result<Vec<Table<UserPayload>>, rusqlite::Error>>()?)
    }).await?;
    // dbg!(&users);
    Ok(Json(users))
}


async fn c(State(state): State<AppState>,Json(payload): Json<UserPayload>) -> Result<impl IntoResponse, HttpResError> {
    let new_user = state.dmail_db.call(move |conn|  {
        Ok(conn.prepare_cached("INSERT INTO user (name, age) VALUES (:name, :age) RETURNING id, name, age, created_at;")?.query_row(named_params! {
            ":name": &payload.name,
            ":age": &payload.age,
        },Table::<UserPayload>::try_from_row)?)
    }).await?;

    Ok(Json(new_user))
}



// async fn cBulk(State(state): State<AppState>,Valid(Json(payload)): Valid<Json<Vec<UserPayload>>>) -> Result<impl IntoResponse, HttpResError> {
//     if payload.is_empty() { return Err(HttpResError::BadRequest("empty payload".to_owned()));}
//     let new_users = state.dmail_db.get().await?.interact(move |conn|  {
//         let values_placeholders = payload.iter().map(|_| "(?,?)").collect::<Vec<&str>>().join(",");
//         let sql = format!("INSERT INTO user (name, age) VALUES {} RETURNING id, name, age, created_at",values_placeholders);
//         let params_list: Vec<Value> = payload.into_iter().flat_map(|user_data| [user_data.name.into(),user_data.age.into()]).collect();
//         conn.prepare_cached(&sql)?.query_map(params_from_iter(params_list), Table::<UserPayload>::try_from_row)?.collect::<Result<Vec<Table<UserPayload>>, Error>>()
//     }).await??;
//     Ok(Json(new_users))
// }


// // #[debug_handler] // 需要单独的添加 cargo add axum-macros
// async fn u(State(state): State<AppState>,Valid(Json(payload)): Valid<Json<Table<UserPayload>>>) -> Result<impl IntoResponse, HttpResError> {
//     state.dmail_db.get().await?.interact(move |conn|  {
//         conn.prepare_cached("UPDATE user SET name = :name, age = :age,updated_at = CURRENT_TIMESTAMP WHERE id = :id")?.execute(named_params!{
//             ":id": &payload.id,
//             ":name": &payload.custom.name,
//             ":age": &payload.custom.age,
//         })
//     }).await??;

//     Ok(())
// }


// async fn uBulk(State(state): State<AppState>,Valid(Json(payload)): Valid<Json<Vec<Table<UserPayload>>>>) -> Result<impl IntoResponse, HttpResError> {
//     if payload.is_empty() {return Err(HttpResError::BadRequest("empty payload".to_owned()));}
//     state.dmail_db.get().await?.interact(move |conn|  {
//         let sql = format!("
//             INSERT INTO user (id, name, age) VALUES {} ON CONFLICT(id) DO UPDATE SET updated_at = CURRENT_TIMESTAMP, \
//             name = excluded.name,\
//             age = excluded.age;",
//             payload.iter().map(|_| "(?,?,?)").collect::<Vec<_>>().join(",")
//         );
//         let params_list: Vec<Value> = payload.into_iter().flat_map(|item_data| {[
//             item_data.id.into(),
//             item_data.custom.name.into(),
//             item_data.custom.age.into(),
//         ]}).collect();
//         conn.prepare_cached(&sql)?.execute(params_from_iter(params_list))
//     }).await??;
//     Ok(())
// }



// async fn d(State(state): State<AppState>,Path(id): Path<i64>,) -> Result<impl IntoResponse, HttpResError> {
//     state.dmail_db.get().await?.interact(move |conn| {
//         conn.prepare_cached("UPDATE user SET deleted_at = CURRENT_TIMESTAMP WHERE id = :id")?.execute(named_params!{":id": id})
//     }).await??;
//     Ok(())
// }


// async fn dBulk(State(state): State<AppState>,Valid(Json(payload)): Valid<Json<Vec<Table<()>>>>) -> Result<impl IntoResponse, HttpResError> {
//     if payload.is_empty() {return Err(HttpResError::BadRequest("empty payload".to_owned()));}
//     state.dmail_db.get().await?.interact(move |conn|  {
//         let sql = format!("UPDATE user SET deleted_at = CURRENT_TIMESTAMP WHERE id IN ({})",payload.iter().map(|_| "?").collect::<Vec<_>>().join(","));
//         let params_list: Vec<Value> = payload.iter().map(|user| user.id.into()).collect();
//         conn.prepare_cached(&sql)?.execute(params_from_iter(params_list))
//     }).await??;
//     Ok(())
// }






// async fn dReal(State(state): State<AppState>,Path(id): Path<i64>,) -> Result<impl IntoResponse, HttpResError> {
//     state.dmail_db.get().await?.interact(move |conn| {
//         conn.prepare_cached("DELETE FROM user WHERE id = :id")?.execute(named_params!{":id": id})
//     }).await??;

//     Ok(())
// }

// async fn dRealBulk(State(state): State<AppState>,Json(payload): Json<Vec<Table<()>>>) -> Result<impl IntoResponse, HttpResError> {
//     if payload.is_empty() {return Err(HttpResError::BadRequest("empty payload".to_owned()));}
//     state.dmail_db.get().await?.interact(move |conn| {
//         let sql = format!("DELETE FROM user WHERE id IN ({})", payload.iter().map(|_| "?").collect::<Vec<&str>>().join(","));
//         let params_list: Vec<Value> = payload.iter().map(|user| user.id.into()).collect();
//         conn.prepare_cached(&sql)?.execute(params_from_iter(params_list))
//     }).await??;
//     Ok(())
// }








