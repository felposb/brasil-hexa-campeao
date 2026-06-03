#[macro_use]
extern crate rocket;
use rocket::{routes, serde::json::Json};
use serde_json::Value;
use std::fs::File;

//Rota get
#[get("/usuarios")]
fn listar_usuarios() -> Json<Vec<Value>> {
    let arquivo = File::open("dados/usuarios.json").expect("erro nao foi possivel abrir arquivo");
    let usuarios: Vec<Value> = serde_json
        ::from_reader(arquivo)
        .expect("erro nao foi possivel converter json");
    Json(usuarios)
}
#[post("/usuarios", data = "<usuario>")]
fn criar_usuarios(usuario: Json<Value>) -> Json<Vec<Value>> {
    let novo_usuario = usuario.into_inner();
    let arquivo =
        File::open("dados/usuarios.json").expect("Erro, nao foi possivel abrir o arquivo");
    let mut usuarios: Vec<Value> =
        serde_json::from_reader(arquivo).expect("Erro, nao foi possivel converter json");
    usuarios.push(novo_usuario);
    let json_formatado = serde_json::to_string_pretty(&usuarios)
        .expect("Não foi possivel transformar o texto em str");
    std::fs::write("dados/usuarios.json", json_formatado).expect("nao foi possivel salvar arquivo");
    Json(usuarios)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![listar_usuarios, criar_usuarios])
}
