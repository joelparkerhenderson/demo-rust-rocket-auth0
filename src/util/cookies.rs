use log::*;
use rocket::http::{Cookie, Cookies};

pub fn require(cookies: &Cookies, cookie: &Cookie) -> Result<rocket::http::Status, rocket::http::Status> {
    info!("util::cookies::require cookie name:{} value:{}", cookie.name(), cookie.value());
    print!("{:?}", cookies);
    if let Some(c) = cookies.get(cookie.name()) {
        if c.value() == cookie.value() {
            info!("util::cookies::require -> Ok");
            return Ok(rocket::http::Status::Ok);
        } else {
            info!("util::cookies::require -> Forbidden");
            return Err(rocket::http::Status::Forbidden);
        }
    } else {
        info!("util::cookies::require -> BadRequest");
        return Err(rocket::http::Status::BadRequest);
    }
}

//TODO add tests

#[cfg(test)]
mod tests {

    use cookie;
    use env_logger;
    use rocket::local::Client;
    use rocket::request::{Request, FromRequest};
    use rocket::http::{Cookie, Cookies};
    use std::cell::{RefCell, RefMut};
    use std::rc::Rc;

    fn fab_key() -> ::cookie::Key {
        ::cookie::Key::from(b"0000000000000000000000000000000000000000000000000000000000000000")
    }

    #[test]
    fn require_with_ok() {
        let key = fab_key();
        let rc_cookie_jar: Rc<RefCell<_>> = Rc::new(RefCell::new(::cookie::CookieJar::new()));
        let mut cookies = ::rocket::http::Cookies::new(rc_cookie_jar.borrow_mut(), &key);
        cookies.add(Cookie::new("a", "b"));
        let x = crate::util::cookies::require(&cookies, &Cookie::new("a", "b"));
        assert!(x.is_ok());
        assert!(x.unwrap() == rocket::http::Status::Ok);
    }

    #[test]
    fn require_with_forbidden() {
        let key = fab_key();
        let rc_cookie_jar: Rc<RefCell<_>> = Rc::new(RefCell::new(::cookie::CookieJar::new()));
        let mut cookies = ::rocket::http::Cookies::new(rc_cookie_jar.borrow_mut(), &key);
        cookies.add(Cookie::new("a", "b"));
        let x = crate::util::cookies::require(&cookies, &Cookie::new("a", "different"));
        assert!(x.is_err());
        assert!(x.unwrap_err() == rocket::http::Status::Forbidden);
    }

    #[test]
    fn require_with_bad_request() {
        let key = fab_key();
        let rc_cookie_jar: Rc<RefCell<_>> = Rc::new(RefCell::new(::cookie::CookieJar::new()));
        let mut cookies = ::rocket::http::Cookies::new(rc_cookie_jar.borrow_mut(), &key);
        cookies.add(Cookie::new("a", "b"));
        let x = crate::util::cookies::require(&cookies, &Cookie::new("different", "b"));
        assert!(x.is_err());
        assert!(x.unwrap_err() == rocket::http::Status::BadRequest);
    }

}
