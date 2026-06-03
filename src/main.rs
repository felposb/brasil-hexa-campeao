#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use serde_json::Value;
use std::fs::File;

#[get("/usuarios")]
fn get_usuarios() -> Json<Vec<Value>> {
    let arquivo = File::open("dados/usuarios.json").expect("Não foi possivel abrir o arquivo");
    let usuarios: Vec<Value> =
        serde_json::from_reader(arquivo).expect("Não foi possivel converter json");
    Json(usuarios)
}
#[post("/usuarios", data = "<usuario>")]
fn create_usuarios(usuario: Json<Value>) -> Json<Vec<Value>> {
    let novo_usuario = usuario.into_inner();
    let arquivo = File::open("dados/usuarios.json").expect("Erro, nao foi possivel abrir arquivo");

    let mut usuarios: Vec<Value> =
        serde_json::from_reader(arquivo).expect("Erro, não foi possivel converter json");
    usuarios.push(novo_usuario);
    let json_formatado = serde_json::to_string_pretty(&usuarios)
        .expect("Não foi possivel transformar o texto em str");
    std::fs::write("dados/usuarios.json", json_formatado)
        .expect("Não foi possivel salvar o arquivo");
    Json(usuarios)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_usuarios, create_usuarios])
}
