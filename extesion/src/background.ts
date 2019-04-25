const GITHUB_URL_PATTERN = "*://github.com/*/blob*";

function getMenuTitle(editorName: string) {
  return `Open with local editor ${editorName}`;
}

type Message = Open | GetConfig;

interface Open {
  type: "Open";
  user: string;
  repository: string;
  revision: string;
  path: string;
  editor: string;
  line?: number;
}

interface GetConfig {
  type: "GetConfig";
}

interface Config {
  browser_list: string[],
    root: string,
    path: string,
    editors: Editor[],
}

type ResponseMessage = Config;

interface Editor {
  kind: string,
    name?: string,
}

function getMessage(url: URL, editor: string): Open | undefined {
  const paths = url.pathname.split("/").filter(p => p !== "");
  if (paths.length === 0) {
    return;
  }
  const user = paths.shift()!;
  const repository = paths.shift()!;
  const kind = paths.shift();
  if (kind !== "blob") {
    return;
  }
  const revision = paths.shift()!;
  const path = paths.join("/");

  const message: Open = {
    type: "Open",
    user,
    repository,
    revision,
    path,
    editor
  };

  const line = Number(url.hash.substring(2));
  if (line !== 0) {
    message.line = line;
  }
  return message;
}

function sendToNative(message: Message, cb: (res: ResponseMessage) => void) {
  console.log(`sending message to local: ${message}`);
  chrome.runtime.sendNativeMessage("jp.rail44.octolo", message, cb);
}

sendToNative({type: "GetConfig"}, (res) => {
  console.log(res);
  for (const editor of res.editor) {
    let editorId = editor.name || editor.kind;
    chrome.contextMenus.create({
      id: `remote-open-link-${editorId}`,
      title: getMenuTitle(editorId),
      contexts: ["link"],
      targetUrlPatterns: [GITHUB_URL_PATTERN],
      onclick: ({ linkUrl }) => {
        if (!linkUrl) {
          return;
        }

        const message = getMessage(new URL(linkUrl), editorId);
        if (!message) {
          return;
        }

        sendToNative(message, (res) => console.log(res));
      }
    });

    chrome.contextMenus.create({
      id: `remote-open-page-${editorId}`,
      title: getMenuTitle(editorId),
      contexts: ["page"],
      documentUrlPatterns: [GITHUB_URL_PATTERN],
      onclick: ({ pageUrl }) => {
        const message = getMessage(new URL(pageUrl), editorId);
        if (!message) {
          return;
        }

        sendToNative(message, (res) => console.log(res));
      }
    });
  }
});
