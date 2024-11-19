use rocket::serde::{json::Json, Deserialize};


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct RegisterData<'r> {
    email: &'r str,
    plaintext_password: &'r str,
    first_name: &'r str,
    last_name: &'r str,
    gender: char
}

#[post("/auth/register", data = "<data>")]
async fn auth_register(data: Json<RegisterData<'_>>) {

}
