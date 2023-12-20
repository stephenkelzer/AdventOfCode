#[cfg(test)]
mod day_20 {
    use itertools::Itertools;
    use num::integer::lcm;
    use std::collections::{HashMap, VecDeque};

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    enum Pulse {
        Low,
        High,
    }

    #[derive(Debug)]
    enum Module {
        Button,
        Broadcaster(Vec<String>),
        FlipFlop(bool, Vec<String>),
        Conjuction(HashMap<String, Pulse>, Vec<String>),
    }

    fn get_modules() -> HashMap<String, Module> {
        // reverse maps destinations to the modules that send to them
        let mut input_tracker: HashMap<String, Vec<String>> = HashMap::new();

        let mut modules = include_str!("input.txt")
            .lines()
            .fold(HashMap::new(), |mut acc, l| {
                let (key, destinations) = l.split_once(" -> ").unwrap();
                let cleaned_key = key.replace("&", "").replace("%", "").to_string();
                let destinations = destinations
                    .split(",")
                    .map(|x| x.trim().to_string())
                    .collect_vec();

                if key == "broadcaster" {
                    acc.insert(
                        "broadcaster".to_string(),
                        Module::Broadcaster(destinations.clone()),
                    );
                } else if key.starts_with("%") {
                    acc.insert(
                        cleaned_key.clone(),
                        Module::FlipFlop(false, destinations.clone()),
                    );
                } else if key.starts_with("&") {
                    acc.insert(
                        cleaned_key.clone(),
                        Module::Conjuction(HashMap::new(), destinations.clone()),
                    );
                } else {
                    unreachable!()
                }

                for d in destinations {
                    match input_tracker.get_mut(&d) {
                        Some(x) => {
                            x.push(cleaned_key.clone());
                        }
                        None => {
                            input_tracker.insert(d, vec![cleaned_key.clone()]);
                        }
                    }
                }

                acc
            });
        modules.insert("button".to_string(), Module::Button);

        for x in modules
            .iter_mut()
            .filter(|x| matches!(x.1, Module::Conjuction(_, _)))
        {
            let name = x.0;
            if let Module::Conjuction(memory, _) = x.1 {
                if let Some(senders) = input_tracker.get(name) {
                    for s in senders {
                        memory.insert(s.to_string(), Pulse::Low);
                    }
                }
            }
        }

        // dbg!(&modules);
        // dbg!(&input_tracker);

        modules
    }

    #[test]
    fn part_01() {
        let mut low_pulse_count: usize = 0;
        let mut high_pulse_count: usize = 0;
        let mut queue: VecDeque<(String, Pulse, String)> = VecDeque::new();

        let mut modules = get_modules();

        for _ in 0..1000 {
            // low_pulse_count += 1; // each button push adds a low pulse

            queue.push_back(("broadcaster".to_string(), Pulse::Low, "button".to_string()));

            while let Some((module_name, pulse, sent_from)) = queue.pop_front() {
                println!("PROCESSING: {} -{:?}-> {}", sent_from, pulse, module_name);
                match pulse {
                    Pulse::Low => {
                        low_pulse_count += 1;
                    }
                    Pulse::High => {
                        high_pulse_count += 1;
                    }
                }

                let module = match modules.get_mut(&module_name) {
                    Some(x) => x,
                    None => {
                        println!("noop");
                        continue;
                    }
                };

                match module {
                    Module::Button => {
                        queue.push_back((
                            "broadcaster".to_string(),
                            Pulse::Low,
                            module_name.clone(),
                        ));
                    }
                    Module::Broadcaster(destinations) => {
                        for destination in destinations {
                            queue.push_back((destination.to_string(), pulse, module_name.clone()));
                        }
                    }
                    Module::FlipFlop(state, destinations) => {
                        // println!("FLIP_STATE_IN: {state}");
                        if pulse == Pulse::Low {
                            match state {
                                true => {
                                    *state = false;
                                    for destination in destinations {
                                        queue.push_back((
                                            destination.to_string(),
                                            Pulse::Low,
                                            module_name.clone(),
                                        ));
                                    }
                                }
                                false => {
                                    *state = true;
                                    for destination in destinations {
                                        queue.push_back((
                                            destination.to_string(),
                                            Pulse::High,
                                            module_name.clone(),
                                        ));
                                    }
                                }
                            }
                        }
                        // println!("FLIP_STATE_OUT: {state}");
                    }
                    Module::Conjuction(memory, destinations) => {
                        // dbg!(&memory);

                        memory.entry(sent_from).and_modify(|x| *x = pulse);

                        let send_pulse = match memory.values().all(|x| *x == Pulse::High) {
                            true => Pulse::Low,
                            false => Pulse::High,
                        };

                        for destination in destinations {
                            queue.push_back((
                                destination.to_string(),
                                send_pulse,
                                module_name.clone(),
                            ));
                        }
                    }
                }
            }
        }

        dbg!(&low_pulse_count);
        dbg!(&high_pulse_count);

        assert_eq!(low_pulse_count * high_pulse_count, 739960225);
    }

