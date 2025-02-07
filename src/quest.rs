use rand::Rng;
use std::fmt::{Display, Formatter, Result};

pub enum ActionTarget {
    Somebody(Somebody),
    Something(Something),
}
impl Default for ActionTarget {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random: bool = rng.gen();
        if random {
            ActionTarget::Somebody(Somebody::default())
        } else {
            ActionTarget::Something(Something::default())
        }
    }
}
impl Display for ActionTarget {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ActionTarget::Somebody(s) => write!(f, "{s}"),
            ActionTarget::Something(s) => write!(f, "the {s}"),
        }
    }
}

pub struct Somebody {
    name: String,
}
impl Default for Somebody {
    fn default() -> Self {
        Self {
            name: String::from("<Name>"),
        }
    }
}
impl Display for Somebody {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.name)
    }
}

pub struct Something {
    name: String,
}
impl Default for Something {
    fn default() -> Self {
        Self {
            name: String::from("<Thing>"),
        }
    }
}
impl Display for Something {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.name)
    }
}

pub struct Somewhere {
    name: String,
}
impl Default for Somewhere {
    fn default() -> Self {
        Self {
            name: String::from("<Location>"),
        }
    }
}
impl Display for Somewhere {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.name)
    }
}

// Atomic Action
#[derive(Default)]
pub struct Capture {
    somebody: Somebody,
}
impl Display for Capture {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "capture {}", self.somebody)
    }
}

#[derive(Default)]
pub struct Damage {
    some: ActionTarget,
}
impl Display for Damage {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "damage {}", self.some)
    }
}

#[derive(Default)]
pub struct Defend {
    some: ActionTarget,
}
impl Display for Defend {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "defend {}", self.some)
    }
}

#[derive(Default)]
pub struct Escort {
    somebody: Somebody,
}
impl Display for Escort {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "escort {}", self.somebody)
    }
}

#[derive(Default)]
pub struct Exchange {
    somebody: Somebody,
    something: Something,
}
impl Display for Exchange {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "exchange for {} with {}", self.something, self.somebody)
    }
}

#[derive(Default)]
pub struct Experiment {
    something: Something,
}
impl Display for Experiment {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "experiment with {}", self.something)
    }
}

#[derive(Default)]
pub struct Explore {}
impl Display for Explore {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "explore a bit around")
    }
}

#[derive(Default)]
pub struct Gather {
    something: Something,
}
impl Display for Gather {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "gather for {}", self.something)
    }
}

#[derive(Default)]
pub struct Give {
    somebody: Somebody,
    something: Something,
}
impl Display for Give {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "give {} to {}", self.something, self.somebody)
    }
}

#[derive(Default)]
pub struct Goto {
    some: Somewhere,
}
impl Display for Goto {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "go to {} ", self.some)
    }
}

#[derive(Default)]
pub struct Kill {
    somebody: Somebody,
}
impl Display for Kill {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "kill {}", self.somebody)
    }
}

#[derive(Default)]
pub struct Listen {
    somebody: Somebody,
}
impl Display for Listen {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "listen {}", self.somebody)
    }
}

#[derive(Default)]
pub struct r#Read {
    something: Something,
}
impl Display for r#Read {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "read {}", self.something)
    }
}

#[derive(Default)]
pub struct Repair {
    something: Something,
}
impl Display for Repair {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "repair {}", self.something)
    }
}

#[derive(Default)]
pub struct Report {
    somebody: Somebody,
}
impl Display for Report {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "report to {}", self.somebody)
    }
}

#[derive(Default)]
pub struct Spy {
    some: ActionTarget,
}
impl Display for Spy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "spy about {}", self.some)
    }
}

#[derive(Default)]
pub struct Stealth {
    somebody: Somebody,
}
impl Display for Stealth {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "sneak behind {}", self.somebody)
    }
}

#[derive(Default)]
pub struct Take {
    somebody: Somebody,
    something: Something,
}
impl Display for Take {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "take the {} from {}", self.something, self.somebody)
    }
}

#[derive(Default)]
pub struct Use {
    something: Something,
}
impl Display for Use {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "use the {}", self.something)
    }
}

