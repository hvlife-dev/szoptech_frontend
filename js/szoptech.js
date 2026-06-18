import {
  errorCallback,
  loadingCallback,
  onLoadCallback,
  resizeCallback,
} from "./modules/WindowEvents";

window.addEventListener("load", loadingCallback);

window.onload = onLoadCallback;
window.onerror = errorCallback;

window.onresize = resizeCallback;

// window.onbeforeunload = () => {
//   console.log("reload");
// };

// window.onblur = () => {
//   console.log("blur");
// };

// window.onabort = () => {
//   console.log("test");
// };

// window.onunload = () => {};

// window.online = () => {
//   console.log("online");
// };
