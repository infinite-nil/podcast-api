#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello world"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!("Hello world", index());
    }
}
