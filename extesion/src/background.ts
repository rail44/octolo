const MENU_TITLE = "Open with local editor";
const GITHUB_URL_PATTERN = "*://github.com/*/blob*";

interface Message {
  user: string;
  repository: string;
  revision: string;
  path: string;
  line?: number;
}

function getMessage(url: URL): Message | undefined {
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

  const message: Message = {
    user,
    repository,
    revision,
    path
  };

  const line = Number(url.hash.substring(2));
  if (line !== 0) {
    message.line = line;
  }
  return message;
}

function sendToNative(message: Message) {
  console.log(`sending message to local: ${message}`);
  chrome.runtime.sendNativeMessage("jp.rail44.octolo", message, r =>
    console.log(`received message from local: ${r}`)
  );
}

chrome.contextMenus.create({
  id: "remote-open-link",
  title: MENU_TITLE,
  contexts: ["link"],
  targetUrlPatterns: [GITHUB_URL_PATTERN],
  onclick: ({ linkUrl }) => {
    if (!linkUrl) {
      return;
    }

    const message = getMessage(new URL(linkUrl));
    if (!message) {
      return;
    }

    sendToNative(message);
  }
});

chrome.contextMenus.create({
  id: "remote-open-page",
  title: MENU_TITLE,
  contexts: ["page"],
  documentUrlPatterns: [GITHUB_URL_PATTERN],
  onclick: ({ pageUrl }) => {
    const message = getMessage(new URL(pageUrl));
    if (!message) {
      return;
    }

    sendToNative(message);
  }
});
