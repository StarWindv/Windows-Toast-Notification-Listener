use crate::types::toast::Toast;
use pyo3::PyResult;
use sha2::{Digest, Sha256};
use windows::ApplicationModel::AppDisplayInfo;
use windows::Foundation::DateTime;
use windows::UI::Notifications::{
    AdaptiveNotificationText, KnownNotificationBindings, Notification, NotificationBinding,
    NotificationVisual, UserNotification,
};
use windows::core::HSTRING;
use windows_collections::IVectorView;
use crate::modules::error::ConvertToPyErr;

/// 解析原生UserNotification为Toast结构体
///
/// ### 参数
/// raw: &UserNotification - 原生Windows通知对象
///
/// ### 特殊实现说明 (因 Windows API 限制的降级处理)
/// 1. logo_uri: AppDisplayInfo::GetLogo返回RandomAccessStreamReference, 无直接AbsoluteUri属性, 故设为空字符串
/// 2. hero_image_uri/inline_images: NotificationBinding无GetImageElements方法, 且AdaptiveNotificationImage类型在windows crate中不存在, 故设为空
/// 3. tag/group: Listener API未暴露该字段 (仅发送通知时可设置) , 故设为空字符串
///
/// ### 返回值
/// Result<Toast>: 成功返回解析后的Toast, 失败返回Windows API错误
pub(crate) fn parse_notification(raw: &UserNotification) -> PyResult<Toast> {
    let id = raw.Id().auto()?;
    let creation_dt: DateTime= raw.CreationTime().auto()?;
    let creation_time = creation_dt.UniversalTime.to_string();

    let app_info = raw.AppInfo().auto()?;
    let display_info: AppDisplayInfo = app_info.DisplayInfo().auto()?;
    let name = display_info
        .DisplayName()
        .auto()?
        .to_string_lossy()
        .to_owned();

    let logo_uri = String::new();

    let notification_content: Notification = raw.Notification().auto()?;
    let visual: NotificationVisual = notification_content.Visual().auto()?;
    let template_name: HSTRING = KnownNotificationBindings::ToastGeneric().auto()?;
    let binding: NotificationBinding = visual.GetBinding(&template_name).auto()?;

    let texts: IVectorView<AdaptiveNotificationText> = binding.GetTextElements().auto()?;
    let mut text_vec = Vec::with_capacity(texts.Size().auto()? as usize);
    for i in 0..texts.Size().unwrap() {
        text_vec.push(texts.GetAt(i).auto()?);
    }
    let title = text_vec
        .first()
        .map(|t| t.Text().unwrap().to_string_lossy().to_owned())
        .unwrap_or_default();
    let message = text_vec
        .iter()
        .skip(1)
        .map(|t| t.Text().unwrap().to_string_lossy().to_owned())
        .collect::<Vec<_>>()
        .join("\n");

    let hero_image_uri = String::new();
    let inline_images = Vec::<String>::new();
    let tag = String::new();
    let group = String::new();

    let mut notif = Toast {
        id,
        name,
        logo_uri,
        title,
        message,
        hero_image_uri,
        inline_images,
        tag,
        group,
        creation_time,
        fingerprint: String::new(),
        fingerprint_without_time: String::new(),
    };

    notif.fingerprint = generate_fingerprint(&notif, true);
    notif.fingerprint_without_time = generate_fingerprint(&notif, false);

    Ok(notif)
}

/// 生成通知指纹 (SHA256哈希)
///
/// ### 逻辑
/// 1. 拼接除fingerprint/fingerprint_without_time外的所有字段 (空格分隔)
/// 2. include_time为true时, 拼接字段包含creation_time; 否则不包含
/// 3. 对拼接字符串做SHA256哈希, 输出十六进制字符串
///
/// ### 参数
/// - notif: &Toast - 待生成指纹的通知对象
/// - include_time: bool - 是否包含创建时间到指纹中
///
/// ### 返回值
/// String: SHA256十六进制指纹字符串
fn generate_fingerprint(notif: &Toast, include_time: bool) -> String {
    let mut parts = vec![
        notif.id.to_string(),
        notif.name.clone(),
        notif.logo_uri.clone(),
        notif.title.clone(),
        notif.message.clone(),
        notif.hero_image_uri.clone(),
        notif.inline_images.join(" "),
        notif.tag.clone(),
        notif.group.clone(),
    ];
    if include_time {
        parts.push(notif.creation_time.clone());
    }
    let concat = parts.join(" ");

    let mut hasher = Sha256::new();
    hasher.update(concat.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}
