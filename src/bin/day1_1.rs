fn main() -> Result<()> {
    let file = std::fs::read_to_string("day1.test")?
        .lines()
        .filter(|line| !line.is_empty());
    // .reduce(|)

    return Ok(());
}
