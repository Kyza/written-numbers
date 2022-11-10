export const ONES = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];
export const TEENS = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
export const TENS = [
    "zero",
    "ten",
    "twenty",
    "thirty",
    "fourty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
];
export const ONES_ILLION_PART_MAP = {
    un: "mi",
    duo: "bi",
    tre: "tri",
    quattuor: "quadri",
    quin: "quinti",
    se: "sexti",
    septe: "septi",
    octo: "octi",
    nove: "noni",
};
export const ILLION_PARTS = [
    ["un", "duo", "tre", "quattuor", "quin", "se", "septe", "octo", "nove"],
    [
        "dec",
        "vigint",
        "trigint",
        "quadragint",
        "quinquagint",
        "sexagint",
        "septuagint",
        "octogint",
        "nonagint",
    ],
    [
        "centi",
        "ducenti",
        "trecenti",
        "quadringenti",
        "quingenti",
        "sescenti",
        "septingenti",
        "octingenti",
        "nongenti",
    ],
];
const ILLION_COMBINERS = new Map();
ILLION_COMBINERS.set(/^[36]([2-5]|0[345])/, "s");
ILLION_COMBINERS.set(/^6(8|0[18])/, "x");
ILLION_COMBINERS.set(/^[79]([28]|08)/, "m");
ILLION_COMBINERS.set(/^[79]([134567]|0[1-7])/, "n");
export { ILLION_COMBINERS };
export const ONLY_ONES_ILLIONS_REGEX = /^[1-9]00$/;
export const TH_LIST = [
    "zero",
    "four",
    "six",
    "seven",
    "ten",
    "eleven",
    "teen",
    "hundred",
    "thousand",
    "illion",
];
