export default class Words {
    count(sentence: string): Map<string, number> {
        return sentence
            .trim()
            .toLowerCase()
            .split(/\s+/)
            .reduce((map, word) => map.set(word, map.get(word) + 1 || 1), new Map())
    }
}