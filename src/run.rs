use lazy_static::*;
use rayon::prelude::*;
use regex::Regex;

use crate::prelude::*;
use crate::system::*;

static PART3_END: u64 = 300;

pub fn write_solution(solution: &Solution) -> Result<()> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let file = dir.join("contest/solution").join(&solution.filename);
    info!("write solution: {}", file.display());
    std::fs::write(file, &solution.solution)?;

    let file = dir.join(&format!("contest/lastrun/prob-{:03}.sol", solution.id));
    info!("write solution: {}", file.display());
    std::fs::write(file, &solution.solution)?;

    Ok(())
}

fn read_solution(id: u64, path: impl AsRef<Path>) -> Result<Solution> {
    lazy_static! {
        static ref POS_RE: Regex = Regex::new(r"\(-?\d+,-?\d+\)").unwrap();
    }

    let solution = std::fs::read_to_string(path.as_ref())?;
    let score = {
        let remove_points = POS_RE.replace_all(solution.trim(), "");
        let action_list = remove_points.split('#');
        action_list.map(|action| action.len()).max().unwrap()
    };
    // TODO: Test it.

    Ok(Solution {
        id,
        score,
        solution: solution.trim().to_string(),
        filename: path.as_ref().display().to_string(),
    })
}

pub fn update_best() -> Result<()> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for id in 1..=PART3_END {
        let submit_file = dir.join(&format!("contest/submit/prob-{:03}.sol", id));
        let submit_solution = read_solution(id, submit_file)?;

        let best_file = dir.join(&format!("contest/best/prob-{:03}.sol", id));
        if !best_file.exists() {
            std::fs::write(best_file, &submit_solution.solution)?;
        } else {
            let best_solution = read_solution(id, &best_file)?;
            if submit_solution.score < best_solution.score {
                println!(
                    "Updating... id: {}, submit score: {} < best score: {}",
                    id, submit_solution.score, best_solution.score
                );
                std::fs::write(&best_file, &submit_solution.solution)?;
            }
        }
    }
    Ok(())
}

fn best_score_for(id: u64) -> Result<usize> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let best_file = dir.join(&format!("contest/best/prob-{:03}.sol", id));
    if !best_file.exists() {
        return Err(failure::format_err!("no best file"));
    }
    let solution = read_solution(id, &best_file)?;
    Ok(solution.score)
}

pub fn run(id: u64) -> Result<()> {
    println!("> Sovling: {}", id);
    let mut system = System::new(id)?;
    system.solve()?;
    let solution = system.solution();
    write_solution(&solution)?;

    if let Ok(best_score) = best_score_for(id) {
        if solution.score == best_score {
            println!(
                "> Done: id: {:03}, score: {}, best_score {} (=)",
                id, solution.score, best_score
            );
        } else if solution.score < best_score {
            println!(
                "> Done: id: {:03}, score: {}, best_score: {} (New!)",
                id, solution.score, best_score
            );
        } else {
            println!(
                "> Done: id: {:03}, score: {}, best_score {}",
                id, solution.score, best_score
            );
        }
    } else {
        println!("> Done: id: {:03}, score: {}", id, solution.score);
    }

    Ok(())
}

pub fn run_all() -> Result<()> {
    (1..=PART3_END)
        .into_par_iter()
        .for_each(|id| run(id).unwrap());
    Ok(())
}

pub fn test_run(id: u64) -> Result<()> {
    println!("> Sovling: {}", id);
    let mut system = System::new(id)?;
    system.solve()?;
    let solution = system.solution();
    println!("> Done: id: {:03}, score: {}", id, solution.score);

    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file = dir.join("contest/testrun").join(&solution.filename);
    println!("write solution: {}", file.display());
    std::fs::write(file, &solution.solution)?;
    Ok(())
}

pub fn run_benchmark(id: u64) -> Result<()> {
    let mut system = System::new(id)?;
    system.solve()
}

pub fn report() -> Result<()> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for sub in &["lastrun", "submit"] {
        println!("{}:", sub);
        for id in 1..=PART3_END {
            let file = dir.join(&format!("contest/{}/prob-{:03}.sol", sub, id));
            let solution = read_solution(id, file)?;

            let best_score = best_score_for(id);
            let info = if let Ok(best_score) = best_score {
                if solution.score < best_score {
                    "(New!)"
                } else if solution.score == best_score {
                    "(*)"
                } else {
                    ""
                }
            } else {
                ""
            };

            println!(
                "id: {:03}, score: {} (best: {}) {}",
                id,
                solution.score,
                best_score.unwrap_or(0),
                info
            );
        }
    }
    Ok(())
}
