use rocket::{
    http::Status,
    serde::{
        json::{json, Json, Value},
        Deserialize,
    },
};
use rocket_db_pools::Connection;

use crate::{
    util::{check_authorized_admin, check_authorized_user},
    DB,
};

use super::{add_error, add_suggestion, get_reports};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetReportsData<'r> {
    jwt: &'r str,
}

#[post("/admin/get_reports", format = "json", data = "<data>")]
pub async fn admin_get_reports(
    mut db: Connection<DB>,
    data: Json<GetReportsData<'_>>,
) -> (Status, Value) {
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
    }

    let reports = match get_reports(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::BadRequest, json!({"error": err})),
    };

    (Status::Ok, json!(reports))
}

#[post("/admin/get_logs", format = "json", data = "<data>")]
pub async fn admin_get_logs(
    mut db: Connection<DB>,
    data: Json<GetReportsData<'_>>,
) -> (Status, Value) {
    if let Some(err) = check_authorized_admin(&mut db, data.jwt).await {
        return err;
    }

    let reports = match get_reports(&mut db).await {
        Ok(val) => val,
        Err(err) => return (Status::BadRequest, json!({"error": err})),
    };

    (Status::Ok, json!(reports))
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReportData<'r> {
    jwt: &'r str,
    title: &'r str,
    message: &'r str,
}

#[post("/report/error", format = "json", data = "<data>")]
pub async fn report_error(mut db: Connection<DB>, data: Json<ReportData<'_>>) -> (Status, Value) {
    if let Some(err) = check_authorized_user(&mut db, data.jwt).await {
        return err;
    }

    if let Err(err) = add_error(&mut db, data.title, data.message).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}

#[post("/report/suggestion", format = "json", data = "<data>")]
pub async fn report_suggestion(
    mut db: Connection<DB>,
    data: Json<ReportData<'_>>,
) -> (Status, Value) {
    if let Some(err) = check_authorized_user(&mut db, data.jwt).await {
        return err;
    }

    if let Err(err) = add_suggestion(&mut db, data.title, data.message).await {
        return (Status::InternalServerError, json!({"error": err}));
    }

    (Status::Ok, json!({}))
}
