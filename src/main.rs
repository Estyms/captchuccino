mod utils;

use std::collections::HashMap;
use std::env;
use std::fs::remove_file;
use std::sync::{Arc, RwLock};
use captcha_rs::Captcha;
use tokio::fs::{create_dir, File, try_exists};
use image::ImageFormat;
use serenity::{async_trait, Client};
use serenity::client::Context;
use serenity::model::channel::{Message};
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;
use serenity::model::id::{ChannelId, RoleId, UserId};
use serenity::model::prelude::Guild;
use serenity::prelude::{EventHandler, GatewayIntents, TypeMapKey};
use crate::utils::captcha_builder::build_captcha;
use crate::utils::i18n::{get_translation, get_env_error_message, get_server_message, get_user_send_error};

struct Handler;

struct CaptchaData;

impl TypeMapKey for CaptchaData {
    type Value = Arc<RwLock<HashMap<UserId, String>>>;
}

async fn get_lock(_ctx: &Context) -> Arc<RwLock<HashMap<UserId, String>>> {
    let captcha_data = _ctx.data.read().await;

    captcha_data.get::<CaptchaData>()
        .expect(get_translation("captcha-get-data-error").as_str()).clone()
}

async fn create_captcha(_ctx: &Context, user_id: UserId) -> Captcha {
    let captcha = build_captcha();

    let captcha_data_lock = get_lock(&_ctx).await;
    {
        let mut data = captcha_data_lock.write()
            .expect(get_translation("mutex-lock-error").as_str());

        let clone_captcha = captcha.text.clone().to_owned();
        let entry = data.entry(user_id).or_insert(clone_captcha.clone());
        *entry = clone_captcha;
    };

    captcha
}

async fn send_captcha(_ctx: &Context, _new_member: Member, msg: &str) {
    let private_channel = _new_member.user.create_dm_channel(&_ctx).await;
    let captcha = create_captcha(&_ctx, _new_member.user.id).await;

    let file_name_string = format!("captcha/{}.jpg", uuid::Uuid::new_v4());
    let file_name = file_name_string.as_str();
    captcha.image.save_with_format(&file_name, ImageFormat::Jpeg)
        .expect(get_translation("save-image-error").as_str());

    if let Ok(channel) = private_channel {
        let file = File::open(&file_name).await
            .expect(get_translation("open-image-error").as_str());

        let files = vec![(&file, file_name)];

        match channel.send_files(&_ctx, files, |m| m.content(msg)).await {
            Ok(_) => {

            }
            Err(_) => {
                ChannelId::from(get_bot_channel_id())
                .send_message(&_ctx, |m| m.content(get_user_send_error(_new_member.user.id.0).as_str()))
                .await.expect(get_translation("server-cantsendmessage-error").as_str());}
        }

        remove_file(file_name_string.as_str())
            .expect(get_translation("delete-image-error").as_str());
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, _ctx: Context, mut _new_member: Member) {
        _new_member.add_role(&_ctx, &get_role_id()).await
            .expect(get_translation("server-cantaddrole-error").as_str());

        let guild = Guild::get(&_ctx, get_guild_id()).await
            .expect(get_translation("server-cantgetguild-error").as_str());

        send_captcha(&_ctx, _new_member,
                     get_server_message(
                         "server-captcha-prompt",
                         guild.name.as_str()
                     ).as_str()
        ).await;
    }

    async fn message(&self, _ctx: Context, _new_message: Message) {
        let guild = Guild::get(&_ctx, get_guild_id()).await
            .expect(get_translation("server-cantgetguild-error").as_str());
        match guild.member(&_ctx, _new_message.author.id).await {
            Ok(member) => {
                if !member.roles.contains(&get_role_id()) {
                    return;
                }
            }
            Err(_) => { return; }
        }


        let mut member = guild.member(&_ctx, _new_message.author.id).await
            .expect(get_translation("server-cantgetmember-error").as_str());

        if let Some(channel) = _new_message.channel(&_ctx).await
            .expect(get_translation("server-cantgetchannel-error").as_str()).private()
        {
            let captcha_data_lock = get_lock(&_ctx).await;

            let res = {
                let mut data = captcha_data_lock.write()
                    .expect(get_translation("mutex-lock-error").as_str());

                match data.get(&member.user.id) {
                    None => None,
                    Some(captcha_text) => {
                        let res = captcha_text.eq(&_new_message.content);
                        if res {
                           data.remove(&member.user.id)
                               .expect(get_translation("captcha-delete-data-error").as_str());
                        }
                        Some(res)
                    }
                }
            };

            match res {
                Some(true) => {
                    member.remove_role(&_ctx, &get_role_id()).await
                        .expect(get_translation("server-cantremoverole-error").as_str());

                    channel.send_message(&_ctx, |m|
                        m.content(get_server_message("server-captcha-validated", guild.name.as_str()))
                    ).await
                        .expect(get_translation("server-cantsendmessage-error").as_str());
                },
                _ => {
                    send_captcha(&_ctx, member, get_translation("server-captcha-incorrect").as_str()).await
                }
            };
        }
    }

    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("{}", get_translation("bot-started"));
    }
}
fn get_role_id() -> RoleId {
    let role_id = env::var("ROLE_ID").expect(get_env_error_message("ROLE_ID").as_str());
    RoleId::from(role_id.parse::<u64>().expect("Cannot parse ROLE_ID to int"))
}

fn get_guild_id() -> u64 {
    let guild_id = env::var("GUILD_ID").expect(get_env_error_message("GUILD_ID").as_str());
    guild_id.parse::<u64>().expect("Cannot parse GUILD_ID to int")
}

fn get_bot_channel_id() -> u64 {
    let channel_id = env::var("BOT_CHANNEL_ID").expect(get_env_error_message("GUILD_ID").as_str());
    channel_id.parse::<u64>().expect("Cannot parse BOT_CHANNEL")
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect(get_env_error_message("DISCORD_TOKEN").as_str());

    if !try_exists("captcha").await.expect(get_translation("exist-dir-error").as_str()) {
        create_dir("captcha").await.expect(get_translation("create-dir-error").as_str());
    }


    let intents = GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler).await.expect(get_translation("client-build-error").as_str());

    {
        let mut data = client.data.write().await;
        data.insert::<CaptchaData>(Arc::new(RwLock::new(HashMap::default())));
    }


    if let Err(why) = client.start().await {
        println!("{}: {why:?}", get_translation("bot-error"))
    }
}
