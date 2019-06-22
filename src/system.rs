use crate::prelude::*;
use crate::task::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    DoNothing,
    TurnClockWise,
    TurnCounterClockWise,
    ExtendManipulator(PosDiff),
    AttachFastWheels,
    Cloning,
    AttachDrill,
}

impl Action {
    fn is_move(&self) -> bool {
        use Action::*;
        match self {
            MoveUp | MoveDown | MoveLeft | MoveRight => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Action::*;
        write!(
            f,
            "{}",
            match self {
                MoveUp => "W".to_string(),
                MoveDown => "S".to_string(),
                MoveLeft => "A".to_string(),
                MoveRight => "D".to_string(),
                DoNothing => "Z".to_string(),
                TurnClockWise => "E".to_string(),
                TurnCounterClockWise => "Q".to_string(),
                AttachFastWheels => "F".to_string(),
                ExtendManipulator(posdiff) => format!("B({},{})", posdiff.dx, posdiff.dy),
                Cloning => "C".to_string(),
                AttachDrill => "L".to_string(),
            }
        )
    }
}

#[derive(Debug, Clone)]
struct Manipulator {
    posdiff: PosDiff,
    reachable_cell: Vec<PosDiff>,
}

impl Manipulator {
    fn plot(posdiff: PosDiff) -> Vec<PosDiff> {
        let pos_dx = posdiff.dx.abs();
        let pos_dy = posdiff.dy.abs();

        let mut plot = vec![];

        let (dx, dy) = if pos_dx < pos_dy {
            (pos_dy, pos_dx)
        } else {
            (pos_dx, pos_dy)
        };
        let mut d = dy - dx;
        let mut y = 0;
        for x in 0..=dx {
            plot.push(PosDiff::new(x, y));
            if d >= 0 {
                y += 1;
                if d > 0 {
                    plot.push(PosDiff::new(x, y));
                }
                d -= 2 * dx;
            }
            d += 2 * dy;
        }

        // Align
        if pos_dx < pos_dy {
            plot = plot.into_iter().map(|p| PosDiff::new(p.dy, p.dx)).collect();
        }

        if posdiff.dx < 0 {
            plot = plot
                .into_iter()
                .map(|p| PosDiff::new(-p.dx, p.dy))
                .collect();
        }
        if posdiff.dy < 0 {
            plot = plot
                .into_iter()
                .map(|p| PosDiff::new(p.dx, -p.dy))
                .collect();
        }

        plot
    }

    fn new(posdiff: PosDiff) -> Manipulator {
        Manipulator {
            posdiff,
            reachable_cell: Manipulator::plot(posdiff),
        }
    }

    pub fn can_mark(&self, pos_angle: PosAngle, map: &Map) -> bool {
        map.is_empty(pos_angle + self.posdiff)
            && self.reachable_cell.iter().all(|reach| {
                let pos = pos_angle + *reach;
                !map.is_wall(pos)
            })
    }
}

type Manipulators = Vec<Manipulator>;

#[derive(Debug, Copy, Clone, Hash)]
enum Order {
    MoveToExtendManipulator(Pos),
    DoExtendManipulator,
    MoveToClone(Pos),
    FindMysterious,
    MoveToMysterious(Pos),
    DoClone,
    MoveToFastWheel(Pos),
    DoFastWheels,
    MoveToDrill(Pos),
    DoDrill,
}

#[derive(Debug)]
pub struct Bot {
    pos_angle: PosAngle,
    manipulators: Manipulators,
    fast_wheel_timer: usize,
    drill_timer: usize,
    order: Option<Order>,
    record: Vec<Action>,
}

impl Bot {
    pub fn new(pos: Pos) -> Bot {
        Bot {
            pos_angle: PosAngle::new(pos, Angle::A0),
            manipulators: vec![
                Manipulator::new(PosDiff::new(0, 0)),
                Manipulator::new(PosDiff::new(1, 0)),
                Manipulator::new(PosDiff::new(1, 1)),
                Manipulator::new(PosDiff::new(1, -1)),
            ],
            fast_wheel_timer: 0,
            drill_timer: 0,
            order: None,
            record: vec![],
        }
    }

    fn mark_map(&self, map: &mut Map) {
        let mut marked = vec![];

        for manipulator in &self.manipulators {
            if manipulator.can_mark(self.pos_angle, map) {
                // println!("maniputor: {:?} is used", manipulator.posdiff);
                marked.push(self.pos_angle + manipulator.posdiff);
            }
        }
        for mark in marked {
            map.mark_pos(mark);
        }
    }

