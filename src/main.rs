use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::{
    CompletionItem, CompletionItemKind, CompletionOptions, CompletionOptionsCompletionItem,
    CompletionParams, CompletionResponse, Documentation, InitializeParams, InitializeResult,
    InitializedParams, InsertTextFormat, MessageType, PositionEncodingKind, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind,
};
use tower_lsp_server::{Client, LanguageServer, LspService, Server};
use tracing::info;

#[derive(Debug)]
struct Backend {
    client: Client,
}

fn init_logging() {
    let _ = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .with_max_level(tracing::Level::INFO)
        .try_init();
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
}

#[tokio::main]
async fn main() {
    init_logging();
    info!("starting stdio server");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
