export declare type WordOptions = {
    and: boolean;
    commas: boolean;
};
export default function toWords(num: bigint | number | string, options?: Partial<WordOptions>): string;
