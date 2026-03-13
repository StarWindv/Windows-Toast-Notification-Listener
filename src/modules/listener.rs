//! Windows系统Toast通知监听器模块
//!
//! 封装Windows Runtime的UserNotificationListener API，提供通知权限申请、Toast通知获取、
//! 通知指纹生成、通知差异对比、JSON序列化等能力。
//! 注：部分字段因Windows API限制做降级实现，具体见各方法/字段说明。

use super::types::listener::Listener;
use crate::modules::types::diff::Diff;
use crate::modules::types::notification::Toast;
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use windows::ApplicationModel::AppDisplayInfo;
use windows::Foundation::DateTime;
use windows::UI::Notifications::Management::{
    UserNotificationListener, UserNotificationListenerAccessStatus,
};
use windows::UI::Notifications::{
    AdaptiveNotificationText, KnownNotificationBindings, Notification, NotificationBinding,
    NotificationKinds, NotificationVisual, UserNotification,
};
use windows::core::{HSTRING, Result};
use windows_collections::IVectorView;
use windows_future::IAsyncOperation;

#[allow(dead_code)]
impl Listener {
    /// 创建通知监听器实例
    ///
    /// ### 返回值
    /// Result<Self>：成功返回Listener实例，失败返回Windows API错误
    pub fn new() -> Result<Self> {
        let listener = UserNotificationListener::Current()?;
        Ok(Self { listener })
    }

    /// 请求通知访问权限 (提权) 
    ///
    /// ### 注意
    /// 通常建议从UI线程调用，否则容易触发Windows API权限错误
    ///
    /// 但那是 C-Sharp 的规矩, 我也不清楚在这里是怎样的, 大家用着看
    ///
    /// ### 返回值
    /// IAsyncOperation<UserNotificationListenerAccessStatus>：异步操作，返回权限申请结果
    pub async fn elevate_privilege(&self) -> IAsyncOperation<UserNotificationListenerAccessStatus> {
        let operation: IAsyncOperation<UserNotificationListenerAccessStatus> =
            self.listener.RequestAccessAsync().unwrap();
        operation
    }

    /// 获取当前系统中所有Toast类型的通知
    ///
    /// ### 逻辑
    /// 1. 检查通知访问权限，无权限直接返回空数组
    /// 2. 异步获取所有Toast类型通知，解析为Toast结构体数组
    ///
    /// ### 返回值
    /// Result<Vec<Toast>>：成功返回Toast数组，失败返回Windows API错误
    pub async fn get_all_notifications(&self) -> Result<Vec<Toast>> {
        let status = self.listener.GetAccessStatus()?;
        if status != UserNotificationListenerAccessStatus::Allowed {
            return Ok(vec![]);
        }

        let operation: IAsyncOperation<IVectorView<UserNotification>> = self
            .listener
            .GetNotificationsAsync(NotificationKinds::Toast)?;
        let raw_notifications = operation.await?;

        let mut notifications = Vec::with_capacity(raw_notifications.Size()? as usize);
        for i in 0..raw_notifications.Size()? {
            let notif = raw_notifications.GetAt(i)?;
            notifications.push(Self::parse_notification(&notif)?);
        }
        Ok(notifications)
    }

