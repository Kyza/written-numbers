import init, { toWords } from "../dist";

await init();

function fact(n: bigint, r = 1n) {
	if (typeof n === "number") n = BigInt(n);
	if (typeof r === "number") r = BigInt(r);

	while (n > 0n) r *= n--;
	return r;
}

// @ts-ignore
window.toWords = toWords;
// @ts-ignore
window.fact = fact;

const input = document.getElementById("input") as HTMLTextAreaElement;
const output = document.getElementById("output") as HTMLParagraphElement;

input.addEventListener("input", () => {
	output.textContent = toWords({
		number: input.value,
		options: {
			language: "la",
		},
		languageOptions: {
			// hundredAnd: true,
			// commas: true,
		},
	});
	console.log(output.textContent);
});
