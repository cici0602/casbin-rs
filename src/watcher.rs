use crate::emitter::EventData;

pub trait Watcher: Send + Sync {
    fn set_update_callback(&mut self, cb: Box<dyn FnMut(String) + Send + Sync>);
    fn update(&mut self, d: EventData);
}

// 简单的CustomWatcher实现
pub struct CustomWatcher {
    callback: Option<Box<dyn FnMut(String) + Send + Sync>>,
}

impl CustomWatcher {
    pub fn new() -> Self {
        Self {
            callback: None,
        }
    }
}

impl Watcher for CustomWatcher {
    fn set_update_callback(&mut self, cb: Box<dyn FnMut(String) + Send + Sync>) {
        self.callback = Some(cb);
    }

    fn update(&mut self, d: EventData) {
        if let Some(ref mut callback) = self.callback {
            // 将EventData转换为字符串payload
            let payload = match d {
                EventData::AddPolicy(_, _, _) => "policy_updated:add_policy".to_string(),
                EventData::AddPolicies(_, _, _) => "policy_updated:add_policies".to_string(),
                EventData::RemovePolicy(_, _, _) => "policy_updated:remove_policy".to_string(),
                EventData::RemovePolicies(_, _, _) => "policy_updated:remove_policies".to_string(),
                EventData::RemoveFilteredPolicy(_, _, _) => "policy_updated:remove_filtered_policy".to_string(),
                EventData::SavePolicy(_) => "policy_updated:save_policy".to_string(),
                EventData::ClearPolicy => "policy_updated:clear_policy".to_string(),
                EventData::ClearCache => "cache_updated:clear_cache".to_string(),
            };
            callback(payload);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emitter::EventData;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_custom_watcher() {
        let mut watcher = CustomWatcher::new();
        let callback_called = Arc::new(AtomicBool::new(false));
        let callback_called_clone = callback_called.clone();
        
        // 设置回调函数，接收字符串payload
        watcher.set_update_callback(Box::new(move |payload: String| {
            // 验证payload格式
            assert!(payload.starts_with("policy_updated:"));
            callback_called_clone.store(true, Ordering::SeqCst);
        }));
        
        // 触发更新
        watcher.update(EventData::AddPolicy("p".to_string(), "p".to_string(), vec!["alice".to_string(), "data1".to_string(), "read".to_string()]));
        
        // 验证回调被调用
        assert!(callback_called.load(Ordering::SeqCst));
        
        // 重置标志
        callback_called.store(false, Ordering::SeqCst);
        
        // 测试另一种事件类型
        watcher.update(EventData::ClearPolicy);
        
        // 验证回调被调用
        assert!(callback_called.load(Ordering::SeqCst));
    }
}
