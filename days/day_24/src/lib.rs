#[cfg(test)]
mod day_24 {
    use itertools::Itertools;
    use num::Integer;

    type Pos = (i128, i128, i128);
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
                    .map(|x| x.parse::<i128>().unwrap())
                    .collect_tuple()
                    .unwrap();
                let (vx, vy, vz) = velocity
                    .split(", ")
                    .map(|x| x.parse::<i128>().unwrap())
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
        let hailstones: Vec<Hailstone> = include_str!("input.txt")
            .lines()
            .map(|l| {
                let (position, velocity) = l.split_once(" @ ").unwrap();
                let (x, y, z) = position
                    .split(", ")
                    .map(|x| x.parse::<i128>().unwrap())
                    .collect_tuple()
                    .unwrap();
                let (vx, vy, vz) = velocity
                    .split(", ")
                    .map(|x| x.parse::<i128>().unwrap())
                    .collect_tuple()
                    .unwrap();
                ((x, y, z), (vx, vy, vz))
            })
            .collect_vec();

        let ((a, b, c), (d, e, f)) = hailstones[0];
        let ((g, h, i), (j, k, l)) = hailstones[1];
        let ((m, n, o), (p, q, r)) = hailstones[2];

        let mut matrix = [
            [
                0,
                l - f,
                e - k,
                0,
                c - i,
                h - b,
                e * c - b * f + h * l - k * i,
            ],
            [
                0,
                r - f,
                e - q,
                0,
                c - o,
                n - b,
                e * c - b * f + n * r - q * o,
            ],
            [
                f - l,
                0,
                j - d,
                i - c,
                0,
                a - g,
                a * f - d * c + j * i - g * l,
            ],
            [
                f - r,
                0,
                p - d,
                o - c,
                0,
                a - m,
                a * f - d * c + p * o - m * r,
            ],
            [
                k - e,
                d - j,
                0,
                b - h,
                g - a,
                0,
                d * b - a * e + g * k - j * h,
            ],
            [
                q - e,
                d - p,
                0,
                b - n,
                m - a,
                0,
                d * b - a * e + m * q - p * n,
            ],
        ];

        for pivot in 0..6 {
            for row in &mut matrix[pivot..] {
                if row[pivot] < 0 {
                    row.iter_mut().for_each(|n| *n = -*n);
                }
            }

            loop {
                for row in &mut matrix[pivot..] {
                    let mut factor = 0;

                    for &next in row.iter() {
                        if next != 0 {
                            if factor == 0 {
                                factor = next.abs();
                            } else {
                                factor = factor.gcd(&next.abs());
                            }
                        }
                    }

                    row.iter_mut().for_each(|c| *c /= factor);
                }

                let column = matrix.map(|row| row[pivot]);

                if column[pivot..].iter().filter(|&&c| c > 0).count() == 1 {
                    let index = column.iter().rposition(|&c| c > 0).unwrap();
                    matrix.swap(pivot, index);
                    break;
                }

                let min = *column[pivot..].iter().filter(|&&c| c > 0).min().unwrap();
                let index = column.iter().rposition(|&c| c == min).unwrap();

                for row in pivot..6 {
                    if row != index && column[row] != 0 {
                        let factor = column[row] / min;

                        for col in 0..7 {
                            matrix[row][col] -= factor * matrix[index][col];
                        }
                    }
                }
            }
        }

        for pivot in (0..6).rev() {
            matrix[pivot][6] /= matrix[pivot][pivot];

            for row in 0..pivot {
                matrix[row][6] -= matrix[pivot][6] * matrix[row][pivot];
            }
        }

        let answer = matrix[0][6] + matrix[1][6] + matrix[2][6];

        assert_eq!(answer, 828418331313365);
    }
}
