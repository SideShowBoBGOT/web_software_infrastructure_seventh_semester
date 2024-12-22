use actix_web::{web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use mongodb::{Collection};
use mongodb::bson::{doc, Document};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Student {
    id: i32,
    name: String,
    surname: String,
    group_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_data: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
struct Group {
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
        "SELECT id, name, surname, group_id, image_data FROM students WHERE id = $1"
    )
        .bind(id.into_inner())
        .fetch_optional(pool.get_ref())
        .await {
        Ok(Some(student)) => HttpResponse::Ok().json(student),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn create_student(mut payload: Multipart, pool: web::Data<PgPool>) -> impl Responder {
    let mut name = String::new();
    let mut surname = String::new();
    let mut group_id = 0;
    let mut image_data = Vec::new();

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
        "INSERT INTO students (name, surname, group_id, image_data) VALUES ($1, $2, $3, $4)"
    )
        .bind(&name)
        .bind(&surname)
        .bind(group_id)
        .bind(&image_data)
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
            "UPDATE students SET name = $1, surname = $2, group_id = $3, image_data = $4 WHERE id = $5"
        )
            .bind(&name)
            .bind(&surname)
            .bind(group_id)
            .bind(&image_data)
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

async fn get_groups(mongo_client: web::Data<Collection<Document>>) -> impl Responder {
    match mongo_client.find(None, None).await {
        Ok(cursor) => {
            let groups: Vec<Document> = cursor.try_collect().await.unwrap_or_default();
            HttpResponse::Ok().json(groups)
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn get_group(id: web::Path<i32>, mongo_client: web::Data<Collection<Document>>) -> impl Responder {
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
    mongo_client: web::Data<Collection<Document>>
) -> impl Responder {
    let max_id = mongo_client
        .find(None, None)
        .await
        .unwrap()
        .try_collect::<Vec<Document>>()
        .await
        .unwrap()
        .iter()
        .filter_map(|doc| doc.get_i32("id").ok())
        .max()
        .unwrap_or(-1);

    let new_group = doc! {
        "id": max_id + 1,
        "name": &group.name
    };

    match mongo_client.insert_one(new_group, None).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn update_group(
    id: web::Path<i32>,
    group: web::Json<GroupInput>,
    mongo_client: web::Data<Collection<Document>>
) -> impl Responder {
    match mongo_client.update_one(
        doc! { "id": id.into_inner() },
        doc! { "$set": { "name": &group.name } },
        None
    ).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn delete_group(
    id: web::Path<i32>,
    mongo_client: web::Data<Collection<Document>>
) -> impl Responder {
    match mongo_client.delete_one(doc! { "id": id.into_inner() }, None).await {
        Ok(_) => HttpResponse::Ok().finish(),
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
            .service(web::scope("/groups")
                .route("", web::get().to(get_groups))
                .route("", web::post().to(create_group))
                .route("/{id}", web::get().to(get_group))
                .route("/{id}", web::put().to(update_group))
                .route("/{id}", web::delete().to(delete_group)))
    );
}