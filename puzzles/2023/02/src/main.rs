use aoc_solver_derive::aoc_solver;

#[aoc_solver(2023, 2, 1)]
fn part1(input: &str) -> String {
    let test = "";

    input.to_string()
}

#[aoc_solver(2023, 2, 2)]
fn part2(input: &str) -> String {
    println!("aaa");
    input.to_string()
}

// TODO:
// Can I adjust the macro to be "single use"? Have one call to the macro at the top of the file and pass
// the two function names to the macro. This would have the added benefit of not duplicating the year, day
// portions on the multi-use setup that is currently setup.

// TODO:
// The above two functions have something fundamentally broken with them.. The autocomplete syntax isn't working.
// I am thinking that the solution is to do something like leaving the original function in place, instead of overwriting it (like we are currently).
// instead, we should create a shadow function next to the original function, call the original function FROM the shadow function (think: pass-through/proxy)
// then, in the main func, call the shadow function..  I believe (hope) that this will lead to the intellisense in the rust-analyzer to work.
// proof: the function below, which doesn't go through the macro works as expected.
#[allow(unused)]
fn test(input: &str) -> String {
    input.replace("abcd", "").to_string()
}
