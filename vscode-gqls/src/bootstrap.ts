import fetch from "node-fetch";
import * as vscode from "vscode";
import { Config } from "./config";
import * as tar from "tar-fs";
import { dirname, join } from "path";
import { createGunzip } from "zlib";
import assert = require("assert");

const GQLS_VERSION = "v0.0.4";

export async function bootstrap(
  context: vscode.ExtensionContext,
  config: Config
): Promise<string> {
  return serverPath(context, config);
}

async function exists(path: vscode.Uri) {
  return await vscode.workspace.fs.stat(path).then(
    () => true,
    () => false
  );
}

async function serverPath(
  context: vscode.ExtensionContext,
  config: Config
): Promise<string> {
  if (config.path) {
    return config.path;
  }

  const ext = process.platform === "win32" ? ".exe" : "";
  const binPath = vscode.Uri.joinPath(context.extensionUri, "bin");
  const path = vscode.Uri.joinPath(binPath, `gqls${ext}@${GQLS_VERSION}`);

  if (await exists(path)) {
    return path.fsPath;
  }

  if (await exists(binPath)) {
    await vscode.workspace.fs.delete(binPath, { recursive: true });
  }
  await vscode.workspace.fs.createDirectory(binPath);

  await downloadServer(path);

  assert(
    await exists(path),
    "failed to download gqls (file was not where it was expected)"
  );

  return path.fsPath;
}

async function downloadServer(path: vscode.Uri) {
  /* eslint-disable @typescript-eslint/naming-convention */
  type Releases = {
    tag_name: string;
    assets: {
      name: string;
      browser_download_url: string;
    }[];
  };
  /* eslint-enable */

  const data = (await fetch(
    "https://api.github.com/repos/andyyu2004/gqls/releases"
  ).then((res) => res.json())) as any as Releases[];
  const downloadUrl = data
    .find((release) => release.tag_name === GQLS_VERSION)
    ?.assets.find((asset) => asset.name === distName())?.browser_download_url;

  if (!downloadUrl) {
    throw new Error(`failed to find release for gqls@${GQLS_VERSION}`);
  }

  const res = await fetch(downloadUrl);

  const bindir = dirname(path.fsPath);
  await new Promise((resolve, reject) => {
    const stream = res.body?.pipe(createGunzip())?.pipe(tar.extract(bindir));
    if (!stream) {
      throw new Error("failed to download gqls");
    }
    stream.on("finish", resolve);
    stream.on("error", reject);
  });

  await vscode.workspace.fs.rename(vscode.Uri.file(join(bindir, "gqls")), path);
}

function distName() {
  switch (process.platform) {
    case "darwin":
      switch (process.arch) {
        case "x64":
          return "gqls-x86_64-apple-darwin.tar.gz";
        case "arm64":
          return "gqls-aarch64-apple-darwin.tar.gz";
      }

    case "linux":
      switch (process.arch) {
        case "x64":
          return "gqls-x86_64-unknown-linux-gnu.tar.gz";
        case "arm64":
          return "gqls-aarch64-unknown-linux-gnu.tar.gz";
      }

    case "win32":
      switch (process.arch) {
        case "x64":
          return "gqls-x86_64-pc-windows-msvc.tar.gz";
        case "arm64":
          return "gqls-aarch64-pc-windows-msvc.tar.gz";
      }

    default:
      throw new Error(
        `there are no prebuilt binaries for your platform: only (windows/darwin/linux)(x64|aarch64).
        please build from source (https://github.com/andyyu2004/gqls) and set \`gqls.path\`.`
      );
  }
}
