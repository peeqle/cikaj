// async fn login() -> HttpResponse {
//     let user_id = "user123";
//     let company = "my_company";
//
//     // Генерируем JWT-токен для пользователя
//     let token = create_jwt(SECRET_KEY, user_id, company);
//
//     HttpResponse::Ok().body(format!("Your token: {}", token))
// }
//
// async fn protected_route(req: HttpRequest) -> HttpResponse {
//     // Извлекаем токен из заголовка Authorization
//     if let Some(auth_header) = req.headers().get("Authorization") {
//         if let Ok(auth_str) = auth_header.to_str() {
//             if auth_str.starts_with("Bearer ") {
//                 let token = &auth_str[7..]; // Убираем "Bearer "
//
//                 // Проверяем токен
//                 match validate_jwt(token, SECRET_KEY) {
//                     Ok(claims) => {
//                         return HttpResponse::Ok().body(format!("Welcome, {}!", claims.sub));
//                     }
//                     Err(_) => {
//                         return HttpResponse::Unauthorized().body("Invalid token");
//                     }
//                 }
//             }
//         }
//     }
//
//     HttpResponse::Unauthorized().body("Authorization token required")
// }