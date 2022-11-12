import hundreds from "./hundreds";
export default function thousands(digits, options) {
    let result = `${hundreds(digits.padStart(3, "0"), options)} thousand`;
    return result;
}
