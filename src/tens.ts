import { TEENS, TENS } from "./constants";
import ones from "./ones";

export default function tens(digits: string): string {
	const tensDigit = digits[0];
	const onesDigit = digits[1];

	if (tensDigit === "0") {
		return ones(digits);
	} else if (tensDigit === "1") {
		return TEENS[onesDigit];
	}

	if (onesDigit === "0") return TENS[tensDigit];
	return `${TENS[tensDigit]}-${ones(onesDigit)}`;
}
