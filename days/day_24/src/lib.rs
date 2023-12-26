#[cfg(test)]
mod day_24 {
    use itertools::Itertools;

    type Pos = (isize, isize, isize);
    type Hailstone = (Pos, Pos);

    #[test]
    fn part_01() {
        fn intersects(
            ((x1, y1, _), (adx, ady, _)): &Hailstone,
            ((x3, y3, _), (bdx, bdy, _)): &Hailstone,
        ) -> Option<(f64, f64)> {
            let (x1, y1, adx, ady, x3, y3, bdx, bdy) = (
                *x1 as f64,
                *y1 as f64,
                *adx as f64,
                *ady as f64,
                *x3 as f64,
                *y3 as f64,
                *bdx as f64,
                *bdy as f64,
            );
            let (x2, y2, x4, y4) = (x1 + adx, y1 + ady, x3 + bdx, y3 + bdy);

            let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4))
                / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));
            let u = ((x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2))
                / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));

            (t > 0.0 && u > 0.0).then_some((x1 + adx * t, y1 + ady * t))
        }

        let hailstones: Vec<Hailstone> = include_str!("input.txt")
            .lines()
            .map(|l| {
                let (position, velocity) = l.split_once(" @ ").unwrap();
                let (x, y, z) = position
                    .split(", ")
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                let (vx, vy, vz) = velocity
                    .split(", ")
                    .map(|x| x.parse::<isize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                ((x, y, z), (vx, vy, vz))
            })
            .collect_vec();

        const LOW: f64 = 200_000_000_000_000.0;
        const HIGH: f64 = 400_000_000_000_000.0;

        let mut total = 0;
        for (i, a) in hailstones.iter().enumerate() {
            for b in hailstones.iter().skip(i + 1) {
                if let Some((x, y)) = intersects(a, b) {
                    if (LOW..=HIGH).contains(&x) && (LOW..=HIGH).contains(&y) {
                        total += 1;
                    }
                }
            }
        }

        assert_eq!(total, 26657);
    }

    #[test]
    fn part_02() {
        assert_eq!(123, 123);
    }
}