// Rule Action
pub enum RuleSubQuest {
    Goto(Box<RuleGoto>),
    GotoReturn(Box<RuleGoto>, Box<Quest>, Goto),
}
impl Default for RuleSubQuest {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..2);
        match random {
            0 => RuleSubQuest::Goto(Box::default()),
            _ => RuleSubQuest::GotoReturn(Box::default(), Box::default(), Goto::default()),
        }
    }
}
impl Display for RuleSubQuest {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RuleSubQuest::Goto(a) => write!(f, "{a}"),
            RuleSubQuest::GotoReturn(a, b, c) => write!(f, "first {a}, then do {b}, after {c}"),
        }
    }
}

pub enum RuleGoto {
    None,
    Explore(Explore),
    Learn(Box<RuleLearn>, Goto),
}
impl Default for RuleGoto {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..3);
        match random {
            0 => RuleGoto::None,
            1 => RuleGoto::Explore(Explore::default()),
            _ => RuleGoto::Learn(Box::new(RuleLearn::default()), Goto::default()),
        }
    }
}
impl Display for RuleGoto {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RuleGoto::None => write!(f, "nothing (You are already at the right place)"),
            RuleGoto::Explore(a) => write!(f, "go {a}"),
            RuleGoto::Learn(a, b) => write!(f, "first learn {a}, then {b}"),
        }
    }
}

pub enum RuleLearn {
    None,
    SubQuest(Box<RuleGoto>, Box<RuleSubQuest>, Listen),
    r#Read(Box<RuleGoto>, Box<RuleGet>, r#Read),
    Fetch(Box<RuleGet>, Box<RuleSubQuest>, Give, Listen),
}
impl Default for RuleLearn {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..4);
        match random {
            0 => RuleLearn::None,
            1 => RuleLearn::SubQuest(Box::default(), Box::default(), Listen::default()),
            2 => RuleLearn::r#Read(Box::default(), Box::default(), r#Read::default()),
            _ => RuleLearn::Fetch(
                Box::default(),
                Box::default(),
                Give::default(),
                Listen::default(),
            ),
        }
    }
}
impl Display for RuleLearn {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RuleLearn::None => write!(f, "nothing (You already know it)"),
            RuleLearn::SubQuest(a, b, c) => write!(f, "first {a}, then do {b}, then {c}"),
            RuleLearn::r#Read(a, b, c) => write!(f, "first {a}, then {b} and {c}"),
            RuleLearn::Fetch(a, b, c, d) => write!(f, "first {a}, then do {b}, {c} and {d}"),
        }
    }
}

pub enum RuleGet {
    None,
    Steal(Box<RuleSteal>),
    Gather(Box<RuleGoto>, Gather),
    Exchange(
        Box<RuleGoto>,
        Box<RuleGet>,
        Box<RuleGoto>,
        Box<RuleSubQuest>,
        Exchange,
    ),
}
impl Default for RuleGet {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..4);
        match random {
            0 => RuleGet::None,
            1 => RuleGet::Steal(Box::default()),
            2 => RuleGet::Gather(Box::default(), Gather::default()),
            _ => RuleGet::Exchange(
                Box::default(),
                Box::default(),
                Box::default(),
                Box::default(),
                Exchange::default(),
            ),
        }
    }
}
impl Display for RuleGet {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RuleGet::None => write!(f, "nothing (You already have it)"),
            RuleGet::Steal(a) => write!(f, "{a}"),
            RuleGet::Gather(a, b) => write!(f, "first {a}, then {b}"),
            RuleGet::Exchange(a, b, c, d, e) => write!(f, "first {a}, then {b}, {c} {d} and {e}"),
        }
    }
}

pub enum RuleSteal {
    Stealth(Box<RuleGoto>, Stealth, Take),
    Kill(Box<RuleGoto>, Box<RuleKill>, Take),
}
impl Default for RuleSteal {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..2);
        match random {
            0 => RuleSteal::Stealth(Box::default(), Stealth::default(), Take::default()),
            _ => RuleSteal::Kill(Box::default(), Box::default(), Take::default()),
        }
    }
}
impl Display for RuleSteal {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RuleSteal::Stealth(a, b, c) => write!(f, "{a} {b},then {c}"),
            RuleSteal::Kill(a, b, c) => write!(f, "{a}, then {b} and {c}"),
        }
    }
}

pub enum RuleSpy {
    Spy(Box<RuleGoto>, Spy, Box<RuleGoto>, Report),
}
impl Default for RuleSpy {
    fn default() -> Self {
        RuleSpy::Spy(
            Box::default(),
            Spy::default(),
            Box::default(),
            Report::default(),
        )
    }
}
impl Display for RuleSpy {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RuleSpy::Spy(a, b, c, d) => write!(f, "{a} {b}, then {c} and {d}"),
        }
    }
}

