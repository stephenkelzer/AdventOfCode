#[cfg(test)]
mod day_19 {
    use std::collections::HashMap;

    #[derive(Debug)]
    enum Operator {
        GreaterThan,
        LessThan,
    }

    #[derive(Debug)]
    struct Workflow {
        name: String,
        steps: Vec<WorkflowStep>,
    }
    impl Workflow {
        fn new(config: &str) -> Self {
            let (name, tail) = config.split_once("{").unwrap();

            let mut steps = vec![];

            for instruction in tail.replace("}", "").trim().split(",") {
                if instruction.contains(":") {
                    let (equation, if_true) = instruction.split_once(":").unwrap();
                    let key = &equation[0..1];
                    let op = &equation[1..2];
                    let val = &equation[2..];
                    steps.push(WorkflowStep::new_eval(key, op, val, if_true));
                } else {
                    steps.push(WorkflowStep::new_go(instruction))
                }
            }

            Self {
                name: name.to_string(),
                steps,
            }
        }
    }

    #[derive(Debug)]
    enum WorkflowStep {
        Eval(char, Operator, usize, String),
        Go(String),
    }

    impl WorkflowStep {
        fn new_eval(input_key: &str, op: &str, val: &str, if_true: &str) -> Self {
            WorkflowStep::Eval(
                input_key.chars().nth(0).unwrap(),
                match op {
                    ">" => Operator::GreaterThan,
                    "<" => Operator::LessThan,
                    _ => unreachable!(),
                },
                val.parse::<usize>().unwrap(),
                if_true.to_string(),
            )
        }

        fn new_go(go_to: &str) -> Self {
            WorkflowStep::Go(go_to.to_string())
        }
    }

