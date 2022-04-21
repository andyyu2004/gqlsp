import { ExtensionContext, workspace } from "vscode";

export interface Config {
  path?: string;
}

export function makeConfig(): Config {
  const cfg = workspace.getConfiguration("gqls");
  return {
    path: cfg.get("path"),
  };
}
