use clap::Parser;
use std::fs::read_to_string;

#[derive(Parser)]
struct Args {
    #[arg(short = 'e', long = "env", value_name = "env", default_value = "prod")]
    env: String,
}

#[derive(Clone, Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn intersects_with(&self, r2: &Range) -> bool {
        return self.start <= r2.end && r2.start <= self.end;
    }

    fn make_intersection(&self, r2: &Range) -> Range {
        return Range {
            start: self.start.max(r2.start),
            end: self.end.min(r2.end),
        };
    }

    fn make_union(&self, r2: &Range) -> Range {
        return Range {
            start: self.start.min(r2.start),
            end: self.end.max(r2.end),
        };
    }

    fn calculate_symmetric_difference(&self, r2: &Range) -> Vec<Range> {
        if !&self.intersects_with(r2) {
            return vec![self.clone()];
        }

        let intersection = self.make_intersection(r2);

        let mut result: Vec<Range> = vec![];
        if self.start.min(r2.start) != intersection.start {
            result.push(Range {
                start: self.start.min(r2.start),
                end: intersection.start,
            })
        }

        if self.end.min(r2.end) != intersection.end {
            result.push(Range {
                start: self.end.min(r2.end),
                end: intersection.end,
            })
        }

        return result;
    }

    fn calculate_symmetric_differences(&self, ranges: &Vec<Range>) -> Vec<Range> {
        let mut diffs: Vec<Range> = vec![];

        ranges.iter().for_each(|r| {
            let sym_defs = self.calculate_symmetric_difference(r);
            diffs.extend(sym_defs);
        });

        if diffs.len() == 0 {
            return diffs;
        }

        diffs.sort_by(|a, b| a.start.cmp(&b.start));
        let mut sorted_diffs: Vec<Range> = vec![diffs.get(0).unwrap().clone()];

        diffs
            .clone()
            .iter()
            .skip(1)
            .enumerate()
            .for_each(|(idx, i)| {
                if idx < diffs.len() {
                    return;
                }

                let last_el = sorted_diffs.last_mut().unwrap();

                if last_el.intersects_with(i) {
                    *last_el = last_el.make_union(i);
                } else {
                    sorted_diffs.push(i.clone());
                }
            });

        return sorted_diffs;
    }

    pub fn make_unions(ranges: Vec<Range>) -> Vec<Range> {
        let mut cloned_ranges: Vec<Range> = vec![ranges.get(0).unwrap().clone()];

        ranges
            .clone()
            .iter()
            .skip(1)
            .enumerate()
            .for_each(|(idx, i)| {
                if idx < ranges.len() {
                    return;
                }

                let last_el = cloned_ranges.last_mut().unwrap();

                if last_el.intersects_with(i) {
                    *last_el = last_el.make_union(i);
                } else {
                    cloned_ranges.push(i.clone());
                }
            });

        return cloned_ranges;
    }
}

#[derive(Debug)]
struct LocationMapper {
    source: usize,
    destination: usize,
    step: usize,
}

fn main() {
    let args = Args::parse();
    let input_filename = match args.env.as_str() {
        "test" => "./inputs/day5.test.in",
        "prod" => "./inputs/day5.in",
        _ => panic!("No env value provided"),
    };

    let file = read_to_string(input_filename).expect("input file to be read");
    let lines = file.trim_end().split("\n").collect::<Vec<&str>>();
    let map_sections = file
        .split("\n\n")
        .skip(1)
        .map(|m| {
            let numbers = m
                .trim()
                .split("\n")
                .skip(1)
                .map(|x| {
                    let nums = x.split(" ").collect::<Vec<&str>>();
                    return LocationMapper {
                        source: nums[1].parse::<usize>().unwrap(),
                        destination: nums[0].parse::<usize>().unwrap(),
                        step: nums[2].parse::<usize>().unwrap(),
                    };
                })
                .collect::<Vec<_>>();

            return numbers;
        })
        .collect::<Vec<_>>();

    let seeds = lines[0].split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .collect::<Vec<&str>>();

    let seed_ranges = seeds
        .chunks(2)
        .map(|x| {
            let range_start = x[0].parse::<usize>().unwrap();
            let range_step = x[1].parse::<usize>().unwrap() - 1;
            return Range {
                start: range_start,
                end: range_start + range_step,
            };
        })
        .collect::<Vec<_>>();

    // println!("maps : {:?}", map_sections);
    // println!("maps len : {}", map_sections.len());

    let mut results: Vec<Range> = vec![];

    seed_ranges.iter().for_each(|range| {
        // println!("{}", ":".repeat(40));
        // println!("RANGE {:?}", range);
        let mut ranges = vec![range.clone()];

        map_sections.iter().for_each(|map_section| {
            // println!("looop map section {:?}", map_section);
            // println!("{:?}", "-".repeat(25));
            let mut mapped_ranges: Vec<Range> = vec![];
            let mut original_intersections: Vec<Range> = vec![];
            // let mut sym_defs: Vec<Range> = vec![];

            ranges.iter().for_each(|range| {
                map_section.iter().for_each(|mapper| {
                    // println!("+++++");
                    // println!("mapper : {:?}", mapper);
                    // println!("mapper : {:?}", ranges);

                    let map_length = mapper.source.max(mapper.destination)
                        - mapper.source.min(mapper.destination);
                    let map_direction = if mapper.destination > mapper.source {
                        "+"
                    } else {
                        "-"
                    };

                    let map_source_range = Range {
                        start: mapper.source,
                        end: mapper.source + mapper.step - 1,
                    };

                    let range_intersects = range.intersects_with(&map_source_range);
                    // println!("intersect : {}", range_intersects);

                    if range_intersects {
                        let intersection = range.make_intersection(&map_source_range);
                        let destination_range = if map_direction == "+" {
                            Range {
                                start: intersection.start + map_length,
                                end: intersection.end + map_length,
                            }
                        } else {
                            Range {
                                start: intersection.start - map_length,
                                end: intersection.end - map_length,
                            }
                        };
                        // println!("range : {:?}", range);
                        // println!("destination range : {:?}", destination_range);

                        mapped_ranges.push(destination_range);
                        original_intersections.push(intersection);
                        // println!("intersections : {:?}", mapped_ranges);
                    }
                });

                mapped_ranges.sort_by(|a, b| return a.start.cmp(&b.start));
                // println!("Original intersection : {:?}", original_intersections);
                let sym_defs = range.calculate_symmetric_differences(&original_intersections);
                // println!("symmetric differences vec : {:?}", sym_defs);
                // sym_defs.extend_from_slice(&intersections);
                if sym_defs.len() > 0 {
                    let union = Range::make_unions(sym_defs);
                    // println!("UNIONZ : {:?}", union);
                    mapped_ranges.extend(union);
                }
            });

            // println!("intersections : {:?}", mapped_ranges);
            // println!("==========");
            if mapped_ranges.len() > 0 {
                ranges = mapped_ranges.clone();
            }
        });

        // println!("ranges : {:?}", ranges);
        results.extend(ranges);
    });

    // println!("{}", total);
    results.sort_by(|a, b| a.start.cmp(&b.start));
    println!("{:?}", results);
    println!("{:?}", results.get(0).unwrap().start);
}
