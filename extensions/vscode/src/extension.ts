import * as vscode from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions } from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
    console.log('Rayan LSP is now active!');

    let serverOptions: ServerOptions = {
        run: { command: 'rayan', args: ['lsp'] },
        debug: { command: 'rayan', args: ['lsp'] }
    };

    let clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'rayan' }],
    };

    client = new LanguageClient(
        'rayanLanguageServer',
        'Rayan Language Server',
        serverOptions,
        clientOptions
    );

    client.start();
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
