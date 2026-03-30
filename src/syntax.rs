mod text;

pub use text::{Text, TextChange};

use smart_default::SmartDefault;
use tracing::info;

#[derive(SmartDefault)]
pub struct SyntaxServer {
    text: Text,
    version_map: std::collections::HashMap<i32, Vec<TextChange>>,
    #[default(0)]
    cur_version: i32,
}

impl SyntaxServer {
    pub fn add_change(&mut self, version: i32, changes: Vec<TextChange>) {
        info!(
            version = version,
            "adding changes for version"
        );
        self.version_map.insert(version, changes);
    }

    pub fn commit(&mut self) -> anyhow::Result<()> {
        let mut expected = self.cur_version + 1;
        info!(
            expected_version = expected,
            "committing changes for version"
        );
        while let Some(changes) = self.version_map.remove(&expected) {
            for change in changes {
                self.text.update(change)?;
            }
            expected += 1;
            self.cur_version = expected - 1;
        }
        Ok(())
    }

    pub fn fresh(&mut self, new_content: String) {
        info!("refreshing content");
        self.text.refresh(new_content);
        self.version_map.clear();
        self.cur_version = 0;
    }

    pub fn get_content(&self) -> String {
        self.text.get_content()
    }
}