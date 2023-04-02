fn main() -> Result<()> {
    let file = std::fs::read_to_string("input.txt")?
        .lines()
        .filter(|line| !line.is_empty());
    // .reduce(|)

    return Ok(());
}
