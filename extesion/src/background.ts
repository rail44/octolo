const EXTENSION_ID = 'jp.rail44.octolo';

const connection = chrome.runtime.connect();
console.log(connection);
const GITHUB_URL_PATTERN = "*://github.com/*/blob*";

function getMenuTitle(editorName: string) {
  return `Open with ${editorName}`;
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
  editor_list: Editor[];
}

type ResponseMessage = Config;

interface Editor {
  shortcut?: string;
  kind: string;
  label: string;
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
  console.log(`sending message to local: ${JSON.stringify(message)}`);
  chrome.runtime.sendNativeMessage(EXTENSION_ID, message, cb);
}

let config;

sendToNative({ type: "GetConfig" }, res => {
  console.log(res);
  config = res;

  connection.onMessage.addListener((msg) => {
    console.log(msg);
    if (msg.kind !== 'getConfig') {
      return;
    }
  });

  for (const editor of config.editor_list) {
    chrome.contextMenus.create({
      id: `remote-open-link-${editor.kind}`,
      title: getMenuTitle(editor.label),
      contexts: ["link"],
      targetUrlPatterns: [GITHUB_URL_PATTERN],
      onclick: ({ linkUrl }) => {
        if (!linkUrl) {
          return;
        }

        const message = getMessage(new URL(linkUrl), editor.kind);
        if (!message) {
          return;
        }

        sendToNative(message, res => console.log(res));
      }
    });

    chrome.contextMenus.create({
      id: `remote-open-page-${editor.kind}`,
      title: getMenuTitle(editor.label),
      contexts: ["page"],
      documentUrlPatterns: [GITHUB_URL_PATTERN],
      onclick: ({ pageUrl }) => {
        const message = getMessage(new URL(pageUrl), editor.kind);
        if (!message) {
          return;
        }

        sendToNative(message, res => console.log(res));
      }
    });
  }
});
