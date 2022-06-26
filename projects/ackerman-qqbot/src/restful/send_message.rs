use multipart::Form;
use reqwest::{multipart, multipart::Part};
use serde_json::Value;

use super::*;

/// `POST /channels/{channel_id}/messages`
///
/// <https://bot.q.qq.com/wiki/develop/api/openapi/message/get_message_of_id.html>
#[derive(Debug)]
pub struct SendMessageRequest {
    pub content: String,
    pub msg_id: String,
    pub user_id: u64,
    pub image_bytes: Vec<u8>,
}
#[derive(Serialize, Deserialize)]
struct MessageReferenceR {
    message_reference: MessageReference,
}

#[derive(Serialize, Deserialize)]
pub struct MessageReference {
    /// 需要引用回复的消息 id
    message_id: String,
    /// 是否忽略获取引用消息详情错误
    ignore_get_message_error: bool,
}

impl SendMessageRequest {
    pub fn end_point(channel_id: u64) -> String {
        if cfg!(debug_assertions) {
            format!("https://sandbox.api.sgroup.qq.com/channels/{channel_id}/messages",)
        }
        else {
            format!("https://api.sgroup.qq.com/channels/{channel_id}/messages",)
        }
    }
    pub async fn send_error(&self, bot: &impl QQBotProtocol, channel_id: u64) {
        todo!()
    }
    pub async fn send(&self, bot: &impl QQBotProtocol, channel_id: u64) -> QQResult {
        let url = Url::from_str(&Self::end_point(channel_id))?;
        let mut image_part = Part::bytes(self.image_bytes.clone()).file_name("file_image");
        // 必须用户 @机器人才能引用
        let _ = MessageReferenceR {
            message_reference: MessageReference { message_id: self.msg_id.to_string(), ignore_get_message_error: true },
        };
        let content = format!("<@!{}> {}", self.user_id, self.content);
        let form = Form::new().text("content", content).text("msg_id", self.msg_id.to_string()).part("file_image", image_part);
        let response = bot.build_request(Method::POST, url).multipart(form).send().await?;
        if response.status().as_u16() > 300 {
            println!("{:#?}", response.json::<Value>().await?)
        }
        // let value: Value = response.json().await?;
        Ok(())
    }
}

pub struct SendArkRequest {}
// SendMessageItem {
//     content: "<@!1234>hello world".to_string(),
//     ark: Ark {
//         ark: ArkBody {
//             template_id: 37,
//             kv: vec![
//                 Kv { key: "#PROMPT#".to_string(), value: "通知提醒".to_string() },
//                 Kv { key: "#METATITLE#".to_string(), value: "标题".to_string() },
//                 Kv { key: "#METASUBTITLE#".to_string(), value: "子标题".to_string() },
//                 Kv {
//                     key: "#METACOVER#".to_string(),
//                     value: "https://vfiles.gtimg.cn/vupload/20211029/bf0ed01635493790634.jpg".to_string(),
//                 },
//                 Kv {
//                     key: "#METAURL#".to_string(),
//                     value: "https://vfiles.gtimg.cn/vupload/20211029/bf0ed01635493790634.jpg".to_string(),
//                 },
//             ],
//         },
//     },
// };
#[derive(Serialize, Deserialize)]
pub struct Ark {
    pub ark: ArkBody,
}

#[derive(Serialize, Deserialize)]
pub struct ArkBody {
    pub template_id: i64,
    pub kv: Vec<Kv>,
}

#[derive(Serialize, Deserialize)]
pub struct Kv {
    pub key: String,
    pub value: String,
}
