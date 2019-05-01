export default class Squares {
    public sumOfSquares: number;
    public squareOfSum: number;
    public difference: number;

    constructor(n: number) {
        this.sumOfSquares = (n * (n + 1) * (n * 2 + 1)) / 6;
        this.squareOfSum = (n * (n + 1) * 0.5) ** 2;
        this.difference = Math.abs(this.sumOfSquares - this.squareOfSum);
    }
}