pub enum RuleCapture {
    Capture(Box<RuleGet>, Box<RuleGoto>, Capture),
}
impl Default for RuleCapture {
    fn default() -> Self {
        RuleCapture::Capture(Box::default(), Box::default(), Capture::default())
    }
}
impl Display for RuleCapture {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RuleCapture::Capture(a, b, c) => write!(f, "{a} then {b} and {c}"),
        }
    }
}

pub enum RuleKill {
    Kill(Box<RuleGoto>, Kill),
}
impl Default for RuleKill {
    fn default() -> Self {
        RuleKill::Kill(Box::default(), Kill::default())
    }
}
impl Display for RuleKill {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            RuleKill::Kill(a, b) => write!(f, "{a} then {b} "),
        }
    }
}

// Main Quest
pub enum Quest {
    Knowledge(Knowledge),
    Comfort(Comfort),
    Reputation(Reputation),
    Serenity(Serenity),
    Protection(Protection),
    Conquest(Conquest),
    Wealth(Wealth),
    Ability(Ability),
    Equipement(Equipement),
}
impl Default for Quest {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..9);
        match random {
            0 => Quest::Knowledge(Knowledge::default()),
            1 => Quest::Comfort(Comfort::default()),
            2 => Quest::Reputation(Reputation::default()),
            3 => Quest::Serenity(Serenity::default()),
            4 => Quest::Protection(Protection::default()),
            5 => Quest::Conquest(Conquest::default()),
            6 => Quest::Wealth(Wealth::default()),
            7 => Quest::Ability(Ability::default()),
            _ => Quest::Equipement(Equipement::default()),
        }
    }
}
impl Display for Quest {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Quest::Knowledge(a) => write!(f, "The quest is {a}"),
            Quest::Comfort(a) => write!(f, "The quest is {a}"),
            Quest::Reputation(a) => write!(f, "The quest is {a}"),
            Quest::Serenity(a) => write!(f, "The quest is {a}"),
            Quest::Protection(a) => write!(f, "The quest is {a}"),
            Quest::Conquest(a) => write!(f, "The quest is {a}"),
            Quest::Wealth(a) => write!(f, "The quest is {a}"),
            Quest::Ability(a) => write!(f, "The quest is {a}"),
            Quest::Equipement(a) => write!(f, "The quest is {a}"),
        }
    }
}

pub enum Knowledge {
    Deliver(Box<RuleGet>, Box<RuleGoto>, Give),
    Spy(Box<RuleSpy>),
    Interview(Box<RuleGoto>, Listen, Box<RuleGoto>, Report),
    Use(Box<RuleGet>, Box<RuleGoto>, Use, Box<RuleGoto>, Give),
}
impl Default for Knowledge {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..4);
        match random {
            0 => Knowledge::Deliver(Box::default(), Box::default(), Give::default()),
            1 => Knowledge::Spy(Box::default()),
            2 => Knowledge::Interview(
                Box::default(),
                Listen::default(),
                Box::default(),
                Report::default(),
            ),
            _ => Knowledge::Use(
                Box::default(),
                Box::default(),
                Use::default(),
                Box::default(),
                Give::default(),
            ),
        }
    }
}
impl Display for Knowledge {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Knowledge::Deliver(a, b, c) => write!(f, "first {a}, then {b} and {c}"),
            Knowledge::Spy(a) => write!(f, "just {a}"),
            Knowledge::Interview(a, b, c, d) => write!(f, "first {a} and {b}, then {c} and {d}"),
            Knowledge::Use(a, b, c, d, e) => write!(f, "first {a}, then {b}, {c} then {d} and {e}"),
        }
    }
}

pub enum Comfort {
    Luxuries(Box<RuleGet>, Box<RuleGoto>, Give),
    Kill(Box<RuleGoto>, Damage, Box<RuleGoto>, Report),
}
impl Default for Comfort {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..2);
        match random {
            0 => Comfort::Luxuries(Box::default(), Box::default(), Give::default()),
            _ => Comfort::Kill(
                Box::default(),
                Damage::default(),
                Box::default(),
                Report::default(),
            ),
        }
    }
}
impl Display for Comfort {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Comfort::Luxuries(a, b, c) => write!(f, "first {a}, then {b} and {c}"),
            Comfort::Kill(a, b, c, d) => write!(f, "{a} and {b} then {c} and {d}"),
        }
    }
}

