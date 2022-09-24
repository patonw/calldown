use std::io::Error;
use csv::ReaderBuilder;
use calldown::*;

fn main() -> Result<(), Error> {
    env_logger::init();
    let start_index = 2;
    let max_temp = 10.0;


    let mut dates: Vec<String> = vec![];
    let mut swappable: Vec<usize> = vec![];
    let mut generals: Vec<String> = vec![];
    let mut initial: Vec<String> = vec![];
    let mut exclusions: Vec<Vec<String>> = vec![];

    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(std::io::stdin());


    let records: Vec<Record> = reader.deserialize()
        .filter_map(|it| it.ok())
        .collect();

    for (index, result) in records.iter().enumerate() {

        dates.push(result.date.to_owned());
        if index >= start_index && result.pinned.filter(|x| *x).is_none() {
            swappable.push(index);
        }

        generals.push(result.general.to_owned());
        initial.push(result.ours.to_owned());

        let ex = result.exclude.split(',')
            .map(|x| x.trim())
            .filter(|x| "-".ne(*x))
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        exclusions.push(ex);
    }

    let initial = initial;

    let objective = VectorObjective(vec![
        Box::new(GeneralAlignmentObjective::new(generals)),
        Box::new(ExclusionObjective::new(exclusions)),
        Box::new(SpacingObjective::new())
    ]);


    let solver = AnnealingSolver::new(max_temp, objective);
    let solution = solver.solve(&initial, &swappable);

    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_writer(std::io::stdout());

    records.iter()
        .zip(solution)
        .map(|(x,y)| x.with_ours(y))
        .for_each(|r| writer.serialize(r).unwrap());

    Ok(())
}
