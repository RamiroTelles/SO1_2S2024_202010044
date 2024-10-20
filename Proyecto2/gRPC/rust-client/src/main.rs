use studentgrpc::student_client::StudentClient;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use studentgrpc::StudentRequest;
use serde::{Deserialize, Serialize};
//use std::thread;
//use std::sync::mpsc;


pub mod studentgrpc {
    tonic::include_proto!("student");
}

#[derive(Deserialize,Serialize)]
struct StudentData {
    name: String,
    age: i32,
    faculty: String,
    discipline: i32,
}

async fn handle_student(student: web::Json<StudentData>) -> impl Responder {

    let addr: &str;
    if student.discipline == 1{
        addr = "http://localhost:50051";
    }else if student.discipline==2{
        addr = "http://localhost:50052";
    }else{
        addr = "http://localhost:50053";
    }


    let mut client = match StudentClient::connect(addr).await {
        Ok(client) => client,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to connect to gRPC server: {}", e)),
    };
    

    let request = tonic::Request::new(StudentRequest {
        name: student.name.clone(),
        age: student.age,
        faculty: student.faculty.clone(),
        discipline: student.discipline,
    });

    //creacion hilo con tokio
    tokio::spawn(async move {
    
        match client.send_student(request).await{
            Ok(response) => {

                println!("RESPONSE={:?}", response);
    
                
            },
            Err(e) =>  println!("ERROR={:?}", e),
        }
        
    });

    HttpResponse::Ok().json(format!("Ok"))
    
}

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    println!("Starting server at http://localhost:8081");
    HttpServer::new(|| {
        App::new()
            .route("/faculty", web::post().to(handle_student))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

