/// VK API клиент.

use crate::{Result, Error};
use crate::bot::vk::{VKConfig, Keyboard};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, error};

/// VK API клиент.
pub struct VKApi {
    config: VKConfig,
    client: reqwest::Client,
    base_url: String,
}

impl VKApi {
    pub fn new(config: VKConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
            base_url: "https://api.vk.com/method".to_string(),
        }
    }

    /// Отправить сообщение.
    ///
    /// # Аргументы
    ///
    /// * `peer_id` - ID получателя
    /// * `message` - Текст сообщения
    /// * `keyboard` - Клавиатура (опционально)
    pub async fn send_message(
        &self,
        peer_id: i64,
        message: &str,
        keyboard: Option<Keyboard>,
    ) -> Result<()> {
        let random_id = rand::random::<i32>();

        let mut params = json!({
            "peer_id": peer_id,
            "message": message,
            "random_id": random_id,
        });

        if let Some(kb) = keyboard {
            params["keyboard"] = serde_json::to_value(kb)
                .map_err(|e| Error::Unknown(e.to_string()))?;
        }

        self.call_method("messages.send", params).await?;

        Ok(())
    }

    /// Отправить фото.
    pub async fn send_photo(
        &self,
        peer_id: i64,
        photo_url: &str,
        message: Option<&str>,
    ) -> Result<()> {
        let random_id = rand::random::<i32>();

        let params = json!({
            "peer_id": peer_id,
            "attachment": photo_url,
            "message": message.unwrap_or(""),
            "random_id": random_id,
        });

        self.call_method("messages.send", params).await?;

        Ok(())
    }

    /// Получить информацию о пользователе.
    pub async fn get_user(&self, user_id: i64) -> Result<VKUser> {
        let params = json!({
            "user_ids": user_id,
            "fields": "photo_200,city,country",
        });

        let response: VKApiResponse<Vec<VKUser>> = self.call_method("users.get", params).await?;

        response.response
            .into_iter()
            .next()
            .ok_or_else(|| Error::NotFound("User not found".to_string()))
    }

    /// Получить информацию о группе.
    pub async fn get_group(&self, group_id: i64) -> Result<VKGroup> {
        let params = json!({
            "group_id": group_id,
            "fields": "description,members_count",
        });

        let response: VKApiResponse<Vec<VKGroup>> = self.call_method("groups.getById", params).await?;

        response.response
            .into_iter()
            .next()
            .ok_or_else(|| Error::NotFound("Group not found".to_string()))
    }

    /// Загрузить фото на сервер VK.
    pub async fn upload_photo(&self, photo_data: Vec<u8>) -> Result<String> {
        // 1. Получить URL для загрузки
        let upload_url = self.get_upload_url().await?;

        // 2. Загрузить фото
        let form = reqwest::multipart::Form::new()
            .part("photo", reqwest::multipart::Part::bytes(photo_data));

        let upload_response: UploadPhotoResponse = self.client
            .post(&upload_url)
            .multipart(form)
            .send()
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?
            .json()
            .await
            .map_err(|e| Error::Unknown(e.to_string()))?;

        // 3. Сохранить фото
        let params = json!({
            "server": upload_response.server,
            "photo": upload_response.photo,
            "hash": upload_response.hash,
        });

        let save_response: VKApiResponse<Vec<VKPhoto>> =
            self.call_method("photos.saveMessagesPhoto", params).await?;

        let photo = save_response.response
            .into_iter()
            .next()
            .ok_or_else(|| Error::Unknown("Failed to save photo".to_string()))?;

        Ok(format!("photo_{}", photo.owner_id, photo.id))
    }

    /// Получить URL для загрузки фото.
    async fn get_upload_url(&self) -> Result<String> {
        let params = json!({});

        let response: VKApiResponse<UploadServer> =
            self.call_method("photos.getMessagesUploadServer", params).await?;

        Ok(response.response.upload_url)
    }

    /// Вызвать метод VK API.
    async fn call_method<T: for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        mut params: serde_json::Value,
    ) -> Result<T> {
        // Добавление обязательных параметров
        params["access_token"] = json!(self.config.access_token);
        params["v"] = json!(self.config.api_version);

        let url = format!("{}/{}", self.base_url, method);

        debug!("Calling VK API: {} with params: {:?}", method, params);

        let response = self.client
            .post(&url)
            .json(&params)
            .send()
            .await
            .map_err(|e| {
                error!("VK API request failed: {}", e);
                Error::Unknown(e.to_string())
            })?;

        let response_text = response.text().await
            .map_err(|e| Error::Unknown(e.to_string()))?;

        debug!("VK API response: {}", response_text);

        serde_json::from_str(&response_text)
            .map_err(|e| {
                error!("Failed to parse VK API response: {}", e);
                Error::Unknown(format!("Parse error: {}", e))
            })
    }
}

/// Ответ VK API.
#[derive(Debug, Deserialize)]
struct VKApiResponse<T> {
    response: T,
}

/// Пользователь VK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VKUser {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub photo_200: Option<String>,
}

/// Группа VK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VKGroup {
    pub id: i64,
    pub name: String,
    pub screen_name: String,
    pub description: Option<String>,
    pub members_count: Option<u32>,
}

/// Фото VK.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VKPhoto {
    pub id: i64,
    pub owner_id: i64,
}

/// Сервер для загрузки.
#[derive(Debug, Deserialize)]
struct UploadServer {
    upload_url: String,
}

/// Ответ загрузки фото.
#[derive(Debug, Deserialize)]
struct UploadPhotoResponse {
    server: i64,
    photo: String,
    hash: String,
}

/// VK API клиент (публичный интерфейс).
pub struct VKApiClient {
    api: VKApi,
}

impl VKApiClient {
    pub fn new(config: VKConfig) -> Self {
        Self {
            api: VKApi::new(config),
        }
    }

    pub async fn send_message(
        &self,
        peer_id: i64,
        message: &str,
    ) -> Result<()> {
        self.api.send_message(peer_id, message, None).await
    }

    pub async fn send_message_with_keyboard(
        &self,
        peer_id: i64,
        message: &str,
        keyboard: Keyboard,
    ) -> Result<()> {
        self.api.send_message(peer_id, message, Some(keyboard)).await
    }
}
