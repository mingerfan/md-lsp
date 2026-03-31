mod text;

pub use text::{Text, TextChange};

use markdown::{Constructs, ParseOptions, mdast};
use smart_default::SmartDefault;
use tracing::info;
use comrak::{self, Arena};
use comrak::nodes::{Ast, NodeValue};

struct Snapshot {
    content: ropey::Rope,
    md_ast: mdast::Node,
}

#[derive(SmartDefault)]
pub struct SyntaxServer {
    text: Text,
    version_map: std::collections::HashMap<i32, Vec<TextChange>>,
    #[default(0)]
    cur_version: i32,
    md_ast: Option<mdast::Node>,
    snapshots_idx: std::collections::VecDeque<i32>,
    snapshots: std::collections::HashMap<i32, Snapshot>,
}

impl SyntaxServer {
    const SNAPSHOT_INTERVAL: usize = 10;

    pub fn add_change(&mut self, version: i32, changes: Vec<TextChange>) {
        info!(version = version, "adding changes for version");
        self.version_map.insert(version, changes);
    }

    fn snapshot_push(&mut self, content: ropey::Rope, md_ast: mdast::Node) {
        if self.snapshots_idx.len() >= Self::SNAPSHOT_INTERVAL
            && let Some(oldest) = self.snapshots_idx.pop_front() {
                self.snapshots.remove(&oldest);
            }
        let snapshot = Snapshot { content, md_ast };
        self.snapshots.insert(self.cur_version, snapshot);
        self.snapshots_idx.push_back(self.cur_version);
    }

    fn snapshot_get(&self, version: i32) -> Option<&Snapshot> {
        self.snapshots.get(&version)
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

        let parse_options = ParseOptions {
            constructs: Constructs {
                math_text: true,
                math_flow: true,
                ..Default::default()
            },
            math_text_single_dollar: true,
            ..Default::default()
        };

        

        self.md_ast = markdown::to_mdast(&self.text.get_string(), &parse_options).ok();

        if let Some(md_ast) = &self.md_ast {
            self.snapshot_push(self.text.get_content().clone(), md_ast.clone());
            self.info_ast();
            info!("initial snapshot created for version 0");
        }
    }

    pub fn info_ast(&self) {
        if let Some(md_ast) = &self.md_ast {
            info!("current AST: {:#?}", md_ast);
        } else {
            info!("no AST available");
        }
    }

    pub fn get_string(&self) -> String {
        self.text.get_string()
    }
}
