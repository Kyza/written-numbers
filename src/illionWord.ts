import combineIllionParts from "./combineIllionParts";
import getIllionPartNumbers from "./getIllionPartNumbers";
import getIllionParts from "./getIllionParts";
import abs from "./util/abs";

export default function illionWord(illionNumber: number | bigint): string {
	const illionPartNumbers = getIllionPartNumbers(abs(illionNumber));

	return `${illionNumber < 0 ? "negative " : ""}${combineIllionParts(
		getIllionParts(illionPartNumbers),
		illionPartNumbers
	)}`;
}
