export default class Gigasecond {
    constructor(private readonly initialDate: Date) {}

    date() {
        return new Date(this.initialDate.getTime() + 10 ** 12)
    }
}