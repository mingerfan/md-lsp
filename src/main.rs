mod log;
mod syntax;

use tokio::sync::Mutex;

use crate::log::init_logging;
use crate::syntax::{SyntaxServer, TextChange};
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::{
    CompletionItem, CompletionOptions, CompletionOptionsCompletionItem, CompletionParams,
    CompletionResponse, InitializeParams, InitializeResult, InitializedParams, MessageType,
    PositionEncodingKind, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind,
};
use tower_lsp_server::{Client, LanguageServer, LspService, Server};
use tracing::{error, info};

struct Backend {
    client: Client,
    syntax: Mutex<SyntaxServer>,
}

impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        info!("initialize request received");

        let completion_provider = CompletionOptions {
            resolve_provider: Some(true),
            trigger_characters: Some(vec!["[".to_string(), "#".to_string(), "/".to_string()]),
            all_commit_characters: Some(vec![
                " ".to_string(),
                "\n".to_string(),
                "\t".to_string(),
                ")".to_string(),
                "]".to_string(),
            ]),
            completion_item: Some(CompletionOptionsCompletionItem {
                label_details_support: Some(true),
            }),
            ..Default::default()
        };

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                position_encoding: Some(PositionEncodingKind::UTF8),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(completion_provider),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("initialized");
        self.client
            .log_message(MessageType::INFO, "tower-lsp-server demo initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        info!("shutdown request received");
        Ok(())
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        info!("completion request received");
        Ok(None)
    }

    async fn completion_resolve(&self, item: CompletionItem) -> Result<CompletionItem> {
        Ok(item)
    }

    async fn did_open(&self, params: tower_lsp_server::ls_types::DidOpenTextDocumentParams) {
        let text = params.text_document.text;
        let mut syntax = self.syntax.lock().await;
        syntax.fresh(text);
    }

    async fn did_save(&self, _params: tower_lsp_server::ls_types::DidSaveTextDocumentParams) {
        
    }

    async fn did_change(&self, params: tower_lsp_server::ls_types::DidChangeTextDocumentParams) {
        let version = params.text_document.version;
        let mut changes = vec![];
        for change in &params.content_changes {
            let range = &change.range;
            // info!(
            //     start_line = range.as_ref().map(|r| r.start.line),
            //     start_character = range.as_ref().map(|r| r.start.character),
            //     end_line = range.as_ref().map(|r| r.end.line),
            //     end_character = range.as_ref().map(|r| r.end.character),
            //     range_version = version,
            //     "content change"
            // );
            let text_change = match range {
                Some(r) if r.start == r.end && !change.text.is_empty() => {
                    TextChange::Insert {
                        start_line: r.start.line as usize,
                        start_character: r.start.character as usize,
                        text: change.text.clone(),
                    }
                },
                Some(r) if r.start != r.end && change.text.is_empty() => {
                    TextChange::Delete {
                        start_line: r.start.line as usize,
                        start_character: r.start.character as usize,
                        end_line: r.end.line as usize,
                        end_character: r.end.character as usize,
                    }
                },
                Some(r) if r.start != r.end && !change.text.is_empty() => {
                    TextChange::Replace {
                        start_line: r.start.line as usize,
                        start_character: r.start.character as usize,
                        end_line: r.end.line as usize,
                        end_character: r.end.character as usize,
                        text: change.text.clone(),
                    }
                },
                None => TextChange::FullReplace { text: change.text.clone() },
                _ => continue, // Ignore unsupported change types
            };
            info!("parsed text change: {}", text_change);
            changes.push(text_change);
        }
        let mut syntax = self.syntax.lock().await;
        syntax.add_change(version, changes);
        if let Err(e) = syntax.commit() {
            error!(version = version, error = %e, "failed to commit changes");
            self.client
                .log_message(MessageType::ERROR, format!("failed to apply document change: {e}"))
                .await;
        }
    }
}

#[tokio::main]
async fn main() {
    init_logging();
    info!("starting stdio server");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    info!("before stdio server started");
    let (service, socket) = LspService::new(|client| Backend { client, syntax: Mutex::new(SyntaxServer::default()) });
    Server::new(stdin, stdout, socket).serve(service).await;
    info!("before stdio server exited");
}
