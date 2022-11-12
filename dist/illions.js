import combineIllionParts from "./combineIllionParts";
import getIllionPartNumbers from "./getIllionPartNumbers";
import getIllionParts from "./getIllionParts";
import hundreds from "./hundreds";
export default function illions(digits, illion, options) {
    const illionPartNumbers = getIllionPartNumbers(illion);
    return `${hundreds(digits.padStart(3, "0"), options)} ${combineIllionParts(getIllionParts(illionPartNumbers), illionPartNumbers)}`;
}
