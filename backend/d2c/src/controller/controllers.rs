use crate::service::text_service::TextService;

pub async fn get_foo() -> String {
    String::from("Foo")
}

pub async fn manage_text() -> String {
    String::from("ManageText")
}
