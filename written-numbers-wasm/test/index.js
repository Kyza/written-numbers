import initWrittenNumbers, { to_words } from "../pkg";

await initWrittenNumbers();

window.to_words = to_words;

function fact(n, r = 1n) {
	if (typeof n === "number") n = BigInt(n);
	if (typeof r === "number") r = BigInt(r);

	while (n > 0n) r *= n--;
	return r;
}

window.fact = fact;

const input = document.getElementById("input");
const output = document.getElementById("output");

input.addEventListener("input", () => {
	output.textContent = to_words(
		input.value,
		{ language: "en" },
		{ commas: "true" }
	);
});