    fn check_part(
        workflows: &HashMap<String, Workflow>,
        part: &HashMap<char, usize>,
    ) -> Option<usize> {
        let mut current_workflow = workflows.get("in").unwrap();

        'OUTER: loop {
            for step in &current_workflow.steps {
                let mut go_to: Option<String> = None;

                match step {
                    WorkflowStep::Eval(key, operator, value, if_true) => {
                        let part_prop = part.get(&key).unwrap();
                        if match operator {
                            Operator::GreaterThan => part_prop > value,
                            Operator::LessThan => part_prop < value,
                        } {
                            go_to = Some(if_true.clone());
                        }
                    }
                    WorkflowStep::Go(workflow_name) => {
                        go_to = Some(workflow_name.clone());
                    }
                }

                if let Some(go_to) = go_to {
                    if go_to == "A" {
                        return Some(part.values().sum());
                    } else if go_to == "R" {
                        return None;
                    } else {
                        current_workflow = workflows.get(&go_to).unwrap();
                        continue 'OUTER;
                    }
                }
            }

            unreachable!()
        }
    }

    #[test]
    fn part_01() {
        let mut workflows = HashMap::<String, Workflow>::new();
        let mut parsing_ratings = false;
        let mut ratings: Vec<HashMap<char, usize>> = vec![];

        for l in include_str!("input.txt").lines() {
            if parsing_ratings {
                // ratings
                ratings.push(l.replace("{", "").replace("}", "").split(",").fold(
                    HashMap::new(),
                    |mut acc, curr| {
                        let (key, value) = curr.split_once("=").unwrap();
                        acc.insert(key.chars().nth(0).unwrap(), value.parse::<usize>().unwrap());
                        acc
                    },
                ));
            } else if l.is_empty() {
                parsing_ratings = true;
            } else {
                // workflows
                let w = Workflow::new(l);
                let wn = w.name.clone();
                workflows.insert(wn, w);
            }
        }

        let mut answer = 0;

        for rating in &ratings {
            if let Some(part_score) = check_part(&workflows, &rating) {
                answer += part_score
            }
        }

        assert_eq!(answer, 409898);
    }

    #[test]
    fn part_02() {
        let mut workflows = HashMap::<String, Workflow>::new();
        let mut parsing_ratings = false;

        for l in include_str!("input.txt").lines() {
            if parsing_ratings {
                // skip
            } else if l.is_empty() {
                parsing_ratings = true;
            } else {
                // workflows
                let w = Workflow::new(l);
                let wn = w.name.clone();
                workflows.insert(wn, w);
            }
        }

        let mut chunks: Vec<(
            (usize, usize),
            (usize, usize),
            (usize, usize),
            (usize, usize),
            &str,
            usize,
        )> = vec![((1, 4000), (1, 4000), (1, 4000), (1, 4000), "in", 0)];
        let mut good_parts: Vec<(
            (usize, usize),
            (usize, usize),
            (usize, usize),
            (usize, usize),
        )> = vec![];
        while let Some(range) = chunks.pop() {
            let (x, m, a, s, wf_name, step_index) = range;
            if wf_name == "A" {
                good_parts.push((x, m, a, s));
                continue;
            } else if wf_name == "R" {
                continue;
            } else if x.0 > x.1 || m.0 > m.1 || a.0 > a.1 || s.0 > s.1 {
                continue;
            }

            match &workflows.get(wf_name).unwrap().steps[step_index] {
                WorkflowStep::Eval(part_key, operation, part_key_value, if_true) => match operation
                {
                    Operator::GreaterThan => match part_key {
                        'x' => {
                            chunks.push(((part_key_value + 1, x.1), m, a, s, if_true.as_str(), 0));
                            chunks.push(((x.0, *part_key_value), m, a, s, wf_name, step_index + 1));
                        }
                        'm' => {
                            chunks.push((x, (part_key_value + 1, m.1), a, s, if_true.as_str(), 0));
                            chunks.push((x, (m.0, *part_key_value), a, s, wf_name, step_index + 1));
                        }
                        'a' => {
                            chunks.push((x, m, (part_key_value + 1, a.1), s, if_true.as_str(), 0));
                            chunks.push((x, m, (a.0, *part_key_value), s, wf_name, step_index + 1));
                        }
                        's' => {
                            chunks.push((x, m, a, (part_key_value + 1, s.1), if_true.as_str(), 0));
                            chunks.push((x, m, a, (s.0, *part_key_value), wf_name, step_index + 1));
                        }
                        _ => unreachable!(),
                    },
                    Operator::LessThan => match part_key {
                        'x' => {
                            chunks.push(((x.0, part_key_value - 1), m, a, s, if_true.as_str(), 0));
                            chunks.push(((*part_key_value, x.1), m, a, s, wf_name, step_index + 1));
                        }
                        'm' => {
                            chunks.push((x, (m.0, part_key_value - 1), a, s, if_true.as_str(), 0));
                            chunks.push((x, (*part_key_value, m.1), a, s, wf_name, step_index + 1));
                        }
                        'a' => {
                            chunks.push((x, m, (a.0, part_key_value - 1), s, if_true.as_str(), 0));
                            chunks.push((x, m, (*part_key_value, a.1), s, wf_name, step_index + 1));
                        }
                        's' => {
                            chunks.push((x, m, a, (s.0, part_key_value - 1), if_true.as_str(), 0));
                            chunks.push((x, m, a, (*part_key_value, s.1), wf_name, step_index + 1));
                        }
                        _ => unreachable!(),
                    },
                },
                WorkflowStep::Go(go_to) => {
                    if go_to == "A" {
                        good_parts.push((x, m, a, s));
                        continue;
                    } else if go_to == "R" {
                        continue;
                    } else {
                        chunks.push((x, m, a, s, go_to, 0));
                        continue;
                    }
                }
            }
        }

        let answer: usize = good_parts
            .iter()
            .map(|(x, m, a, s)| {
                (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1)
            })
            .sum();

        assert_eq!(answer, 113057405770956);
    }
}
