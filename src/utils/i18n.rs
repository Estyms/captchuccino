use std::env;
use r_i18n::{I18n, I18nConfig};



pub fn get_translation(translation_key: &str) -> String {
    let config: I18nConfig = I18nConfig{
        locales: &["en", "fr"],
        directory: "translations"
    };
    let mut r_i18n = I18n::configure(&config);
    let locale = env::var("LANG")
        .expect(get_env_error_message("LANG").as_str());

    r_i18n.set_current_lang(locale.as_str());
    r_i18n.t(translation_key).to_string()
}

pub fn get_server_message(translation_key: &str, server_name: &str) -> String {
    get_translation(translation_key).replace("SERVER_NAME", server_name)
}

pub fn get_env_error_message(env_name: &str) -> String {
    let config: I18nConfig = I18nConfig{
        locales: &["en", "fr"],
        directory: "translations"
    };
    let mut r_i18n = I18n::configure(&config);
    let locale = env::var("LANG")
        .expect("LANG ENV NOT SET");

    r_i18n.set_current_lang(locale.as_str());
    let error_msg = r_i18n.t("env-error");
    match error_msg.to_string().contains("ENV_VAR") {
        true => {error_msg.to_string().replace("ENV_VAR", env_name)}
        false => panic!("Cannot find env-error")
    }
}

pub fn get_user_send_error(user_id: u64) -> String {
    let config: I18nConfig = I18nConfig{
        locales: &["en", "fr"],
        directory: "translations"
    };

    let mut r_i18n = I18n::configure(&config);
    let locale = env::var("LANG")
        .expect("LANG ENV NOT SET");

    r_i18n.set_current_lang(locale.as_str());
    let error_msg = r_i18n.t("server-userhasclosedDM-error");
    match error_msg.to_string().contains("USERID") {
        true => {error_msg.to_string().replace("USERID", format!("{}", user_id).as_str())}
        false => panic!("Cannot find env-error")
    }
}