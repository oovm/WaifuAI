use super::*;
use crate::wss::MessageAttachment;

/// `POST /channels/{channel_id}/messages`
///
/// <https://bot.q.qq.com/wiki/develop/api/openapi/message/get_message_of_id.html>
#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageRequest {
    pub content: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub msg_id: String,
    #[serde(skip_serializing)]
    pub image: Option<MessageAttachment>,
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
    pub async fn send(&self, bot: &impl QQBotProtocol, channel_id: u64, user_id: u64) -> AckermanResult {
        let url = Url::from_str(&Self::end_point(channel_id))?;
        let response = bot
            .build_request(Method::POST, url)
            .header(CONTENT_TYPE, "application/json")
            .body(format!("{}", to_string(self)?))
            .send()
            .await?;
        if response.status().as_u16() > 300 {
            println!("{}", response.status().as_u16())
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
