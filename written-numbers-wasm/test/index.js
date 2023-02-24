import initWrittenNumbers, { send_example_to_js, to_words } from "../pkg";

await initWrittenNumbers();

window.send_example_to_js = send_example_to_js;
window.to_words = to_words;

function fact(n, r = 1n) {
	if (typeof n === "number") n = BigInt(n);
	if (typeof r === "number") r = BigInt(r);

	while (n > 0n) r *= n--;
	return r;
}

window.fact = fact;
