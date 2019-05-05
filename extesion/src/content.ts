import hotkeys from "hotkeys-js";
import { Config } from "./config";
import { URL_REGEXP } from "./util";

chrome.runtime.sendMessage({ kind: "getConfig" }, (config: Config) => {
  for (const editor of config.editor_list) {
    if (editor.shortcut) {
      hotkeys(editor.shortcut, () => {
        const url = document.location.href;
        if (!URL_REGEXP.test(url)) {
          return;
        }
        chrome.runtime.sendMessage({
          kind: "open",
          url: document.location.href,
          editor: editor.kind
        });
      });
    }
  }
});
