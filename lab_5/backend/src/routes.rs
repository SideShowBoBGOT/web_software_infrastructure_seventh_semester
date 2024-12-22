use actix_web::{web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client as PgClient};
use mongodb::{Collection};
use mongodb::bson::{doc, Document};

#[derive(Serialize, Deserialize)]
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

async fn get_students(pg_client: web::Data<PgClient>) -> impl Responder {
    match pg_client.query("SELECT id, name, surname, group_id FROM students", &[]).await {
        Ok(rows) => {
            let students: Vec<Student> = rows.iter().map(|row| Student {
                id: row.get(0),
                name: row.get(1),
                surname: row.get(2),
                group_id: row.get(3),
                image_data: None,
            }).collect();
            HttpResponse::Ok().json(students)
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

async fn get_student(id: web::Path<i32>, pg_client: web::Data<PgClient>) -> impl Responder {
    match pg_client.query_one(
        "SELECT id, name, surname, group_id, image_data FROM students WHERE id = $1",
        &[id.as_ref()]
    ).await {
        Ok(row) => {
            let student = Student {
                id: row.get(0),
                name: row.get(1),
                surname: row.get(2),
                group_id: row.get(3),
                image_data: row.get(4),
            };
            HttpResponse::Ok().json(student)
        },
        Err(_) => HttpResponse::NotFound().finish()
    }
}

async fn create_student(mut payload: Multipart, pg_client: web::Data<PgClient>) -> impl Responder {
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

    match pg_client.execute(
        "INSERT INTO students (name, surname, group_id, image_data) VALUES ($1, $2, $3, $4)",
        &[&name, &surname, &group_id, &image_data]
    ).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn update_student(
    id: web::Path<i32>,
    mut payload: Multipart,
    pg_client: web::Data<PgClient>
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

    let query = if has_new_image {
        "UPDATE students SET name = $1, surname = $2, group_id = $3, image_data = $4 WHERE id = $5"
    } else {
        "UPDATE students SET name = $1, surname = $2, group_id = $3 WHERE id = $4"
    };

    let result = if has_new_image {
        pg_client.execute(query, &[&name, &surname, &group_id, &image_data, id.as_ref()]).await
    } else {
        pg_client.execute(query, &[&name, &surname, &group_id, id.as_ref()]).await
    };

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn delete_student(id: web::Path<i32>, pg_client: web::Data<PgClient>) -> impl Responder {
    match pg_client.execute("DELETE FROM students WHERE id = $1", &[id.as_ref()]).await {
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