    fn number_of_possible_mark_with_this_pos(
        pos_angle: PosAngle,
        manipulators: &[Manipulator],
        map: &Map,
    ) -> usize {
        manipulators
            .iter()
            .filter(|manipulator| manipulator.can_mark(pos_angle, map))
            .count()
    }

    fn number_of_ajd_empty_cells_of_manipulators_with_this_pos(
        pos_angle: PosAngle,
        manipulators: &[Manipulator],
        map: &Map,
    ) -> usize {
        let mut manipulator_cells = HashSet::new();
        let mut empty_cells = HashSet::new();
        for manipulator in manipulators {
            let pos = pos_angle + manipulator.posdiff;
            manipulator_cells.insert(pos);
            for diff in PosDiff::gen_all_diff() {
                let adj = pos + *diff;
                if map.is_empty(adj) {
                    empty_cells.insert(adj);
                }
            }
        }
        empty_cells.difference(&manipulator_cells).count()
    }

    #[allow(dead_code)]
    fn number_of_manipulators_which_has_empty_adj_cell_with_this_pos(
        pos_angle: PosAngle,
        manipulators: &[Manipulator],
        map: &Map,
    ) -> usize {
        let mut manipulator_cells = HashSet::new();
        for manipulator in manipulators {
            let pos = pos_angle + manipulator.posdiff;
            manipulator_cells.insert(pos);
        }
        manipulators
            .iter()
            .filter(|manipulator| {
                let manipulator_pos = pos_angle + manipulator.posdiff;
                PosDiff::gen_all_diff().iter().any(|diff| {
                    let adj = manipulator_pos + *diff;
                    map.is_empty(adj) && !manipulator_cells.contains(&adj)
                })
            })
            .count()
    }

    fn find_extend_manipulator_position(&self) -> PosDiff {
        // MVP
        if self.manipulators.len() % 2 == 0 {
            // Attach north
            PosDiff::new(1, (self.manipulators.len() / 2) as i32)
        } else {
            // Attach south
            PosDiff::new(1, -((self.manipulators.len() / 2) as i32))
        }
    }

