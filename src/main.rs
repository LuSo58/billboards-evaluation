use std::collections::HashMap;
use chrono::NaiveTime;

#[derive(Eq, PartialEq, Debug, Clone)]
struct LogEntry {
    time: NaiveTime,
    team: String,
}

impl LogEntry {
    pub fn new(time: NaiveTime, team: String) -> Self {
        Self { time, team }
    }
    pub fn time(&self) -> NaiveTime {
        self.time
    }
    pub fn team(&self) -> &str {
        &self.team
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct BoardPart {
    size: u64,
    log: Vec<LogEntry>,
}

impl BoardPart {
    pub fn new(size: u64, log: Vec<LogEntry>) -> Self {
        Self { size, log }
    }
    pub fn parse(size: u64, log: &str) -> Result<Self, String> {
        let log = log.lines()
            .enumerate()
            .map(|(idx, line)| {
                match line.split_once(',') {
                    None => Err(format!("Line {idx} does not contain a comma: {line:?}")),
                    Some((time_str, team_str)) => {
                        time_str.parse()
                            .map_err(|err| {
                                format!("Could not parse timestamp on line {idx} = {time_str:?}: {err}")
                            })
                            .map(|time| {
                                LogEntry::new(time, team_str.to_string())
                            })
                    }
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::new(size, log))
    }
    pub fn size(&self) -> u64 {
        self.size
    }
    pub fn log(&self) -> &Vec<LogEntry> {
        &self.log
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Board(Vec<BoardPart>);

impl Board {
    fn evaluate(&self) -> HashMap<String, u64> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    macro_rules! log_entry {
        ($h:literal, $m:literal, $s:literal, $team:literal) => {
            LogEntry::new(NaiveTime::from_hms_opt($h, $m, $s).unwrap(), $team.to_string())
        };
    }

    #[test]
    fn parsing() {
        let reference = BoardPart::new(3, vec![
            log_entry!(14, 1, 2, "red"),
            log_entry!(14, 1, 8, "red"),
            log_entry!(14, 1, 40, "blue"),
            log_entry!(14, 1, 52, "green"),
            log_entry!(14, 2, 20, "green"),
            log_entry!(14, 2, 24, "orange"),
            log_entry!(14, 2, 41, "red"),
            log_entry!(14, 2, 59, "yellow"),
            log_entry!(14, 3, 4, "yellow"),
            log_entry!(14, 4, 8, "orange"),
            log_entry!(14, 4, 10, "orange"),
            log_entry!(14, 4, 13, "orange"),
            log_entry!(14, 4, 15, "orange"),
            log_entry!(14, 5, 0, "red"),
            log_entry!(14, 5, 7, "blue"),
            log_entry!(14, 5, 30, "red"),
            log_entry!(14, 6, 44, "blue"),
            log_entry!(14, 6, 56, "red"),
            log_entry!(14, 7, 23, "blue"),
            log_entry!(14, 7, 48, "red"),
            log_entry!(14, 7, 57, "blue"),
            log_entry!(14, 8, 2, "yellow"),
            log_entry!(14, 8, 6, "yellow"),
            log_entry!(14, 14, 2, "orange"),
        ]);
        let text = r#"14:01:02,red
14:01:08,red
14:01:40,blue
14:01:52,green
14:02:20,green
14:02:24,orange
14:02:41,red
14:02:59,yellow
14:03:04,yellow
14:04:08,orange
14:04:10,orange
14:04:13,orange
14:04:15,orange
14:05:00,red
14:05:07,blue
14:05:30,red
14:06:44,blue
14:06:56,red
14:07:23,blue
14:07:48,red
14:07:57,blue
14:08:02,yellow
14:08:06,yellow
14:14:02,orange
"#;
        let parsed = BoardPart::parse(3, text);
        assert_eq!(parsed, Ok(reference))
    }
}

fn main() {
    println!("Hello, world!");
}
