import hundreds from "./hundreds";
export default function thousands(digits) {
    let result = `${hundreds(digits.padStart(3, "0"))} thousand`;
    return result;
}
