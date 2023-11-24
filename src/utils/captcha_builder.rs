use captcha_rs::{Captcha, CaptchaBuilder};

pub fn build_captcha() -> Captcha {
    CaptchaBuilder::new()
        .length(5)
        .width(200)
        .height(100)
        .dark_mode(true)
        .complexity(5)
        .compression(40)
        .build()
}