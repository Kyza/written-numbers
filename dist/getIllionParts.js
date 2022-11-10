import { ILLION_PARTS } from "./constants";
export default function getIllionParts(partNumbers) {
    const parts = [];
    for (let chunkI = 0; chunkI < partNumbers.length; chunkI++) {
        const chunk = partNumbers[chunkI];
        const part = [];
        for (let digitI = 0; digitI < chunk.length; digitI++) {
            const digit = chunk[digitI];
            const partString = ILLION_PARTS[digitI % 3][Number(digit) - 1];
            if (digit !== "0") {
                part.push(partString);
            }
            else {
                part.push("");
            }
        }
        parts.push(part);
    }
    return parts;
}
