use tower_cookies::Cookie;

pub fn create_cookie(key: &str, value: String, http_only: bool) -> Cookie<'_> {
    let mut cookie = Cookie::new(key.to_owned(), value);
    cookie.set_http_only(http_only);
    cookie.set_path("/");
    cookie
}

pub fn remove_cookie(key: &str) -> Cookie<'_> {
    create_cookie(key, "".to_owned(), false)
}
