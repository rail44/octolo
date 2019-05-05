export const URL_PATTERN = "*://github.com/*/blob*";

function matchPatternToRegExp(pattern: string) {
  if (pattern === "") {
    return /^(?:http|https|file|ftp|app):\/\//;
  }

  const schemeSegment = "(\\*|http|https|file|ftp)";
  const hostSegment = "(\\*|(?:\\*\\.)?(?:[^/*]+))?";
  const pathSegment = "(.*)";
  const matchPatternRegExp = new RegExp(
    `^${schemeSegment}://${hostSegment}/${pathSegment}$`
  );

  let match = matchPatternRegExp.exec(pattern);
  if (!match) {
    throw new TypeError(`"${pattern}" is not a valid MatchPattern`);
  }

  let [, scheme, host, path] = match;
  if (!host) {
    throw new TypeError(`"${pattern}" does not have a valid host`);
  }

  let regex = "^";

  if (scheme === "*") {
    regex += "(http|https)";
  } else {
    regex += scheme;
  }

  regex += "://";

  if (host && host === "*") {
    regex += "[^/]+?";
  } else if (host) {
    if (host.match(/^\*\./)) {
      regex += "[^/]*?";
      host = host.substring(2);
    }
    regex += host.replace(/\./g, "\\.");
  }

  if (path) {
    if (path === "*") {
      regex += "(/.*)?";
    } else if (path.charAt(0) !== "/") {
      regex += "/";
      regex += path.replace(/\./g, "\\.").replace(/\*/g, ".*?");
      regex += "/?";
    }
  }

  regex += "$";
  return new RegExp(regex);
}

export const URL_REGEXP = matchPatternToRegExp(URL_PATTERN);
