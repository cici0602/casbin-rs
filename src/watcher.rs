use crate::emitter::EventData;

pub trait Watcher: Send + Sync {
    fn set_update_callback(&mut self, cb: Box<dyn FnMut(String) + Send + Sync>);
    fn update(&mut self, d: EventData);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct TestWatcher {
        callback: Option<Box<dyn FnMut(String) + Send + Sync>>,
        received_data: Arc<Mutex<Vec<String>>>,
    }

    impl TestWatcher {
        fn new() -> Self {
            Self {
                callback: None,
                received_data: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn get_received_data(&self) -> Vec<String> {
            self.received_data.lock().unwrap().clone()
        }
    }

    impl Watcher for TestWatcher {
        fn set_update_callback(
            &mut self,
            cb: Box<dyn FnMut(String) + Send + Sync>,
        ) {
            self.callback = Some(cb);
        }

        fn update(&mut self, d: EventData) {
            let data_str = d.to_string();
            if let Some(ref mut callback) = self.callback {
                callback(data_str.clone());
            }
            self.received_data.lock().unwrap().push(data_str);
        }
    }

    #[test]
    fn test_watcher_callback_receives_string_data() {
        let mut watcher = TestWatcher::new();
        let received_data = Arc::clone(&watcher.received_data);

        watcher.set_update_callback(Box::new(move |data: String| {
            received_data
                .lock()
                .unwrap()
                .push(format!("callback: {}", data));
        }));

        let event_data = EventData::AddPolicy(
            "p".to_string(),
            "policy".to_string(),
            vec!["alice".to_string(), "data1".to_string(), "read".to_string()],
        );

        watcher.update(event_data);

        let data = watcher.get_received_data();
        assert_eq!(data.len(), 2);
        assert!(data[0].contains("callback: Type: AddPolicy"));
        assert!(data[1].contains("Type: AddPolicy"));
    }
}
