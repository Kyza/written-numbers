import hundreds from "./hundreds";
import illions from "./illions";
import ones from "./ones";
import tens from "./tens";
import thousands from "./thousands";
import trimStart from "./util/trimStart";
export default function toWords(num) {
    // Ensure we're working with a string.
    // Use BigInt only if it's a whole number.
    if (typeof num === "number" && Number.isInteger(num))
        num = BigInt(num);
    // Try to convert to a string without errors to the best of my ability.
    else if (typeof num === "number")
        num = num.toLocaleString("fullwide", {
            useGrouping: false,
            maximumSignificantDigits: 21,
        });
    // BigInt is safe.
    if (typeof num === "bigint")
        num = num.toString();
    let words = "";
    let decimal = "";
    // Handle negative numbers.
    if (num.startsWith("-")) {
        words += "negative ";
        num = num.slice(1);
    }
    // Split the decimal off to handle later.
    if (num.includes(".")) {
        const [a, b] = num.split(".");
        num = a;
        decimal = b;
    }
    // Remove any leading zeros.
    num = trimStart(num, "0");
    const chunks = [];
    let chunkI = 0;
    chunkLoop: for (let i = num.length; i >= 0; i -= 3, chunkI++) {
        const chunk = num.slice(Math.max(i - 3, 0), i);
        // If the chunk is empty, skip it.
        if (chunk.padStart(3, "0") === "000")
            continue;
        switch (chunkI) {
            case 0:
                switch (chunk.length) {
                    case 1:
                        chunks.push(ones(chunk));
                        continue chunkLoop;
                    case 2:
                        chunks.push(tens(chunk));
                        continue chunkLoop;
                    default:
                        chunks.push(hundreds(chunk));
                        break;
                }
                break;
            case 1:
                chunks.push(thousands(chunk));
                break;
            default:
                chunks.push(illions(chunk, chunkI - 1));
                break;
        }
    }
    words += chunks.reverse().join(" ");
    // Wordify the decimal digits.
    if (decimal.length > 0) {
        let decimalWords = "";
        for (let i = decimal.length - 1; i >= 0; i--) {
            const digit = decimal[i];
            // Ignore any trailing zeros.
            if (decimalWords.length === 0 && digit === "0")
                continue;
            decimalWords = ` ${ones(digit)}${decimalWords}`;
        }
        words += ` point${decimalWords}`;
    }
    return words;
}