pub enum Reputation {
    Obtain(Box<RuleGet>, Box<RuleGoto>, Give),
    Kill(Box<RuleGoto>, Box<RuleKill>, Box<RuleGoto>, Report),
    Visit(Box<RuleGoto>, Box<RuleGoto>, Report),
}
impl Default for Reputation {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..4);
        match random {
            0 => Reputation::Obtain(Box::default(), Box::default(), Give::default()),
            1 => Reputation::Kill(
                Box::default(),
                Box::default(),
                Box::default(),
                Report::default(),
            ),
            _ => Reputation::Visit(Box::default(), Box::default(), Report::default()),
        }
    }
}
impl Display for Reputation {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Reputation::Obtain(a, b, c) => write!(f, "first {a}, then {b} and {c}"),
            Reputation::Kill(a, b, c, d) => write!(f, "{a} and {b}, then {c} and {d}"),
            Reputation::Visit(a, b, c) => write!(f, "first {a} then {b}, and {c}"),
        }
    }
}

pub enum Serenity {
    Revenge(Box<RuleGoto>, Damage),
    Capture1(Box<RuleGet>, Box<RuleGoto>, Use, Box<RuleGoto>, Give),
    Capture2(
        Box<RuleGet>,
        Box<RuleGoto>,
        Use,
        Capture,
        Box<RuleGoto>,
        Give,
    ),
    Check1(Box<RuleGoto>, Listen, Box<RuleGoto>, Report),
    Check2(Box<RuleGoto>, Take, Box<RuleGoto>, Give),
    Recover(Box<RuleGet>, Box<RuleGoto>, Give),
    Rescue(Box<RuleGoto>, Damage, Escort, Box<RuleGoto>, Report),
}
impl Default for Serenity {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..7);
        match random {
            0 => Serenity::Revenge(Box::default(), Damage::default()),
            1 => Serenity::Capture1(
                Box::default(),
                Box::default(),
                Use::default(),
                Box::default(),
                Give::default(),
            ),
            2 => Serenity::Capture2(
                Box::default(),
                Box::default(),
                Use::default(),
                Capture::default(),
                Box::default(),
                Give::default(),
            ),
            3 => Serenity::Check1(
                Box::default(),
                Listen::default(),
                Box::default(),
                Report::default(),
            ),
            4 => Serenity::Check2(
                Box::default(),
                Take::default(),
                Box::default(),
                Give::default(),
            ),
            5 => Serenity::Recover(Box::default(), Box::default(), Give::default()),
            _ => Serenity::Rescue(
                Box::default(),
                Damage::default(),
                Escort::default(),
                Box::default(),
                Report::default(),
            ),
        }
    }
}
impl Display for Serenity {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Serenity::Revenge(a, b) => write!(f, "{a} and {b}"),
            Serenity::Capture1(a, b, c, d, e) => {
                write!(f, "fisrt {a} then {b}, {c} after that {d} and {e}")
            }
            Serenity::Capture2(a, b, c, d, e, g) => {
                write!(f, "first {a}, {b} and {c} , {d} then {e} and {g}")
            }
            Serenity::Check1(a, b, c, d) => write!(f, "{a} and {b}, then {c} and {d}"),
            Serenity::Check2(a, b, c, d) => write!(f, "{a} and {b}, then {c} and {d}"),
            Serenity::Recover(a, b, c) => write!(f, "first {a}, {b} and {c}"),
            Serenity::Rescue(a, b, c, d, e) => write!(f, "{a} and {b}, {c}, then {d} and {e}"),
        }
    }
}

pub enum Protection {
    Attack(Box<RuleGoto>, Damage, Box<RuleGoto>, Report),
    Treat1(Box<RuleGet>, Box<RuleGoto>, Use),
    Treat2(Box<RuleGoto>, Repair),
    Diversion1(Box<RuleGet>, Box<RuleGoto>, Use),
    Diversion2(Box<RuleGoto>, Damage),
    Assemble(Box<RuleGoto>, Repair),
    Guard(Box<RuleGoto>, Defend),
}
impl Default for Protection {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..7);
        match random {
            0 => Protection::Attack(
                Box::default(),
                Damage::default(),
                Box::default(),
                Report::default(),
            ),
            1 => Protection::Treat1(Box::default(), Box::default(), Use::default()),
            2 => Protection::Treat2(Box::default(), Repair::default()),
            3 => Protection::Diversion1(Box::default(), Box::default(), Use::default()),
            4 => Protection::Diversion2(Box::default(), Damage::default()),
            5 => Protection::Assemble(Box::default(), Repair::default()),
            _ => Protection::Guard(Box::default(), Defend::default()),
        }
    }
}
impl Display for Protection {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Protection::Attack(a, b, c, d) => write!(f, "{a} and {b}, then {c} and {d}"),
            Protection::Treat1(a, b, c) => write!(f, "fisrt {a}, then {b} and {c}"),
            Protection::Treat2(a, b) => write!(f, "{a} and {b}"),
            Protection::Diversion1(a, b, c) => write!(f, "first {a}, then {b} and {c}"),
            Protection::Diversion2(a, b) => write!(f, "{a} and {b}"),
            Protection::Assemble(a, b) => write!(f, "{a} and {b}"),
            Protection::Guard(a, b) => write!(f, "{a} and {b}"),
        }
    }
}

