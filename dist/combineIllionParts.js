import { ILLION_COMBINERS, ONES_ILLION_PART_MAP, ONLY_ONES_ILLIONS_REGEX, } from "./constants";
export default function combineIllionParts(parts, illionParts) {
    let result = "";
    let i = 0;
    for (const chunk of parts) {
        // Make sure to add the illi between every chunk.
        if (i > 0) {
            result += "lli";
        }
        const ones = chunk[0];
        const tens = chunk[1];
        const hundreds = chunk[2];
        const illionChunkNumbers = illionParts[i];
        // If there is nothing, it's a nillion.
        if (ones === "" && tens === "" && hundreds === "") {
            // Handle just nillion because why not.
            if (result === "") {
                result += "ni";
                continue;
            }
            if (result.endsWith("lli")) {
                result += "ni";
            }
        }
        // Grab the correct combiner for after the ones.
        let illionsCombiner = "";
        for (const [regex, combiner] of ILLION_COMBINERS) {
            if (regex.test(illionChunkNumbers)) {
                illionsCombiner = combiner;
            }
        }
        // If there is only a ones, add the correct special ones (million, billion, ...).
        if (ONLY_ONES_ILLIONS_REGEX.test(illionChunkNumbers)) {
            result += ONES_ILLION_PART_MAP[ones];
        } // Otherwise add the ones and the combiner that was found for it.
        else if (ones !== "") {
            result += `${ones}${illionsCombiner}`;
        }
        // Add the tens.
        if (tens !== "") {
            result += tens;
            switch (tens) {
                // Deci and viginti are always `i`.
                case "dec":
                case "vigint":
                    result += "i";
                    break;
                // Everything else is i if there is a hundreds, and a if there is not.
                default:
                    result += hundreds !== "" ? "a" : "i";
                    break;
            }
        }
        // Add the hundreds.
        if (hundreds !== "") {
            result += hundreds;
        }
        i++;
    }
    // Add the ending.
    result += "llion";
    return result;
}
