const HOGE_EXTENSION_ID = 'jp.rail44.octolo';

const hoge_connection = chrome.runtime.connect();

console.log('hoge');

hoge_connection.postMessage({kind: 'getConfig'}); 
