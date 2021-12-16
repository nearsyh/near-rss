use std::env;

pub fn is_debug() -> bool {
    env::var("DEBUG").is_ok()
}

pub async fn init_data(services: &super::Services) {
    if is_debug() {
        let user = services
            .user_service
            .register("email", "password")
            .await
            .unwrap();
        services
            .subscription_service
            .add_subscription_from_url(&user.id, "https://blogs.nearsyh.me/atom.xml")
            .await
            .unwrap();
    } else {
        if let Ok(email) = env::var("EMAIL") {
            if let Ok(password) = env::var("PASSWORD") {
                services
                    .user_service
                    .register(&email, &password)
                    .await
                    .unwrap();
            }
        }
    }
}

pub async fn get_user_token(services: &super::Services) -> String {
    services
        .user_service
        .login("nearsy.h@gmail.com", "1234")
        .await
        .unwrap()
        .cltoken
}
