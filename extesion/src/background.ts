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
    path,
  };

  const line = Number(url.hash.substring(2));
  if (line !== 0) {
    message.line = line;
  }
  return message;
}

function send(url: string) {
  const message = getMessage(new URL(url));
  if (!message) {
    return;
  }
  console.log(message);
  chrome.runtime.sendNativeMessage("jp.rail44.octolo", message, r =>
    console.log(r)
  );
}

chrome.contextMenus.create({
  id: "remote-open-link",
  title: "Open with external editor",
  contexts: ["link"],
  targetUrlPatterns: ["*://github.com/*/blob*"],
  onclick: ({ linkUrl }) => {
    send(linkUrl!);
  }
});

chrome.contextMenus.create(
  {
    id: "remote-open-page",
    title: "Open with external editor",
    contexts: ["page"],
    documentUrlPatterns: ["*://github.com/*/blob*"],
    onclick: ({pageUrl}) => {
      send(pageUrl!);
    }
  },
);