    fn apply_action(&mut self, action: Action, map: &mut Map, fast_wheel_second_move: bool) {
        use Action::*;
        match action {
            MoveUp | MoveDown | MoveLeft | MoveRight => {
                self.pos_angle = {
                    let next = self.pos_angle.apply_action(action);
                    if fast_wheel_second_move {
                        if self.drill_timer > 0 {
                            if map.is_in_range(next.pos) && map.is_wall(next.pos) {
                                map.do_drill(next.pos);
                                next
                            } else {
                                // Drill, but don't move out of boundaries.
                                self.pos_angle
                            }
                        } else if !map.is_free(next.pos) {
                            // collide in fast wheel 2nd move is okay. Don't move.
                            self.pos_angle
                        } else {
                            next
                        }
                    } else {
                        if self.drill_timer > 0 && map.is_wall(next.pos) {
                            map.do_drill(next.pos);
                        }
                        assert!(map.is_free(next.pos));
                        next
                    }
                };
            }
            DoNothing => {
                // Do nothing
            }
            TurnClockWise | TurnCounterClockWise => {
                self.pos_angle = self.pos_angle.apply_action(action);
            }
            ExtendManipulator(posdiff) => {
                // TODO: Assert posdiff is valid or not
                self.manipulators.push(Manipulator::new(posdiff));
                debug!("extend manipulator: bot: {:?}", self);
            }
            Cloning => {
                // No effect on this bot.
            }
            AttachFastWheels => {
                self.fast_wheel_timer += 50;
            }
            AttachDrill => {
                self.drill_timer += 30;
            }
        }

        // Update order
        if let Some(order) = self.order {
            match order {
                Order::MoveToExtendManipulator(pos) => {
                    if self.pos_angle.pos == pos {
                        self.order = Some(Order::DoExtendManipulator);
                    }
                }
                Order::DoExtendManipulator => {
                    self.order = None;
                }
                Order::MoveToClone(pos) => {
                    if self.pos_angle.pos == pos {
                        self.order = Some(Order::FindMysterious);
                    }
                }
                Order::FindMysterious => unreachable!(),
                Order::MoveToMysterious(pos) => {
                    if self.pos_angle.pos == pos {
                        self.order = Some(Order::DoClone);
                    }
                }
                Order::DoClone => {
                    self.order = None;
                }
                Order::MoveToFastWheel(pos) => {
                    if self.pos_angle.pos == pos {
                        self.order = Some(Order::DoFastWheels)
                    }
                }
                Order::DoFastWheels => {
                    self.order = None;
                }
                Order::MoveToDrill(pos) => {
                    if self.pos_angle.pos == pos {
                        self.order = Some(Order::DoDrill)
                    }
                }
                Order::DoDrill => {
                    self.order = None;
                }
            }
        }

        // Record action
        if !fast_wheel_second_move {
            if let Action::ExtendManipulator(posdiff) = action {
                self.record.push(Action::ExtendManipulator(
                    posdiff.turn(self.pos_angle.angle),
                ));
            } else {
                self.record.push(action);
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct PosAngle {
    pos: Pos,
    angle: Angle,
}

impl std::ops::Add<PosDiff> for PosAngle {
    type Output = Pos;
    fn add(self, diff: PosDiff) -> Pos {
        self.pos + diff.turn(self.angle)
    }
}

impl PosAngle {
    fn new(pos: Pos, angle: Angle) -> PosAngle {
        PosAngle { pos, angle }
    }

    pub fn apply_action(&self, action: Action) -> PosAngle {
        use Action::*;
        match action {
            MoveUp => PosAngle::new(self.pos + PosDiff::new(0, 1), self.angle),
            MoveDown => PosAngle::new(self.pos + PosDiff::new(0, -1), self.angle),
            MoveLeft => PosAngle::new(self.pos + PosDiff::new(-1, 0), self.angle),
            MoveRight => PosAngle::new(self.pos + PosDiff::new(1, 0), self.angle),
            TurnClockWise => PosAngle::new(self.pos, self.angle.turn_clock_wise()),
            TurnCounterClockWise => PosAngle::new(self.pos, self.angle.turn_counter_clock_wise()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MoveStep {
    pub pos_angle: PosAngle,
    pub len: usize,
    pub mark_number: usize,
    pub adj_empty_number: usize,
    pub prev_action: Option<Action>,
    pub prev: Option<Rc<MoveStep>>,
}

impl MoveStep {
    // fn actions(mut step: Rc<MoveStep>) -> Vec<Action> {
    //     let mut actions: Vec<Action> = vec![];
    //     while let Some(prev_step) = step.clone().prev.as_ref() {
    //         actions.push(step.prev_action.unwrap());
    //         step = prev_step.clone();
    //     }
    //     actions.reverse();
    //     actions
    // }

    fn first_action(&self) -> Action {
        let mut action = None;
        let mut step: &MoveStep = self;
        while let Some(prev_step) = step.prev.as_ref() {
            action = step.prev_action;
            step = &prev_step.as_ref();
        }
        action.unwrap()
    }
}

pub struct System {
    map: Map,
    bots: Vec<Bot>,
    booster_pos: HashMap<Pos, BoosterKind>,
    mysterious_pos: HashSet<Pos>,
}

impl System {
    pub fn new(id: u64) -> Result<System> {
        let task = Task::read_with_id(id)?;
        let map = Map::new(task);
        let bot = Bot::new(map.bot_start_pos);

        let booster_pos = map
            .boosters
            .iter()
            .filter(|booster| booster.kind != BoosterKind::Mysterious)
            .map(|booster| (booster.pos, booster.kind))
            .collect();
        let mysterious_pos = map
            .boosters
            .iter()
            .filter(|booster| booster.kind == BoosterKind::Mysterious)
            .map(|booster| booster.pos)
            .collect();
        Ok(System {
            map,
            bots: vec![bot],
            booster_pos,
            mysterious_pos,
        })
    }

    fn has_booster(&self, booster: BoosterKind) -> bool {
        self.booster_pos.iter().any(|(_, kind)| *kind == booster)
    }

    fn order_swap(&mut self, bot_a: usize, bot_b: usize) {
        match (self.bots[bot_a].order, self.bots[bot_b].order) {
            (Some(Order::MoveToMysterious(pos)), None) => {
                if self.distance(self.bots[bot_a].pos_angle.pos, pos).unwrap()
                    > self.distance(self.bots[bot_b].pos_angle.pos, pos).unwrap()
                {
                    self.bots[bot_a].order = None;
                    self.bots[bot_b].order = Some(Order::MoveToMysterious(pos));
                }
            }
            (Some(Order::MoveToClone(pos)), None) => {
                if self.distance(self.bots[bot_a].pos_angle.pos, pos).unwrap()
                    > self.distance(self.bots[bot_b].pos_angle.pos, pos).unwrap()
                {
                    self.bots[bot_a].order = None;
                    self.bots[bot_b].order = Some(Order::MoveToClone(pos));
                }
            }
            _ => {}
        }
    }

    pub fn solve(&mut self) -> Result<()> {
        self.bots[0].mark_map(&mut self.map);

        while self.map.empty_cell_count != 0 {
            for i in 0..self.bots.len() {
                for j in 0..self.bots.len() {
                    self.order_swap(i, j);
                }
            }

            debug!("empty cell: {}", self.map.empty_cell_count);
            for i in 0..self.bots.len() {
                if self.map.empty_cell_count == 0 {
                    break;
                }
                debug!(
                    "  turn: {}, bot {}, pos: {:?}",
                    self.bots[i].record.len(),
                    i,
                    self.bots[i].pos_angle.pos
                );

                // Find and apply action
                let action = {
                    if let Some(order) = self.bots[i].order {
                        match order {
                            Order::MoveToExtendManipulator(pos) => {
                                self.move_to_action(&self.bots[i], pos)
                            }
                            Order::DoExtendManipulator => Ok(Action::ExtendManipulator(
                                self.bots[i].find_extend_manipulator_position(),
                            )),
                            Order::MoveToClone(pos) => self.move_to_action(&self.bots[i], pos),
                            Order::FindMysterious => {
                                let step = self.find_mysterious(&self.bots[i])?;
                                self.bots[i].order =
                                    Some(Order::MoveToMysterious(step.pos_angle.pos));
                                Ok(step.first_action())
                            }
                            Order::MoveToMysterious(pos) => self.move_to_action(&self.bots[i], pos),
                            Order::DoClone => Ok(Action::Cloning),
                            Order::MoveToFastWheel(pos) => self.move_to_action(&self.bots[i], pos),
                            Order::DoFastWheels => Ok(Action::AttachFastWheels),
                            Order::MoveToDrill(pos) => self.move_to_action(&self.bots[i], pos),
                            Order::DoDrill => Ok(Action::AttachDrill),
                        }
                    } else if let Ok(step) = self.find_booster_near(
                        &self.bots[i],
                        // TODO: Support Drill
                        // &[BoosterKind::FastWheels, BoosterKind::Drill],
                        &[BoosterKind::FastWheels],
                        5,
                    ) {
                        let booster = self.booster_pos.remove(&step.pos_angle.pos).unwrap();
                        match booster {
                            BoosterKind::FastWheels => {
                                self.bots[i].order =
                                    Some(Order::MoveToFastWheel(step.pos_angle.pos));
                            }
                            BoosterKind::Drill => {
                                self.bots[i].order = Some(Order::MoveToDrill(step.pos_angle.pos));
                            }
                            _ => unreachable!(),
                        }
                        Ok(step.first_action())
                    } else if let Ok(step) = self.find_booster(
                        &self.bots[i],
                        &[BoosterKind::ExtendManipulator, BoosterKind::Cloning],
                    ) {
                        debug!("> Found booster");
                        // Remove it here.
                        // It's okay for other bots picked up earlier than this bot by accident. That should not have any bad affect
                        // bacause boosters are shared.
                        let booster = self.booster_pos.remove(&step.pos_angle.pos).unwrap();
                        match booster {
                            BoosterKind::ExtendManipulator => {
                                self.bots[i].order =
                                    Some(Order::MoveToExtendManipulator(step.pos_angle.pos));
                                Ok(step.first_action())
                            }
                            BoosterKind::Cloning => {
                                self.bots[i].order = Some(Order::MoveToClone(step.pos_angle.pos));
                                Ok(step.first_action())
                                // self.bots[i].picking_booster = Some(booster)
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        match self.find_mark_move(&self.bots[i]) {
                            Ok(step) => Ok(step.first_action()),
                            Err(_) => {
                                assert!(self.bots[i].fast_wheel_timer > 0);
                                Ok(Action::DoNothing)
                            }
                        }
                    }
                }?;

                debug!(
                    ">> turn: {}, fast_wheel_timer: {}, acton: {:?}",
                    self.bots[i].record.len(),
                    self.bots[i].fast_wheel_timer,
                    action
                );

                // Save this here because apply_action can change timer value
                let can_use_fast_wheel_in_this_turn = self.bots[i].fast_wheel_timer > 0;
                let can_use_drill_in_this_turn = self.bots[i].drill_timer > 0;
;
                if can_use_fast_wheel_in_this_turn && action.is_move() {
                    self.bots[i].apply_action(action, &mut self.map, false);
                    self.bots[i].mark_map(&mut self.map);
                    self.bots[i].apply_action(action, &mut self.map, true);
                    self.bots[i].mark_map(&mut self.map);
                } else {
                    self.bots[i].apply_action(action, &mut self.map, false);
                    self.bots[i].mark_map(&mut self.map);
                }

                // Decrement only when they had positive values before apply action
                if can_use_fast_wheel_in_this_turn {
                    self.bots[i].fast_wheel_timer -= 1;
                }
                if can_use_drill_in_this_turn {
                    self.bots[i].drill_timer -= 1;
                }

                debug!("bot's pos: {:?}", self.bots[i].pos_angle.pos);
                debug!("empty cell count: {:?}", self.map.empty_cell_count);

                // System wide effects
                if let Action::Cloning = action {
                    self.bots.push(Bot::new(self.bots[i].pos_angle.pos));
                }
            }
        }
        Ok(())
    }

    fn move_to_action(&self, bot: &Bot, goal: Pos) -> Result<Action> {
        Ok(self.find_move_to(bot, |pos| pos == goal)?.first_action())
    }

    fn find_booster(&self, bot: &Bot, boosters: &[BoosterKind]) -> Result<Rc<MoveStep>> {
        if boosters.iter().all(|booster| !self.has_booster(*booster)) {
            Err(failure::err_msg("booster is no longer available"))
        } else {
            self.find_move_to(bot, |pos| {
                if let Some(found_booster) = self.booster_pos.get(&pos) {
                    boosters.iter().any(|booster| booster == found_booster)
                } else {
                    false
                }
            })
        }
    }

    fn find_booster_near(
        &self,
        bot: &Bot,
        boosters: &[BoosterKind],
        near_len: usize,
    ) -> Result<Rc<MoveStep>> {
        let step = self.find_booster(bot, boosters)?;
        if step.len <= near_len {
            Ok(step)
        } else {
            Err(failure::err_msg("near booster is not found "))
        }
    }

    fn find_mysterious(&self, bot: &Bot) -> Result<Rc<MoveStep>> {
        self.find_move_to(bot, |pos| self.mysterious_pos.contains(&pos))
    }

    fn distance(&self, a: Pos, b: Pos) -> Result<usize> {
        let mut visited = HashSet::new();
        visited.insert(a);

        let mut q = VecDeque::new();
        q.push_back((a, 0));
        while let Some((pos, len)) = q.pop_front() {
            if pos == b {
                return Ok(len);
            }
            for d in PosDiff::gen_all_diff() {
                let next = pos + *d;
                if !visited.contains(&next) {
                    visited.insert(next);
                    q.push_back((next, len + 1));
                }
            }
        }
        Err(failure::err_msg("can not reach b"))
    }

    fn find_move_to<P>(&self, bot: &Bot, predicate: P) -> Result<Rc<MoveStep>>
    where
        P: Fn(Pos) -> bool,
    {
        let mut q = VecDeque::new();
        q.push_back(Rc::new(MoveStep {
            pos_angle: bot.pos_angle,
            len: 0,
            mark_number: 0,
            adj_empty_number: 0,
            prev_action: None,
            prev: None,
        }));

        let mut visited = HashSet::new();
        visited.insert(bot.pos_angle);

        use Action::*;
        while let Some(current_step) = q.pop_front() {
            let can_use_drill = bot.drill_timer > current_step.len;
            let can_use_fast_wheel = bot.fast_wheel_timer > current_step.len;

            for action in &[MoveUp, MoveDown, MoveLeft, MoveRight] {
                // TODO: Refactor with xxx.
                let next_pos_angle = {
                    let next_pos_angle = current_step.pos_angle.apply_action(*action);
                    if !self.map.is_in_range(next_pos_angle.pos) {
                        continue;
                    }
                    if !can_use_drill && !self.map.is_free(next_pos_angle.pos) {
                        continue;
                    }
                    if can_use_fast_wheel && action.is_move() {
                        let next_next_pos_angle = next_pos_angle.apply_action(*action);
                        if !self.map.is_in_range(next_next_pos_angle.pos) {
                            next_pos_angle
                        } else if self.map.is_free(next_next_pos_angle.pos) || can_use_drill {
                            next_next_pos_angle
                        } else {
                            next_pos_angle
                        }
                    } else {
                        next_pos_angle
                    }
                };

                let next = Rc::new(MoveStep {
                    pos_angle: next_pos_angle,
                    len: current_step.len + 1,
                    mark_number: 0,
                    adj_empty_number: 0,
                    prev_action: Some(*action),
                    prev: Some(current_step.clone()),
                });
                if predicate(next_pos_angle.pos) {
                    return Ok(next);
                }
                if !visited.contains(&next_pos_angle) {
                    visited.insert(next_pos_angle);
                    q.push_back(next);
                }
            }
        }
        Err(failure::err_msg(
            "find_move_to: Can not reach found booster",
        ))
    }

    fn find_mark_move(&self, bot: &Bot) -> Result<Rc<MoveStep>> {
        use Action::*;

        // debug!("turn: {}, bot.pos: {:?}", self.record.len(), self.bot.pos);
        let mut q = VecDeque::new();
        q.push_back(Rc::new(MoveStep {
            pos_angle: bot.pos_angle,
            len: 0,
            mark_number: 0,
            adj_empty_number: 0,
            prev_action: None,
            prev: None,
        }));

        let mut visited = HashSet::new();
        visited.insert(bot.pos_angle);

        let mut best: Option<Rc<MoveStep>> = None;

        while let Some(current_step) = q.pop_front() {
            let can_use_drill = bot.drill_timer > current_step.len;
            let can_use_fast_wheel = bot.fast_wheel_timer > current_step.len;
            let actions = match current_step.pos_angle.angle {
                Angle::A0 => [
                    MoveUp,
                    MoveDown,
                    MoveRight,
                    MoveLeft,
                    TurnClockWise,
                    TurnCounterClockWise,
                ],
                Angle::A90 => [
                    MoveRight,
                    MoveLeft,
                    MoveDown,
                    MoveUp,
                    TurnClockWise,
                    TurnCounterClockWise,
                ],
                Angle::A180 => [
                    MoveDown,
                    MoveUp,
                    MoveLeft,
                    MoveRight,
                    TurnClockWise,
                    TurnCounterClockWise,
                ],
                Angle::A270 => [
                    MoveLeft,
                    MoveRight,
                    MoveUp,
                    MoveDown,
                    TurnClockWise,
                    TurnCounterClockWise,
                ],
            };
            for action in &actions {
                let next_pos_angle = {
                    let next_pos_angle = current_step.pos_angle.apply_action(*action);
                    if !self.map.is_in_range(next_pos_angle.pos) {
                        continue;
                    }
                    if !can_use_drill && !self.map.is_free(next_pos_angle.pos) {
                        continue;
                    }
                    if can_use_fast_wheel && action.is_move() {
                        let next_next_pos_angle = next_pos_angle.apply_action(*action);
                        if !self.map.is_in_range(next_next_pos_angle.pos) {
                            next_pos_angle
                        } else if self.map.is_free(next_next_pos_angle.pos) || can_use_drill {
                            next_next_pos_angle
                        } else {
                            next_pos_angle
                        }
                    } else {
                        next_pos_angle
                    }
                };
                let next = Rc::new(MoveStep {
                    pos_angle: next_pos_angle,
                    len: current_step.len + 1,
                    mark_number: Bot::number_of_possible_mark_with_this_pos(
                        next_pos_angle,
                        &bot.manipulators,
                        &self.map,
                    ),
                    adj_empty_number: Bot::number_of_ajd_empty_cells_of_manipulators_with_this_pos(
                        next_pos_angle,
                        &bot.manipulators,
                        &self.map,
                    ),
                    prev_action: Some(*action),
                    prev: Some(current_step.clone()),
                });
                if next.mark_number > 0 {
                    match &best {
                        None => {
                            best = Some(next.clone());
                        }
                        Some(prev_best) => {
                            if prev_best.len == next.len
                                && (prev_best.mark_number < next.mark_number
                                    || (prev_best.mark_number == next.mark_number
                                        && prev_best.adj_empty_number < next.adj_empty_number))
                            {
                                best = Some(next.clone());
                            }
                        }
                    }
                }
                if best.is_none() && !visited.contains(&next_pos_angle) {
                    visited.insert(next_pos_angle);
                    q.push_back(next);
                }
            }
        }
        best.ok_or_else(|| failure::err_msg("Can not reach empty cell targets"))
    }

    pub fn dump_record(&self) -> String {
        self.bots
            .iter()
            .map(|bot| {
                bot.record
                    .iter()
                    .map(|record| record.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("#")
    }

    pub fn solution(&self) -> Solution {
        let id = self.map.id;
        let score = self.bots.iter().map(|bot| bot.record.len()).max().unwrap();
        Solution {
            id,
            score,
            solution: self.dump_record(),
            filename: format!("prob-{:03}-score-{:08}-ai-drill.sol", id, score),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Solution {
    pub id: u64,
    pub score: usize,
    pub solution: String,
    pub filename: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn system_test() -> Result<()> {
        let mut system = System::new(1)?;
        system.solve()?;

        let mut system = System::new(2)?;
        system.solve()?;

        let mut system = System::new(21)?;
        system.solve()?;
        assert_eq!(system.solution().score, 1313);
        assert_eq!(system.solution().solution, "DDDDDDDDDWWWWDDWWWWDDDDDDWWWWDDDDDDDDDWDDB(1,2)WWWWWAB(1,-2)AWWQWWWWWWWWWAWWWWWWSSAQAAAAAWAEWSSAAAQDDDDDSQDWDDDDSSSSAASSSSSSSSSSSSAAEAASQDDWDDWWWAAEWAAAAAAAAAAAAAAAWDDWEWSSSEWDDDWWDWAWWQWWWASQAASQDDDDDDDDDDDAAAESAAASSSSSDDDDWEWASSSSQSSSQSWWQWWWAAAAASSSSESSWWWWWWDDDDDDDDDDWWEWWWWDDDDDDEDEDSSSSSDDDDDDDDDDDDDDDDDDAAAWAWQWWWWWWAAQAAAASSQSSDWWAWASAAADDDSSSSSSSSSSSSSSSSSSSSSSDDFSSSSSAEWWAAWDWDDDDDDDWASSAAWWWDDAAAAAAWADWDDDDWEAAWWWWWDDDDDEDDDAAAAAAAAWWAAAAAAAQAAAASQSSSQDSSSSSSSSSSSSAFSSSAEAASWWADFDWDWWAAAWEDDWAAAAWDDWSAAASZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZWSQWDDSDSSAAASAAAAAAAAAAADDWDWWEWWWAQAWWWWEWWAWWWWWWWWWWWWWWWWAWWWWWQQSDDDDDDDWDDDQDDDDDDDDDDDDDDSDDDDDDDDDDDDDDDDDDWDDSSESWWWAAEAWWWSSAASSSQSSAWWWAWEAWWWSSAAAASSAAASQSDDSQDWDAAAASEASSWWAEWWWAAAAAAWWEWSSSSQSSWWWWAAAASAAASASQSAAAAAASEAWDDDDDDDWWAAWAWDDSAAAAAAAASAAAASSSASSSSSSSSSSSSDSSDSSASSDQDSDDQDAAASSSDDSSDDDDAAAASAAADDDSSSSDDDDDDSSSSESSSSSWWWEWDDDDAAAWAAAAADSSAASQSAAWWAASSWWDWWWSSSAAAWWWSSSDDDDDDDDWWDDDDWWWWDDDDWWWWDDDDDDDDDDDDSSSSDDAASSSSSSSSSAAAAAAAAAEWAAASQDWDDDDDDDDDDDWWWWWWWWWWWWWWWWDQDWWWSSSAAWWWWWWWWWWWWDWWWAAAWWWSSSSSSSSSSDDDDDDDDDWWWWWWWWWDDWWSSSDDDDDDDSASSSSSSSSSAAAAAASSSSSSSSSSSSSSSSSSSSSDDDDDDDDDSASSSWWWAAAASSSWWWAAAAAAESAASAAADDDWWWWWWWWWWWWWWWWWWWWWWWWAAAAAAAASSSSSSSSAAAAAAAAASSSSAAAAAAAAAAAWWWWWAAWWWWAWWWWWWWWWWWWWWWWW");

        // let mut system = System::new(221)?;
        // system.solve()?;
        // assert_eq!(system.solution().score, 1094);
        Ok(())
    }

    #[test]
    fn plot_test() {
        assert_eq!(
            Manipulator::plot(PosDiff::new(0, 0)),
            vec![PosDiff::new(0, 0)]
        );
        assert_eq!(
            Manipulator::plot(PosDiff::new(1, 0)),
            vec![PosDiff::new(0, 0), PosDiff::new(1, 0)]
        );
        assert_eq!(
            Manipulator::plot(PosDiff::new(1, 0)),
            vec![PosDiff::new(0, 0), PosDiff::new(1, 0)]
        );
        assert_eq!(
            Manipulator::plot(PosDiff::new(-1, 0)),
            vec![PosDiff::new(0, 0), PosDiff::new(-1, 0)]
        );
        assert_eq!(
            Manipulator::plot(PosDiff::new(0, 1)),
            vec![PosDiff::new(0, 0), PosDiff::new(0, 1)]
        );
        assert_eq!(
            Manipulator::plot(PosDiff::new(0, -1)),
            vec![PosDiff::new(0, 0), PosDiff::new(0, -1)]
        );

        assert_eq!(
            Manipulator::plot(PosDiff::new(1, 1)),
            vec![PosDiff::new(0, 0), PosDiff::new(1, 1)]
        );

        assert_eq!(
            Manipulator::plot(PosDiff::new(3, 0)),
            vec![
                PosDiff::new(0, 0),
                PosDiff::new(1, 0),
                PosDiff::new(2, 0),
                PosDiff::new(3, 0)
            ]
        );

        assert_eq!(
            Manipulator::plot(PosDiff::new(2, 1)),
            vec![
                PosDiff::new(0, 0),
                PosDiff::new(1, 0),
                PosDiff::new(1, 1),
                PosDiff::new(2, 1)
            ]
        );

        assert_eq!(
            Manipulator::plot(PosDiff::new(3, 1)),
            vec![
                PosDiff::new(0, 0),
                PosDiff::new(1, 0),
                PosDiff::new(2, 1),
                PosDiff::new(3, 1)
            ]
        );
        assert_eq!(
            Manipulator::plot(PosDiff::new(4, 1)),
            vec![
                PosDiff::new(0, 0),
                PosDiff::new(1, 0),
                PosDiff::new(2, 0),
                PosDiff::new(2, 1),
                PosDiff::new(3, 1),
                PosDiff::new(4, 1)
            ]
        );

        assert_eq!(
            Manipulator::plot(PosDiff::new(2, -1)),
            vec![
                PosDiff::new(0, 0),
                PosDiff::new(1, 0),
                PosDiff::new(1, -1),
                PosDiff::new(2, -1)
            ]
        );

        assert_eq!(
            Manipulator::plot(PosDiff::new(-2, -1)),
            vec![
                PosDiff::new(0, 0),
                PosDiff::new(-1, 0),
                PosDiff::new(-1, -1),
                PosDiff::new(-2, -1)
            ]
        );

        assert_eq!(
            Manipulator::plot(PosDiff::new(-2, 1)),
            vec![
                PosDiff::new(0, 0),
                PosDiff::new(-1, 0),
                PosDiff::new(-1, 1),
                PosDiff::new(-2, 1)
            ]
        );

        assert_eq!(
            Manipulator::plot(PosDiff::new(2, 2)),
            vec![PosDiff::new(0, 0), PosDiff::new(1, 1), PosDiff::new(2, 2),]
        );
        assert_eq!(
            Manipulator::plot(PosDiff::new(3, 2)),
            vec![
                PosDiff::new(0, 0),
                PosDiff::new(1, 0),
                PosDiff::new(1, 1),
                PosDiff::new(2, 1),
                PosDiff::new(2, 2),
                PosDiff::new(3, 2)
            ]
        );
        assert_eq!(
            Manipulator::plot(PosDiff::new(6, 2)),
            vec![
                PosDiff::new(0, 0),
                PosDiff::new(1, 0),
                PosDiff::new(2, 1),
                PosDiff::new(3, 1),
                PosDiff::new(4, 1),
                PosDiff::new(5, 2),
                PosDiff::new(6, 2)
            ]
        );
        assert_eq!(
            Manipulator::plot(PosDiff::new(4, 3)),
            vec![
                PosDiff::new(0, 0),
                PosDiff::new(1, 0),
                PosDiff::new(1, 1),
                PosDiff::new(2, 1),
                PosDiff::new(2, 2),
                PosDiff::new(3, 2),
                PosDiff::new(3, 3),
                PosDiff::new(4, 3),
            ]
        );

        assert_eq!(
            Manipulator::plot(PosDiff::new(-3, 4)),
            vec![
                PosDiff::new(0, 0),
                PosDiff::new(0, 1),
                PosDiff::new(-1, 1),
                PosDiff::new(-1, 2),
                PosDiff::new(-2, 2),
                PosDiff::new(-2, 3),
                PosDiff::new(-3, 3),
                PosDiff::new(-3, 4),
            ]
        );
    }
}
