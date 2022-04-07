import * as vscode from "vscode";
import * as lc from "vscode-languageclient/node";

export function activate(context: vscode.ExtensionContext) {
  context.subscriptions.push(
    vscode.commands.registerCommand("gqls.restart-server", () => {})
  );
  const opt: lc.Executable = { command: "gqls" };
  const serverOptions: lc.ServerOptions = {
    run: opt,
    debug: opt,
  };

  const clientOptions: lc.LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "graphql" }],
  };

  const client = new lc.LanguageClient(
    "gqls",
    "gqls",
    serverOptions,
    clientOptions
  );

  context.subscriptions.push(client.start());
}

export function deactivate() {}
