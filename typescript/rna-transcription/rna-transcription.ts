class Transcriptor {
    toRna(sequence: string) {
        if (/[^CGAT]/g.test(sequence)) {
            throw new Error("Invalid input DNA.")
        }

        return sequence.split("").map((c: "C" | "G" | "A" | "T") => {
            switch (c) {
                case "C": return "G"
                case "G": return "C"
                case "A": return "U"
                case "T": return "A"
            }
        }).join("")
    }
}

export default Transcriptor