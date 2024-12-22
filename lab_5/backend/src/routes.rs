use actix_web::{web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use mongodb::{Collection};
use mongodb::bson::{doc};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Student {
    id: i32,
    name: String,
    surname: String,
    group_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    id: i32,
    name: String,
}

async fn get_students(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, Student>(
        "SELECT id, name, surname, group_id FROM students"
    )
        .fetch_all(pool.get_ref())
        .await {
        Ok(students) => HttpResponse::Ok().json(students),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn get_student(id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, Student>(
        "SELECT id, name, surname, group_id FROM students WHERE id = $1"
    )
        .bind(id.into_inner())
        .fetch_optional(pool.get_ref())
        .await {
        Ok(Some(student)) => HttpResponse::Ok().json(student),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct ImageData {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_data: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_type: Option<String>,
}

async fn get_student_image(id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as::<_, ImageData>(
        "SELECT image_data, image_type FROM students WHERE id = $1",
    )
        .bind(id.into_inner())
        .fetch_optional(pool.get_ref())
        .await {
        Ok(Some(row)) => {
            if let (Some(image_data), Some(image_type)) = (row.image_data, row.image_type) {
                let content_type = match image_type.as_str() {
                    "image/png" => "image/png",
                    "image/jpeg" => "image/jpeg",
                    _ => "application/octet-stream",
                };
                HttpResponse::Ok()
                    .content_type(content_type)
                    .body(image_data)
            } else {
                HttpResponse::NotFound().finish()
            }
        },
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn create_student(mut payload: Multipart, pool: web::Data<PgPool>) -> impl Responder {
    let mut name = String::new();
    let mut surname = String::new();
    let mut group_id = 0;
    let mut image_data = Vec::new();
    let mut image_type = String::new();

    while let Some(item) = payload.next().await {
        if let Ok(mut field) = item {
            let content_disposition = field.content_disposition();
            let field_name = content_disposition.get_name().unwrap_or("");

            match field_name {
                "studentName" => {
                    while let Some(chunk) = field.next().await {
                        if let Ok(data) = chunk {
                            name = String::from_utf8_lossy(&data).to_string();
                        }
                    }
                },
                "studentSurname" => {
                    while let Some(chunk) = field.next().await {
                        if let Ok(data) = chunk {
                            surname = String::from_utf8_lossy(&data).to_string();
                        }
                    }
                },
                "studentGroup" => {
                    while let Some(chunk) = field.next().await {
                        if let Ok(data) = chunk {
                            group_id = String::from_utf8_lossy(&data).parse().unwrap_or(0);
                        }
                    }
                },
                "studentPhoto" => {
                    // Get content type from the field
                    if let Some(content_type) = field.content_type() {
                        image_type = content_type.to_string();
                        // Validate image type
                        if !matches!(image_type.as_str(), "image/jpeg" | "image/png") {
                            return HttpResponse::BadRequest().body("Invalid image format. Only JPEG and PNG are supported.");
                        }
                    }
                    while let Some(chunk) = field.next().await {
                        if let Ok(data) = chunk {
                            image_data.extend_from_slice(&data);
                        }
                    }
                },
                _ => {}
            }
        }
    }

    match sqlx::query(
        "INSERT INTO students (name, surname, group_id, image_data, image_type) VALUES ($1, $2, $3, $4, $5)"
    )
        .bind(&name)
        .bind(&surname)
        .bind(group_id)
        .bind(&image_data)
        .bind(&image_type)
        .execute(pool.get_ref())
        .await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn update_student(
    id: web::Path<i32>,
    mut payload: Multipart,
    pool: web::Data<PgPool>
) -> impl Responder {
    let mut name = String::new();
    let mut surname = String::new();
    let mut group_id = 0;
    let mut image_data = Vec::new();
    let mut image_type = String::new();
    let mut has_new_image = false;

    while let Some(item) = payload.next().await {
        if let Ok(mut field) = item {
            let content_disposition = field.content_disposition();
            let field_name = content_disposition.get_name().unwrap_or("");

            match field_name {
                "studentName" => {
                    while let Some(chunk) = field.next().await {
                        if let Ok(data) = chunk {
                            name = String::from_utf8_lossy(&data).to_string();
                        }
                    }
                },
                "studentSurname" => {
                    while let Some(chunk) = field.next().await {
                        if let Ok(data) = chunk {
                            surname = String::from_utf8_lossy(&data).to_string();
                        }
                    }
                },
                "studentGroup" => {
                    while let Some(chunk) = field.next().await {
                        if let Ok(data) = chunk {
                            group_id = String::from_utf8_lossy(&data).parse().unwrap_or(0);
                        }
                    }
                },
                "studentPhoto" => {
                    has_new_image = true;
                    if let Some(content_type) = field.content_type() {
                        image_type = content_type.to_string();
                        if !matches!(image_type.as_str(), "image/jpeg" | "image/png") {
                            return HttpResponse::BadRequest().body("Invalid image format. Only JPEG and PNG are supported.");
                        }
                    }
                    while let Some(chunk) = field.next().await {
                        if let Ok(data) = chunk {
                            image_data.extend_from_slice(&data);
                        }
                    }
                },
                _ => {}
            }
        }
    }

    let result = if has_new_image {
        sqlx::query(
            "UPDATE students SET name = $1, surname = $2, group_id = $3, image_data = $4, image_type = $5 WHERE id = $6"
        )
            .bind(&name)
            .bind(&surname)
            .bind(group_id)
            .bind(&image_data)
            .bind(&image_type)
            .bind(id.into_inner())
            .execute(pool.get_ref())
            .await
    } else {
        sqlx::query(
            "UPDATE students SET name = $1, surname = $2, group_id = $3 WHERE id = $4"
        )
            .bind(&name)
            .bind(&surname)
            .bind(group_id)
            .bind(id.into_inner())
            .execute(pool.get_ref())
            .await
    };

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn delete_student(id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query("DELETE FROM students WHERE id = $1")
        .bind(id.into_inner())
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn get_groups(mongo_client: web::Data<Collection<Group>>) -> impl Responder {
    match mongo_client.find(None, None).await {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<Group>>().await {
                Ok(groups) => {
                    HttpResponse::Ok().json(groups)
                },
                Err(e) => {
                    HttpResponse::InternalServerError().body(e.to_string())
                }
            }
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

async fn get_group(id: web::Path<i32>, mongo_client: web::Data<Collection<Group>>) -> impl Responder {
    match mongo_client.find_one(doc! { "id": id.into_inner() }, None).await {
        Ok(Some(group)) => HttpResponse::Ok().json(group),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

#[derive(Deserialize)]
struct GroupInput {
    name: String,
}

async fn create_group(
    group: web::Json<GroupInput>,
    mongo_client: web::Data<Collection<Group>>
) -> impl Responder {
    let max_id = mongo_client
        .find(None, None)
        .await
        .unwrap()
        .try_collect::<Vec<Group>>()
        .await
        .unwrap()
        .iter()
        .map(|doc| doc.id)
        .max()
        .unwrap_or(-1);

    let new_group = Group {
        id: max_id + 1,
        name: group.name.clone(),
    };

    match mongo_client.insert_one(new_group, None).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn update_group(
    id: web::Path<i32>,
    group: web::Json<GroupInput>,
    mongo_client: web::Data<Collection<Group>>
) -> impl Responder {
    match mongo_client.update_one(
        doc! { "id": id.into_inner() },
        doc! { "$set": { "name": &group.name } },
        None
    ).await {
        Ok(result) => {
            if result.modified_count == 0 {
                HttpResponse::NotFound().finish()
            } else {
                HttpResponse::Ok().finish()
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn delete_group(
    id: web::Path<i32>,
    mongo_client: web::Data<Collection<Group>>,
    pool: web::Data<PgPool>
) -> impl Responder {

    match sqlx::query("SELECT COUNT(*) FROM students WHERE group_id = $1")
        .bind(id.as_ref())
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(row) => {
            let count: i64 = row.get(0);
            if count > 0 {
                return HttpResponse::BadRequest().body("Cannot delete group with existing students");
            }
        },
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }

    // If no students exist in this group, proceed with deletion
    match mongo_client.delete_one(doc! { "id": id.into_inner() }, None).await {
        Ok(result) => {
            if result.deleted_count == 0 {
                HttpResponse::NotFound().finish()
            } else {
                HttpResponse::Ok().finish()
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/students")
                .route("", web::get().to(get_students))
                .route("", web::post().to(create_student))
                .route("/{id}", web::get().to(get_student))
                .route("/{id}", web::put().to(update_student))
                .route("/{id}", web::delete().to(delete_student)))
                .route("/image/{id}", web::get().to(get_student_image))
            .service(web::scope("/groups")
                .route("", web::get().to(get_groups))
                .route("", web::post().to(create_group))
                .route("/{id}", web::get().to(get_group))
                .route("/{id}", web::put().to(update_group))
                .route("/{id}", web::delete().to(delete_group)))
    );
}