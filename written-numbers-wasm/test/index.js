import initWrittenNumbers, { send_example_to_js, to_words } from "../pkg";

await initWrittenNumbers();

window.send_example_to_js = send_example_to_js;
window.to_words = to_words;
