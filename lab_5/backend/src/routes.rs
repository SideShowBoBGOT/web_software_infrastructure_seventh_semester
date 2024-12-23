use actix_web::{web, HttpResponse, Responder};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use mongodb::Collection;
use mongodb::bson::doc;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct Student {
    id: i32,
    name: String,
    surname: String,
    group_id: i32,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct ImageData {
    #[serde(skip_serializing_if = "Option::is_none")]
    image_data: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    id: i32,
    name: String,
}

#[derive(Deserialize)]
struct GroupInput {
    name: String,
}

#[derive(Default)]
struct StudentForm {
    id: i32,
    name: String,
    surname: String,
    group_id: i32,
    image_data: Option<(Vec<u8>, String)>,
}

// const MAX_IMAGE_SIZE: usize = 5 * 1024 * 1024; // 5MB

async fn process_multipart_fields(mut payload: Multipart) -> Result<StudentForm, HttpResponse> {
    let mut student: StudentForm = Default::default();

    while let Some(Ok(mut field)) = payload.next().await {
        let field_name = field.content_disposition()
            .get_name()
            .unwrap_or("")
            .to_string();

        match field_name.as_str() {
            "studentPhoto" => {
                if let Some(content_type) = field.content_type() {
                    let image_type = content_type.to_string();
                    let mut image_data = Vec::new();

                    while let Some(chunk) = field.next().await {
                        match chunk {
                            Ok(data) => image_data.extend_from_slice(&data),
                            Err(e) => return Err(HttpResponse::BadRequest()
                                .body(format!("Failed to read image data: {e}")))
                        }
                    }

                    if !image_data.is_empty() {
                        student.image_data = Some((image_data, image_type));
                    } else {
                        return Err(HttpResponse::BadRequest()
                            .body("Empty image data received"));
                    }
                } else {
                    return Err(HttpResponse::BadRequest()
                        .body("Invalid image format. Content-Type is missing."));
                }
            },
            "studentId" | "studentName" | "studentSurname" | "studentGroup" => {
                let mut field_value = Vec::new();
                while let Some(chunk) = field.next().await {
                    match chunk {
                        Ok(data) => field_value.extend_from_slice(&data),
                        Err(e) => return Err(HttpResponse::BadRequest()
                            .body(format!("Failed to read field data: {e}")))
                    }
                }

                let value = String::from_utf8_lossy(&field_value).to_string();
                match field_name.as_str() {
                    "studentId" => student.id = value.parse().unwrap_or(0),
                    "studentName" => student.name = value,
                    "studentSurname" => student.surname = value,
                    "studentGroup" => student.group_id = value.parse().unwrap_or(0),
                    _ => unreachable!()
                }
            },
            unrecognized_key => return Err(HttpResponse::BadRequest()
                .body(format!("Unrecognized key: {unrecognized_key}")))
        }
    }
    Ok(student)
}

async fn get_students(pool: web::Data<PgPool>) -> impl Responder {
    log::debug!("Fetching all students");
    let query = "SELECT id, name, surname, group_id FROM students";
    match sqlx::query_as::<_, Student>(query)
        .fetch_all(pool.get_ref())
        .await {
        Ok(students) => {
            log::info!("Successfully retrieved {} students", students.len());
            HttpResponse::Ok().json(students)
        },
        Err(e) => {
            log::error!("Failed to fetch students: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

async fn get_student(id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    log::debug!("Fetching student with id: {}", id);
    let query = "SELECT id, name, surname, group_id FROM students WHERE id = $1";
    match sqlx::query_as::<_, Student>(query)
        .bind(id.as_ref())
        .fetch_optional(pool.get_ref())
        .await {
        Ok(Some(student)) => {
            log::debug!("Successfully retrieved student: {} {}", student.name, student.surname);
            HttpResponse::Ok().json(student)
        },
        Ok(None) => {
            log::debug!("Student not found with id: {}", id);
            HttpResponse::NotFound().finish()
        },
        Err(e) => {
            log::error!("Failed to fetch student: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

async fn get_student_image(id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    log::debug!("Fetching image for student with id: {}", id);
    let query = "SELECT image_data, image_type FROM students WHERE id = $1";
    match sqlx::query_as::<_, ImageData>(query)
        .bind(id.as_ref())
        .fetch_optional(pool.get_ref())
        .await {
        Ok(Some(row)) => {
            if let (Some(image_data), Some(image_type)) = (row.image_data.as_ref(), row.image_type.as_ref()) {
                log::debug!("Successfully retrieved image of type: {} for student id: {}", image_type, id);
                HttpResponse::Ok()
                    .content_type(image_type.as_str())
                    .body(image_data.clone())
            } else {
                log::debug!("No image found for student id: {}", id);
                HttpResponse::NotFound().finish()
            }
        },
        Ok(None) => {
            log::debug!("Student not found with id: {}", id);
            HttpResponse::NotFound().finish()
        },
        Err(e) => {
            log::error!("Query error in get_student_image: {}", e);
            HttpResponse::InternalServerError()
                .body(format!("Query error in get_student_image: {}", e.to_string()))
        }
    }
}

async fn create_student(payload: Multipart, pool: web::Data<PgPool>) -> impl Responder {
    log::debug!("Processing new student creation request");
    let student_form = match process_multipart_fields(payload).await {
        Ok(fields) => {
            log::debug!("Successfully processed multipart fields");
            fields
        },
        Err(response) => {
            log::error!("Failed to process multipart fields");
            return response
        },
    };

    let query = match &student_form.image_data {
        Some((image_data, image_type)) => {
            log::debug!("Creating student with image, type: {}", image_type);
            sqlx::query(
                "INSERT INTO students (name, surname, group_id, image_data, image_type)
                 VALUES ($1, $2, $3, $4, $5)"
            )
                .bind(&student_form.name)
                .bind(&student_form.surname)
                .bind(student_form.group_id)
                .bind(image_data)
                .bind(image_type)
        },
        None => {
            log::debug!("Creating student without image");
            sqlx::query(
                "INSERT INTO students (name, surname, group_id)
                 VALUES ($1, $2, $3)"
            )
                .bind(&student_form.name)
                .bind(&student_form.surname)
                .bind(student_form.group_id)
        }
    };

    match query.execute(pool.get_ref()).await {
        Ok(_) => {
            log::info!("Successfully created new student: {}", student_form.name);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("Failed to create student: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

async fn update_student(
    id: web::Path<i32>,
    payload: Multipart,
    pool: web::Data<PgPool>
) -> impl Responder {
    log::debug!("Processing update request for student id: {}", id.as_ref());
    let student_form = match process_multipart_fields(payload).await {
        Ok(fields) => {
            log::debug!("Successfully processed multipart fields for update");
            fields
        },
        Err(response) => {
            log::error!("Failed to process multipart fields for update");
            return response
        },
    };

    let query = match &student_form.image_data {
        Some((image_data, image_type)) => {
            log::debug!("Updating student with new image, type: {}", image_type);
            sqlx::query(
                "UPDATE students
                 SET name = $1, surname = $2, group_id = $3, image_data = $4, image_type = $5
                 WHERE id = $6"
            )
                .bind(&student_form.name)
                .bind(&student_form.surname)
                .bind(student_form.group_id)
                .bind(image_data)
                .bind(image_type)
                .bind(id.as_ref())
        },
        None => {
            log::debug!("Updating student without changing image");
            sqlx::query(
                "UPDATE students
                 SET name = $1, surname = $2, group_id = $3
                 WHERE id = $4"
            )
                .bind(&student_form.name)
                .bind(&student_form.surname)
                .bind(student_form.group_id)
                .bind(id.as_ref())
        }
    };

    match query.execute(pool.get_ref()).await {
        Ok(_) => {
            log::info!("Successfully updated student id: {}", id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("Error updating student: {}", e);
            HttpResponse::InternalServerError()
                .body(format!("Error update_student: {}", e.to_string()))
        }
    }
}

async fn delete_student(id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    log::debug!("Attempting to delete student with id: {}", id.as_ref());
    match sqlx::query("DELETE FROM students WHERE id = $1")
        .bind(id.as_ref())
        .execute(pool.get_ref())
        .await {
        Ok(_) => {
            log::info!("Successfully deleted student with id: {}", id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("Failed to delete student: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

async fn get_groups(mongo_client: web::Data<Collection<Group>>) -> impl Responder {
    log::debug!("Fetching all groups");
    match mongo_client.find(None, None).await {
        Ok(cursor_group) => {
            match cursor_group.try_collect::<Vec<Group>>().await {
                Ok(groups) => {
                    log::info!("Successfully retrieved {} groups", groups.len());
                    HttpResponse::Ok().json(groups)
                },
                Err(e) => {
                    log::error!("Failed to collect groups: {}", e);
                    HttpResponse::InternalServerError().body(e.to_string())
                }
            }
        },
        Err(e) => {
            log::error!("Failed to fetch groups: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

async fn get_group(id: web::Path<i32>, mongo_client: web::Data<Collection<Group>>) -> impl Responder {
    log::debug!("Fetching group with id: {}", id.as_ref());
    match mongo_client.find_one(doc! { "id": id.as_ref() }, None).await {
        Ok(Some(group)) => {
            log::debug!("Successfully retrieved group: {}", group.name);
            HttpResponse::Ok().json(group)
        },
        Ok(None) => {
            log::debug!("Group not found with id: {}", id);
            HttpResponse::NotFound().finish()
        },
        Err(e) => {
            log::error!("Failed to fetch group: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

async fn create_group(
    group: web::Json<GroupInput>,
    mongo_client: web::Data<Collection<Group>>
) -> impl Responder {
    log::debug!("Creating new group with name: {}", group.name);
    match mongo_client.find(None, None).await {
        Ok(cursor_group) => {
            match cursor_group.try_collect::<Vec<Group>>().await {
                Ok(collected_groups) => {
                    let max_id = collected_groups
                        .iter()
                        .map(|doc| doc.id)
                        .max()
                        .unwrap_or(-1);

                    let new_group = Group {
                        id: max_id + 1,
                        name: group.name.clone(),
                    };

                    log::debug!("Inserting new group with id: {}", new_group.id);
                    match mongo_client.insert_one(new_group, None).await {
                        Ok(_) => {
                            log::info!("Successfully created new group: {}", group.name);
                            HttpResponse::Ok().finish()
                        },
                        Err(e) => {
                            log::error!("Failed to insert group: {}", e);
                            HttpResponse::InternalServerError().body(e.to_string())
                        }
                    }
                },
                Err(e) => {
                    log::error!("Failed to collect groups: {}", e);
                    HttpResponse::InternalServerError().body(e.to_string())
                }
            }
        },
        Err(e) => {
            log::error!("Failed to fetch groups for id generation: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

async fn update_group(
    id: web::Path<i32>,
    group: web::Json<GroupInput>,
    mongo_client: web::Data<Collection<Group>>
) -> impl Responder {
    log::debug!("Updating group id: {} with new name: {}", id, group.name);
    match mongo_client.update_one(
        doc! { "id": id.as_ref() },
        doc! { "$set": { "name": &group.name } },
        None
    ).await {
        Ok(result) if result.modified_count > 0 => {
            log::info!("Successfully updated group id: {}", id);
            HttpResponse::Ok().finish()
        },
        Ok(_) => {
            log::debug!("Group not found with id: {}", id);
            HttpResponse::NotFound().finish()
        },
        Err(e) => {
            log::error!("Failed to update group: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

async fn delete_group(
    id: web::Path<i32>,
    mongo_client: web::Data<Collection<Group>>,
    pool: web::Data<PgPool>
) -> impl Responder {
    log::debug!("Attempting to delete group with id: {}", id);

    let count: i64 = match sqlx::query("SELECT COUNT(*) FROM students WHERE group_id = $1")
        .bind(id.as_ref())
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(row) => {
            let count: i64 = row.get(0);
            log::debug!("Found {} students in group {}", count, id);
            count
        },
        Err(e) => {
            log::error!("Error checking for students in group: {}", e);
            return HttpResponse::InternalServerError().body(e.to_string())
        },
    };

    if count > 0 {
        log::warn!("Cannot delete group {} as it contains {} students", id, count);
        return HttpResponse::BadRequest().body("Cannot delete group with existing students");
    }

    match mongo_client.delete_one(doc! { "id": id.as_ref() }, None).await {
        Ok(result) if result.deleted_count > 0 => {
            log::info!("Successfully deleted group with id: {}", id);
            HttpResponse::Ok().finish()
        },
        Ok(_) => {
            log::debug!("Group not found with id: {}", id);
            HttpResponse::NotFound().finish()
        },
        Err(e) => {
            log::error!("Failed to delete group: {}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/students")
                    .route("", web::get().to(get_students))
                    .route("", web::post().to(create_student))
                    .route("/{id}", web::get().to(get_student))
                    .route("/{id}", web::put().to(update_student))
                    .route("/{id}", web::delete().to(delete_student))
                    .route("/image/{id}", web::get().to(get_student_image))
            )
            .service(
                web::scope("/groups")
                    .route("", web::get().to(get_groups))
                    .route("", web::post().to(create_group))
                    .route("/{id}", web::get().to(get_group))
                    .route("/{id}", web::put().to(update_group))
                    .route("/{id}", web::delete().to(delete_group))
            )
    );
}