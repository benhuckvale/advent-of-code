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
    let (column1, column2) = parseFile(filename: filename)
    
    print("Column 1:", column1)
    print("Column 2:", column2)
}

main()
