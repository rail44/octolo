import hotkeys from "hotkeys-js";
import { Config } from "./config";

chrome.runtime.sendMessage({ kind: "getConfig" }, (config: Config) => {
  for (const editor of config.editor_list) {
    if (editor.shortcut) {
      hotkeys(editor.shortcut, () => {
        chrome.runtime.sendMessage({
          kind: "open",
          url: document.location.href,
          editor: editor.kind
        });
      });
    }
  }
});