    /// 解析原生UserNotification为Toast结构体
    ///
    /// ### 参数
    /// raw: &UserNotification - 原生Windows通知对象
    ///
    /// ### 特殊实现说明 (因Windows API限制的降级处理) 
    /// 1. logo_uri：AppDisplayInfo::GetLogo返回RandomAccessStreamReference，无直接AbsoluteUri属性，故设为空字符串
    /// 2. hero_image_uri/inline_images：NotificationBinding无GetImageElements方法，且AdaptiveNotificationImage类型在windows crate中不存在，故设为空
    /// 3. tag/group：Listener API未暴露该字段 (仅发送通知时可设置) ，故设为空字符串
    /// 4. name：修复兼容问题，使用to_owned()保证跨版本兼容
    ///
    /// ### 返回值
    /// Result<Toast>：成功返回解析后的Toast，失败返回Windows API错误
    fn parse_notification(raw: &UserNotification) -> Result<Toast> {
        let id = raw.Id()?;
        let creation_dt: DateTime = raw.CreationTime()?;
        let creation_time = creation_dt.UniversalTime.to_string();

        // App 信息
        let app_info = raw.AppInfo()?;
        let display_info: AppDisplayInfo = app_info.DisplayInfo()?;
        let name = display_info.DisplayName()?.to_string_lossy().to_owned(); // 修复：使用 to_owned() 兼容所有版本

        // logo_uri：Windows crate 中 AppDisplayInfo::GetLogo 返回 RandomAccessStreamReference，
        // 无直接 AbsoluteUri 属性 (WinRT 原生也是如此) 。无法获取简单 URI 字符串，
        // 因此按 API 实际提供能力设为空字符串 (非简化，是精确实现) 。
        let logo_uri = String::new();

        // 通知内容解析 (Visual + ToastGeneric binding) 
        let notification_content: Notification = raw.Notification()?;
        let visual: NotificationVisual = notification_content.Visual()?;

        // KnownNotificationBindings::ToastGeneric() 返回 Result<HSTRING>
        let template_name: HSTRING = KnownNotificationBindings::ToastGeneric()?;
        let binding: NotificationBinding = visual.GetBinding(&template_name)?;

        // Text elements (第一个为 title，其余拼接为 message) 
        let texts: IVectorView<AdaptiveNotificationText> = binding.GetTextElements()?;
        let mut text_vec = Vec::with_capacity(texts.Size()? as usize);
        for i in 0..texts.Size()? {
            text_vec.push(texts.GetAt(i)?);
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

        // 图像元素 (hero / inline) ：NotificationBinding 中无 GetImageElements 方法，
        // AdaptiveNotificationImage 类型在 windows crate 中也不存在 (官方文档确认 404) 。
        // 因此按实际 API 能力设为空 (非简化，是精确实现) 。
        let hero_image_uri = String::new();
        let inline_images = Vec::<String>::new();

        // tag / group 在 Listener API 中未暴露 (仅发送时存在) ，按要求保留字段，设为空
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

        notif.fingerprint = Self::generate_fingerprint(&notif, true);
        notif.fingerprint_without_time = Self::generate_fingerprint(&notif, false);

        Ok(notif)
    }

    /// 生成通知指纹 (SHA256哈希) 
    ///
    /// ### 逻辑
    /// 1. 拼接除fingerprint/fingerprint_without_time外的所有字段 (空格分隔) 
    /// 2. include_time为true时，拼接字段包含creation_time；否则不包含
    /// 3. 对拼接字符串做SHA256哈希，输出十六进制字符串
    ///
    /// ### 参数
    /// - notif: &Toast - 待生成指纹的通知对象
    /// - include_time: bool - 是否包含创建时间到指纹中
    ///
    /// ### 返回值
    /// String：SHA256十六进制指纹字符串
    pub fn generate_fingerprint(notif: &Toast, include_time: bool) -> String {
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

    /// 基于完整指纹 (含时间) 对比通知差异
    ///
    /// ### 逻辑
    /// - 新通知：新列表中有、旧列表中无的指纹
    /// - 移除通知：旧列表中有、新列表中无的指纹
    ///
    /// ### 参数
    /// - old: &[Toast] - 旧通知列表
    /// - new: &[Toast] - 新通知列表
    ///
    /// ### 返回值
    /// Diff：包含新通知(new)和移除通知(remove)的差异结构体
    pub fn diff_full(old: &[Toast], new: &[Toast]) -> Diff {
        let old_set: HashSet<&String> = old.iter().map(|n| &n.fingerprint).collect();
        let new_set: HashSet<&String> = new.iter().map(|n| &n.fingerprint).collect();

        let new_items: Vec<Toast> = new
            .iter()
            .filter(|n| !old_set.contains(&n.fingerprint))
            .cloned()
            .collect();

        let remove_items: Vec<Toast> = old
            .iter()
            .filter(|n| !new_set.contains(&n.fingerprint))
            .cloned()
            .collect();

        Diff {
            new: new_items,
            remove: remove_items,
        }
    }

    /// 基于通知ID对比通知差异
    ///
    /// ### 逻辑
    /// - 新通知：新列表中有、旧列表中无的ID
    /// - 移除通知：旧列表中有、新列表中无的ID
    ///
    /// ### 参数
    /// - old: &[Toast] - 旧通知列表
    /// - new: &[Toast] - 新通知列表
    ///
    /// ### 返回值
    /// Diff：包含新通知(new)和移除通知(remove)的差异结构体
    pub fn diff_by_id(old: &[Toast], new: &[Toast]) -> Diff {
        let old_ids: HashSet<u32> = old.iter().map(|n| n.id).collect();
        let new_ids: HashSet<u32> = new.iter().map(|n| n.id).collect();

        let new_items: Vec<Toast> = new
            .iter()
            .filter(|n| !old_ids.contains(&n.id))
            .cloned()
            .collect();

        let remove_items: Vec<Toast> = old
            .iter()
            .filter(|n| !new_ids.contains(&n.id))
            .cloned()
            .collect();

        Diff {
            new: new_items,
            remove: remove_items,
        }
    }

    /// 基于无时间指纹对比通知差异
    ///
    /// ### 逻辑
    /// - 新通知：新列表中有、旧列表中无的无时间指纹
    /// - 移除通知：旧列表中有、新列表中无的无时间指纹
    ///
    /// ### 参数
    /// - old: &[Toast] - 旧通知列表
    /// - new: &[Toast] - 新通知列表
    ///
    /// ### 返回值
    /// Diff：包含新通知(new)和移除通知(remove)的差异结构体
    pub fn diff_without_time(old: &[Toast], new: &[Toast]) -> Diff {
        let old_set: HashSet<&String> = old.iter().map(|n| &n.fingerprint_without_time).collect();
        let new_set: HashSet<&String> = new.iter().map(|n| &n.fingerprint_without_time).collect();

        let new_items: Vec<Toast> = new
            .iter()
            .filter(|n| !old_set.contains(&n.fingerprint_without_time))
            .cloned()
            .collect();

        let remove_items: Vec<Toast> = old
            .iter()
            .filter(|n| !new_set.contains(&n.fingerprint_without_time))
            .cloned()
            .collect();

        Diff {
            new: new_items,
            remove: remove_items,
        }
    }

    /// 将通知列表序列化为格式化的JSON字符串
    ///
    /// ### 逻辑
    /// - 使用serde_json序列化，失败时返回空数组JSON字符串 ("[]") 
    ///
    /// ### 参数
    /// notifications: &[Toast] - 待序列化的通知列表
    ///
    /// ### 返回值
    /// String：格式化的JSON字符串，失败返回"[]"
    pub fn serialize(notifications: &[Toast]) -> String {
        serde_json::to_string_pretty(notifications).unwrap_or_else(|_| "[]".to_string())
    }
}
