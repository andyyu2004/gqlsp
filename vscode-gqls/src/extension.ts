import * as vscode from "vscode";
import * as lc from "vscode-languageclient/node";

interface Context {
  client: lc.LanguageClient;
}
let gcx: Context | undefined;

export function activate(context: vscode.ExtensionContext) {
  context.subscriptions.push(
    vscode.commands.registerCommand("gqls.restart-server", async () => {
      await vscode.window.showInformationMessage("Restarting gqls...");
      deactivate();
      while (context.subscriptions.length > 0) {
        context.subscriptions.pop()!.dispose();
      }

      activate(context);
    })
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

  gcx = { client };

  context.subscriptions.push(client.start());
}

export function deactivate() {
  gcx?.client.stop();
  gcx = undefined;
}
