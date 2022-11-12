import ones from "./ones";
import tens from "./tens";
import { WordOptions } from "./toWords";

export default function hundreds(
	digits: string,
	options: WordOptions
): string {
	const hundredsDigit = digits[0];
	const tensDigit = digits[1];
	const onesDigit = digits[2];

	if (hundredsDigit === "0" && tensDigit === "0") return ones(onesDigit);
	if (hundredsDigit === "0") return tens(`${tensDigit}${onesDigit}`);

	let result = `${ones(hundredsDigit)} hundred`;

	if (tensDigit === "0" && onesDigit === "0") return result;
	if (tensDigit === "0") return `${result} ${ones(onesDigit)}`;

	return `${result}${options.and ? " and" : ""} ${tens(
		`${tensDigit}${onesDigit}`
	)}`;
}
