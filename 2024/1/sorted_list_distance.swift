import Foundation

func parseFile(filename: String) -> ([Int], [Int]) {
    do {
        let content = try String(contentsOfFile: filename)

        let pairs = content
            .split(separator: "\n")
            .compactMap { line -> (Int, Int)? in
                let numbers = line.split(whereSeparator: { $0.isWhitespace }).compactMap { Int($0) }
                return numbers.count == 2 ? (numbers[0], numbers[1]) : nil
            }

        let column1 = pairs.map { $0.0 }
        let column2 = pairs.map { $0.1 }

        return (column1, column2)
    } catch {
        print("Error reading file: \(error)")
        exit(1)
    }
}

func main() {
    guard CommandLine.arguments.count > 1 else {
        print("Usage: sorted_list_distance <filename>")
        exit(1)
    }

    let filename = CommandLine.arguments[1]
    var (column1, column2) = parseFile(filename: filename)
    
    // Part 1
    column1.sort()
    column2.sort()

    let distance = zip(column1, column2)
        .map { abs($0 - $1) }
        .reduce(0, +)

    // Part 2
    let column2_occurrences_for_value = column2.reduce(into: [Int: Int]()) { counts, value in
        counts[value, default: 0] += 1
    }

    let similarity = column1
        // multiple each value in column 1 by the number of occurences in column 2
        .map { value in column2_occurrences_for_value[value, default: 0] * value }
        // Sum the result
        .reduce(0, +)

    print("Distance: ", distance)
    print("Similarity: ", similarity)
}

main()
