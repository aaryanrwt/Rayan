use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tracing::info;

pub struct RayanLsp {
    client: Client,
}

impl RayanLsp {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    async fn validate_document(&self, uri: Url, text: String) {
        let mut diagnostics = Vec::new();

        // Parse the ASG from the document text
        if let Err(e) = rayan_parser::parse_literate_document(&text) {
            // Report a parsing error diagnostic
            let diagnostic = Diagnostic::new_simple(
                Range::new(Position::new(0, 0), Position::new(0, 0)),
                format!("Rayan Parse Error: {}", e),
            );
            diagnostics.push(diagnostic);
        }

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for RayanLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        info!("Rayan LSP Initializing");
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        info!("Rayan LSP Initialized");
        self.client
            .log_message(MessageType::INFO, "Rayan Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.validate_document(uri, text).await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.pop() {
            self.validate_document(uri, change.text).await;
        }
    }
}

pub async fn run_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(RayanLsp::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
