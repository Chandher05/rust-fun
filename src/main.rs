use rocket::response::content::RawJson;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, launch, post, routes};

#[derive(Deserialize, Serialize)]
struct UserData {
    name: String,
    age: u8,
}

#[derive(Serialize)]
struct Greeting {
    message: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[get("/question")]
fn question_get() -> &'static str {
    "Question for the day: What is your name?"
}


#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[post("/submit", format = "json", data = "<user_data>")]
fn submit(user_data: String) -> RawJson<String> {
    // Deserialize the input JSON string into UserData
    let user: UserData = serde_json::from_str(&user_data).unwrap();

    // Process the data to create a greeting message
    let response = Greeting {
        message: format!("Hello, {}, who is {} years old!", user.name, user.age),
    };

    // Serialize the response into a JSON string
    let serialized = serde_json::to_string(&response).unwrap();

    // Return the JSON string as a RawJson response
    RawJson(serialized)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello, submit, question_get])
}
