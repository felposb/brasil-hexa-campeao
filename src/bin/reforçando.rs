#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use serde_json::{Value, json};
use std::fs::File;

#[get("/usuarios")]
fn listar_usuarios() -> Json<Vec<Value>>{
  let carregar = File::open("dados/usuarios.json")
    .expect("Não foi possivel carregar o arquivo");
  let usuarios: Vec<Value> = serde_json::from_reader(carregar)
    .expect("Nao foi possivel converter em json");
  Json(usuarios)
}
#[post("/usuarios/<id>", data = "<usuario>")]
fn criar_usuarios(id: u64, usuario: Json<Value>) -> Json<Value> {
  let novo_usuario = usuario.into_inner();
  let carregar = File::open("dados/usuarios.json")
    .expect("Erro, nao foi possivel");
  let mut usuarios: Vec<Value> = serde_json::from_reader(carregar)
    .expect("Erro, não foi possivel converter json");
  for usuario in &usuarios {
    let id_usuario = usuario.get("id").and_then(|valor| valor.as_u64());
    if id_usuario == Some(id){
      return Json(json!({
        "erro": "Ja existe esse usuario"
      }));
    }
  }
  usuarios.push(novo_usuario.clone());
  let json_formatado = serde_json::to_string_pretty(&usuarios)
    .expect("Erro ao formatar usuario");
  std::fs::write("dados/usuarios.json", json_formatado)
    .expect("não foi possivel salvar o arquivo");
  Json(json!({
    "mensagem": "Criado com sucesso",
    "usuario": novo_usuario
  }))
}


#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes!(listar_usuarios, criar_usuarios))
}