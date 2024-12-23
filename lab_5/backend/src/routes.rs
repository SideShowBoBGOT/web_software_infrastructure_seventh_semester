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

async fn process_multipart_fields(mut payload: Multipart) -> Result<StudentForm, HttpResponse> {
    let mut student: StudentForm = Default::default();

    // println!("Start process_multipart_fields");

    while let Some(Ok(mut field)) = payload.next().await {
        let field_name = field.content_disposition()
            .get_name()
            .unwrap_or("")
            .to_string();
        
        // println!("field_name: {}", field_name);

        let field_value = field.next().await
            .and_then(|c| c.ok())
            .map(|data| String::from_utf8_lossy(&data).to_string());

        // println!("field_value: {:?}", field_value);

        if let Some(value) = field_value {
            match field_name.as_str() {
                "studentId" => student.id = value.parse::<i32>().unwrap_or(0),
                "studentName" => student.name = value,
                "studentSurname" => student.surname = value,
                "studentGroup" => student.group_id = value.parse().unwrap_or(0),
                "studentPhoto" => {
                    if let Some(content_type) = field.content_type() {
                        let image_type = content_type.to_string();
                        if matches!(image_type.as_str(), "image/jpeg" | "image/png" | "image/jpg") {
                            let mut image_data = Vec::new();
                            while let Some(Ok(chunk)) = field.next().await {
                                image_data.extend_from_slice(&chunk);
                            }
                            student.image_data = Some((image_data, image_type));
                        }
                    }
                    if student.image_data.is_none() {
                        return Err(HttpResponse::BadRequest()
                            .body("Invalid image format. Only JPEG and PNG are supported."));
                    }
                }
                unrecognized_key => return Err(HttpResponse::BadRequest()
                        .body(format!("Unrecognized key: {unrecognized_key}")))
            }
        }
    }

    Ok(student)
}

async fn get_students(pool: web::Data<PgPool>) -> impl Responder {
    let query = "SELECT id, name, surname, group_id FROM students";
    match sqlx::query_as::<_, Student>(query)
        .fetch_all(pool.get_ref())
        .await {
        Ok(students) => HttpResponse::Ok().json(students),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn get_student(id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    let query = "SELECT id, name, surname, group_id FROM students WHERE id = $1";
    match sqlx::query_as::<_, Student>(query)
        .bind(id.into_inner())
        .fetch_optional(pool.get_ref())
        .await {
        Ok(Some(student)) => HttpResponse::Ok().json(student),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn get_student_image(id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    let query = "SELECT image_data, image_type FROM students WHERE id = $1";
    match sqlx::query_as::<_, ImageData>(query)
        .bind(id.into_inner())
        .fetch_optional(pool.get_ref())
        .await {
        Ok(Some(row)) => {
            if let (Some(image_data), Some(image_type)) = (row.image_data, row.image_type) {
                let content_type = match image_type.as_str() {
                    "image/png" | "image/jpeg" | "image/jpg" => image_type.as_str(),
                    content_type => return HttpResponse::InternalServerError()
                        .body(format!("Invalid image format stored on server: {content_type}")),
                };
                HttpResponse::Ok()
                    .content_type(content_type)
                    .body(image_data)
            } else {
                HttpResponse::NotFound().finish()
            }
        },
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Query error in get_student_image: {}", e.to_string()))
    }
}

async fn create_student(payload: Multipart, pool: web::Data<PgPool>) -> impl Responder {
    let student_form = match process_multipart_fields(payload).await {
        Ok(fields) => fields,
        Err(response) => return response,
    };

    let query = match &student_form.image_data {
        Some((image_data, image_type)) => {
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
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn update_student(
    id: web::Path<i32>,
    payload: Multipart,
    pool: web::Data<PgPool>
) -> impl Responder {
    // println!("Start update_student");
    let student_form = match process_multipart_fields(payload).await {
        Ok(fields) => fields,
        Err(response) => return response,
    };

    // println!("update_student values: {}, {}, {}", student_form.name, student_form.surname, student_form.group_id);

    let query = match &student_form.image_data {
        Some((image_data, image_type)) => {
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
                .bind(id.into_inner())
        },
        None => {
            sqlx::query(
                "UPDATE students
                 SET name = $1, surname = $2, group_id = $3
                 WHERE id = $4"
            )
                .bind(&student_form.name)
                .bind(&student_form.surname)
                .bind(student_form.group_id)
                .bind(id.into_inner())
        }
    };

    match query.execute(pool.get_ref()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Error update_student: {}", e.to_string()))
    }
}

async fn delete_student(id: web::Path<i32>, pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query("DELETE FROM students WHERE id = $1")
        .bind(id.into_inner())
        .execute(pool.get_ref())
        .await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn get_groups(mongo_client: web::Data<Collection<Group>>) -> impl Responder {
    match mongo_client.find(None, None).await {
        Ok(cursor_group) => {
            match cursor_group.try_collect::<Vec<Group>>().await {
                Ok(groups) => HttpResponse::Ok().json(groups),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string())
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn get_group(id: web::Path<i32>, mongo_client: web::Data<Collection<Group>>) -> impl Responder {
    match mongo_client.find_one(doc! { "id": id.into_inner() }, None).await {
        Ok(Some(group)) => HttpResponse::Ok().json(group),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn create_group(
    group: web::Json<GroupInput>,
    mongo_client: web::Data<Collection<Group>>
) -> impl Responder {
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

                    match mongo_client.insert_one(new_group, None).await {
                        Ok(_) => HttpResponse::Ok().finish(),
                        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
                    }
                },
                Err(e) => HttpResponse::InternalServerError().body(e.to_string())
            }
        },
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
        Ok(result) if result.modified_count > 0 => HttpResponse::Ok().finish(),
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}

async fn delete_group(
    id: web::Path<i32>,
    mongo_client: web::Data<Collection<Group>>,
    pool: web::Data<PgPool>
) -> impl Responder {
    // Check for existing students in group
    let count: i64 = match sqlx::query("SELECT COUNT(*) FROM students WHERE group_id = $1")
        .bind(id.as_ref())
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(row) => row.get(0),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    if count > 0 {
        return HttpResponse::BadRequest().body("Cannot delete group with existing students");
    }

    match mongo_client.delete_one(doc! { "id": id.into_inner() }, None).await {
        Ok(result) if result.deleted_count > 0 => HttpResponse::Ok().finish(),
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
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