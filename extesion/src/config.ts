export interface Config {
  editor_list: Editor[];
}

interface Editor {
  shortcut?: string;
  kind: string;
  label: string;
}
