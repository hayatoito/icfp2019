// use std::collections::HashSet;
// use std::io::prelude::*;

// use crate::prelude::*;
use crate::task::*;

// puzzle ::= bNum, eNum, tSize, vMin, vMax, mNum, fNum, dNum,rNum,cNum, xNum # iSqs # oSqs
// iSqs, oSqs ::= repSep (point,”,”)

// 1,1,150,400,1200,6,10,5,1,3,4#(73,61),(49,125),(73,110),(98,49),(126,89),(68,102),(51,132),(101,123),(22,132),(71,120),(97,129),(118,76),(85,100),(88,22),(84,144),(93,110),(96,93),(113,138),(91,52),(27,128),(84,140),(93,143),(83,17),(123,85),(50,74),(139,97),(101,110),(77,56),(86,23),(117,59),(133,126),(83,135),(76,90),(70,12),(12,141),(116,87),(102,76),(19,138),(86,129),(86,128),(83,60),(100,98),(60,105),(61,103),(94,99),(130,124),(141,132),(68,84),(86,143),(72,119)#(145,82),(20,65),(138,99),(38,137),(85,8),(125,104),(117,48),(57,48),(64,119),(3,25),(40,22),(82,54),(121,119),(1,34),(43,98),(97,120),(10,90),(15,32),(41,13),(86,40),(3,83),(2,127),(4,40),(139,18),(96,49),(53,22),(5,103),(112,33),(38,47),(16,121),(133,99),(113,45),(50,5),(94,144),(16,0),(93,113),(18,141),(36,25),(56,120),(3,126),(143,144),(99,62),(144,117),(48,97),(69,9),(0,9),(141,16),(55,68),(81,3),(47,53)

#[derive(Default)]
pub struct Puzzle {
    pub b_num: u64,
    pub e_num: u64,
    pub t_size: u64,
    pub v_min: u64,
    pub v_max: u64,
    pub m_num: u64,
    pub f_num: u64,
    pub r_num: u64,
    pub c_num: u64,
    pub x_num: u64,
    pub i_sqs: Vec<Pos>,
    pub o_sqs: Vec<Pos>,
    // My fileds
    pub max_x: i32,
    pub max_y: i32,
}

impl Puzzle {
    pub fn parse(s: &str) -> Puzzle {
        let sss = s.split('#').collect::<Vec<_>>();
        let i_sqs = Task::parse_tour(sss[1]);
        let o_sqs = Task::parse_tour(sss[2]);

        let all = {
            let mut all = i_sqs.clone();
            all.extend(&o_sqs);
            all
        };

        let max_x = all.iter().map(|p| p.x).max().unwrap() + 1;
        let max_y = all.iter().map(|p| p.y).max().unwrap() + 1;
        Puzzle {
            i_sqs,
            o_sqs,
            max_x,
            max_y,
            ..Default::default()
        }
    }

    #[cfg(test)]
    fn dump_map(&self) -> String {
        let mut rectangles = vec![vec![' '; self.max_y as usize]; self.max_x as usize];
        for p in &self.i_sqs {
            rectangles[p.x as usize][p.y as usize] = 'o';
        }
        for p in &self.o_sqs {
            rectangles[p.x as usize][p.y as usize] = 'x';
        }

        (0..self.max_y as usize)
            .rev()
            .map(|y| {
                (0..self.max_x as usize)
                    .map(|x| rectangles[x][y])
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn puzzle_dump_test() {
        let puzzle = "1,1,150,400,1200,6,10,5,1,3,4#(73,61),(49,125),(73,110),(98,49),(126,89),(68,102),(51,132),(101,123),(22,132),(71,120),(97,129),(118,76),(85,100),(88,22),(84,144),(93,110),(96,93),(113,138),(91,52),(27,128),(84,140),(93,143),(83,17),(123,85),(50,74),(139,97),(101,110),(77,56),(86,23),(117,59),(133,126),(83,135),(76,90),(70,12),(12,141),(116,87),(102,76),(19,138),(86,129),(86,128),(83,60),(100,98),(60,105),(61,103),(94,99),(130,124),(141,132),(68,84),(86,143),(72,119)#(145,82),(20,65),(138,99),(38,137),(85,8),(125,104),(117,48),(57,48),(64,119),(3,25),(40,22),(82,54),(121,119),(1,34),(43,98),(97,120),(10,90),(15,32),(41,13),(86,40),(3,83),(2,127),(4,40),(139,18),(96,49),(53,22),(5,103),(112,33),(38,47),(16,121),(133,99),(113,45),(50,5),(94,144),(16,0),(93,113),(18,141),(36,25),(56,120),(3,126),(143,144),(99,62),(144,117),(48,97),(69,9),(0,9),(141,16),(55,68),(81,3),(47,53)";

        let _a = Puzzle::parse(puzzle).dump_map();
        // println!("{}", a);
        // assert_eq!(a, "".to_string());
    }
}
