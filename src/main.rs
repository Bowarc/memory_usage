struct Process {
    pid: String,
    // The index, in the name, of where the 'target_process' match starts
    match_index: u64,
    name: String,
    mem: String,
    virt_mem: String,
}

fn read_env() -> (String, mem::Prefix) {
    const USAGE: &str = "
Usage: memory_usage <process> [prefix]

Arguments:
  Required:
                      <process>
                      The name of the target process to monitor.

  Optional:
                      [prefix]
                      Format for displaying memory sizes:
                      `decimal` for base 10 (default),
                      `binary` for base 2. 

Example:
  memory_usage my_process decimal
  memory_usage my_process binary

Source:
  https://github.com/bowarc/memory_usage";

    let argv = std::env::args().collect::<Vec<_>>();

    let argc = argv.len();

    if argc == 1 {
        eprintln!("Expected at least one argument.\n{USAGE}");
        std::process::exit(1)
    }

    if argc > 3 {
        eprintln!("Too much arguments\n{USAGE}");
        std::process::exit(1)
    }

    let target_process = argv.get(1).unwrap().clone();
    let prefix = argv
        .get(2)
        .map(|prefix_str| match prefix_str.to_lowercase().as_str() {
            "decimal" => mem::Prefix::Decimal,
            "binary" => mem::Prefix::Binary,
            _ => {
                eprintln!("Invalid prefix '{prefix_str}'\n{USAGE}");
                std::process::exit(1)
            }
        })
        .unwrap_or(mem::Prefix::Decimal);

    (target_process, prefix)
}

fn display(filtered_processes: &[Process]) {
    // ┌─────┬──────┬────────┬────────────────┐
    // │ Pid │ Name │ Memory │ Virtual memory │
    // ├─────┼──────┼────────┼────────────────┤
    // └─────┴──────┴────────┴────────────────┘

    const PADDING: usize = 1; // Padding on each side
    const PID_LABEL: &str = "Pid";
    const NAME_LABEL: &str = "Name";
    const MEM_LABEL: &str = "Memory";
    const VIRT_MEM_LABEL: &str = "Virtual memory";

    let (pid_cell_size, name_cell_size, mem_cell_size, virt_mem_cell_size) = filtered_processes
        .iter()
        .map(|process| {
            (
                process.pid.len(),
                process.name.len(),
                process.mem.len(),
                process.virt_mem.len(),
            )
        })
        .fold(
            (
                PID_LABEL.len() + PADDING * 2,
                NAME_LABEL.len() + PADDING * 2,
                MEM_LABEL.len() + PADDING * 2,
                VIRT_MEM_LABEL.len() + PADDING * 2,
            ),
            |(max_pid_len, max_name_len, max_mem_len, max_virt_mem_len),
             (pid, name, mem, virt_mem)| {
                (
                    max_pid_len.max(pid + PADDING * 2),
                    max_name_len.max(name + PADDING * 2),
                    max_mem_len.max(mem + PADDING * 2),
                    max_virt_mem_len.max(virt_mem + PADDING * 2),
                )
            },
        );

    let header = format!(
        "\u{2502}{:^pid_cell_size$}\u{2502}{:^name_cell_size$}\u{2502}{:^mem_cell_size$}\u{2502}{:^virt_mem_cell_size$}\u{2502}",
        PID_LABEL, NAME_LABEL, MEM_LABEL, VIRT_MEM_LABEL
    );
    // let header_len_no_unicode =
    //     pid_cell_size + name_cell_size + mem_cell_size + virt_mem_cell_size + (5/* Separators*/);

    let top_bar = format!(
        "\u{250C}{}\u{252C}{}\u{252C}{}\u{252C}{}\u{2510}",
        "\u{2500}".repeat(pid_cell_size),
        "\u{2500}".repeat(name_cell_size),
        "\u{2500}".repeat(mem_cell_size),
        "\u{2500}".repeat(virt_mem_cell_size)
    );
    let middle_bar = format!(
        "\u{251C}{}\u{253C}{}\u{253C}{}\u{253C}{}\u{2524}",
        "\u{2500}".repeat(pid_cell_size),
        "\u{2500}".repeat(name_cell_size),
        "\u{2500}".repeat(mem_cell_size),
        "\u{2500}".repeat(virt_mem_cell_size)
    );
    let bottom_bar = format!(
        "\u{2514}{}\u{2534}{}\u{2534}{}\u{2534}{}\u{2518}",
        "\u{2500}".repeat(pid_cell_size),
        "\u{2500}".repeat(name_cell_size),
        "\u{2500}".repeat(mem_cell_size),
        "\u{2500}".repeat(virt_mem_cell_size)
    );

    println!("{top_bar}\n{header}\n{middle_bar}");

    filtered_processes.iter().for_each(|process| {
        println!(
            "\u{2502}{:^pid_cell_size$}\u{2502}{:^name_cell_size$}\u{2502}{:^mem_cell_size$}\u{2502}{:^virt_mem_cell_size$}\u{2502}",
            process.pid, process.name, process.mem, process.virt_mem
        )
    });
    println!("{bottom_bar}");
}

fn main() {
    let system = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::nothing()
            .with_processes(sysinfo::ProcessRefreshKind::nothing().with_memory()),
    );

    let (target_process, prefix) = read_env();

    let mut filtered_processes = system
        .processes()
        .iter()
        .filter_map(|(pid, process)| {
            let Some(process_name) = process.name().to_str() else {
                eprintln!(
                    "Failed to convert name of pid {pid}: Invalid unicode '{:?}'",
                    process.name()
                );
                return None;
            };

            let match_index = process_name.find(&target_process)? as u64;

            if !process_name.contains(&target_process) {
                return None;
            }

            Some(Process {
                pid: pid.to_string(),
                match_index,
                name: process_name.to_string(),
                mem: mem::format(process.memory(), &prefix),
                virt_mem: mem::format(process.virtual_memory(), &prefix),
            })
        })
        .collect::<Vec<Process>>();
    filtered_processes.sort_unstable_by(|p1, p2| p1.match_index.cmp(&p2.match_index));

    display(&filtered_processes)
}
