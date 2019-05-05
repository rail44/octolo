import { Config } from "./config";

chrome.runtime.sendMessage({ kind: "getConfig" }, (config: Config) => {
  for (const editor of config.editor_list) {
    if (editor.shortcut) {
      console.log(editor);
      window.addEventListener("keydown", event => {
        console.log(event.key);
        if (event.key === editor.shortcut) {
          chrome.runtime.sendMessage({
            kind: "open",
            url: document.location.href,
            editor: editor.kind
          });
        }
      });
    }
  }
});
