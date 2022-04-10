import * as vscode from "vscode";
import { workspace } from "vscode";
import * as lc from "vscode-languageclient/node";

interface LspContext {
  client: lc.LanguageClient;
  subscriptions: vscode.Disposable[];
}
let lcx: LspContext | undefined;

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

  lcx = { client, subscriptions: context.subscriptions };
  context.subscriptions.push(client.start());

  context.subscriptions.push(
    vscode.commands.registerCommand("gqls.syntax-tree", showSyntaxTree(lcx!))
  );
}

export function deactivate() {
  lcx?.client.stop();
  lcx = undefined;
}

const showSyntaxTree = (lcx: LspContext) => async () => {
  const editor = activeEditor();
  if (!editor) {
    return;
  }

  const astUri = vscode.Uri.parse("gqls://syntaxtree/tree.sexp");
  const emitter = new vscode.EventEmitter<vscode.Uri>();
  const tdcp: vscode.TextDocumentContentProvider = {
    onDidChange: emitter.event,
    provideTextDocumentContent(
      _uri: vscode.Uri,
      token: vscode.CancellationToken
    ): vscode.ProviderResult<string> {
      const params: SyntaxTreeParams = {
        textDocument: {
          uri: editor.document.uri.toString(),
          version: editor.document.version,
        },
      };
      return lcx.client.sendRequest(syntaxTree, params, token);
    },
  };

  vscode.workspace.onDidChangeTextDocument(
    (e) => isGqlDocument(e.document) && emitter.fire(astUri),
    tdcp,
    lcx.subscriptions
  );
  vscode.window.onDidChangeActiveTextEditor(
    (e) => e && isGqlDocument(e.document) && emitter.fire(astUri),
    tdcp,
    lcx.subscriptions
  );

  lcx.subscriptions.push(
    workspace.registerTextDocumentContentProvider("gqls", tdcp)
  );

  const document = await vscode.workspace.openTextDocument(astUri);

  await vscode.window.showTextDocument(document, {
    viewColumn: vscode.ViewColumn.Two,
    preserveFocus: true,
  });
};

export interface SyntaxTreeParams {
  textDocument: lc.VersionedTextDocumentIdentifier;
  range?: lc.Range;
}

export const syntaxTree = new lc.RequestType<SyntaxTreeParams, string, void>(
  "gqls/syntaxTree"
);

export function isGqlDocument(
  document: vscode.TextDocument
): document is GqlDocument {
  return document.languageId === "graphql" && document.uri.scheme === "file";
}

export function isGqlEditor(editor: vscode.TextEditor): editor is GqlEditor {
  return isGqlDocument(editor.document);
}

export interface GqlDocument extends vscode.TextDocument {}

export interface GqlEditor extends vscode.TextEditor {
  document: GqlDocument;
}

function activeEditor(): GqlEditor | undefined {
  const editor = vscode.window.activeTextEditor;
  return editor && isGqlEditor(editor) ? editor : undefined;
}