pub enum Conquest {
    Attack(Box<RuleGoto>, Damage),
    Steal(Box<RuleGoto>, Box<RuleSteal>, Box<RuleGoto>, Give),
}
impl Default for Conquest {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..2);
        match random {
            0 => Conquest::Attack(Box::default(), Damage::default()),
            _ => Conquest::Steal(
                Box::default(),
                Box::default(),
                Box::default(),
                Give::default(),
            ),
        }
    }
}
impl Display for Conquest {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Conquest::Attack(a, b) => write!(f, "{a} and {b}"),
            Conquest::Steal(a, b, c, d) => write!(f, "{a} and {b}, then {c} and {d}"),
        }
    }
}

pub enum Wealth {
    Gather(Box<RuleGoto>, Box<RuleGet>),
    Steal(Box<RuleGoto>, Box<RuleSteal>),
    Repair(Repair),
}
impl Default for Wealth {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..3);
        match random {
            0 => Wealth::Gather(Box::default(), Box::default()),
            1 => Wealth::Steal(Box::default(), Box::default()),
            _ => Wealth::Repair(Repair::default()),
        }
    }
}
impl Display for Wealth {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Wealth::Gather(a, b) => write!(f, "{a} and {b}"),
            Wealth::Steal(a, b) => write!(f, "{a} and {b}"),
            Wealth::Repair(a) => write!(f, "just {a}"),
        }
    }
}

pub enum Ability {
    Assemble(Repair, Use),
    Obtain(Box<RuleGet>, Use),
    Use(Use),
    Practice1(Damage),
    Practice2(Use),
    Research1(Box<RuleGet>, Use),
    Research2(Box<RuleGet>, Experiment),
}
impl Default for Ability {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..7);
        match random {
            0 => Ability::Assemble(Repair::default(), Use::default()),
            1 => Ability::Obtain(Box::default(), Use::default()),
            2 => Ability::Use(Use::default()),
            3 => Ability::Practice1(Damage::default()),
            4 => Ability::Practice2(Use::default()),
            5 => Ability::Research1(Box::default(), Use::default()),
            _ => Ability::Research2(Box::default(), Experiment::default()),
        }
    }
}
impl Display for Ability {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Ability::Assemble(a, b) => write!(f, "{a} and {b}"),
            Ability::Obtain(a, b) => write!(f, "first {a} then {b}"),
            Ability::Use(a) => write!(f, "just {a}"),
            Ability::Practice1(a) => write!(f, "just {a}"),
            Ability::Practice2(a) => write!(f, "just {a}"),
            Ability::Research1(a, b) => write!(f, "you need to {a} and {b}"),
            Ability::Research2(a, b) => write!(f, "yout need to {a} and {b}"),
        }
    }
}

pub enum Equipement {
    Assemble(Repair),
    Deliver(Box<RuleGet>, Box<RuleGoto>, Give),
    Steal(Box<RuleSteal>),
    Trade(Box<RuleGoto>, Exchange),
}
impl Default for Equipement {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..4);
        match random {
            0 => Equipement::Assemble(Repair::default()),
            1 => Equipement::Deliver(Box::default(), Box::default(), Give::default()),
            2 => Equipement::Steal(Box::default()),
            _ => Equipement::Trade(Box::default(), Exchange::default()),
        }
    }
}
impl Display for Equipement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Equipement::Assemble(a) => write!(f, "just {a}"),
            Equipement::Deliver(a, b, c) => write!(f, "first {a}, then {b} and {c}"),
            Equipement::Steal(a) => write!(f, "just {a}"),
            Equipement::Trade(a, b) => write!(f, "{a} and {b}"),
        }
    }
}