    #[test]
    fn part_02() {
        /*

        kr
        zs
        kf
        qk

         */

        let mut queue: VecDeque<(String, Pulse, String)> = VecDeque::new();

        let mut modules = get_modules();

        let mut button_press_count = 0;

        let mut kr_sent_high_at = 0;
        let mut zs_sent_high_at = 0;
        let mut kf_sent_high_at = 0;
        let mut qk_sent_high_at = 0;

        while kr_sent_high_at == 0
            || zs_sent_high_at == 0
            || kf_sent_high_at == 0
            || qk_sent_high_at == 0
        {
            // BUTTON PRESS
            queue.push_back(("broadcaster".to_string(), Pulse::Low, "button".to_string()));
            button_press_count += 1;

            while let Some((module_name, pulse, sent_from)) = queue.pop_front() {
                if sent_from == "kr" && pulse == Pulse::High {
                    println!("KR MADE IT! {}", button_press_count);
                    kr_sent_high_at = button_press_count;
                } else if sent_from == "zs" && pulse == Pulse::High {
                    println!("ZS MADE IT! {}", button_press_count);
                    zs_sent_high_at = button_press_count;
                } else if sent_from == "kf" && pulse == Pulse::High {
                    println!("KF MADE IT! {}", button_press_count);
                    kf_sent_high_at = button_press_count;
                } else if sent_from == "qk" && pulse == Pulse::High {
                    println!("QK MADE IT! {}", button_press_count);
                    qk_sent_high_at = button_press_count;
                }

                // println!("PROCESSING: {} -{:?}-> {}", sent_from, pulse, module_name);

                let module = match modules.get_mut(&module_name) {
                    Some(x) => x,
                    None => {
                        println!("noop");
                        continue;
                    }
                };

                match module {
                    Module::Button => {
                        queue.push_back((
                            "broadcaster".to_string(),
                            Pulse::Low,
                            module_name.clone(),
                        ));
                    }
                    Module::Broadcaster(destinations) => {
                        for destination in destinations {
                            queue.push_back((destination.to_string(), pulse, module_name.clone()));
                        }
                    }
                    Module::FlipFlop(state, destinations) => {
                        // println!("FLIP_STATE_IN: {state}");
                        if pulse == Pulse::Low {
                            match state {
                                true => {
                                    *state = false;
                                    for destination in destinations {
                                        queue.push_back((
                                            destination.to_string(),
                                            Pulse::Low,
                                            module_name.clone(),
                                        ));
                                    }
                                }
                                false => {
                                    *state = true;
                                    for destination in destinations {
                                        queue.push_back((
                                            destination.to_string(),
                                            Pulse::High,
                                            module_name.clone(),
                                        ));
                                    }
                                }
                            }
                        }
                        // println!("FLIP_STATE_OUT: {state}");
                    }
                    Module::Conjuction(memory, destinations) => {
                        // dbg!(&memory);

                        memory.entry(sent_from).and_modify(|x| *x = pulse);

                        let send_pulse = match memory.values().all(|x| *x == Pulse::High) {
                            true => Pulse::Low,
                            false => Pulse::High,
                        };

                        for destination in destinations {
                            queue.push_back((
                                destination.to_string(),
                                send_pulse,
                                module_name.clone(),
                            ));
                        }
                    }
                }
            }
        }

        dbg!(kr_sent_high_at);
        dbg!(zs_sent_high_at);
        dbg!(kf_sent_high_at);
        dbg!(qk_sent_high_at);

        let answer = vec![
            kr_sent_high_at,
            zs_sent_high_at,
            kf_sent_high_at,
            qk_sent_high_at,
        ]
        .iter()
        .fold(1usize, |acc, &num| lcm(acc, num));

        assert_eq!(answer, 231897990075517);
    }
}
